use langchain_rust::semantic_router::{RouteLayerBuilder, Router};

#[tokio::main]
async fn main() {
    let captial_route = Router::new(
        "captial",
        &[
            "Capital of France is Paris.",
            "What is the captial of France?",
        ],
    );
    let weather_route = Router::new(
        "temperature",
        &[
            "What is the temperature?",
            "Is it raining?",
            "Is it cloudy?",
        ],
    );
    let router_layer = RouteLayerBuilder::default()
        .add_route(captial_route)
        .add_route(weather_route)
        .threshold(0.82)
        .build()
        .await
        .unwrap();

    let routes = router_layer
        .call("What is the temperature in Peru?")
        .await
        .unwrap();

    println!("{:?}", routes);
}
