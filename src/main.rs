use axum::Router;
use basic_strategies::{LexicographicallyFirst, MinOpptMoves, Random};

#[tokio::main]
pub async fn main() {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(
            Router::new()
                .nest_service(
                    "/random",
                    engine_trait::server::serve_engine(Random {}).await,
                )
                .nest_service(
                    "/min_oppt_moves",
                    engine_trait::server::serve_engine(MinOpptMoves {}).await,
                )
                .nest_service(
                    "/lexicographically_first",
                    engine_trait::server::serve_engine(LexicographicallyFirst {}).await,
                )
                .into_make_service(),
        )
        .await
        .unwrap();
}
