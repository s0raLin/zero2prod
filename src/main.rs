use std::net::TcpListener;

use zero2prod;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //如果绑定失败，则会返回Error
    //否则，在Server上调用await
    let listener = TcpListener::bind("127.0.0.1:0")?;
    zero2prod::run(listener)?.await
}
