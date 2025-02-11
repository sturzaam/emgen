mod entities;

pub use entities::EntityGenerator;

use crate::databases::Database;

#[async_trait::async_trait]
pub trait MutableDatabase<D: Database> {
    fn database(&mut self) -> &mut D;
}

#[async_trait::async_trait]
pub trait Generator<D: Database>: MutableDatabase<D> { }
