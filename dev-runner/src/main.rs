#![allow(unused)]
extern crate super_html_ast as html_ast;

// pub mod cli;

// #[tokio::main]
// async fn main() {
//     // let cli = CommandLineInterface::load();
//     // cli.execute().await
//     dev();
// }

const HR_RULE: &'static str = "-------------------------------------------------------------------------------";

fn main() {
    let source = include_str!("../../notes/format-testing.html");
    let html_tree = html_ast::parser::parse_from_fragment(source).expect("parsed html tree");
    // eprintln!("HTML TREE: {:#?}", html_tree);
    // let text = html_ast::text_format::text_format_html(tree);
    // eprintln!("{HR_RULE}");
    let markdown_doc = html_ast::markdown::to_markdown_document(&[html_tree.clone()]);
    let markdown_doc = markdown_doc.normalize();
    eprintln!("MARKDOWN TREE: {:#?}", markdown_doc);
    eprintln!("{HR_RULE}");
    let markdown_string = markdown_doc.to_string();
    eprintln!("{markdown_string}");
    // let markdown_string = markdown_tree
}

