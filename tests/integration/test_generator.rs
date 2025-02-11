#[cfg(test)]
mod tests {

    use emgen::databases::Database;
    use emgen::databases::SchemaInfo;
    use emgen::generators::MutableDatabase;
    use emgen::databases::mssql::Mssql;
    use emgen::generators::EntityGenerator;

    use crate::tiberius_config;
    use crate::ephemeral_client;
    use crate::ENTITY;

    #[tokio::test]
    async fn test_create_table_and_generate_entities() {
        ephemeral_client()
            .await
            .expect("Failed to create table");
        let config = tiberius_config()
            .await
            .expect("Failed to configure client");
        let database = Mssql::default();
        let mut generate = EntityGenerator::new(database);
        generate.database().connect(config).await;
        generate.database().set_table_name("entity".to_string());
        generate.database().query_schema_info().await;
        insta::assert_snapshot!(generate.entity().await, @r##"
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

    #[tokio::test]
    async fn test_generate_schema() {
        let schema_info: Vec<SchemaInfo> = vec![
            SchemaInfo {
                table_name: "entity".to_string(),
                column_name: "id".to_string(),
                data_type: "int".to_string(),
                is_nullable: "NO".to_string(),
            },
            SchemaInfo {
                table_name: "entity".to_string(),
                column_name: "name".to_string(),
                data_type: "varchar".to_string(),
                is_nullable: "NO".to_string(),
            },
            SchemaInfo {
                table_name: "entity".to_string(),
                column_name: "active".to_string(),
                data_type: "bit".to_string(),
                is_nullable: "NO".to_string(),
            },
            SchemaInfo {
                table_name: "entity".to_string(),
                column_name: "belongs_to_id".to_string(),
                data_type: "int".to_string(),
                is_nullable: "NO".to_string(),
            },
        ];
        let syntax_tree = syn::parse_file(ENTITY).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);

        let mut database = Mssql::default();
        database.set_table_name("entity".to_string());
        database.set_schema_info(schema_info);
        let mut generate = EntityGenerator::new(database);
        let entity_schema = generate.entity().await;

        assert_eq!(formatted, entity_schema);

        insta::assert_snapshot!(entity_schema, @r##"
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
        #[sea_orm(table_name = "entity")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            pub name: String,
            pub active: bool,
            pub belongs_to_id: i32,
        }
        "##);
    }
}
