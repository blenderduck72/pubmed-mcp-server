use super::models;

pub async fn pub_med_article_set_to_markdown(
    pub_med_article_set: models::fetch::PubMedArticleSet
) -> String {
    let sections: Vec<String> = pub_med_article_set
        .articles
        .into_iter()
        .filter_map(|article| article.medline_citation)
        .filter_map(
            |citation| {
                let body = citation
                    .article
                    .and_then(|article| article.r#abstract)
                    .and_then(|abs| abs.abstract_texts)
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|abs_text| abs_text.value)
                    .map(|v| v.0)
                    .collect::<Vec<_>>()
                    .join("\n\n");

                if body.is_empty() {
                    return None;
                }
                let pmid = citation
                    .pmid
                    .value
                    .unwrap_or_else(|| "unknown".to_string());
                Some(format!(
                    "# PMID: {pmid}\n\n{body}"
                ))
            },
        )
        .collect();

    sections.join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_fixture() -> anyhow::Result<models::fetch::PubMedArticleSet> {
        let article = models::fetch::PubMedArticleBuilder::default()
            .medline_citation(Some(
                models::fetch::MedlineCitationBuilder::default()
                    .pmid(
                        models::fetch::PmidBuilder::default()
                            .value(Some(
                                "123".into(),
                            ))
                            .version(Some("1".into()))
                            .build()?,
                    )
                    .date_created(Some(
                        models::fetch::YearMonthDayDateBuilder::default()
                            .year(Some(
                                "2020".into(),
                            ))
                            .month(Some(
                                "01".into(),
                            ))
                            .day(Some(
                                "04".into(),
                            ))
                            .build()?,
                    ))
                    .article(Some(
                        models::fetch::ArticleBuilder::default()
                            .pub_model(Some(
                                models::constants::PubModel::Electronic,
                            ))
                            .journal(Some(
                                models::fetch::JournalBuilder::default()
                                    .issn(Some(
                                        models::fetch::IssnBuilder::default()
                                            .issn_type(Some(models::constants::IssnType::Electronic))
                                            .value(Some(
                                                "1234".into(),
                                            ))
                                            .build()?,
                                    ))
                                    .journal_issue(Some(models::fetch::JournalIssueBuilder::default()
                                        .cited_medium(Some(models::constants::CitedMedium::Internet))
                                        .volume(Some("123".into()))
                                        .issue(Some("123".into()))
                                        .pub_date(Some(models::fetch::YearMonthDayDateBuilder::default()
                                            .year(Some("2020".into()))
                                            .month(Some("01".into()))
                                            .day(Some("04".into()))
                                            .build()?))
                                        .build()?))
                                    .title(Some("Test".into()))
                                    .iso_abbreviation(Some("Science".into()))
                                    .build()?,
                            ))
                            .article_title(Some(
                                models::fetch::MarkdownElementBuilder::default()
                                    .value(Some(models::fetch::MarkdownText("test".into())))
                                    .build()?
                            ))
                            .elocation_id(Some(
                                models::fetch::ElocationIdBuilder::default()
                                    .eld_type(Some(models::constants::ElocationIdType::DigitialObjectIdentifier))
                                    .valid_y_n(Some(models::constants::ValidYesNo::Y))
                                    .value(Some("10.0001/science.0000001".into()))
                                    .build()?
                            ))
                            .r#abstract(
                                Some(models::fetch::AbstractBuilder::default()
                                    .abstract_texts(
                                        Some(vec![
                                            models::fetch::AbstractTextBuilder::default()
                                                .label(None)
                                                .nlm_category(None)
                                                .value(Some(models::fetch::MarkdownText("Test Text".into())))
                                                .build()?
                                        ])
                                )
                                .copyright_information(None)
                                .build()?)
                            )
                            .author_list(Some(
                                models::fetch::AuthorListBuilder::default()
                                    .complete_y_n(Some(models::constants::ValidYesNo::Y))
                                    .authors(
                                        vec![
                                            models::fetch::AuthorBuilder::default()
                                                .valid_y_n(Some(models::constants::ValidYesNo::Y))
                                                .last_name(Some("Smith".into()))
                                                .fore_name(Some("John".into()))
                                                .initials(None)
                                                .affiliation_infos(
                                                    Some(vec![
                                                        models::fetch::AffiliationInfoBuilder::default()
                                                            .affiliations(vec![
                                                                models::fetch::AffiliationBuilder::default()
                                                                    .value(Some("Evil Corp".into())).build()?
                                                            ])
                                                            .build()?
                                                    ])
                                                )
                                                .build()?
                                        ]
                                    )
                                    .build()?
                            ))
                            .language(
                                Some(models::fetch::LanguageBuilder::default().value(Some(
                                    models::constants::LanguageOption::English
                                ))
                                .build()?)
                            )
                            .data_bank_list(
                                Some(models::fetch::DataBankListBuilder::default()
                                    .complete_y_n(Some(models::constants::ValidYesNo::Y))
                                    .databanks(
                                        vec![
                                            models::fetch::DataBankBuilder::default()
                                                .data_bank_name(Some("test".into()))
                                                .accession_number_list(Some(
                                                    models::fetch::AccessionNumberListBuilder::default()
                                                        .accession_numbers(vec![
                                                            models::fetch::AccessionNumberBuilder::default()
                                                                .value(Some("123".into()))
                                                                .build()?
                                                        ])
                                                        .build()?
                                                ))
                                                .build()?
                                        ]
                                    )
                                    .build()?)
                            )
                            .publication_list_type(
                                Some(models::fetch::PublicationTypeListBuilder::default()
                                    .publication_types(
                                        vec![
                                            models::fetch::PublicationTypeBuilder::default()
                                                .ui(None)
                                                .value(Some("123".into()))
                                                .build()?
                                        ]
                                    )
                                    .build()?)
                            )
                            .article_date(
                                Some(models::fetch::YearMonthDayDateBuilder::default()
                                    .year(Some("2020".into()))
                                    .month(Some("01".into()))
                                    .day(Some("04".into()))
                                    .build()?)
                            )
                            .pagination(None)
                            .grant_list(None)
                            .vernacular_title(None)
                            .build()?,
                    ))
                    .date_completed(None)
                    .date_revised(None)
                    .suppl_mesh_list(None)
                    .comments_corrections_list(None)
                    .gene_symbol_list(None)
                    .medline_journal_info(
                        Some(models::fetch::MedlineJournalInfoBuilder::default()
                            .country(Some("United States".into()))
                            .medline_ta(Some("Science".into()))
                            .nlm_unique_id(Some("0000001".into()))
                            .issn_linking(Some("0001-0001".into()))
                            .build()?)
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
                    .number_of_references(None)
                    .personal_name_subject_list(None)
                    .other_id(None)
                    .other_abstract(None)
                    .keyword_list(None)
                    .space_flight_mission(None)
                    .coi_statement(None)
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
                                                        .major_topic_y_n(Some(models::constants::ValidYesNo::Y))
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
                                                    .hour(None)
                                                    .minute(None)
                                                    .pub_status(Some(models::constants::PubStatus::Pmc))
                                                    .build()?
                                            ]
                                        )
                                    )
                                    .build()?
                            )
                        )
                        .publication_status(Some(models::constants::PubStatus::Ppublish))
                        .article_id_list(None)
                        .reference_lists(None)
                        .build()?
                )
            )
            .build()?;

        let pub_med_article_set =
            models::fetch::PubMedArticleSetBuilder::default()
                .articles(vec![
                    article,
                ])
                .build()?;

        Ok(pub_med_article_set)
    }

    #[tokio::test]
    async fn test_pub_med_article_set_to_markdown_should_create_markdown_string()
    -> anyhow::Result<()> {
        let fix = simple_fixture()?;
        let markdown = pub_med_article_set_to_markdown(fix).await;
        let expected = "# PMID: 123\n\nTest Text";
        assert_eq!(
            markdown,
            expected
        );

        Ok(())
    }
}
