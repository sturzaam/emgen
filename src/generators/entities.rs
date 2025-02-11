use convert_case::{Case, Casing};

use crate::databases::Database;
use super::{MutableDatabase, Generator};

pub struct EntityGenerator<D: Database> {
    database: D,
}

impl<D: Database> EntityGenerator<D> {
    pub fn new(database: D) -> Self {
        Self { database }
    }

    pub async fn entity(&mut self) -> String {
        let mut entity_string = String::new();
        entity_string.push_str("#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]\n");
        entity_string.push_str(&format!("#[sea_orm(table_name = \"{}\")]\n", self.database().get_table_name()));
        entity_string.push_str("pub struct Model {\n");
        for column in self.database.get_schema_info() {
            let rust_type = self.database.map_sql_type_to_rust(&column.data_type, column.is_nullable == "YES");
            if column.column_name == "id" {
                entity_string.push_str("#[sea_orm(primary_key)]\n");
            }
            entity_string.push_str(&format!("pub {}: {},\n", column.column_name.to_case(Case::Snake), rust_type));
        }
    
        entity_string.push_str("}\n");
    
        prettyplease::unparse(&syn::parse_file(&entity_string).unwrap())
    }
    
}

impl<D: Database> MutableDatabase<D> for EntityGenerator<D> {
    fn database(&mut self) -> &mut D {
        &mut self.database
    }
}


impl<D: Database> Generator<D> for EntityGenerator<D> {}