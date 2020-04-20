#[macro_use]
extern crate diesel;
use diesel::{
    expression::BoxableExpression,
    pg::{Pg, PgConnection},
    query_dsl::{QueryDsl, RunQueryDsl},
    r2d2::{ConnectionManager, Pool},
    sql_types::Bool,
    ExpressionMethods,
};

// Declare the SQL table
table! {
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
    let _update_query = diesel::update(tunnels::table)
        .filter(find_user(Name::Adam))
        .set(tunnels::active.eq(false))
        .execute(&conn);

    // Query some rows
    let _select_query: Vec<Tunnel> = tunnels::table
        .filter(find_user(Name::Brian))
        .get_results(&conn)
        .unwrap();
}

fn find_user(name: Name) -> Box<dyn BoxableExpression<tunnels::table, Pg, SqlType = Bool>> {
    match name {
        Name::Adam => Box::new(tunnels::name.eq("adam")),
        Name::Brian => Box::new(tunnels::name.eq("brian")),
    }
}
