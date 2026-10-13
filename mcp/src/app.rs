use super::mcp::Mcp;
use axum::response::IntoResponse;
use axum::routing;
use rmcp::transport::{
    StreamableHttpServerConfig, StreamableHttpService,
    streamable_http_server::session::local::LocalSessionManager,
};
use tokio_util::sync::CancellationToken;

async fn health_check() -> impl IntoResponse {
    "running".into_response()
}

pub async fn build_app(
    mcp: Mcp,
    cancellation_token: Option<&CancellationToken>,
) -> axum::Router {
    let streamable_http_config = match cancellation_token {
        Some(token) => StreamableHttpServerConfig::default()
            .with_cancellation_token(token.child_token())
            .with_legacy_session_mode(false),
        None => StreamableHttpServerConfig::default()
            .with_legacy_session_mode(false),
    };
    let service = StreamableHttpService::new(
        move || Ok(mcp.to_owned()),
        LocalSessionManager::default().into(),
        streamable_http_config,
    );

    axum::Router::new()
        .nest_service(
            "/mcp", service,
        )
        .route(
            "/health",
            routing::get(health_check),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use httpmock::prelude::*;
    use tower::ServiceExt;

    mod test_health_check {
        use super::*;

        #[tokio::test]
        async fn should_return_running() -> anyhow::Result<()> {
            let server = MockServer::start();
            let mock = server.mock(
                |when, then| {
                    when.any_request();
                    then.status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body("error");
                },
            );
            let client = client::Client::new(
                Some(server.url("test")),
                Some("fetch".to_string()),
                Some("search".to_string()),
            )?;

            let mcp = Mcp::new(client);
            let app = build_app(mcp, None).await;
            let request = Request::get("/health").body(Body::empty())?;
            let response = app
                .oneshot(request)
                .await?;
            mock.assert_calls(0);

            assert_eq!(
                response.status(),
                StatusCode::OK
            );

            let body = response
                .into_body()
                .collect()
                .await?
                .to_bytes();

            assert_eq!(
                &body[..],
                b"running"
            );

            Ok(())
        }
    }

    mod test_list_tools {
        use super::*;

        #[tokio::test]
        async fn should_include_desired_tools() -> anyhow::Result<()> {
            let server = MockServer::start();
            let mock = server.mock(
                |when, then| {
                    when.any_request();
                    then.status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body("error");
                },
            );
            let client = client::Client::new(
                Some(server.url("test")),
                Some("fetch".to_string()),
                Some("search".to_string()),
            )?;
            let mcp = Mcp::new(client);
            let app = build_app(mcp, None).await;

            let payload = serde_json::json!(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "tools/list",
                    "params": {}
                }
            );
            let request = Request::post("/mcp")
                .header(
                    "host",
                    "localhost",
                )
                .header(
                    "accept",
                    "application/json, text/event-stream",
                )
                .header(
                    "content-type",
                    "application/json",
                )
                .body(Body::from(serde_json::to_vec(&payload)?))?;
            let response = app
                .oneshot(request)
                .await?;

            assert_eq!(
                response.status(),
                StatusCode::OK
            );
            mock.assert_calls(0);

            let bytes = response
                .into_body()
                .collect()
                .await?
                .to_bytes()
                .to_vec();
            let text = String::from_utf8(bytes)?;
            let rpc_response: serde_json::Value = text
                .lines()
                .filter_map(|line| line.strip_prefix("data:"))
                .filter_map(|data| serde_json::from_str(data.trim()).ok())
                .find(|msg: &serde_json::Value| msg["id"] == 1)
                .ok_or_else(|| {
                    anyhow::anyhow!("Invalid JSON-RPC response: {text}")
                })?;

            let expected = serde_json::json!(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "tools": [
                            {
                                "name": "PubMed Search",
                                "description": "Searches PubMed using an Essie Syntax Expression, and returns a list of abstracts",
                                "inputSchema": {
                                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                                    "properties": {
                                        "expression": { "type": "string"},
                                        "retstart": {
                                            "format": "uint32",
                                            "minimum": 0,
                                            "type": "integer"
                                        }
                                    },
                                    "required": ["expression"],
                                    "type": "object"
                                }

                            }
                        ]
                    }
                }
            );
            assert_eq!(
                rpc_response,
                expected
            );

            Ok(())
        }
    }

    mod test_pubmed_search {
        use super::*;

        #[tokio::test]
        async fn should_return_markdown() -> anyhow::Result<()> {
            let server = MockServer::start();
            let expression = "cancer";
            let search_body = r#"
                <?xml version="1.0" encoding="UTF-8" ?>
                <!DOCTYPE eSearchResult PUBLIC "-//NLM//DTD esearch 20060628//EN" "https://eutils.ncbi.nlm.nih.gov/eutils/dtd/20060628/esearch.dtd">
                <eSearchResult>
                    <Count>1</Count>
                    <RetMax>20</RetMax>
                    <RetStart>0</RetStart>
                    <IdList>
                        <Id>123</Id>
                    </IdList>
                    <TranslationSet/>
                    <QueryTranslation>"cancer"[All Fields]</QueryTranslation>
                </eSearchResult>
            "#;
            let fetch_body = r#"
            <?xml version="1.0" encoding="UTF-8" ?>
            <!DOCTYPE eSearchResult PUBLIC "-//NLM//DTD esearch 20060628//EN" "https://eutils.ncbi.nlm.nih.gov/eutils/dtd/20060628/esearch.dtd">
            <PubmedArticleSet>
                <PubmedArticle>
                    <MedlineCitation Status="MEDLINE" Owner="NLM" IndexingMethod="Manual">
                        <PMID Version="1">123</PMID>
                        <DateCompleted>
                            <Year>2020</Year>
                            <Month>01</Month>
                            <Day>04</Day>
                        </DateCompleted>
                        <DateRevised>
                            <Year>
                                2026
                            </Year>
                            <Month>
                                05
                            </Month>
                            <Day>
                                02
                            </Day>
                        </DateRevised>
                        <Article PubModel="Print-Electronic">
                            <Journal>
                                <ISSN IssnType="Electronic">1234-5678</ISSN>
                                <JournalIssue CitedMedium="Internet">
                                    <Volume>123</Volume>
                                    <Issue>576</Issue>
                                    <PubDate>
                                        <Year>2020</Year>
                                        <Month>01</Month>
                                        <Day>04</Day>
                                    </PubDate>
                                </JournalIssue>
                                <Title>Test</Title>
                                <ISOAbbreviation>Science</ISOAbbreviation>
                            </Journal>
                            <ArticleTitle>
                                Test
                            </ArticleTitle>
                            <Pagination>
                                <StartPage>1</StartPage>
                                <EndPage>2</EndPage>
                                <MedlinePgn>7</MedlinePgn>
                            </Pagination>
                            <ELocationID EIdType="doi" ValidYN="Y">10.0001/science.0000001</ELocationID>
                            <Abstract>
                                <AbstractText>
                                    Test
                                </AbstractText>
                            </Abstract>
                            <AuthorList CompleteYN="Y">
                                <Author ValidYN="Y">
                                    <LastName>Smith</LastName>
                                    <ForeName>John</ForeName>
                                    <Initials>J</Initials>
                                    <AffiliationInfo>
                                        <Affiliation>
                                            Example, Inc.
                                        </Affiliation>
                                    </AffiliationInfo>
                                </Author>
                            </AuthorList>
                            <Language>eng</Language>
                            <DataBankList CompleteYN="Y">
                                <DataBank>
                                    <DataBankName>GEO</DataBankName>
                                    <AccessionNumberList>
                                        <AccessionNumber>GSE1234</AccessionNumber>
                                    </AccessionNumberList>
                                </DataBank>
                            </DataBankList>
                            <GrantList CompleteYN="Y">
                                <Grant>
                                    <GrantID>IOU101</GrantID>
                                    <Agency>Example, Inc.</Agency>
                                    <Acronym>Evil Corp</Acronym>
                                    <Country>United States</Country>
                                </Grant>
                            </GrantList>
                            <PublicationTypeList>
                                <PublicationType UI="1234">Journal example</PublicationType>
                            </PublicationTypeList>
                            <ArticleDate DateType="Electronic">
                                <Year>2020</Year>
                                <Month>01</Month>
                                <Day>04</Day>
                            </ArticleDate>
                        </Article>
                        <MedlineJournalInfo>
                            <Country>United States</Country>
                            <MedlineTA>Science</MedlineTA>
                            <NlmUniqueID>0000001</NlmUniqueID>
                            <ISSNLinking>0001-0001</ISSNLinking>
                        </MedlineJournalInfo>
                        <ChemicalList>
                            <Chemical>
                                <RegistryNumber>0</RegistryNumber>
                                <NameOfSubstance UI="0001">Region</NameOfSubstance>
                            </Chemical>
                        </ChemicalList>
                        <CitationSubSet>IM</CitationSubSet>
                        <MeshHeadingList>
                            <MeshHeading>
                                <DescriptorName UI="D000001" MajorTopic="N">Algorithms</DescriptorName>
                            </MeshHeading>
                        </MeshHeadingList>
                    </MedlineCitation>
                    <PubmedData>
                        <History>
                            <PubMedPubDate PubStatus="entrez">
                                <Year>2020</Year>
                                <Month>01</Month>
                                <Day>04</Day>
                                <Hour>8</Hour>
                                <Minute>0</Minute>
                            </PubMedPubDate>
                        </History>
                        <PublicationStatus>ppublish</PublicationStatus>
                        <ArticleIdList>
                            <ArticleId IdType="pubmed">00000001</ArticleId>
                        </ArticleIdList>
                        <ReferenceList>
                            <Reference>
                                <Citation>Evil Corp Page 7</Citation>
                            </Reference>
                        </ReferenceList>
                    </PubmedData>
                </PubmedArticle>
            </PubmedArticleSet>
            "#;

            let search_mock = server.mock(
                |when, then| {
                    when.method(GET)
                        .path("/search")
                        .query_param(
                            "db", "pubmed",
                        )
                        .query_param(
                            "term", expression,
                        )
                        .query_param(
                            "retmax", "5",
                        );
                    then.status(200)
                        .body(search_body);
                },
            );
            let fetch_mock = server.mock(
                |when, then| {
                    when.method(GET)
                        .path("/fetch")
                        .query_param(
                            "db", "pubmed",
                        )
                        .query_param(
                            "id", "123",
                        );

                    then.status(200)
                        .body(fetch_body);
                },
            );

            let client = client::Client::new(
                Some(server.base_url()),
                Some("fetch".to_string()),
                Some("search".to_string()),
            )?;
            let mcp = Mcp::new(client);
            let app = build_app(mcp, None).await;
            let payload = serde_json::json!(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "tools/call",
                    "params": {
                        "name": "PubMed Search",
                        "arguments": {
                            "expression": expression,
                            "retstart": null
                        }
                    },
                }
            );
            let request = Request::post("/mcp")
                .header(
                    "host",
                    "localhost",
                )
                .header(
                    "accept",
                    "application/json, text/event-stream",
                )
                .header(
                    "content-type",
                    "application/json",
                )
                .body(Body::from(serde_json::to_vec(&payload)?))?;
            let response = app
                .oneshot(request)
                .await?;

            assert_eq!(
                response.status(),
                StatusCode::OK,
            );

            let bytes = response
                .into_body()
                .collect()
                .await?
                .to_bytes()
                .to_vec();
            let text = String::from_utf8(bytes)?;
            let rcp_response: serde_json::Value = text
                .lines()
                .filter_map(|line| line.strip_prefix("data:"))
                .filter_map(|data| serde_json::from_str(data.trim()).ok())
                .find(|msg: &serde_json::Value| msg["id"] == 1)
                .ok_or_else(|| {
                    anyhow::anyhow!("Invalid JSON-RPC response: {text}")
                })?;
            search_mock.assert();
            fetch_mock.assert();

            let search_response = serde_json::json!(
                {
                    "content": format!(
                        "# PMID: 123\n\n\n{}Test\n{}",
                        " ".repeat(36),
                        " ".repeat(32),
                    ),
                    "ret_start": 0,
                    "ret_max": 20
                }
            );
            let expected = serde_json::json!(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "content": [
                            {
                                "type": "text",
                                "text": search_response.to_string()
                            }
                        ],
                        "structuredContent": search_response,
                        "isError": false
                    }
                }
            );

            assert_eq!(
                rcp_response,
                expected
            );

            Ok(())
        }
    }
}
