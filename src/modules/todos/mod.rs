use actix_web::web;

pub mod controllers;
pub mod dto;
pub mod models;
pub mod repositories;
pub mod services;

pub fn configure(config: &mut web::ServiceConfig) {
    controllers::todo_controller::configure(config)
}
