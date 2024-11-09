use actix_web::{post, HttpResponse};
use actix_web::web::{Data, Json};
use bcrypt::{hash_with_salt, DEFAULT_COST};
use chrono::Utc;
use email_address::EmailAddress;
use jsonwebtoken::Header;
use sea_orm::{EntityTrait, SqlErr};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use log::error;
use crate::{AppState, SALT, SIGN_SECRET};
use crate::db::prelude::Users;
use crate::db::users::ActiveModel;

const MINIMAL_PASSWORD_LENGTH: usize = 8;
/// 1 Month
const TOKEN_EXPIRATION_TIME: i64 = 60*60*24*30;

#[derive(Deserialize)]
struct UserSchema {
    email: EmailAddress,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct JwtClaims {
    sub: Uuid,
    iat: i64,
    exp: i64,
}

#[post("/auth/register")]
pub async fn register(app_state: Data<AppState>, user_schema: Json<UserSchema>) -> HttpResponse {
    if user_schema.password.len() < MINIMAL_PASSWORD_LENGTH {
        return HttpResponse::BadRequest().body("The password is less than 8 characters");
    }

    let user = ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(user_schema.email.to_string()),
        password: Set(
            hash_with_salt(&user_schema.password, DEFAULT_COST, *SALT).unwrap().to_string()
        ),
    };

    let user = match Users::insert(
        user
    ).exec_with_returning(&app_state.db).await {
        Ok(u) => { u }
        Err(e) => {
            match e.sql_err() {
                None => {}
                Some(e) => {
                    if let SqlErr::UniqueConstraintViolation(_) = e {
                        return HttpResponse::BadRequest().body("User with provided email is already registered")
                    }
                }
            }

            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &JwtClaims {
            sub: user.id,
            iat: Utc::now().timestamp(),
            exp: Utc::now().timestamp() + TOKEN_EXPIRATION_TIME,
        },
        &*SIGN_SECRET
    ).unwrap();

    HttpResponse::Ok().body(token)
}

#[post("/auth/login")]
pub async fn login(app_state: Data<AppState>, user_schema: Json<UserSchema>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

