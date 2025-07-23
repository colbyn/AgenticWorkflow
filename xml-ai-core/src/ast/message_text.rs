// use html_ast::{Element, Node};

use crate::ast::message::MsgNode;

impl MsgNode {
    pub fn to_markdown_text(&self) -> String {
        // if let Ok(text_only) = self.children.clone().extract_text_strict() {
        //     return text_only.join("")
        // }
        let markdown_doc = html_ast::markdown::to_markdown_document(self.children.as_node_slice());
        let markdown_doc = markdown_doc.normalize();
        let markdown_string = markdown_doc.to_string();
        markdown_string
        // self.children
        //     .iter()
        //     .flat_map(|x| x.to_owned().flatten())
        //     .filter_map(|x| {
        //         match x {
        //             Node::Element(element) => {
        //                 Some(to_text_content(element))
        //             },
        //             Node::Text(_) => {
        //                 None
        //             }
        //             Node::Fragment(_) => {
        //                 panic!("NOT POSSIBLE")
        //             },
        //         }
        //     })
        //     .collect::<Vec<_>>()
        //     .join("\n")
    }
    pub fn to_plain_text(&self) {
        unimplemented!("TODO")
    }
}


// fn to_text_content(element: Element) -> String {
//     element.children.extract_text_strict().unwrap().join("")
// }
