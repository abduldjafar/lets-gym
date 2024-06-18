use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse};
use serde::Serialize;

pub type Result<T> = core::result::Result<T,Error>;

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    LoginFail,
    DatabaseError(String),
    DataExist(String),
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}


impl std::error::Error for Error {}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        // Implement the conversion logic here
        // This could involve mapping the SurrealDB error to your custom error type
       Error::DatabaseError(error.to_string()) // Example conversion
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match &self {
            Error::LoginFail => {
                let mut response = Response::new(Body::new("login failed".to_string()));
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR.into();
                response
            },
            Error::DatabaseError(error) => {
				let mut response = Response::new(Body::new("There was a problem with the database".to_string()));
                println!("{}",error);
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR.into();

                response
                
			},
            Error::DataExist(id) => {
				let mut response = Response::new(Body::new(format!("Data with {} already registered",id)));
                println!("{} already registered",id);

                *response.status_mut() = StatusCode::NOT_ACCEPTABLE.into();

                response
                
			},
        }
    }
}