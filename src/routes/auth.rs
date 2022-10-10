use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

use crate::models::{Invitation, Pool, SlimUser, User};
use crate::error::ServiceError;
use crate::utils::hash_password;

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

pub async fn generate_invitation(
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    web::block(move || create_invitation(invitation_data.into_inner().email, pool)).await??;
    Ok(HttpResponse::Ok().finish())
}

fn create_invitation(
    eml: String,
    pool: web::Data<Pool>,
) -> Result<(), crate::error::ServiceError> {
    let inv = dbg!(create_inv_query(eml, pool)?);
    //TODO: send inv email here
    Ok(())
}

fn create_inv_query(
    eml: String,
    pool: web::Data<Pool>,
) -> Result<Invitation, crate::error::ServiceError> {
    use crate::schema::invitations::dsl::invitations;

    let new_inv: Invitation = eml.into();
    let mut conn = pool.get()?;
    let created_inv = diesel::insert_into(invitations)
        .values(&new_inv)
        .get_result(&mut conn)?;
    Ok(created_inv)
}
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(
    invitation_id: web::Path<String>,
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    web::block(move || {
        create_user_query(
            invitation_id.into_inner(),
            user_data.into_inner(),
            pool,
        )
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

fn create_user_query(
    inv_id: String,
    user_data: UserData,
    pool: web::Data<Pool>,
) -> Result<SlimUser, crate::error::ServiceError> {
    use crate::schema::invitations::dsl::{email, id, invitations};
    use crate::schema::users::dsl::users;
    let inv_id = uuid::Uuid::parse_str(&inv_id)?;

    let mut conn  = pool.get()?;
    invitations
        .filter(id.eq(inv_id))
        .filter(email.eq(&user_data.email))
        .load::<Invitation>(&mut conn)
        .map_err(|_db_err| ServiceError::BadRequest("Invalid Invitation".into()))
        .and_then(|mut res| {
            if let Some(invitation) = res.pop() {
                if invitation.expires_at > chrono::Local::now().naive_local() {
                    let password: String = hash_password(&user_data.password)?;
                    let user = User::from_details(&user_data.username, invitation.email, password);
                    let inserted_user: User = 
                        diesel::insert_into(users).values(&user).get_result(&mut conn)?;
                        dbg!(&inserted_user);
                        return Ok(inserted_user.into());
                }
            }
            Err(ServiceError::BadRequest("Invalid Invitation".into()))
        })
}
