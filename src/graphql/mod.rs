pub mod mutations;
pub mod queries;

use tide::{http::mime, Request, Response, StatusCode, Body};

use async_graphql::{
    Schema, EmptySubscription,
    http::{playground_source, GraphQLPlaygroundConfig, receive_json},
};

use crate::State;

use crate::util::constant::CFG;
use crate::config::mongo;
use crate::graphql::{queries::QueryRoot, mutations::MutationRoot};

type ActixSchema = Schema<
    queries::QueryRoot,
    mutations::MutationRoot,
    async_graphql::EmptySubscription,
>;

pub async fn build_schema() -> ActixSchema {
    let mongo_ds = mongo::DataSource::init().await;

    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(mongo_ds)
        .finish()
}

pub async fn graphql(req: Request<State>) -> tide::Result {
    let schema = req.state().schema.clone();
    let gql_resp = schema.execute(receive_json(req).await?).await;

    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(Body::from_json(&gql_resp)?);

    Ok(resp.into())
}

pub async fn graphiql(_: Request<State>) -> tide::Result {
    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(playground_source(GraphQLPlaygroundConfig::new(
        CFG.get("GRAPHQL_PATH").unwrap(),
    )));
    resp.set_content_type(mime::HTML);

    Ok(resp.into())
}