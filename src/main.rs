use crate::config::load::load_or_create_config;

pub mod agents;
pub mod config;
pub mod models;
pub mod prompts;
pub mod providers;
pub mod tools;
pub mod ui;

fn main() {
   load_or_create_config();
}
