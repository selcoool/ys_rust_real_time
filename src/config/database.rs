use sqlx::MySqlPool;

pub async fn connect_db() -> MySqlPool {
    MySqlPool::connect(
        "mysql://root:159753@localhost:3306/ys_rust_real_time"
    )
    .await
    .unwrap()
}