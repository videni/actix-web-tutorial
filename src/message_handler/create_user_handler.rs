use crate::db::ConnectionPool;
use crate::message::user::CreateUser;
use crate::message_handler::MessageHandler;
use crate::model::user::{NewUser, User as UserModel};
use crate::service::security::encoder::Argon2PasswordEncoder;
use crate::service::security::encoder::PasswordEncoder;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

pub struct CreateUserHandler(pub ConnectionPool);

#[async_trait]
impl MessageHandler for CreateUserHandler {
    type Message = CreateUser;
    type Reply = UserModel;
    type Error = DieselError;

    async fn handle(&self, msg: CreateUser) -> Result<UserModel, DieselError> {
        use crate::schema::user::table;

        let encoder = Argon2PasswordEncoder {};

        let new_user = NewUser {
            username: msg.username.clone(),
            email: msg.email,
            password: encoder.encode_password(&msg.password, None),
        };

        let conn = &self.0.get().unwrap();

        let user = diesel::insert_into(table)
            .values(new_user)
            .get_result::<UserModel>(conn)?;

        return Ok(user);
    }
}
