use async_graphql::Context;

use crate::config::mongo::DataSource;
use crate::util::constant::GqlResult;
use crate::services::users::{
    self
};
use crate::models::users::{User, NewUser, SignInfo};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // get user info by email
    async fn get_user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::get_user_by_email(db, &email).await
    }

    // get user info by username
    async fn get_user_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::get_user_by_username(db, &username).await
    }

    async fn user_sign_in(
        &self,
        ctx: &Context<'_>,
        unknown_user: NewUser,
    ) -> GqlResult<SignInfo> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::user_sign_in(db, unknown_user).await
    }

    // Get all Users,
    async fn all_users(
        &self,
        ctx: &Context<'_>,
        token: String,
    ) -> GqlResult<Vec<User>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::all_users(db, &token).await
    }
}