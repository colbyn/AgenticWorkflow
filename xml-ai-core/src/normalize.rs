#![allow(unused)]
use crate::ast::{document::DocumentChildNode, message::MsgNode, prompt::{PromptChildNode, PromptNode}};
use crate::runtime::DocumentContext;

impl DocumentContext {
    pub fn normalize(self) -> Self {
        unimplemented!("TODO")
    }
}

impl DocumentChildNode {
    pub fn normalize(self, document_context: &DocumentContext) -> Self {
        match self {
            Self::Prompt(x) => Self::Prompt(x.normalize(document_context)),
        }
    }
}

impl PromptNode {
    pub fn normalize(self, document_context: &DocumentContext) -> Self {
        let PromptNode { settings, children } = self;
        let children = children
            .into_iter()
            .map(|x| x.normalize(document_context))
            .collect::<Vec<_>>();
        Self { settings, children }
    }
}

impl PromptChildNode {
    pub fn normalize(self, document_context: &DocumentContext) -> Self {
        match self {
            Self::Msg(x) => Self::Msg(x.normalize(document_context)),
            Self::Breakpoint(x) => Self::Breakpoint(x),
            Self::Set(x) => Self::Set(x),
        }
    }
}

impl MsgNode {
    pub fn normalize(self, document_context: &DocumentContext) -> Self {
        let MsgNode { role, breakpoint_mode, children } = self;
        let children = children
            .flatten()
            .into_iter()
            .map(|node| {
                match node {
                    html_ast::Node::Element(element) => html_ast::Node::Element(normalize_element(element, document_context)),
                    html_ast::Node::Text(text) => html_ast::Node::Text(text),
                    html_ast::Node::Fragment(_) => panic!("NOT POSSIBLE"),
                }
            })
            .collect::<Vec<_>>();
        let children = html_ast::Fragment::from_nodes(children);
        Self {
            role,
            breakpoint_mode,
            children,
        }
    }
}

fn normalize_element(element: html_ast::Element, document_context: &DocumentContext) -> html_ast::Element {
    let p_tag = html_ast::TagBuf::from("p");
    let pre_tag = html_ast::TagBuf::from("pre");
    unimplemented!("TODO")
}

