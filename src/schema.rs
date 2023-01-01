// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ltree"))]
    pub struct Ltree;
}

diesel::table! {
    use diesel::sql_types::*;
    // The diesel codegen adds this
    //use super::sql_types::Ltree;
    // Replacing it with this fixes the problem:
    use diesel_ltree::sql_types::Ltree;

    foo (id) {
        id -> Int4,
        tree_path -> Ltree,
    }
}
