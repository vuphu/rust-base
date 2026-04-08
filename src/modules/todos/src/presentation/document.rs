use crate::presentation::controllers::todo_controller::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get_todos, create_todo))]
pub struct ApiDoc;
