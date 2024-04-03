use langchain_rust::llm::openai::OpenAI;
use langchain_rust::{
    chain::{options::ChainCallOptions, Chain, SQLDatabaseChain, SQLDatabaseChainBuilder},
    tools::{postgres::PostgreSQLEngine, SQLDatabaseBuilder},
};

use std::{
    env,
    io::{self, Write},
}; // Include io Library for terminal input

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let options = ChainCallOptions::default();
    let llm = OpenAI::default();

    let db = std::env::var("DATABASE_URL").unwrap();
    let engine = PostgreSQLEngine::new(&db).await.unwrap();
    let db = SQLDatabaseBuilder::new(engine).build().await.unwrap();
    let chain: SQLDatabaseChain = SQLDatabaseChainBuilder::new()
        .llm(llm)
        .top_k(4)
        .database(db)
        .options(options)
        .build()
        .expect("Failed to build LLMChain");

    print!("Please enter a question: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    let input_variables = chain.prompt_builder().query(input).build();
    match chain.invoke(input_variables).await {
        Ok(result) => {
            println!("Result: {:?}", result);
        }
        Err(e) => panic!("Error invoking LLMChain: {:?}", e),
    }
}
