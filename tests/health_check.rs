use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    // 需要引入reqWest对应用程序执行HTTP请求
    let client = reqwest::Client::new();

    //执行
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // 断言
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// 在后台某处启动应用程序
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    //启动服务器作为后台任务
    //tokio::spawn返回一个指向spawned future的handle
    //但是这里没有用它
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
