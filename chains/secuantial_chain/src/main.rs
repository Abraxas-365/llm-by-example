use langchain_rust::{
    chain::{Chain, LLMChainBuilder},
    llm::openai::{OpenAI, OpenAIModel},
    prompt::HumanMessagePromptTemplate,
    prompt_args, sequential_chain, template_jinja2,
};
use std::io::{self, Write}; // Include io Library for terminal input

#[tokio::main]
async fn main() {
    let llm = OpenAI::default().with_model(OpenAIModel::Gpt35);
    let prompt = HumanMessagePromptTemplate::new(template_jinja2!(
        "Give me a creative name for a store that sells: {{producto}}",
        "producto"
    ));

    let get_name_chain = LLMChainBuilder::new()
        .prompt(prompt)
        .llm(llm.clone())
        .output_key("name")
        .build()
        .unwrap();

    let prompt = HumanMessagePromptTemplate::new(template_jinja2!(
        "Give me a slogan for the following store: {{name}}",
        "name"
    ));
    let get_slogan_chain = LLMChainBuilder::new()
        .prompt(prompt)
        .llm(llm.clone())
        .output_key("slogan")
        .build()
        .unwrap();

    let sequential_chain = sequential_chain!(get_name_chain, get_slogan_chain);

    print!("Please enter a product: ");
    io::stdout().flush().unwrap(); // Display prompt to terminal

    let mut product = String::new();
    io::stdin().read_line(&mut product).unwrap(); // Get product from terminal input

    let product = product.trim();
    let output = sequential_chain
        .execute(prompt_args! {
            "producto" => product
        })
        .await
        .unwrap();

    println!("Name: {}", output["name"]);
    println!("Slogan: {}", output["slogan"]);
}
