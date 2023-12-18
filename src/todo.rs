mod index;
mod table;
use actix_web::{get, post, web, HttpResponse, Responder};
use tracing::instrument;

#[get("/")]
#[instrument(skip_all)]
async fn todo_page() -> impl Responder {
    index::TodoPage {}
}

#[get("/table")]
#[instrument(skip_all)]
async fn todo_table(db: web::Data<sqlx::Pool<sqlx::Postgres>>) -> impl Responder {
    match table::Todo::get_table(&db).await {
        Ok(v) => HttpResponse::Ok().body(table::TodoTable { todos: v }.to_string()),
        Err(_) => HttpResponse::BadRequest().body("データ取得失敗"),
    }
}

#[post("/api/add")]
#[instrument(skip_all)]
async fn add(
    form: web::Form<index::FormData>,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> impl Responder {
    match form.into_inner().add(&db).await {
        Ok(_) => match table::Todo::get_table(&db).await {
            Ok(v) => HttpResponse::Ok().body(table::TodoTable { todos: v }.to_string()),
            Err(_) => HttpResponse::BadRequest().body("データを追加できたが、データ追加に失敗"),
        },
        Err(_) => HttpResponse::BadRequest().body("データ追加失敗"),
    }
}

#[post("/api/del/{id}")]
#[instrument(skip_all)]
async fn del(path: web::Path<i32>, db: web::Data<sqlx::Pool<sqlx::Postgres>>) -> impl Responder {
    let id = path.into_inner();
    match table::Todo::del(id, db).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().body("データ削除に失敗"),
    }
}
