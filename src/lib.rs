use serde::{Deserialize, Serialize};

pub mod admin;
pub mod auth;
pub mod fs;
pub mod public;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<Data> {
    code: isize,
    message: String,
    data: Option<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NullResponse;
