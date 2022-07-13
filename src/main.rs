mod config;
mod util;
mod graphql;
mod services;
mod models;

use crate::util::constant::CFG;

use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};

use crate::graphql::{build_schema, graphql, graphiql};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    let schema = build_schema().await;
    let app_state = State { schema: schema };
    let mut app = tide::with_state(app_state);

    //environment variables defined in .env file
    app.at(CFG.get("GRAPHQL_PATH").unwrap()).post(graphql);
    app.at(CFG.get("GRAPHIQL_PATH").unwrap()).get(graphiql);

    // suggest to specify urls
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    app.with(cors);

    app.listen(format!(
        "{}:{}",
        CFG.get("ADDRESS").unwrap(),
        CFG.get("PORT").unwrap()
    ))
    .await?;

    Ok(())
}

#[derive(Clone)]
pub struct State {
    pub schema: async_graphql::Schema<
        graphql::queries::QueryRoot,
        graphql::mutations::MutationRoot,
        async_graphql::EmptySubscription,
    >,
}