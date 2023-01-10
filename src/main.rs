use handler::{
    create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler,
    health_checker_handler, todos_list_handler,
};

#[macro_use]
extern crate rocket;

mod handler;
mod model;
mod response;

#[launch]
fn rocket() -> _ {
    let app_data = model::AppState::init();
    rocket::build().manage(app_data).mount(
        "/api",
        routes![
            health_checker_handler,
            todos_list_handler,
            create_todo_handler,
            get_todo_handler,
            edit_todo_handler,
            delete_todo_handler
        ],
    )
}
