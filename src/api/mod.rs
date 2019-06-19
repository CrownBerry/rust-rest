use std::error::Error;

use diesel::result::Error as DieselError;
use rocket::{http::Status, Request};
use rocket::response::{Responder, Response};
use rocket_contrib::json::Json;
use serde_derive::{Deserialize, Serialize};

pub mod auth;

#[derive(Serialize, Deserialize)]
pub struct SuccessResponse {
    success: bool
}

impl SuccessResponse {
    pub fn new(is: bool) -> SuccessResponse {
        SuccessResponse { success: is }
    }
}

#[derive(Debug)]
pub enum ApiError {
    DieselError(DieselError)
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub type ApiResult<T> = Result<Json<T>, ApiError>;

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
        let message = match self {
            ApiError::DieselError(error) => {
                if error == DieselError::NotFound {
                    return Err(Status::NotFound);
                }
                String::from(error.description())
            }
        };

        Json(ErrorResponse { status: "error", message }).respond_to(request)
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        ApiError::DieselError(error)
    }
}
