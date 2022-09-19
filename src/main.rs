fn main() {
    #[cfg(feature = "unit")]
    let db = MockDatabase {};
    #[cfg(feature = "it")]
    let db = SqlitDatabase {};
    #[cfg(feature = "prod")]
    let db = PostgreSqlDatabase {};
    db.do_stuff();
}

trait Database {
    fn do_stuff(self: Self);
}

#[cfg(feature = "unit")]
struct MockDatabase {}
#[cfg(feature = "it")]
struct SqlitDatabase {}
#[cfg(feature = "prod")]
struct PostgreSqlDatabase {}

#[cfg(feature = "unit")]
impl Database for MockDatabase {
    fn do_stuff(self: Self) {
        println!("Do mock stuff");
    }
}

#[cfg(feature = "it")]
impl Database for SqlitDatabase {
    fn do_stuff(self: Self) {
        println!("Do stuff with SQLite");
    }
}

#[cfg(feature = "prod")]
impl Database for PostgreSqlDatabase {
    fn do_stuff(self: Self) {
        println!("Do stuff with PostgreSQL");
    }
}
