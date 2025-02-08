#[cfg(test)]
mod tests {
    use crate::connect_client;
    use emgen::entity_generator::information_schema;
    use emgen::entity_generator::generate_schema;

    #[tokio::test]
    async fn test_create_table_and_generate_entities() {
        let client = connect_client().await.expect("Failed to connect client");
        let entities = information_schema(client, "entity").await;

        assert!(!entities.is_empty(), "Entities should not be empty");
        insta::assert_snapshot!(generate_schema(entities.clone()), @r##"
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
        #[sea_orm(table_name = "entity")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            pub name: String,
            pub active: bool,
            pub belongs_to_id: Option<i32>,
        }
        "##);
    }
}
