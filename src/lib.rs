pub mod configuration;
pub mod routes;
// pub mod stratup;

use std::{net::TcpListener, sync::Arc};

use actix_web::{dev::Server, web::{self, Form}, App, HttpRequest, HttpResponse, HttpServer, Responder};

use routes::health_check;
use routes::subscriptions;
use sqlx::{PgConnection, PgPool};

// 在正常情况下返回Server,并删除了async关键字
pub fn run(
    listener: TcpListener,
    connection: PgPool //新增的内容
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
            // 使用App上的app_data方法可以向应用程序添加状态信息
            .app_data(db_pool.clone())  
            

    })
    .listen(listener)?
    .run(); //这里没有.await
    Ok(server)
}

