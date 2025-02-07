pub mod test_entity;

const ENTITY: &str = stringify! {
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
    #[sea_orm(table_name = "entity")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
        pub active: bool,
        pub belongs_to_id: i32,
    }
};