pub use sea_orm_migration::prelude::*;

mod m20241222_175303_create_uom_table;
mod m20241225_085030_create_category_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
            Box::new(m20241222_175303_create_uom_table::Migration),
            Box::new(m20241225_085030_create_category_table::Migration),
        ]
  }
}
