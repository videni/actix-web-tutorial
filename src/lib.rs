
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate log;
#[macro_use]
extern crate actix_cors;
#[macro_use]
extern crate lazy_static;

pub mod service;
pub mod model;
pub mod message;
pub mod message_handler;
pub mod schema;
pub mod db;
pub mod error;
pub mod prelude;
pub mod app;
pub mod controller;
pub mod route;