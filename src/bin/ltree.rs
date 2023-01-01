use diesel_ltree_mwe::{establish_connection};
use diesel_ltree_mwe::models::*;
use diesel::prelude::*;
use diesel_ltree::Ltree;

fn main() {
    use diesel_ltree_mwe::schema::foo;

    let mut connection = establish_connection();
    
    // Make some foo
    let result = diesel::insert_into(foo::table)
        .values(&vec![
          foo::tree_path.eq(Ltree("foo".to_string()))
        ])
        .execute(&mut connection)
        .expect("Error inserting foo");

    println!("Made some foo {}", result);

    // Get some foo
    let result = foo::table
          .limit(5)
          .load::<Foo>(&mut connection)
          .expect("Error loading foo");

    for foo in result {
      println!("Foo Id {} Path {:?}", foo.id, foo.tree_path);
    }
}