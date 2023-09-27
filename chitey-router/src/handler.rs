use std::future::Future;

use crate::resource::Responder;

pub struct Error {
    cause: Box<dyn std::error::Error>,
}
