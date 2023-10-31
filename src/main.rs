use basic_strategies::Random;

#[tokio::main]
pub async fn main() {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(
            engine_trait::server::serve_engine(Random {})
                .await
                .into_make_service(),
        )
        .await
        .unwrap();
}
