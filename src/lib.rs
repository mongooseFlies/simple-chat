use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utils;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Client {
    Subscribe {
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Server {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Err(String),
}
