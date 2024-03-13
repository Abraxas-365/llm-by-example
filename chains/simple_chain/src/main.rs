use langchain_rust::{
    chain::{Chain, LLMChainBuilder},
    llm::openai::{OpenAI, OpenAIModel},
    prompt::HumanMessagePromptTemplate,
    prompt_args, template_jinja2,
};
use std::io::{self, Write}; // Include io Library for terminal input

#[tokio::main]
async fn main() {
    let prompt = HumanMessagePromptTemplate::new(template_jinja2!(
        "Dame un nombre creativo para una tienda que vende: {{producto}}",
        "producto"
    ));

    let llm = OpenAI::default().with_model(OpenAIModel::Gpt35);
    let chain = LLMChainBuilder::new()
        .prompt(prompt)
        .llm(llm)
        .build()
        .unwrap();

    print!("Please enter a product: ");
    io::stdout().flush().unwrap(); // Display prompt to terminal

    let mut product = String::new();
    io::stdin().read_line(&mut product).unwrap(); // Get product from terminal input

    let product = product.trim();

    let output = chain
        .invoke(prompt_args!["producto" => product]) // Use product input here
        .await
        .unwrap();

    println!("Output: {}", output);
}
