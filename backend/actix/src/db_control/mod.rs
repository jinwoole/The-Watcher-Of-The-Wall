use tokio_postgres::{Client, NoTls};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Database {
    client: Client,
}

// 분명 멀티스레딩 문제가 발생할거고, DB연결하는 Instance는 안전하게 보호되어야 함
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
}

impl Database {
    // 새로운 Database 인스턴스를 생성하는 함수
    pub async fn new(connection_string: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Database { client })
    }

    // SQL 쿼리를 실행하고 결과를 반환하는 함수
    pub async fn execute_query(&self, query: &str) -> Result<Vec<tokio_postgres::Row>, Box<dyn std::error::Error>> {
        let rows = self.client.query(query, &[]).await?;
        Ok(rows)
    }
}

pub fn load_connection_string() -> String {

    // 환경 변수에서 데이터베이스 연결 정보 설정
    let db_host = env::var("DB_HOST").expect("DB_HOST 환경 변수가 설정되지 않았습니다.");
    let db_user = env::var("DB_USER").expect("DB_USER 환경 변수가 설정되지 않았습니다.");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD 환경 변수가 설정되지 않았습니다.");
    let db_name = env::var("DB_NAME").expect("DB_NAME 환경 변수가 설정되지 않았습니다.");

    let connection_string = format!(
        "host={} user={} password={} dbname={}",
        db_host, db_user, db_password, db_name
    );

    connection_string
}
