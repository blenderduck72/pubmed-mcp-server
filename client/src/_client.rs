use super::constants;
use super::errors;
use super::models;
use reqwest::Client as ReqwestClient;
use reqwest::header;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
enum RetMod {
    Json,
    Xml,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Db {
    PubMed,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct FetchParams {
    id: String,
    db: Db,
    retmod: RetMod,
}

impl FetchParams {
    fn new(ids: Vec<String>) -> Self {
        Self {
            id: ids.join(","),
            db: Db::PubMed,
            retmod: RetMod::Xml,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct SearchParams {
    db: Db,
    term: String,
    retstart: Option<u32>,
    retmax: Option<u32>,
}
impl SearchParams {
    pub fn new(
        db: Db,
        term: impl Into<String>,
        retstart: Option<u32>,
        retmax: Option<u32>,
    ) -> Self {
        Self {
            db,
            term: term.into(),
            retstart,
            retmax,
        }
    }
}

#[derive(Clone)]
pub struct Client {
    client: ReqwestClient,
    base_url: String,
    fetch_path: String,
    search_path: String,
}
impl Client {
    pub fn new(
        base_url: Option<String>,
        fetch_path: Option<String>,
        search_path: Option<String>,
    ) -> anyhow::Result<Self> {
        let mut headers_map = HeaderMap::new();
        headers_map.insert(
            header::ACCEPT,
            "application/json"
                .parse()
                .map_err(|_e| errors::ClientError::InitiationHeader)?,
        );
        let reqwest_client = ReqwestClient::builder()
            .default_headers(headers_map)
            .build()
            .map_err(errors::ClientError::InitiationReqwestClient)?;

        Ok(Self {
            client: reqwest_client,
            base_url: base_url.unwrap_or(constants::BASE_URL.into()),
            fetch_path: fetch_path.unwrap_or(constants::FETCH_PATH.into()),
            search_path: search_path.unwrap_or(constants::SEARCH_PATH.into()),
        })
    }

    fn search_url(&self) -> String {
        format!(
            "{base_url}/{search_path}",
            base_url = self.base_url,
            search_path = self.search_path
        )
    }

    fn fetch_url(&self) -> String {
        format!(
            "{base_url}/{search_path}",
            base_url = self.base_url,
            search_path = self.fetch_path
        )
    }

    async fn parse_e_search_result(
        xml: &str
    ) -> Result<models::search::ESearchResult, errors::ClientError> {
        let result = quick_xml::de::from_str(xml)
            .map_err(errors::ClientError::ParseSearchXml)?;

        Ok(result)
    }

    pub async fn parse_e_fetch_result(
        xml: &str
    ) -> Result<models::fetch::PubMedArticleSet, errors::ClientError> {
        let result = quick_xml::de::from_str(xml)
            .map_err(errors::ClientError::ParseFetchXml)?;

        Ok(result)
    }

    pub async fn search(
        &self,
        expression: &str,
        retstart: Option<u32>,
        retmax: Option<u32>,
    ) -> Result<models::search::ESearchResult, errors::ClientError> {
        let search_parms = SearchParams::new(
            Db::PubMed,
            expression,
            retstart,
            retmax,
        );
        let url = self.search_url();
        println!("{}", url);
        let result = self
            .client
            .get(url)
            .query(&search_parms)
            .send()
            .await
            .map_err(errors::ClientError::SearchReqwest)?
            .error_for_status()
            .map_err(errors::ClientError::SearchReqwest)?
            .text()
            .await
            .map_err(errors::ClientError::SearchSerialization)?;

        let search_result = Self::parse_e_search_result(&result).await?;

        Ok(search_result)
    }

    pub async fn fetch(
        &self,
        ids: &[String],
    ) -> Result<models::fetch::PubMedArticleSet, errors::ClientError> {
        let fetch_params = FetchParams::new(ids.to_owned());
        let result = self
            .client
            .get(self.fetch_url())
            .query(&fetch_params)
            .send()
            .await
            .map_err(errors::ClientError::FetchReqwest)?
            .error_for_status()
            .map_err(errors::ClientError::FetchReqwest)?
            .text()
            .await
            .map_err(errors::ClientError::FetchSerialization)?;

        let fetch_result = Self::parse_e_fetch_result(&result).await?;

        Ok(fetch_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    mod test_new {
        use super::*;

        #[tokio::test]
        async fn test_should_use_contants() -> anyhow::Result<()> {
            let client = Client::new(
                None, None, None,
            )?;

            assert_eq!(
                client.base_url,
                constants::BASE_URL,
            );
            assert_eq!(
                client.fetch_path,
                constants::FETCH_PATH
            );
            assert_eq!(
                client.search_path,
                constants::SEARCH_PATH
            );

            Ok(())
        }

        #[tokio::test]
        async fn test_should_set_default_headers() -> anyhow::Result<()> {
            let server = MockServer::start();
            let mock = server.mock(
                |when, then| {
                    when.method(GET)
                        .path("/test")
                        .header(
                            "accept",
                            "application/json",
                        );
                    then.status(200)
                        .body("hello");
                },
            );
            let client = Client::new(
                Some(server.url("/test")),
                Some("fetch".to_string()),
                Some("search".to_string()),
            )?;
            client
                .client
                .get(server.url("/test"))
                .send()
                .await?;
            mock.assert();

            Ok(())
        }
    }

    mod test_search_url {
        use super::*;

        #[tokio::test]
        async fn test_should_form_search_correctly() -> anyhow::Result<()> {
            let base_url = "https://foo".to_string();
            let fetch_path = "fetch".to_string();
            let search_path = "search".to_string();

            let expected_url = format!("{base_url}/{search_path}");

            let client = Client::new(
                Some(base_url),
                Some(fetch_path),
                Some(search_path),
            )?;

            assert_eq!(
                client.search_url(),
                expected_url
            );
            Ok(())
        }
    }

    mod test_fetch_url {
        use super::*;

        #[tokio::test]
        async fn test_should_form_fetch_url_correctly() -> anyhow::Result<()> {
            let base_url = "https://foo".to_string();
            let fetch_path = "fetch".to_string();
            let search_path = "search".to_string();

            let expected_url = format!("{base_url}/{fetch_path}");

            let client = Client::new(
                Some(base_url),
                Some(fetch_path),
                Some(search_path),
            )?;

            assert_eq!(
                client.fetch_url(),
                expected_url
            );
            Ok(())
        }
    }

    mod test_search {
        use super::*;

        #[tokio::test]
        async fn test_should_return_e_search_result() -> anyhow::Result<()> {
            let server = MockServer::start();
            let expression = "cancer";
            let fetch_path = "fetch".to_string();
            let search_path = "search".to_string();
            let response_body = r#"
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
            let expected_e_search_result =
                models::search::ESearchResultBuilder::default()
                    .count(1)
                    .ret_max(20)
                    .ret_start(0)
                    .id_list(
                        models::search::IdListBuilder::default()
                            .ids(vec![
                                models::search::IdBuilder::default()
                                    .value(Some(
                                        "123".to_string(),
                                    ))
                                    .build()?,
                            ])
                            .build()?,
                    )
                    .query_translation(
                        models::search::QueryTranslationBuilder::default()
                            .value(Some(
                                r#""cancer"[All Fields]"#.to_string(),
                            ))
                            .build()?,
                    )
                    .translation_set(
                        models::search::TranslationSetBuilder::default()
                            .translations(None)
                            .build()?,
                    )
                    .build()?;

            let mock = server.mock(
                |when, then| {
                    when.method(GET)
                        .path("/search")
                        .query_param(
                            "db", "pubmed",
                        )
                        .query_param(
                            "term", expression,
                        );
                    then.status(200)
                        .body(response_body);
                },
            );

            let client = Client::new(
                Some(server.base_url()),
                Some(fetch_path),
                Some(search_path),
            )?;
            let e_search_result = client
                .search(
                    expression, None, None,
                )
                .await?;
            mock.assert();
            assert_eq!(
                expected_e_search_result,
                e_search_result
            );

            Ok(())
        }

        #[tokio::test]
        async fn test_should_handle_retstart() -> anyhow::Result<()> {
            let server = MockServer::start();
            let expression = "cancer";
            let fetch_path = "fetch".to_string();
            let search_path = "search".to_string();
            let response_body = r#"
                <?xml version="1.0" encoding="UTF-8" ?>
                <!DOCTYPE eSearchResult PUBLIC "-//NLM//DTD esearch 20060628//EN" "https://eutils.ncbi.nlm.nih.gov/eutils/dtd/20060628/esearch.dtd">
                <eSearchResult>
                    <Count>1</Count>
                    <RetMax>20</RetMax>
                    <RetStart>10</RetStart>
                    <IdList>
                        <Id>123</Id>
                    </IdList>
                    <TranslationSet/>
                    <QueryTranslation>"cancer"[All Fields]</QueryTranslation>
                </eSearchResult>
            "#;
            let expected_e_search_result =
                models::search::ESearchResultBuilder::default()
                    .count(1)
                    .ret_max(20)
                    .ret_start(10)
                    .id_list(
                        models::search::IdListBuilder::default()
                            .ids(vec![
                                models::search::IdBuilder::default()
                                    .value(Some(
                                        "123".to_string(),
                                    ))
                                    .build()?,
                            ])
                            .build()?,
                    )
                    .query_translation(
                        models::search::QueryTranslationBuilder::default()
                            .value(Some(
                                r#""cancer"[All Fields]"#.to_string(),
                            ))
                            .build()?,
                    )
                    .translation_set(
                        models::search::TranslationSetBuilder::default()
                            .translations(None)
                            .build()?,
                    )
                    .build()?;

            let mock = server.mock(
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
                            "retstart", "10",
                        );
                    then.status(200)
                        .body(response_body);
                },
            );

            let client = Client::new(
                Some(server.base_url()),
                Some(fetch_path),
                Some(search_path),
            )?;
            let e_search_result = client
                .search(
                    expression,
                    Some(10),
                    None,
                )
                .await?;
            println!(
                "{:#?}",
                e_search_result
            );
            mock.assert();
            assert_eq!(
                expected_e_search_result,
                e_search_result
            );

            Ok(())
        }

        #[tokio::test]
        async fn test_should_handle_retmax() -> anyhow::Result<()> {
            let server = MockServer::start();
            let expression = "cancer";
            let fetch_path = "fetch".to_string();
            let search_path = "search".to_string();
            let response_body = r#"
                <?xml version="1.0" encoding="UTF-8" ?>
                <!DOCTYPE eSearchResult PUBLIC "-//NLM//DTD esearch 20060628//EN" "https://eutils.ncbi.nlm.nih.gov/eutils/dtd/20060628/esearch.dtd">
                <eSearchResult>
                    <Count>1</Count>
                    <RetMax>15</RetMax>
                    <RetStart>10</RetStart>
                    <IdList>
                        <Id>123</Id>
                    </IdList>
                    <TranslationSet/>
                    <QueryTranslation>"cancer"[All Fields]</QueryTranslation>
                </eSearchResult>
            "#;
            let expected_e_search_result =
                models::search::ESearchResultBuilder::default()
                    .count(1)
                    .ret_max(20)
                    .ret_start(10)
                    .ret_max(15)
                    .id_list(
                        models::search::IdListBuilder::default()
                            .ids(vec![
                                models::search::IdBuilder::default()
                                    .value(Some(
                                        "123".to_string(),
                                    ))
                                    .build()?,
                            ])
                            .build()?,
                    )
                    .query_translation(
                        models::search::QueryTranslationBuilder::default()
                            .value(Some(
                                r#""cancer"[All Fields]"#.to_string(),
                            ))
                            .build()?,
                    )
                    .translation_set(
                        models::search::TranslationSetBuilder::default()
                            .translations(None)
                            .build()?,
                    )
                    .build()?;

            let mock = server.mock(
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
                            "retstart", "10",
                        )
                        .query_param(
                            "retmax", "15",
                        );
                    then.status(200)
                        .body(response_body);
                },
            );

            let client = Client::new(
                Some(server.base_url()),
                Some(fetch_path),
                Some(search_path),
            )?;
            let e_search_result = client
                .search(
                    expression,
                    Some(10),
                    Some(15),
                )
                .await?;
            mock.assert();
            assert_eq!(
                expected_e_search_result,
                e_search_result
            );

            Ok(())
        }

        #[tokio::test]
        async fn test_should_handle_search_reqwest_serialization_error()
        -> anyhow::Result<()> {
            let server = MockServer::start();
            let expression = "cancer";
            let fetch_path = "fetch".to_string();
            let search_path = "search".to_string();
            let response_body = r#"
                bad response
            "#;
            let mock = server.mock(
                |when, then| {
                    when.method(GET)
                        .path("/search")
                        .query_param(
                            "db", "pubmed",
                        )
                        .query_param(
                            "term", expression,
                        );
                    then.status(200)
                        .body(response_body);
                },
            );

            let client = Client::new(
                Some(server.base_url()),
                Some(fetch_path),
                Some(search_path),
            )?;
            let request_result = client
                .search(
                    expression, None, None,
                )
                .await;
            mock.assert();

            match request_result {
                Ok(_) => Err(anyhow::anyhow!("nope")),
                Err(errors::ClientError::ParseSearchXml(_)) => Ok(()),
                _ => Err(anyhow::anyhow!("nope")),
            }
        }

        #[tokio::test]
        async fn test_should_handle_bad_request_error() -> anyhow::Result<()> {
            let server = MockServer::start();
            let expression = "cancer";
            let fetch_path = "fetch".to_string();
            let search_path = "search".to_string();

            let mock = server.mock(
                |when, then| {
                    when.any_request();
                    then.status(500)
                        .body("Internal Server Error");
                },
            );
            let client = Client::new(
                Some(server.base_url()),
                Some(fetch_path),
                Some(search_path),
            )?;
            let result = client
                .search(
                    expression, None, None,
                )
                .await;
            mock.assert();
            match result {
                Err(errors::ClientError::SearchReqwest(_)) => Ok(()),
                _ => Err(anyhow::anyhow!("Unexpected Error or Ok status")),
            }
        }
    }

    mod test_fetch {
        use super::*;
        use reqwest::StatusCode;

        use super::Client;
        use super::models;

        async fn pub_med_article_set_fixture() -> anyhow::Result<(
            String,
            models::fetch::PubMedArticleSet,
        )> {
            let fetch_body = r#"
            <PubmedArticleSet>
                <PubmedArticle>
                    <MedlineCitation Status="MEDLINE" Owner="NLM" IndexingMethod="Manual">
                        <PMID Version="1">1234</PMID>
                        <DateCompleted>
                            <Year>2020</Year>
                            <Month>01</Month>
                            <Day>04</Day>
                        </DateCompleted>
                        <DateRevised>
                            <Year>2026</Year>
                            <Month>05</Month>
                            <Day>02</Day>
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
                            <ArticleTitle>Test</ArticleTitle>
                            <Pagination>
                                <StartPage>1</StartPage>
                                <EndPage>2</EndPage>
                                <MedlinePgn>7</MedlinePgn>
                            </Pagination>
                            <ELocationID EIdType="doi" ValidYN="Y">10.0001/science.0000001</ELocationID>
                            <Abstract>
                                <AbstractText>Test</AbstractText>
                            </Abstract>
                            <AuthorList CompleteYN="Y">
                                <Author ValidYN="Y">
                                    <LastName>Smith</LastName>
                                    <ForeName>John</ForeName>
                                    <Initials>J</Initials>
                                    <AffiliationInfo>
                                        <Affiliation>Example, Inc.</Affiliation>
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
                        <CitationSubset>IM</CitationSubset>
                        <MeshHeadingList>
                            <MeshHeading>
                                <DescriptorName UI="D000001" MajorTopicYN="N">Algorithms</DescriptorName>
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
            let article = models::fetch::PubMedArticleBuilder::default()
                .medline_citation(
                    Some(
                    models::fetch::MedlineCitationBuilder::default()
                        .pmid(
                            models::fetch::PmidBuilder::default()
                                .value(Some(
                                    "1234".into(),
                                ))
                                .version(Some("1".into()))
                                .build()?,
                        )
                        .date_created(None)
                        .date_completed(
                            Some(
                                models::fetch::YearMonthDayDateBuilder::default()
                                    .year(Some("2020".into()))
                                    .month(Some("01".into()))
                                    .day(Some("04".into()))
                                    .build()?
                                )
                        )
                        .date_revised(
                            Some(
                                models::fetch::YearMonthDayDateBuilder::default()
                                    .year(Some("2026".into()))
                                    .month(Some("05".into()))
                                    .day(Some("02".into()))
                                    .build()?
                                )
                        )
                        .article(
                            Some(
                                models::fetch::ArticleBuilder::default()
                                    .pub_model(Some(
                                        models::constants::PubModel::PrintElectronic,
                                    ))
                                    .journal(Some(
                                        models::fetch::JournalBuilder::default()
                                            .issn(Some(
                                                models::fetch::IssnBuilder::default()
                                                    .issn_type(Some(models::constants::IssnType::Electronic))
                                                    .value(Some(
                                                        "1234-5678".into(),
                                                    ))
                                                    .build()?,
                                            ))
                                            .journal_issue(
                                                Some(
                                                    models::fetch::JournalIssueBuilder::default()
                                                        .volume(Some("123".into()))
                                                        .issue(Some("576".into()))
                                                        .pub_date(
                                                            Some(
                                                                models::fetch::YearMonthDayDateBuilder::default()
                                                                .year(Some("2020".into()))
                                                                .month(Some("01".into()))
                                                                .day(Some("04".into()))
                                                                .build()?
                                                            )
                                                        )
                                                        .cited_medium(Some(models::constants::CitedMedium::Internet))
                                                        .build()?
                                                )
                                            )
                                            .title(Some("Test".into()))
                                            .iso_abbreviation(Some("Science".into()))
                                            .build()?,
                                    ))
                                    .article_title(
                                        Some(
                                            models::fetch::MarkdownElementBuilder::default()
                                                .value(Some(models::fetch::MarkdownText("Test".into())))
                                                .build()?
                                        )
                                    )
                                    .pagination(
                                       Some(
                                         models::fetch::PaginationBuilder::default()
                                            .start_page(Some("1".into()))
                                            .end_page(Some("2".into()))
                                            .medline_pgn(Some("7".into()))
                                            .build()?
                                       )
                                    )
                                    .elocation_ids(
                                        Some(vec![
                                            models::fetch::ElocationIdBuilder::default()
                                                .eld_type(Some(models::constants::ElocationIdType::DigitialObjectIdentifier))
                                                .valid_y_n(Some(models::constants::ValidYesNo::Y))
                                                .value(Some("10.0001/science.0000001".into()))
                                                .build()?
                                        ])
                                    )
                                    .r#abstract(
                                        Some(
                                            models::fetch::AbstractBuilder::default()
                                                .abstract_texts(
                                                    Some(vec![
                                                        models::fetch::AbstractTextBuilder::default()
                                                            .label(None)
                                                            .nlm_category(None)
                                                            .value(Some(models::fetch::MarkdownText("Test".into())))
                                                            .build()?
                                                    ])
                                        )
                                        .copyright_information(None)
                                        .build()?)
                                    )
                                    .author_list(
                                        Some(
                                            models::fetch::AuthorListBuilder::default()
                                                .complete_y_n(Some(models::constants::ValidYesNo::Y))
                                                .authors(
                                                    vec![
                                                        models::fetch::AuthorBuilder::default()
                                                            .valid_y_n(Some(models::constants::ValidYesNo::Y))
                                                            .last_name(Some("Smith".into()))
                                                            .fore_name(Some("John".into()))
                                                            .initials(Some("J".into()))
                                                            .affiliation_infos(
                                                                Some(vec![
                                                                    models::fetch::AffiliationInfoBuilder::default()
                                                                        .affiliations(vec![
                                                                            models::fetch::AffiliationBuilder::default()
                                                                                .value(Some("Example, Inc.".into())).build()?
                                                                        ])
                                                                        .build()?
                                                                ])
                                                            )
                                                            .build()?
                                                    ]
                                                )
                                                .build()?
                                        )
                                    )
                                    .language(
                                        Some(models::fetch::LanguageBuilder::default().value(Some(
                                            models::constants::LanguageOption::English
                                        ))
                                        .build()?)
                                    )
                                    .data_bank_list(
                                        Some(
                                            models::fetch::DataBankListBuilder::default()
                                                .complete_y_n(Some(models::constants::ValidYesNo::Y))
                                                .databanks(
                                                    vec![
                                                        models::fetch::DataBankBuilder::default()
                                                            .data_bank_name(Some("GEO".into()))
                                                            .accession_number_list(
                                                                Some(
                                                                    models::fetch::AccessionNumberListBuilder::default()
                                                                        .accession_numbers(vec![
                                                                            models::fetch::AccessionNumberBuilder::default()
                                                                                .value(Some("GSE1234".into()))
                                                                                .build()?
                                                                        ])
                                                                        .build()?
                                                                )
                                                            )
                                                            .build()?
                                                    ]
                                                )
                                                .build()?
                                        )
                                    )
                                    .grant_list(
                                        Some(
                                            models::fetch::GrantListBuilder::default()
                                                .complete_y_n(Some(models::constants::ValidYesNo::Y))
                                                .grants(
                                                    vec![
                                                        models::fetch::GrantBuilder::default()
                                                            .id(Some("IOU101".into()))
                                                            .agency(Some("Example, Inc.".into()))
                                                            .acronym(Some("Evil Corp".into()))
                                                            .country(Some("United States".into()))
                                                            .build()?
                                                    ]
                                                )
                                                .build()?
                                        )

                                    )
                                    .publication_list_type(
                                        Some(
                                            models::fetch::PublicationTypeListBuilder::default()
                                                .publication_types(
                                                    vec![
                                                        models::fetch::PublicationTypeBuilder::default()
                                                            .ui(Some("1234".into()))
                                                            .value(Some("Journal example".into()))
                                                            .build()?
                                                    ]
                                                )
                                                .build()?
                                        )
                                    )
                                    .article_date(
                                        Some(models::fetch::YearMonthDayDateBuilder::default()
                                            .year(Some("2020".into()))
                                            .month(Some("01".into()))
                                            .day(Some("04".into()))
                                            .build()?)
                                    )
                                    .vernacular_title(None)
                                    .build()?
                            )
                        )
                        .medline_journal_info(
                            Some(
                                models::fetch::MedlineJournalInfoBuilder::default()
                                    .country(Some("United States".into()))
                                    .medline_ta(Some("Science".into()))
                                    .nlm_unique_id(Some("0000001".into()))
                                    .issn_linking(Some("0001-0001".into()))
                                    .build()?
                            )
                        )
                        .chemical_list(
                            Some(
                                models::fetch::ChemicalListBuilder::default()
                                    .chemicals(
                                        vec![
                                            models::fetch::ChemicalBuilder::default()
                                                .registry_number(Some("0".into()))
                                                .name_of_substance(
                                                    Some(
                                                        models::fetch::NameOfSubstanceBuilder::default()
                                                        .ui(Some("0001".into()))
                                                        .value(Some("Region".into()))
                                                        .build()?
                                                    )
                                                )
                                                .build()?
                                        ]
                                    )
                                    .build()?
                            )
                        )
                        .citation_subset(
                            Some(models::constants::CitationSubSet::IM)
                        )
                        .mesh_heading_list(
                            Some(
                                models::fetch::MeshHeadingListBuilder::default()
                                    .mesh_headings(
                                        vec![
                                            models::fetch::MeshHeadingBuilder::default()
                                                .descriptor_name(
                                                    Some(
                                                        models::fetch::DescriptorNameBuilder::default()
                                                            .ui(Some("D000001".into()))
                                                            .major_topic_y_n(Some(models::constants::ValidYesNo::N))
                                                            .value(Some("Algorithms".into()))
                                                            .build()?
                                                    )
                                                )
                                                .qualifer_names(None)
                                                .build()?
                                        ]
                                    )
                                    .build()?
                            )
                        )
                        .suppl_mesh_list(None)
                        .comments_corrections_list(None)
                        .gene_symbol_list(None)
                        .number_of_references(None)
                        .personal_name_subject_list(None)
                        .other_id(None)
                        .other_abstract(None)
                        .keyword_list(None)
                        .space_flight_mission(None)
                        .coi_statement(None)
                        .build()?,
                ))
                .pubmed_data(
                    Some(
                        models::fetch::PubMedDataBuilder::default()
                            .history(
                                Some(
                                    models::fetch::HistoryBuilder::default()
                                        .pub_med_pub_dates(
                                            Some(
                                                vec![
                                                    models::fetch::PubMedDateBuilder::default()
                                                        .year(Some("2020".into()))
                                                        .month(Some("01".into()))
                                                        .day(Some("04".into()))
                                                        .hour(Some("8".into()))
                                                        .minute(Some("0".into()))
                                                        .pub_status(Some(models::constants::PubStatus::Entrez))
                                                        .build()?
                                                ]
                                            )
                                        )
                                        .build()?
                                )
                            )
                            .publication_status(Some(models::constants::PubStatus::Ppublish))
                            .article_id_list(
                                Some(
                                    models::fetch::ArticleIdListBuilder::default()
                                        .article_ids(
                                            Some(
                                                vec![
                                                    models::fetch::ArticleIdBuilder::default()
                                                        .id_type(Some(models::constants::ArticleIdType::Pubmed))
                                                        .value(Some("00000001".into()))
                                                        .build()?
                                                ]
                                            )
                                        )
                                        .build()?
                                )
                            )
                            .reference_lists(
                                Some(
                                    vec![
                                        models::fetch::ReferenceListBuilder::default()
                                            .references(
                                                Some(vec![
                                                    models::fetch::ReferenceBuilder::default()
                                                        .citation(
                                                            Some(
                                                                models::fetch::MarkdownElementBuilder::default()
                                                                    .value(
                                                                        Some(models::fetch::MarkdownText("Evil Corp Page 7".into()))
                                                                    )
                                                                    .build()?
                                                            )
                                                        )
                                                        .article_id_list(None)
                                                        .build()?
                                                ])
                                            )
                                            .build()?
                                    ]
                                )
                            )
                            .build()?
                    )
                )
                .build()?;
            let expected_fetch_result =
                models::fetch::PubMedArticleSetBuilder::default()
                    .articles(vec![
                        article,
                    ])
                    .build()?;

            Ok((
                fetch_body.into(),
                expected_fetch_result,
            ))
        }

        #[tokio::test]
        async fn test_should_return_fetch_results() -> anyhow::Result<()> {
            let (xml, expected_pub_med_article_set) =
                pub_med_article_set_fixture().await?;
            let pmid = expected_pub_med_article_set
                .articles
                .first()
                .and_then(
                    |article| {
                        article
                            .medline_citation
                            .as_ref()
                    },
                )
                .and_then(
                    |citation| {
                        citation
                            .pmid
                            .value
                            .as_ref()
                    },
                )
                .unwrap();
            let server = MockServer::start();
            let mock = server.mock(
                |when, then| {
                    when.method(GET)
                        .path("/fetch")
                        .query_param("id", pmid)
                        .query_param(
                            "db", "pubmed",
                        );
                    then.status(200)
                        .body(&xml);
                },
            );

            let client = Client::new(
                Some(server.base_url()),
                Some("fetch".into()),
                Some("search".into()),
            )?;
            let ids = vec![pmid.clone()];
            let pub_med_article_set = client
                .fetch(&ids)
                .await?;

            mock.assert_calls(1);
            assert_eq!(
                pub_med_article_set,
                expected_pub_med_article_set
            );

            Ok(())
        }

        #[tokio::test]
        async fn test_should_handle_fetch_reqwest_serialization_error()
        -> anyhow::Result<()> {
            let server = MockServer::start();
            let id = "12345";
            let response_body = "bad response";

            let mock = server.mock(
                |when, then| {
                    when.method(GET)
                        .path("/fetch")
                        .query_param("id", id)
                        .query_param(
                            "db", "pubmed",
                        );

                    then.status(StatusCode::OK)
                        .body(response_body);
                },
            );

            let client = Client::new(
                Some(server.base_url()),
                Some("fetch".into()),
                Some("search".into()),
            )?;
            let ids = vec![id.to_string()];
            let request_result = client
                .fetch(&ids)
                .await;

            mock.assert();
            match request_result {
                Ok(_) => Err(anyhow::anyhow!("nope")),
                Err(errors::ClientError::ParseFetchXml(_)) => Ok(()),
                _ => Err(anyhow::anyhow!("nope")),
            }
        }

        #[tokio::test]
        async fn test_should_handle_bad_request_error() -> anyhow::Result<()> {
            let server = MockServer::start();
            let mock = server.mock(
                |when, then| {
                    when.any_request();
                    then.status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body("Internal Server Error");
                },
            );
            let client = Client::new(
                Some(server.base_url()),
                Some("fetch".into()),
                Some("search".into()),
            )?;

            let ids = vec!["13456".to_string()];
            let result = client
                .fetch(&ids)
                .await;
            mock.assert();

            match result {
                Err(errors::ClientError::FetchReqwest(_)) => Ok(()),
                _ => Err(anyhow::anyhow!("Unexpected Error")),
            }
        }
    }
}
