#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Could Not Initialize PubMed Client Accept Header")]
    InitiationHeader,

    #[error("Could Not Initialize PubMed Client: {0}")]
    InitiationReqwestClient(reqwest::Error),

    #[error("Could not parse XML from PubMed Doc: {0}")]
    ParseFetchXml(quick_xml::errors::serialize::DeError),

    #[error("Could not parse XML from Search Query: {0}")]
    ParseSearchXml(quick_xml::errors::serialize::DeError),

    #[error("Could not retrieve search results: {0}")]
    SearchReqwest(reqwest::Error),

    #[error("Could not serialize search results: {0}")]
    SearchSerialization(reqwest::Error),

    #[error("Could not retrieve fetch results: {0}")]
    FetchReqwest(reqwest::Error),

    #[error("Could not serialize fetch results: {0}")]
    FetchSerialization(reqwest::Error),
}
