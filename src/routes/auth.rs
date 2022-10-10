use actix_identity::Identity;
use actix_web::{web, HttpResponse, HttpRequest, Responder, HttpMessage};
use diesel::prelude::*;
use serde::Deserialize;

use crate::{models::{Invitation, Pool, SlimUser, User}, utils::verify};

#[derive(Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

pub type LoggedUser = SlimUser;

pub async fn login(request: HttpRequest, auth_data: web::Json<AuthData>, pool: web::Data<Pool>) -> Result<HttpResponse, actix_web::Error> {
    let user = web::block(move || login_query(auth_data.into_inner(), pool)).await??;
    match Identity::login(&request.extensions(), user.username) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

fn login_query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<LoggedUser, crate::error::ServiceError> {
    use crate::schema::users::dsl::{users, email};

    let mut conn = pool.get()?;
    let user = users
        .filter(email.eq(auth_data.email))
        .first::<User>(&mut conn)?;
    if verify(&user.hash, &auth_data.password)? {
        Ok(user.into())
    } else {
        Err(crate::error::ServiceError::Unauthorized)
    }
}

pub async fn logout(user: Identity) -> HttpResponse {
    user.logout();
    HttpResponse::Ok().finish()
}

pub async fn get_me(user: Option<Identity>) -> Result<HttpResponse, actix_web::Error> {
    // TODO: Generate a token for the user
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user.id().unwrap()))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}