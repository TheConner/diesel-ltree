use diesel::prelude::*;
use diesel_ltree::Ltree;

#[derive(Queryable)]
pub struct Foo {
    pub id: i32,
    pub tree_path: Ltree
}