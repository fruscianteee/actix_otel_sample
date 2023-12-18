use tracing::{info, instrument};

#[derive(askama::Template, Debug, Default)]
#[template(path = "table.html")]
pub struct TodoTable {
    pub todos: Vec<Todo>,
}

#[derive(Debug, Default)]
pub struct Todo {
    pub id: i32,
    pub text: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

impl Todo {
    #[instrument(skip_all)]
    pub async fn get_table(
        db: &actix_web::web::Data<sqlx::Pool<sqlx::Postgres>>,
    ) -> anyhow::Result<Vec<Self>> {
        let result = sqlx::query_as!(Todo, "select * from todo")
            .fetch_all(db.as_ref()) // <-- here
            .await
            .unwrap();
        Ok(result)
    }

    #[instrument(skip_all)]
    pub async fn del(
        target: i32,
        db: actix_web::web::Data<sqlx::Pool<sqlx::Postgres>>,
    ) -> anyhow::Result<()> {
        let mut tx = db.begin().await.expect("transaction error.");
        let _ = sqlx::query!("delete from todo where id = $1;", target)
            .execute(tx.as_mut()) // <-- here
            .await?;

        tx.commit().await?;
        info!("delete todo!");
        Ok(())
    }
}
