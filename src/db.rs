use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use std::env;
use tokio_postgres::NoTls;

//type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;
//static DB_URL: &str = "postgres://postgres:postgres@localhost:5432/axum";
static mut DB_POOL: Option<Pool<PostgresConnectionManager<NoTls>>> = None;

pub async fn connect() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", database_url);
    let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
    unsafe {
        DB_POOL = Some(pool);
    }
}
pub async fn get_pool() -> Pool<PostgresConnectionManager<NoTls>> {
    unsafe {
        return DB_POOL.as_ref().unwrap().clone();
    }
}
