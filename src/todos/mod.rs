use actix_web::web;

mod controllers;
mod entities;
mod repositories;
mod services;
mod dto;

pub fn configure(config: &mut web::ServiceConfig) {
    controllers::todo_controller::configure(config)
}
