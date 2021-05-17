use crate::app::AppState;
use crate::db::ConnectionPool;
use crate::error::Error;
use crate::message::user::CreateUser;
use crate::message_handler::create_user_handler::CreateUserHandler;
use crate::message_handler::MessageHandler;
use crate::service::jwt::CanGenerateJwt;
use actix_web::{web, HttpResponse, Result};
use diesel::dsl::exists;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

pub async fn register(
    registration: web::Json<Register>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let registration = &registration.into_inner();
    registration.validate()?;

    let db_pool = app_state.db_connection_pool.clone();
    validate_user_exists(&registration.username, &db_pool)?;

    let create_user = CreateUser {
        username: registration.username.clone(),
        password: registration.password.clone(),
        user_id: Uuid::new_v4(),
        email: if registration.email.is_empty() {
            Some(registration.email.clone())
        } else {
            None
        },
    };

    let user = CreateUserHandler(db_pool).handle(create_user).await?;

    Ok(HttpResponse::Ok().json(json!({
        "token": user.generate_jwt().unwrap(),
    })))
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct Register {
    #[validate(length(min = 6, max = 30, message = "Username must be 6-30 charaters long"))]
    pub username: String,
    #[validate(email(message = "Email not valid"))]
    pub email: String,
    #[validate(length(min = 8, max = 30, message = "Password must be 8-30 chracters long"))]
    pub password: String,
}

// Check if username is taken, it is best to do this
// using a custom validator But I didn't find a way to pass
// db connection, so let use this workaround instead.
fn validate_user_exists(username: &str, conn: &ConnectionPool) -> Result<(), ValidationErrors> {
    use crate::schema::user::{columns, table};

    let conn = &conn.get().unwrap();

    let exists =
        diesel::select(exists(table.filter(columns::username.eq(username)))).get_result(conn);
    if let Ok(true) = exists {
        let mut errors = ValidationErrors::new();
        errors.add("username", ValidationError::new("usename-is-taken"));

        return Err(errors);
    }

    Ok(())
}
