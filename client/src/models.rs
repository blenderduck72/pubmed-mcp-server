pub mod constants {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub enum PubModel {
        #[serde(rename = "Print")]
        Print,
        #[serde(rename = "Print-Electronic")]
        PrintElectronic,

        #[serde(rename = "Electronic")]
        Electronic,

        #[serde(rename = "Electronic-Print")]
        ElectronicPrint,

        #[serde(rename = "Electronic-eCollection")]
        ElectronicECollection,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub enum IssnType {
        Print,
        Electronic,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub enum CitedMedium {
        Internet,
        Print,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub enum ValidYesNo {
        Y,
        N,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub enum ElocationIdType {
        #[serde(rename = "doi")]
        DigitialObjectIdentifier,

        #[serde(rename = "pii")]
        PublisherItemIdentifier,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(
        rename_all(
            deserialize = "UPPERCASE",
            serialize = "lowercase"
        )
    )]
    pub enum NlmCategory {
        Unassigned,
        Background,
        Objective,
        Methods,
        Results,
        Conclusions,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub enum LanguageOption {
        #[serde(rename = "afr")]
        Afrikaans,

        #[serde(rename = "alb")]
        Albanian,

        #[serde(rename = "amh")]
        Amharic,

        #[serde(rename = "ara")]
        Arabic,

        #[serde(rename = "arm")]
        Armenian,

        #[serde(rename = "bos")]
        Bosnian,

        #[serde(rename = "bul")]
        Bulgarian,

        #[serde(rename = "cat")]
        Catalan,

        #[serde(rename = "chi")]
        Chinese,

        #[serde(rename = "cze")]
        Czech,

        #[serde(rename = "dan")]
        Danish,

        #[serde(rename = "dut")]
        Dutch,

        #[serde(rename = "eng")]
        English,

        #[serde(rename = "esp")]
        Esperanto,

        #[serde(rename = "est")]
        Estonian,

        #[serde(rename = "fin")]
        Finnish,

        #[serde(rename = "fre")]
        French,

        #[serde(rename = "geo")]
        Georgian,

        #[serde(rename = "ger")]
        German,

        #[serde(rename = "gla")]
        ScottishGaelic,

        #[serde(rename = "gre")]
        GreekModern,

        #[serde(rename = "heb")]
        Hebrew,

        #[serde(rename = "hin")]
        Hindi,

        #[serde(rename = "hrv")]
        Croatian,

        #[serde(rename = "hun")]
        Hungarian,

        #[serde(rename = "ice")]
        Icelandic,

        #[serde(rename = "ind")]
        Indonesian,

        #[serde(rename = "ita")]
        Italian,

        #[serde(rename = "jpn")]
        Japanese,

        #[serde(rename = "kin")]
        Kinyarwanda,

        #[serde(rename = "kor")]
        Korean,

        #[serde(rename = "lat")]
        Latin,

        #[serde(rename = "lav")]
        Latvian,

        #[serde(rename = "lit")]
        Lithuanian,

        #[serde(rename = "mac")]
        Macedonian,

        #[serde(rename = "mal**")]
        Malayalam,

        #[serde(rename = "mao")]
        Maori,

        #[serde(rename = "may")]
        Malay,

        #[serde(rename = "mul")]
        MultipleLanguages,

        #[serde(rename = "nor")]
        Norwegian,

        #[serde(rename = "per")]
        PersianIranian,

        #[serde(rename = "pol")]
        Polish,

        #[serde(rename = "por")]
        Portuguese,

        #[serde(rename = "pus**")]
        Pushto,

        #[serde(rename = "rum")]
        RomanianRumanianMoldovan,

        #[serde(rename = "rus")]
        Russian,

        #[serde(rename = "san")]
        Sanskrit,

        #[serde(rename = "slv")]
        Slovak,

        #[serde(rename = "spa")]
        Spanish,

        #[serde(rename = "srb")]
        Serbian,

        #[serde(rename = "swe")]
        Swedish,

        #[serde(rename = "tha")]
        Thai,

        #[serde(rename = "tur")]
        Turkish,

        #[serde(rename = "ukr")]
        Ukranian,

        #[serde(rename = "und")]
        Undetermined,

        #[serde(rename = "vie")]
        Vietnamese,

        #[serde(rename = "wel")]
        Welsh,

        Other(String),
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(
        rename_all(
            deserialize = "UPPERCASE",
            serialize = "lowercase"
        )
    )]
    pub enum CitationSubSet {
        // citations from Abridged Index Medicus journals, a list of about 120
        // core clinical, English language journals.
        AIM,
        // citations from non-Index Medicus journals in the field of
        // biotechnology (not currently used).
        B,
        // citations from non-Index Medicus journals in the field of
        // communication disorders (not currently used).
        C,
        // citations from dental journals.
        D,
        // citations in the field of bioethics (includes records from the
        // former BIOETHICS database).
        E,
        // older citations from one journal prior to its selection for Index
        // Medicus; used to augment the database for
        // NLM International MEDLARS Centers (not currently used).
        F,
        //  citations from non-Index Medicus journals in the field of health
        // administration (includes records from the former HealthSTAR
        // database).
        H,
        // citations from Index Medicus journals.
        IM,
        // citations in the field of population information (not currently
        // used; on records from the former POPLINE® database).
        J,
        // citations from non-Index Medicus journals relating to consumer health.
        K,
        // citations from nursing journals.
        N,
        // pre-1966 citations from the older print indices of the Cumulated
        // Index Medicus (CIM) and the Current List of Medical Literature
        // (CLML) (see more information about OLDMEDLINE). See Additional
        // information/background: notations for specialized treatment of
        // selected elements for OLDMEDLINE subset records, e.g., <DateCreated>
        // and <DateCompleted>.
        OM,
        // citations in the field of the history of medicine (includes records
        // from the former HISTLINE® database).
        Q,
        // citations from non-Index Medicus journals in the field of the
        // history of medicine. (For NLM use effective in late 2006 because
        // they require special handling at NLM; not a subset of Q; some
        // journals previously designated as Q are now QIS.)
        QIS,
        // QO is subset of Q - indicates older history of medicine journal
        // citations that were created before the former HISTLINE file was
        // converted to a MEDLINE-like format. (For NLM use because they
        // require special handling at NLM.)
        QO,
        // citations from non-Index Medicus journals in the field of population
        // and reproduction (not currently used).
        R,
        // citations in the field of space life sciences (includes records from
        // the former SPACELINE™ database).
        S,
        // citations from non-Index Medicus journals in the field of health
        // technology assessment (includes records from the former
        // HealthSTAR database).
        T,
        //  citations in the field of AIDS/HIV (includes records from the
        // former AIDSLINE® database).
        X,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(
        rename_all(
            deserialize = "UPPERCASE",
            serialize = "lowercase"
        )
    )]
    pub enum SourceId {
        // National Aeronautics and Space Administration; not currently used
        Nasa,
        // Kennedy Institute of Ethics, Georgetown University; not currently
        // used
        Kie,
        // Population Information Program, Johns Hopkins School of Public
        // Health; not currently used
        Pip,
        //  former NLM POPLINE database; not currently used
        Pop,
        // Annual Review of Population Law; not currently used
        Arpl,
        // Carolina Population Center; not currently used
        Cpc,
        // Population Index; not currently used
        Ind,
        // Center for Population and Family Health Library/Information
        // Program; not currently used
        Cpfh,
        // Current List of Medical Literature
        Clml,
        // Index Medicus; reserved for future use (intended for pre-1966
        // publications)
        Im,
        // Quarterly Cumulative Index to Current Literature; reserved for
        // future use (intended for pre-1966 publications)
        Qcicl,
        // Quarterly Cumulated Index Medicusreserved for future use (intended
        // for pre-1966 publications)
        Qcim,
        // Surgeon General's Catalog; reserved for future use
        Sgc,
        // National Reference Center for Biomedical Literature (for the KIE
        // Reference Library shelving location)
        Nrcbl,
        // U.S. National Library of Medicine (for the PMC or NIH Manuscript
        // System identifiers)
        Nlm,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(
        rename_all(
            deserialize = "UPPERCASE",
            serialize = "lowercase"
        )
    )]
    pub enum AbstractType {
        // American Association of Medical Colleges; not currently used
        Aamc,
        // Special HIV/AIDS publications with abstracts written by someone
        // other than the author
        Aids,
        // Kennedy Institute of Ethics, Georgetown University; not currently used
        Kie,
        // Population Information Program, Johns Hopkins School of Public
        // Health; not currently used
        Pip,
        // National Aeronautics and Space Administration; not currently used
        Nasa,
        // journal editorial staff, typically for citations to older articles
        // that did not contain abstracts when originally published
        #[serde(rename = "Publisher")]
        Publisher,

        #[serde(rename(deserialize = "plain-language-summary"))]
        PlainLanguageSummary,

        Other(String),
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(
        rename_all = "lowercase",
        from = "String"
    )]
    pub enum PubStatus {
        Received,
        Revised,
        Accepted,
        Epublish,
        Ppublish,
        AheadOfPrint,
        Retracted,
        Ecollection,
        Pmc,
        Pmcr,
        PubMed,
        PubMedr,
        Premedline,
        Medline,
        Entrez,
        #[serde(rename = "pmc-release")]
        PmcRelase,
        // catch-all for statuses not listed above, keeps the raw value
        Other(String),
    }

    impl From<String> for PubStatus {
        fn from(value: String) -> Self {
            match value.as_str() {
                "received" => PubStatus::Received,
                "revised" => PubStatus::Revised,
                "accepted" => PubStatus::Accepted,
                "epublish" => PubStatus::Epublish,
                "ppublish" => PubStatus::Ppublish,
                "aheadofprint" => PubStatus::AheadOfPrint,
                "retracted" => PubStatus::Retracted,
                "ecollection" => PubStatus::Ecollection,
                "pmc" => PubStatus::Pmc,
                "pmcr" => PubStatus::Pmcr,
                "pubmed" => PubStatus::PubMed,
                "pubmedr" => PubStatus::PubMedr,
                "premedline" => PubStatus::Premedline,
                "medline" => PubStatus::Medline,
                "entrez" => PubStatus::Entrez,
                "pmc-release" => PubStatus::PmcRelase,
                _ => PubStatus::Other(value),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "lowercase")]
    pub enum ArticleIdType {
        Doi,
        Pii,
        Pmcpid,
        Pmpid,
        Pmc,
        Mid,
        Sici,
        Pubmed,
        Medline,
        Pmcid,
    }
}

pub mod fetch {
    use super::constants;
    use crate::models::constants::{IssnType, ValidYesNo};
    use derive_builder::Builder;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Default, Builder)]
    struct InlineChildren {
        #[serde(
            rename = "$value",
            default
        )]
        nodes: Vec<InlineNode>,
    }

    #[derive(Debug, Clone)]
    enum InlineNode {
        Text(String),
        I(InlineChildren),
        B(InlineChildren),
        U(InlineChildren),
        Sup(InlineChildren),
        Sub(InlineChildren),
        Math(InlineChildren),
        Mi(InlineChildren),
        Mn(InlineChildren),
        Mo(InlineChildren),
        Mtext(InlineChildren),
        Mrow(InlineChildren),
        Other(InlineChildren),
    }

    impl<'de> Deserialize<'de> for InlineNode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde::de::{EnumAccess, VariantAccess, Visitor};
            use std::fmt;

            struct NodeVisitor;

            impl<'de> Visitor<'de> for NodeVisitor {
                type Value = InlineNode;

                fn expecting(
                    &self,
                    f: &mut fmt::Formatter,
                ) -> fmt::Result {
                    f.write_str("an inline node")
                }

                fn visit_enum<A>(
                    self,
                    data: A,
                ) -> Result<Self::Value, A::Error>
                where
                    A: EnumAccess<'de>,
                {
                    let (tag, variant): (String, _) = data.variant()?;
                    match tag.as_str() {
                        "$text" => {
                            Ok(InlineNode::Text(variant.newtype_variant()?))
                        }
                        "i" => Ok(InlineNode::I(variant.newtype_variant()?)),
                        "b" => Ok(InlineNode::B(variant.newtype_variant()?)),
                        "u" => Ok(InlineNode::U(variant.newtype_variant()?)),
                        "sup" => {
                            Ok(InlineNode::Sup(variant.newtype_variant()?))
                        }
                        "sub" => {
                            Ok(InlineNode::Sub(variant.newtype_variant()?))
                        }
                        "math" => {
                            Ok(InlineNode::Math(variant.newtype_variant()?))
                        }
                        "mi" => Ok(InlineNode::Mi(variant.newtype_variant()?)),
                        "mn" => Ok(InlineNode::Mn(variant.newtype_variant()?)),
                        "mo" => Ok(InlineNode::Mo(variant.newtype_variant()?)),
                        "mtext" => {
                            Ok(InlineNode::Mtext(variant.newtype_variant()?))
                        }
                        "mrow" => {
                            Ok(InlineNode::Mrow(variant.newtype_variant()?))
                        }
                        _ => Ok(InlineNode::Other(variant.newtype_variant()?)),
                    }
                }
            }

            deserializer.deserialize_enum(
                "InlineNode",
                &[],
                NodeVisitor,
            )
        }
    }

    fn render_inline_nodes(nodes: Vec<InlineNode>) -> String {
        let mut out = String::new();
        for node in nodes {
            match node {
                InlineNode::Text(t) => out.push_str(&t),
                InlineNode::I(c) => out.push_str(&format!(
                    "*{}*",
                    render_inline_nodes(c.nodes)
                )),
                InlineNode::B(c) => out.push_str(&format!(
                    "**{}**",
                    render_inline_nodes(c.nodes)
                )),
                InlineNode::U(c) => out.push_str(&format!(
                    "_{}_",
                    render_inline_nodes(c.nodes)
                )),
                InlineNode::Sup(c) => out.push_str(&format!(
                    "^{}",
                    render_inline_nodes(c.nodes)
                )),
                InlineNode::Sub(c) => out.push_str(&format!(
                    "~{}~",
                    render_inline_nodes(c.nodes)
                )),
                InlineNode::Math(c) => out.push_str(&format!(
                    "$${}$",
                    render_inline_nodes(c.nodes)
                )),
                InlineNode::Mi(c)
                | InlineNode::Mn(c)
                | InlineNode::Mo(c)
                | InlineNode::Mtext(c)
                | InlineNode::Mrow(c)
                | InlineNode::Other(c) => {
                    out.push_str(&render_inline_nodes(c.nodes))
                }
            }
        }
        out
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(from = "Vec<InlineNode>")]
    pub struct MarkdownText(pub String);

    impl From<Vec<InlineNode>> for MarkdownText {
        fn from(nodes: Vec<InlineNode>) -> Self {
            MarkdownText(render_inline_nodes(nodes))
        }
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    pub struct MarkdownElement {
        #[serde(
            rename(
                deserialize = "$value",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<MarkdownText>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct YearMonthDayDate {
        #[serde(skip_serializing_if = "Option::is_none")]
        year: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        month: Option<String>, // can be a number like 12 or a 3 digit abbreviation for a month

        #[serde(skip_serializing_if = "Option::is_none")]
        day: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Issn {
        #[serde(
            rename(
                deserialize = "@IssnType",
                serialize = "issn_type"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        issn_type: Option<IssnType>,

        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct JournalIssue {
        #[serde(
            rename(
                deserialize = "@CitedMedium",
                serialize = "cited_medium"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub cited_medium: Option<constants::CitedMedium>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub volume: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub issue: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub pub_date: Option<YearMonthDayDate>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Journal {
        #[serde(
            rename(deserialize = "ISSN"),
            skip_serializing_if = "Option::is_none"
        )]
        pub issn: Option<Issn>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub journal_issue: Option<JournalIssue>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,

        #[serde(
            rename(deserialize = "ISOAbbreviation"),
            skip_serializing_if = "Option::is_none"
        )]
        pub iso_abbreviation: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Pagination {
        start_page: Option<String>,
        end_page: Option<String>,
        medline_pgn: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct ElocationId {
        #[serde(
            rename(
                deserialize = "@EIdType",
                serialize = "eld_type"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        eld_type: Option<constants::ElocationIdType>,

        #[serde(
            rename(
                deserialize = "@ValidYN",
                serialize = "valid_y_n"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        valid_y_n: Option<constants::ValidYesNo>,

        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct AbstractText {
        #[serde(
            rename(
                deserialize = "@Label",
                serialize = "label"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub label: Option<String>,

        #[serde(
            rename(deserialize = "@NlmCategory"),
            skip_serializing_if = "Option::is_none"
        )]
        pub nlm_category: Option<constants::NlmCategory>,

        #[serde(
            rename(
                deserialize = "$value",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<MarkdownText>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Abstract {
        #[serde(
            rename(deserialize = "AbstractText"),
            skip_serializing_if = "Option::is_none"
        )]
        pub abstract_texts: Option<Vec<AbstractText>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub copyright_information: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Affiliation {
        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct AffiliationInfo {
        #[serde(rename(deserialize = "Affiliation"))]
        pub affiliations: Vec<Affiliation>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Author {
        #[serde(
            rename(deserialize = "@ValidYN"),
            skip_serializing_if = "Option::is_none"
        )]
        pub valid_y_n: Option<constants::ValidYesNo>,
        pub last_name: Option<String>,
        pub fore_name: Option<String>,
        pub initials: Option<String>,
        #[serde(rename(deserialize = "AffiliationInfo"))]
        pub affiliation_infos: Option<Vec<AffiliationInfo>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct AuthorList {
        #[serde(
            rename(deserialize = "@CompleteYN"),
            skip_serializing_if = "Option::is_none"
        )]
        pub complete_y_n: Option<constants::ValidYesNo>,

        #[serde(rename(deserialize = "Author"))]
        pub authors: Vec<Author>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Language {
        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        value: Option<constants::LanguageOption>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct AccessionNumber {
        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct AccessionNumberList {
        #[serde(rename(deserialize = "AccessionNumber"))]
        accession_numbers: Vec<AccessionNumber>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct DataBank {
        pub data_bank_name: Option<String>,
        pub accession_number_list: Option<AccessionNumberList>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct DataBankList {
        #[serde(rename(deserialize = "@CompleteYN"))]
        pub complete_y_n: Option<constants::ValidYesNo>,

        #[serde(rename(deserialize = "DataBank"))]
        pub databanks: Vec<DataBank>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Grant {
        #[serde(rename(deserialize = "GrantID"))]
        pub id: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub acronym: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub agency: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub country: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct GrantList {
        #[serde(
            rename(deserialize = "@CompleteYN"),
            skip_serializing_if = "Option::is_none"
        )]
        pub complete_y_n: Option<constants::ValidYesNo>,

        #[serde(rename(deserialize = "Grant"))]
        pub grants: Vec<Grant>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct PublicationType {
        #[serde(
            rename(deserialize = "@UI"),
            skip_serializing_if = "Option::is_none"
        )]
        pub ui: Option<String>,

        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct PublicationTypeList {
        #[serde(rename(deserialize = "PublicationType"))]
        pub publication_types: Vec<PublicationType>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct NameOfSubstance {
        #[serde(
            rename(deserialize = "@UI"),
            skip_serializing_if = "Option::is_none"
        )]
        pub ui: Option<String>,

        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Chemical {
        #[serde(skip_serializing_if = "Option::is_none")]
        registry_number: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        name_of_substance: Option<NameOfSubstance>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct ChemicalList {
        #[serde(rename(deserialize = "Chemical"))]
        pub chemicals: Vec<Chemical>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct SupplMeshName {
        #[serde(rename(deserialize = "@Type"))]
        suppl_type: Option<String>,

        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct SupplMeshList {
        suppl_mesh_name: Vec<SupplMeshName>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct CommentsCorrections {
        #[serde(rename(deserialize = "@RefType"))]
        pub ref_type: Option<String>,

        #[serde(rename(deserialize = "PMID"))]
        pub pmid: Option<Pmid>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub ref_source: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub note: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct CommentsCorrectionsList {
        comments_corrections: Vec<CommentsCorrections>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct GeneSymbol {
        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct GeneSymbolList {
        #[serde(rename(deserialize = "GeneSymbol"))]
        pub gene_symbols: Vec<GeneSymbol>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct DescriptorName {
        #[serde(
            rename(deserialize = "@UI"),
            skip_serializing_if = "Option::is_none"
        )]
        pub ui: Option<String>,

        #[serde(
            rename(deserialize = "@MajorTopicYN"),
            skip_serializing_if = "Option::is_none"
        )]
        pub major_topic_y_n: Option<constants::ValidYesNo>,

        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct QualifierName {
        #[serde(
            rename(
                deserialize = "@UI",
                serialize = "ui"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub ui: Option<String>,

        #[serde(
            rename(deserialize = "@MajorTopicYN"),
            skip_serializing_if = "Option::is_none"
        )]
        pub major_topic_y_n: Option<constants::ValidYesNo>,

        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct MeshHeading {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub descriptor_name: Option<DescriptorName>,

        #[serde(rename(deserialize = "QualifierName"))]
        pub qualifer_names: Option<Vec<QualifierName>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct MeshHeadingList {
        #[serde(rename(deserialize = "MeshHeading"))]
        pub mesh_headings: Vec<MeshHeading>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Article {
        #[serde(
            rename(deserialize = "@PubModel"),
            skip_serializing_if = "Option::is_none"
        )]
        pub pub_model: Option<constants::PubModel>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub journal: Option<Journal>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub article_title: Option<MarkdownElement>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub pagination: Option<Pagination>,

        #[serde(
            rename(deserialize = "ELocationID"),
            skip_serializing_if = "Option::is_none"
        )]
        pub elocation_ids: Option<Vec<ElocationId>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#abstract: Option<Abstract>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub author_list: Option<AuthorList>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub language: Option<Language>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub data_bank_list: Option<DataBankList>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub grant_list: Option<GrantList>,

        #[serde(
            rename(deserialize = "PublicationTypeList"),
            skip_serializing_if = "Option::is_none"
        )]
        pub publication_list_type: Option<PublicationTypeList>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub vernacular_title: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub article_date: Option<YearMonthDayDate>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct MedlineJournalInfo {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub country: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename(deserialize = "MedlineTA"))]
        pub medline_ta: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename(deserialize = "NlmUniqueID"))]
        pub nlm_unique_id: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename(deserialize = "ISSNLinking"))]
        pub issn_linking: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct PersonalSubject {
        pub last_name: Option<String>,
        pub fore_name: Option<String>,
        pub initials: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct PersonalNameSubjectList {
        #[serde(rename(deserialize = "PersonalSubject"))]
        pub personal_subjects: Option<Vec<PersonalSubject>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct OtherId {
        #[serde(
            rename(deserialize = "@Source"),
            skip_serializing_if = "Option::is_none"
        )]
        pub source: Option<constants::SourceId>,

        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct OtherAbstract {
        #[serde(
            rename(deserialize = "@Type"),
            skip_serializing_if = "Option::is_none"
        )]
        pub abstract_type: Option<constants::AbstractType>,

        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Keyword {
        #[serde(
            rename(deserialize = "@MajorTopicYN"),
            skip_serializing_if = "Option::is_none"
        )]
        pub major_topic_y_n: Option<ValidYesNo>,

        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct KeywordList {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub keywords: Option<Vec<Keyword>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct MedlineCitation {
        #[serde(rename(deserialize = "PMID"))]
        pub pmid: Pmid,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_created: Option<YearMonthDayDate>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_completed: Option<YearMonthDayDate>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_revised: Option<YearMonthDayDate>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub article: Option<Article>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub medline_journal_info: Option<MedlineJournalInfo>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub chemical_list: Option<ChemicalList>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub suppl_mesh_list: Option<SupplMeshList>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub citation_subset: Option<constants::CitationSubSet>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub comments_corrections_list: Option<CommentsCorrectionsList>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub gene_symbol_list: Option<GeneSymbolList>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub mesh_heading_list: Option<MeshHeadingList>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub number_of_references: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub personal_name_subject_list: Option<PersonalNameSubjectList>,

        #[serde(
            rename(deserialize = "OtherID"),
            skip_serializing_if = "Option::is_none"
        )]
        pub other_id: Option<OtherId>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub other_abstract: Option<OtherAbstract>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub keyword_list: Option<KeywordList>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub space_flight_mission: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub coi_statement: Option<MarkdownElement>, // Conflict Of Interest Statement
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct PubMedDate {
        #[serde(
            rename(deserialize = "@PubStatus"),
            skip_serializing_if = "Option::is_none"
        )]
        pub pub_status: Option<constants::PubStatus>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub year: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub month: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub day: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub hour: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub minute: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct History {
        #[serde(
            rename(deserialize = "PubMedPubDate"),
            skip_serializing_if = "Option::is_none"
        )]
        pub pub_med_pub_dates: Option<Vec<PubMedDate>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct ArticleId {
        #[serde(
            rename(deserialize = "@IdType"),
            skip_serializing_if = "Option::is_none"
        )]
        id_type: Option<constants::ArticleIdType>,

        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct ArticleIdList {
        #[serde(
            rename(deserialize = "ArticleId"),
            skip_serializing_if = "Option::is_none"
        )]
        article_ids: Option<Vec<ArticleId>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Reference {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub citation: Option<MarkdownElement>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub article_id_list: Option<ArticleIdList>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct ReferenceList {
        #[serde(
            rename(deserialize = "Reference"),
            skip_serializing_if = "Option::is_none"
        )]
        pub references: Option<Vec<Reference>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct PubMedData {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub history: Option<History>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub publication_status: Option<constants::PubStatus>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub article_id_list: Option<ArticleIdList>,

        #[serde(
            rename(deserialize = "ReferenceList"),
            skip_serializing_if = "Option::is_none"
        )]
        pub reference_lists: Option<Vec<ReferenceList>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct PubMedArticle {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub medline_citation: Option<MedlineCitation>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub pubmed_data: Option<PubMedData>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct PubMedArticleSet {
        #[serde(rename(deserialize = "PubmedArticle"))]
        pub articles: Vec<PubMedArticle>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    pub struct Pmid {
        #[serde(
            rename(deserialize = "@Version"),
            skip_serializing_if = "Option::is_none"
        )]
        pub version: Option<String>,

        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<String>,
    }
}

pub mod search {
    use derive_builder::Builder;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    pub struct QueryTranslation {
        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    pub struct Id {
        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    pub struct IdList {
        #[serde(rename(deserialize = "Id"))]
        pub ids: Vec<Id>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    pub struct To {
        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    pub struct From {
        #[serde(
            rename(
                deserialize = "$text",
                serialize = "value"
            ),
            skip_serializing_if = "Option::is_none"
        )]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct Translation {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to: Option<To>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub from: Option<From>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct TranslationSet {
        #[serde(
            rename(deserialize = "Translation"),
            skip_serializing_if = "Option::is_none"
        )]
        pub translations: Option<Vec<Translation>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
    #[serde(
        rename_all(
            deserialize = "PascalCase",
            serialize = "camelCase"
        )
    )]
    pub struct ESearchResult {
        pub count: u32,
        pub ret_max: u32,
        pub ret_start: u32,
        pub id_list: IdList,
        pub translation_set: TranslationSet,
        pub query_translation: QueryTranslation,
    }
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn temp_test() -> anyhow::Result<()> {
        let data = r#"
<?xml version= "1.0"?>
<!DOCTYPE PubmedArticleSet PUBLIC "-//NLM//DTD PubMedArticle, 1st January 2025//EN" "https://dtd.nlm.nih.gov/ncbi/pubmed/out/pubmed_250101.dtd">
<PubmedArticleSet>
    <PubmedArticle>
        <MedlineCitation Status= "Publisher" Owner= "NLM">
            <PMID Version= "1">42864228</PMID>
            <DateRevised>
                <Year>2026</Year>
                <Month>10</Month>
                <Day>12</Day>
            </DateRevised>
            <Article PubModel= "Print-Electronic">
                <Journal>
                    <ISSN IssnType= "Electronic">1873-7560</ISSN>
                    <JournalIssue CitedMedium= "Internet">
                        <PubDate>
                            <Year>2026</Year>
                            <Month>Oct</Month>
                            <Day>12</Day>
                        </PubDate>
                    </JournalIssue>
                    <Title>European urology</Title>
                    <ISOAbbreviation>Eur Urol</ISOAbbreviation>
                </Journal>
                <ArticleTitle><sup>177</sup>Lu-Prostate-specific Membrane Antigen Prior to
    Stereotactic Ablative Radiotherapy in Oligorecurrent Prostate Cancer: Secondary and Exploratory
    End Point Results from the Randomized Phase 2 LUNAR Trial.</ArticleTitle>
                <ELocationID EIdType= "pii" ValidYN= "Y">S0302-2838(26)02376-6</ELocationID>
                <ELocationID EIdType= "doi" ValidYN= "Y">10.1016/j.eururo.2026.09.005</ELocationID>
                <Abstract>
                    <AbstractText>The phase 2 randomized LUNAR trial previously demonstrated that
    neoadjuvant <sup>177</sup>Lu-PNT2002 before stereotactic body radiotherapy (SBRT) significantly
    improved progression-free survival (PFS) in patients with oligorecurrent hormone-sensitive
    prostate cancer (orHSPC). We report updated efficacy and safety outcomes with extended
    follow-up. Patients with prostate-specific membrane antigen (PSMA) positron emission
    tomography/computed tomography (PET/CT)-detected orHSPC and one to five metastatic lesions
    amenable to SBRT were randomized 1:1 to SBRT alone or two cycles of neoadjuvant <sup>177</sup>Lu-PNT2002
    followed by SBRT. The primary end point was PFS, defined asnew lesions on PSMA PET/CT,
    initiation of salvage hormonal therapy, or death. Secondary end points included hormonal
    therapy-free survival (HT-FS) and 24-mo progression-free status. Exploratory end points included
    polymetastatic PFS (poly-PFS), castration-resistant disease-free survival (CRFS), and
    conventional imaging progression-free survival (CT-PFS). Between September 2022 and November
    2023, 92 patients were randomized to SBRT alone (n&#xa0;=&#xa0;47) or <sup>177</sup>Lu-PNT2002
    plus SBRT (n&#xa0;=&#xa0;45). At a median follow-up of 31.9 mo, <sup>177</sup>Lu-PNT2002
    significantly improved PFS (median 17.6 vs 7.4 mo; hazard ratio [HR] 0.38, 95% confidence
    interval [CI]&#xa0;=&#xa0;0.23-0.61; p&#xa0;&lt;&#xa0;0.001) and HT-FS (HR 0.57, 95%
    CI&#xa0;=&#xa0;0.34-0.95; p&#xa0;=&#xa0;0.03). Exploratory analyses demonstrated improved
    poly-PFS (HR 0.28, 95% CI&#xa0;=&#xa0;0.08-1.03;p&#xa0;=&#xa0;0.04), CRFS (HR 0.13, 95%
    CI&#xa0;=&#xa0;0.02-1.10; p&#xa0;=&#xa0;0.03), and CT-PFS (HR 0.46, 95%
    CI&#xa0;=&#xa0;0.19-1.09; p&#xa0;=&#xa0;0.07). Nonew safety signals were observed. With extended
    follow-up, neoadjuvant <sup>177</sup>Lu-PNT2002 before SBRT produced durable improvements in PFS
    and HT-FS. Whilepoly-PFS, CRFS, and CT-PFS were exploratory end points, their improvement
    provides hypothesis-generating evidence that the addition of radioligand therapy to SBRT may
    alter disease trajectory beyond delaying PSMA PET/CT-detected recurrence.</AbstractText>
                    <CopyrightInformation>Copyright &#xa9; 2026 The Author(s). Publishedby Elsevier
    B.V. All rights reserved.</CopyrightInformation>
                </Abstract>
                <AuthorList CompleteYN= "Y">
                    <Author ValidYN= "Y">
                        <LastName>Kishan</LastName>
                        <ForeName>Amar U</ForeName>
                        <Initials>AU</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department ofRadiation Oncology, University of California,
    Los Angeles, USA; Department of Nuclear Medicine and Theranostics, University of California, Los
    Angeles, CA, USA; Department of Urology, University of California, Los Angeles, CA, USA.
    Electronicaddress: aukishan@mednet.ucla.edu.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Wilhalme</LastName>
                        <ForeName>Holly</ForeName>
                        <Initials>H</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Medicine Statistical Core, University of
    California, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Felix</LastName>
                        <ForeName>Carol</ForeName>
                        <Initials>C</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Radiation Oncology, University of California,
    Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Enke</LastName>
                        <ForeName>Johanna</ForeName>
                        <Initials>J</Initials>
                        <AffiliationInfo>
                            <Affiliation>Ahmanson Translational Theranostics Division, Department of
    Nuclear Medicine and Theranostics, University of California, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Ali</LastName>
                        <ForeName>Farzana Z</ForeName>
                        <Initials>FZ</Initials>
                        <AffiliationInfo>
                            <Affiliation>Ahmanson Translational Theranostics Division, Department of
    Nuclear Medicine and Theranostics, University of California, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Nabong</LastName>
                        <ForeName>Rejah</ForeName>
                        <Initials>R</Initials>
                        <AffiliationInfo>
                            <Affiliation>Ahmanson Translational Theranostics Division, Department of
    Nuclear Medicine and Theranostics, University of California, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Juarez-Casillas</LastName>
                        <ForeName>Jesus E</ForeName>
                        <Initials>JE</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Radiation Oncology, University of California,
    Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Flores</LastName>
                        <ForeName>Kevin</ForeName>
                        <Initials>K</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Radiation Oncology, University of California,
    Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Ma</LastName>
                        <ForeName>T Martin</ForeName>
                        <Initials>TM</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Radiation Oncology, University of Washington,
    Seattle, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Ludwig</LastName>
                        <ForeName>Vinicius</ForeName>
                        <Initials>V</Initials>
                        <AffiliationInfo>
                            <Affiliation>Ahmanson Translational Theranostics Division, Department of
    Nuclear Medicine and Theranostics, University of California, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Nakayama</LastName>
                        <ForeName>Mariko</ForeName>
                        <Initials>M</Initials>
                        <AffiliationInfo>
                            <Affiliation>Ahmanson Translational Theranostics Division, Department of
    Nuclear Medicine and Theranostics, University of California, Los Angeles, USA; Department of
    Radiological Sciences, University of California, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Ells</LastName>
                        <ForeName>Zachary</ForeName>
                        <Initials>Z</Initials>
                        <AffiliationInfo>
                            <Affiliation>Ahmanson Translational Theranostics Division, Department of
    Nuclear Medicine and Theranostics, University of California, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Dahlbom</LastName>
                        <ForeName>Magnus</ForeName>
                        <Initials>M</Initials>
                        <AffiliationInfo>
                            <Affiliation>Ahmanson Translational Theranostics Division, Department of
    Nuclear Medicine and Theranostics, University of California, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Lauria</LastName>
                        <ForeName>Michael</ForeName>
                        <Initials>M</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Radiation Oncology, University ofCalifornia,
    Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Meyer</LastName>
                        <ForeName>Catherine</ForeName>
                        <Initials>C</Initials>
                        <AffiliationInfo>
                            <Affiliation>Ahmanson Translational Theranostics Division, Department of
    Nuclear Medicine and Theranostics, University of California, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Cao</LastName>
                        <ForeName>Minsong</ForeName>
                        <Initials>M</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Radiation Oncology, University of California,
    San Francisco, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Weidhaas</LastName>
                        <ForeName>Joanne B</ForeName>
                        <Initials>JB</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Radiation Oncology, University of California,
    Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Telesca</LastName>
                        <ForeName>Donatello</ForeName>
                        <Initials>D</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Biostatistics, University of California, Los
    Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>McGreevy</LastName>
                        <ForeName>Kristen</ForeName>
                        <Initials>K</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Biostatistics, University of California, Los
    Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Nickols</LastName>
                        <ForeName>Nicholas G</ForeName>
                        <Initials>NG</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Radiation Oncology, University of California,
    Los Angeles, USA; VA Greater Los Angeles Healthcare System, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Parmisano</LastName>
                        <ForeName>Sophia</ForeName>
                        <Initials>S</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Radiation Oncology, University of California,
    Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Brisbane</LastName>
                        <ForeName>Wayne</ForeName>
                        <Initials>W</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Urology, University of California, Los
    Angeles, CA, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Marks</LastName>
                        <ForeName>Leonard</ForeName>
                        <Initials>L</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Urology, University of California, Los
    Angeles, CA, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Singer</LastName>
                        <ForeName>Adam E</ForeName>
                        <Initials>AE</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Medicine, University of California, Los
    Angeles, CA, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Rettig</LastName>
                        <ForeName>Matthew B</ForeName>
                        <Initials>MB</Initials>
                        <AffiliationInfo>
                            <Affiliation>VA Greater Los Angeles Healthcare System, Los Angeles, USA;
    Department of Medicine, University of California, Los Angeles, CA, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Reiter</LastName>
                        <ForeName>Robert E</ForeName>
                        <Initials>RE</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Urology, University of California, Los
    Angeles, CA, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Valle</LastName>
                        <ForeName>Luca F</ForeName>
                        <Initials>LF</Initials>
                        <AffiliationInfo>
                            <Affiliation>Department of Radiation Oncology, University of California,
    Los Angeles, USA; VA Greater Los Angeles Healthcare System, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Allen-Auerbach</LastName>
                        <ForeName>Martin</ForeName>
                        <Initials>M</Initials>
                        <AffiliationInfo>
                            <Affiliation>Ahmanson Translational Theranostics Division, Department of
    Nuclear Medicine and Theranostics, University of California, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Czernin</LastName>
                        <ForeName>Johannes</ForeName>
                        <Initials>J</Initials>
                        <AffiliationInfo>
                            <Affiliation>Ahmanson Translational Theranostics Division, Department of
    Nuclear Medicine and Theranostics, Universityof California, Los Angeles, USA.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                    <Author ValidYN="Y">
                    <LastName>Steinberg</LastName>
                    <ForeName>Michael L</ForeName>
                    <Initials>ML</Initials>
                    <AffiliationInfo>
                        <Affiliation>Department of Radiation Oncology, University of California, Los
    Angeles, USA.</Affiliation>
                    </AffiliationInfo>
                    </Author>
                    <Author ValidYN= "Y">
                        <LastName>Calais</LastName>
                        <ForeName>Jeremie</ForeName>
                        <Initials>J</Initials>
                        <AffiliationInfo>
                            <Affiliation>Ahmanson Translational Theranostics Division, Department of
    Nuclear Medicine and Theranostics, University of California, Los Angeles, USA; Nuclear Medicine
    Division, Medical Imaging Department, Gustave Roussy, Paris-Saclay University, France; 3 BioMaps
    Multimodal Biomedical Imaging Laboratory, INSERM UMR 1281, CNRS UMR90, CEA, Paris-Saclay
    University, France.</Affiliation>
                        </AffiliationInfo>
                    </Author>
                </AuthorList>
                <Language>eng</Language>
                <PublicationTypeList>
                    <PublicationType UI= "D016428">Journal Article</PublicationType>
                </PublicationTypeList>
                <ArticleDate DateType= "Electronic">
                    <Year>2026</Year>
                    <Month>10</Month>
                    <Day>12</Day>
                </ArticleDate>
            </Article>
            <MedlineJournalInfo>
                <Country>Switzerland</Country>
                <MedlineTA>Eur Urol</MedlineTA>
                <NlmUniqueID>7512719</NlmUniqueID>
                <ISSNLinking>0302-2838</ISSNLinking>
            </MedlineJournalInfo>
            <CitationSubset>IM</CitationSubset>
            <KeywordList Owner= "NOTNLM">
                <Keyword MajorTopicYN= "N">PSMA</Keyword>
                <Keyword MajorTopicYN= "N">metastasis-directed therapy</Keyword>
                <Keyword MajorTopicYN= "N">oligometastatic</Keyword>
                <Keyword MajorTopicYN= "N">radioligand</Keyword>
            </KeywordList>
        </MedlineCitation>
        <PubmedData>
            <History>
                <PubMedPubDate PubStatus= "medline">
                    <Year>2026</Year>
                    <Month>10</Month>
                    <Day>13</Day>
                    <Hour>0</Hour>
                    <Minute>16</Minute>
                </PubMedPubDate>
                <PubMedPubDate PubStatus= "pubmed">
                    <Year>2026</Year>
                    <Month>10</Month>
                    <Day>13</Day>
                    <Hour>0</Hour>
                    <Minute>16</Minute>
                </PubMedPubDate>
                <PubMedPubDate PubStatus= "received">
                    <Year>2026</Year>
                    <Month>7</Month>
                    <Day>28</Day>
                </PubMedPubDate>
                <PubMedPubDate PubStatus= "revised">
                    <Year>2026</Year>
                    <Month>9</Month>
                    <Day>2</Day>
                </PubMedPubDate>
                <PubMedPubDate PubStatus= "accepted">
                    <Year>2026</Year>
                    <Month>9</Month>
                    <Day>9</Day>
                </PubMedPubDate>
                <PubMedPubDate PubStatus= "entrez">
                    <Year>2026</Year>
                    <Month>10</Month>
                    <Day>12</Day>
                    <Hour>22</Hour>
                    <Minute>1</Minute>
                </PubMedPubDate>
            </History>
            <PublicationStatus>aheadofprint</PublicationStatus>
            <ArticleIdList>
                <ArticleId IdType= "pubmed">42864228</ArticleId>
                <ArticleId IdType= "doi">10.1016/j.eururo.2026.09.005</ArticleId>
                <ArticleId IdType= "pii">S0302-2838(26)02376-6</ArticleId>
            </ArticleIdList>
        </PubmedData>
    </PubmedArticle></PubmedArticleSet>
        "#;

        use super::super::Client;

        let pub_med_article_set = Client::parse_e_fetch_result(data).await?;
        println!(
            "{:#?}",
            pub_med_article_set
        );

        Ok(())
    }
}
