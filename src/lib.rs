mod todo;
use actix_web::web;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(todo::todo_page);
    cfg.service(todo::todo_table);
    cfg.service(todo::add);
    cfg.service(todo::del);
}
