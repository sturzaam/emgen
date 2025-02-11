#[cfg(test)]
mod tests {

    use std::env;
    use emgen::databases::Database;
    use emgen::databases::mssql::Mssql;
    use convert_case::{Casing, Case};

    #[test]
    fn test_map_sql_type_to_rust() {
        let database = Mssql::default();
        insta::assert_yaml_snapshot!(database.map_sql_type_to_rust("int", false), @"i32");
        insta::assert_yaml_snapshot!(database.map_sql_type_to_rust("varchar", true), @"Option<String>");
        insta::assert_yaml_snapshot!(database.map_sql_type_to_rust("datetime", false), @r#""chrono::DateTime<chrono::Utc>""#);
        insta::assert_yaml_snapshot!(database.map_sql_type_to_rust("decimal", false), @"f64");
        insta::assert_yaml_snapshot!(database.map_sql_type_to_rust("uniqueidentifier", false), @r#""uuid::Uuid""#);
    }

    #[test]
    fn test_case_conversion() {
        insta::assert_yaml_snapshot!("entiy_name".to_case(Case::Pascal), @"EntiyName");
        insta::assert_yaml_snapshot!("EntityName".to_case(Case::Snake), @"entity_name");
    }
}
