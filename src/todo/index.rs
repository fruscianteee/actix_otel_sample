#[derive(askama::Template)]
#[template(path = "index.html")]
pub struct TodoPage;

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    pub text: String,
}

impl FormData {
    pub async fn add(
        self,
        db: &actix_web::web::Data<sqlx::Pool<sqlx::Postgres>>,
    ) -> anyhow::Result<()> {
        let mut tx = db.begin().await.expect("transaction error.");
        let _ = sqlx::query!("insert into todo (text) values ($1)", self.text)
            .execute(tx.as_mut()) // <-- here
            .await?;

        tx.commit().await?;
        Ok(())
    }
}
