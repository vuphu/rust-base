use crate::modules::todos::controllers::todo_controller::{__path_create_todo, __path_get_todos};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get_todos, create_todo))]
pub struct ApiDoc;
