use async_graphql::Context;

use crate::config::mongo::DataSource;
use crate::util::constant::GqlResult;
use crate::services::users::{
    self,
};

use crate::models::users::{User, NewUser};

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    // Add new user
    async fn user_register(
        &self,
        ctx: &Context<'_>,
        new_user: NewUser,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::user_register(db, new_user).await
    }

    // Change user password
    async fn user_change_password(
        &self,
        ctx: &Context<'_>,
        cur_password: String,
        new_password: String,
        token: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::user_change_password(
            db,
            &cur_password,
            &new_password,
            &token,
        )
        .await
    }

    // update user profile
    async fn user_update_profile(
        &self,
        ctx: &Context<'_>,
        new_user: NewUser,
        token: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::user_update_profile(db, new_user, &token).await
    }
}