use actix_web::web;

mod controllers;
mod entities;
mod repositories;
mod services;

pub fn configure(config: &mut web::ServiceConfig) {
    controllers::todo_controller::configure(config)
}
