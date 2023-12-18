mod todo;

pub fn scoped_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(todo::todo_page);
    cfg.service(todo::todo_table);
    cfg.service(todo::add);
    cfg.service(todo::del);
}
