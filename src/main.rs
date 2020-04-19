#[macro_use]
extern crate diesel;
use diesel::{
    pg::{Pg, PgConnection},
    query_dsl::{QueryDsl, RunQueryDsl},
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods,
};

// Declare the SQL table
table! {
    use diesel::sql_types::*;
    tunnels (id) {
        id -> BigInt,
        name -> Text,
        active -> Bool,
    }
}

// Declare a Rust struct that maps to that table
#[derive(Debug, Queryable, Identifiable)]
struct Tunnel {
    id: i64,
    name: String,
    active: bool,
}

enum Name {
    Adam,
    Brian,
}

fn main() {
    // Set up the connection
    let manager = ConnectionManager::<PgConnection>::new("");
    let pool = Pool::builder().build(manager).unwrap();
    let conn = pool.get().unwrap();

    // Update some rows
    let tunnels_to_update = get_tunnels(Name::Adam);
    diesel::update(tunnels_to_update)
        //         ^^^^^^^^^^^^^^^^^ the trait `diesel::Identifiable` is not implemented for
        // `diesel::query_builder::BoxedSelectStatement<'_, (diesel::sql_types::BigInt,
        // diesel::sql_types::Text, diesel::sql_types::Bool), tunnels::table, diesel::pg::Pg>`
        .set(tunnels::active.eq(true))
        .execute(&conn);
}

// Queries all tunnels that meet a filter based on name.
fn get_tunnels<'a>(name: Name) -> tunnels::BoxedQuery<'a, Pg> {
    let mut query = tunnels::table.select(tunnels::all_columns).into_boxed();
    match name {
        Name::Adam => query = query.filter(tunnels::name.eq("adam")),
        Name::Brian => query = query.filter(tunnels::name.eq("brian")),
    }
    query
}
