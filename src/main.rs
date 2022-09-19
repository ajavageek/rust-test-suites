#[cfg(all(feature = "unit", feature = "it"))]
compile_error!("feature \"unit\" and feature \"it\" cannot be enabled at the same time");
#[cfg(all(feature = "unit", feature = "prod"))]
compile_error!("feature \"unit\" and feature \"prod\" cannot be enabled at the same time");
#[cfg(all(feature = "it", feature = "prod"))]
compile_error!("feature \"it\" and feature \"prod\" cannot be enabled at the same time");

#[cfg(feature = "prod")]
fn main() {
    let db = PostgreSqlDatabase {};
    println!("{}", db.do_stuff());
}

trait Database {
    fn do_stuff(self: Self) -> &'static str;
}

#[cfg(feature = "unit")]
struct MockDatabase {}
#[cfg(feature = "it")]
struct SqlitDatabase {}
#[cfg(feature = "prod")]
struct PostgreSqlDatabase {}

#[cfg(feature = "unit")]
impl Database for MockDatabase {
    fn do_stuff(self: Self) -> &'static str {
        "Do mock stuff"
    }
}

#[cfg(feature = "it")]
impl Database for SqlitDatabase {
    fn do_stuff(self: Self) -> &'static str {
        "Do stuff with SQLite"
    }
}

#[cfg(feature = "prod")]
impl Database for PostgreSqlDatabase {
    fn do_stuff(self: Self) -> &'static str {
        "Do stuff with PostgreSQL"
    }
}

#[test]
#[cfg(feature = "unit")]
fn test_unit() {
    let db = MockDatabase {};
    assert_eq!(db.do_stuff(), "Do mock stuff");
}

#[test]
#[cfg(feature = "it")]
fn test_it() {
    let db = SqlitDatabase {};
    assert_eq!(db.do_stuff(), "Do stuff with SQLite");
}
