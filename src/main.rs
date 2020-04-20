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

// Filters that might be applied to find a tunnel
pub struct Filter {
    name: Name,
    active: bool,
}

enum Name {
    Adam,
    Brian,
}

impl Filter {
    // The `Filter` type can be mapped 1:1 onto sets of Diesel filter predicates.
    // The predicates can be applied to either updates or selects!
    fn as_diesel(&self) -> Vec<Box<dyn BoxableExpression<tunnels::table, Pg, SqlType = Bool>>> {
        let f0 = match self.name {
            Name::Adam => tunnels::name.eq("adam"),
            Name::Brian => tunnels::name.eq("brian"),
        };
        let f1 = if self.active {
            tunnels::active.eq(true)
        } else {
            tunnels::active.eq(false)
        };
        vec![Box::new(f0), Box::new(f1)]
    }
}

fn main() {
    // Set up the connection
    let manager = ConnectionManager::<PgConnection>::new("");
    let pool = Pool::builder().build(manager).unwrap();
    let conn = pool.get().unwrap();

    let filter = Filter {
        name: Name::Adam,
        active: false,
    };

    // Update some rows, applying filters
    let mut update_query = diesel::update(tunnels::table).into_boxed();
    for f in filter.as_diesel() {
        update_query = update_query.filter(f);
    }
    let _deleted_tunnels = update_query.set(tunnels::active.eq(false)).execute(&conn);

    // Query some rows, applying filters
    let mut q = tunnels::table.into_boxed();
    for f in filter.as_diesel() {
        q = q.filter(f);
    }
    let _tunnels: Vec<Tunnel> = q.get_results(&conn).unwrap();
}
