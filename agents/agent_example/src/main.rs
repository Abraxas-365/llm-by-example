use std::{env, sync::Arc};

use langchain_rust::{
    agent::{AgentExecutor, ChatOutputParser, ConversationalAgentBuilder},
    chain::{options::ChainCallOptions, Chain},
    llm::openai::OpenAI,
    memory::SimpleMemory,
    prompt_args,
    tools::Wolfram,
};

use std::io::{self, Write}; // Include io Library for terminal input

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let llm = OpenAI::default();
    let memory = SimpleMemory::new();
    let wolfram_tool = Wolfram::default();
    let agent = ConversationalAgentBuilder::new()
        .tools(vec![Arc::new(wolfram_tool)])
        .output_parser(ChatOutputParser::new().into())
        .options(ChainCallOptions::new().with_max_tokens(1000))
        .build(llm)
        .unwrap();

    print!("Please enter a question: ");
    io::stdout().flush().unwrap(); // Display prompt to terminal

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Get product from terminal input

    let input = input.trim();
    let input_variables = prompt_args! {
        "input" => input,
    };

    let executor = AgentExecutor::from_agent(agent).with_memory(memory.into());
    match executor.invoke(input_variables).await {
        Ok(result) => {
            println!("Result: {:?}", result);
        }
        Err(e) => panic!("Error invoking LLMChain: {:?}", e),
    }
}
