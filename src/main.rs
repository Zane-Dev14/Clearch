// src/main.rs
use std::process;
use clap::Parser;
use sys_info::{linux_os_release, os_release, os_type};
use llms::gpt::GPT4Model;

mod traits;
mod llms;

#[derive(Parser)]
#[command(
name = "Clearch",
author = "Advaith Narayanan <advaith@glitchy.systems>",
about = "Search using the command line",
after_help = "Note: To redirect errors to stdout, use 2>&1.\n\
Example usages:\n\
- Provide a search query: clearch -q \"search term\"\n\
- Redirect errors: clearch -q \"search term\" 2>&1"
)]
struct Gemini {
    #[arg(short = 'q', long = "specify", help = "Specify the search query to perform")]
    search_query: Option<String>,
}

#[tokio::main]
async fn main() {
    // Retrieve OS information
    let os_type = os_type().unwrap_or_else(|_| "Unknown".to_string());
    let os_release = os_release().unwrap_or_else(|_| "Unknown".to_string());
    let linux_distro = linux_os_release()
    .map(|info| info.pretty_name)
    .unwrap_or(None)
    .unwrap_or_else(|| "Unknown".to_string());

    println!(
        "OS: {}  OS REL: {} Linux: {}",
        os_type, os_release, linux_distro
    );

    // Parse command line arguments
    let search = Gemini::parse();

    // Initialize the GPT-4 model with a hardcoded API key
    let gpt_model = GPT4Model::new();

    if let Some(query) = search.search_query {
        println!("Searching for: {}", query);

        // Send the query to GPT-4
        match gpt_model.req(&query).await {
            Ok(response) => println!("Response: {}", response),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    } else {
        eprintln!("Error: Please provide a search query using -q or --specify.");
        process::exit(1);
    }
}
