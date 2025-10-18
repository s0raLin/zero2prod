use std::{fmt::format, net::TcpListener};

use reqwest::Client;
use scraper::{Html, Selector};
use sqlx::{Connection, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};

use sqlx::prelude::Executor;  // 或者 use sqlx::Executor;

struct TestApp {
    address: String,
    connect_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    // 需要引入reqWest对应用程序执行HTTP请求
    let client = reqwest::Client::new();

    //执行
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // 断言
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// 在后台某处启动应用程序
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    // 通过下面代码将数据库名随机化，前提是先创建对应的数据库
    let mut configuration = get_configuration().expect("读取配置失败");
    configuration.database.database_name = Uuid::new_v4().to_string();
    
    
    let address = format!("http://127.0.0.1:{}", port);

    let connect_pool = configure_database(&configuration.database).await;
    let server = zero2prod::run(listener, connect_pool.clone()).expect("Failed to bind address");
    //启动服务器作为后台任务
    //tokio::spawn返回一个指向spawned future的handle
    //但是这里没有用它
    let _ = tokio::spawn(server);

    TestApp {
        address,
        connect_pool,
    }
}

async fn configure_database(config: &DatabaseSettings)->PgPool {
    // 创建数据库
    let mut connection = PgConnection::connect(&config.connection_string_without_db()).await.expect("数据库连接失败");
    connection.execute(format!(r#"create database "{}";"#, &config.database_name).as_str()).await.expect("创建数据库失败");

    // 迁移数据库
    let connection_pool = PgPool::connect(&config.connection_string()).await.expect("迁移数据库连接失败");
    sqlx::migrate!("tests/migrations").run(&connection_pool).await.expect("迁移数据库失败");

    connection_pool
}

#[tokio::test]
async fn subcribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let connection_pool = app.connect_pool;

    let body = "name=cangli&email=gl0wniapar@gmail.com";
    let response = client
        .post(format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("select email, name from subscriptions").fetch_one(&connection_pool).await.expect("Failed to fetch");


    assert_eq!(saved.email, "gl0wniapar@gmail.com");
    assert_eq!(saved.name, "cangli");
    
}

#[tokio::test]
async fn subcribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![("name=le%20guin", "missing the email")];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(400, response.status().as_u16())
    }
}


