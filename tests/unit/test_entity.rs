#[cfg(test)]
mod tests {
    use crate::ENTITY;
    use emgen::entity_generator;
    use convert_case::{Casing, Case};
    use std::env;

    #[test]
    fn test_map_sql_type_to_rust() {
        insta::assert_yaml_snapshot!(entity_generator::map_sql_type_to_rust("int", false), @"i32");
        insta::assert_yaml_snapshot!(entity_generator::map_sql_type_to_rust("varchar", true), @"Option<String>");
        insta::assert_yaml_snapshot!(entity_generator::map_sql_type_to_rust("datetime", false), @r#""chrono::DateTime<chrono::Utc>""#);
        insta::assert_yaml_snapshot!(entity_generator::map_sql_type_to_rust("decimal", false), @"f64");
        insta::assert_yaml_snapshot!(entity_generator::map_sql_type_to_rust("uniqueidentifier", false), @r#""uuid::Uuid""#);
    }

    #[test]
    fn test_case_conversion() {
        insta::assert_yaml_snapshot!("entiy_name".to_case(Case::Pascal), @"EntiyName");
        insta::assert_yaml_snapshot!("EntityName".to_case(Case::Snake), @"entity_name");
    }

    #[test]
    fn test_generate_entity() {
        let syntax_tree = syn::parse_file(ENTITY).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        let columns = vec![
            ("id", "int", "NO"),
            ("name", "varchar", "NO"),
            ("active", "bit", "NO"),
            ("belongs_to_id", "int", "NO"),
        ].into_iter()
        .map(|(a, b, c)| (a.to_string(), b.to_string(), c.to_string()))
        .collect();

        let entity_string = entity_generator::generate_entity(columns, "entity");
        assert_eq!(formatted, entity_string);
        insta::assert_snapshot!(entity_string, @r##"
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

    #[test]
    fn test_generate_schema() {
        let schema: Vec<entity_generator::SchemaInfo> = vec![
            entity_generator::SchemaInfo {
                table_name: "entity".to_string(),
                column_name: "id".to_string(),
                data_type: "int".to_string(),
                is_nullable: "NO".to_string(),
            },
            entity_generator::SchemaInfo {
                table_name: "entity".to_string(),
                column_name: "name".to_string(),
                data_type: "varchar".to_string(),
                is_nullable: "NO".to_string(),
            },
            entity_generator::SchemaInfo {
                table_name: "entity".to_string(),
                column_name: "active".to_string(),
                data_type: "bit".to_string(),
                is_nullable: "NO".to_string(),
            },
            entity_generator::SchemaInfo {
                table_name: "entity".to_string(),
                column_name: "belongs_to_id".to_string(),
                data_type: "int".to_string(),
                is_nullable: "NO".to_string(),
            },
        ];
        let entity_schema = entity_generator::generate_schema(schema);
        let syntax_tree = syn::parse_file(ENTITY).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
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
