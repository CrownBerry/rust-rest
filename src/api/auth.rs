use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token};
use rocket::{http::Status, Outcome};
use rocket::request::{self, FromRequest, Request};
use serde_derive::{Deserialize, Serialize};

use crate::user::model::User;

pub struct ApiKey(pub String);

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    success: bool,
    token: String
}

impl TokenResponse {
    pub fn from(token: String) -> TokenResponse {
        TokenResponse {
            success: true,
            token
        }
    }
}

static SECRET_KEY: &[u8] = b"secret key";

fn read_token(key: &str) -> Result<String, String> {
    let token = Token::<Header, Registered>::parse(key)
        .map_err(|_| "Unable to parse key to string")?;

    if token.verify(SECRET_KEY, Sha256::new()) {
        token.claims.sub.ok_or("Claims not valid".to_string())
    } else {
        Err("Token not valid".to_string())
    }
}

pub fn generate_token(user: User) -> Result<String, Status> {
    let header: Header = Default::default();
    let claims = Registered {
        sub: Some(user.username.into()),
        ..Default::default()
    };
    let token = Token::new(header, claims);

    token.signed(SECRET_KEY, Sha256::new())
        .map_err(|_| Status::InternalServerError)
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        match read_token(keys[0]) {
            Ok(claim) => Outcome::Success(ApiKey(claim)),
            Err(_) => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
