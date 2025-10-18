use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;
use zero2prod;

use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 如果不能读取配置的话，则发生panic
    let configuration = get_configuration().expect("读取配置失败");
    // 移除硬编码值，从配置中读取
    let address = format!("127.0.0.1:{}", configuration.application_port);
    //如果绑定失败，则会返回Error
    //否则，在Server上调用await
    println!();
    let listener = TcpListener::bind(address)?;
    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("数据库连接失败");
    zero2prod::run(listener, connection_pool)?.await
}
