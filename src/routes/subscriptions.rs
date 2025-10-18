use actix_web::{web::{self, Form}, HttpResponse, Responder};
use sqlx::{PgConnection, PgPool};

use uuid::Uuid;
use chrono::Utc;


#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}


pub async fn subscriptions(
    form: Form<FormData>,
    //从应用程序状态中取出连接
    pool: web::Data<PgPool>
) -> impl Responder {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscriptions_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    ).execute(pool.get_ref()).await {
      Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
