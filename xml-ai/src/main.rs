use crate::cli::CommandLineInterface;

extern crate super_html_ast as html_ast;

pub mod cli;

#[tokio::main]
async fn main() {
    let cli = CommandLineInterface::load();
    cli.execute().await
}
