#![allow(unused)]
use crate::{Element, Fragment, Node, TagBuf};

// ————————————————————————————————————————————————————————————————————————————
// DATA TYPES - BASICS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
enum ListType {
    Unordered, Ordered
}

#[derive(Debug, Clone)]
struct ListItemType {
    pub index: usize,
}

#[derive(Debug, Clone)]
enum BlockType {
    Paragraph,
    Pre,
    List(ListType),
    ListItem(ListItemType),
    BlockQuote,
}

#[derive(Debug, Clone)]
enum InlineType {
    CodeSpan,
}

#[derive(Debug, Clone)]
enum FormatterFrame {
    Block(BlockType),
    Inline(InlineType),
}


// ————————————————————————————————————————————————————————————————————————————
// DATA TYPES - ENVIRONMENT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Default)]
struct Scope {
    pub stack: Vec<FormatterFrame>,
}

impl Scope {
    pub fn with_frame(&self, formatter_frame: FormatterFrame) -> Self {
        let mut next = self.clone();
        next.stack.push(formatter_frame);
        next
    }
}

// ————————————————————————————————————————————————————————————————————————————
// DATA TYPES - TEXT FRAGMENTS
// ————————————————————————————————————————————————————————————————————————————


#[derive(Debug, Clone)]
enum NewlineType {
    SoftNewline,
    HardNewline,
}

#[derive(Debug, Clone)]
enum TextNode {
    Text(String),
    Newline(NewlineType),
}


// ————————————————————————————————————————————————————————————————————————————
// DATA TYPES - FORMATTER
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Default)]
struct Buffer {
    pub nodes: Vec<TextNode>,
}

impl Buffer {
    pub fn push_text_node(&mut self, text_node: TextNode) {
        self.nodes.push(text_node);
    }
    pub fn push_text(&mut self, text: impl AsRef<str>) {
        let text = text.as_ref();
        self.push_text_node(TextNode::Text(text.to_string()));
    }
    pub fn push_soft_newline(&mut self) {
        self.push_text_node(TextNode::Newline(NewlineType::SoftNewline));
    }
    pub fn push_hard_newline(&mut self) {
        self.push_text_node(TextNode::Newline(NewlineType::HardNewline));
    }
    pub fn finalize(&self) -> String {
        let mut output_string = String::default();
        for entry in self.nodes.iter() {
            match entry {
                TextNode::Text(text) => {
                    output_string.push_str(text);
                }
                TextNode::Newline(NewlineType::HardNewline) => {
                    output_string.push_str("\n\n");
                }
                TextNode::Newline(NewlineType::SoftNewline) => {
                    output_string.push_str("\n");
                }
            }
        }
        output_string
    }
}

// impl Formatter {
//     pub fn enter(&self, tag: &TagBuf) -> Self {
//         let mut next = self.clone();
//         next
//     }
// }

// ————————————————————————————————————————————————————————————————————————————
// HTML IMPLEMENTATION
// ————————————————————————————————————————————————————————————————————————————

impl Node {
    fn apply_formatter(&self, buffer: &mut Buffer, scope: &Scope) {
        match self {
            Self::Element(element) => element.apply_formatter(buffer, scope),
            Self::Fragment(fragment) => fragment.apply_formatter(buffer, scope),
            Self::Text(text) => buffer.push_text(text),
        }
    }
}

impl Element {
    fn apply_formatter(&self, buffer: &mut Buffer, scope: &Scope) {
        match self.tag.as_normalized() {
            "p" => {
                let ref scope = scope.with_frame(FormatterFrame::Block(BlockType::Paragraph));
                self.children.apply_formatter(buffer, scope);
            },
            "pre" => {
                let ref scope = scope.with_frame(FormatterFrame::Block(BlockType::Pre));
                self.children.apply_formatter(buffer, scope);
            },
            "ul" => {
                unimplemented!("TODO")
            },
            "ol" => {
                unimplemented!("TODO")
            },
            "li" => {
                unimplemented!("TODO")
            },
            "code" => {
                unimplemented!("TODO")
            },
            "blockquote" => {
                let ref scope = scope.with_frame(FormatterFrame::Block(BlockType::BlockQuote));
                self.children.apply_formatter(buffer, scope);
            },
            tag => {
                unimplemented!("TODO: {tag:?}")
            }
        }
    }
}

impl Fragment {
    fn apply_formatter(&self, buffer: &mut Buffer, scope: &Scope) {
        self.iter().for_each(|x| x.apply_formatter(buffer, scope));
    }
}

// ————————————————————————————————————————————————————————————————————————————
// ENTRYPOINT
// ————————————————————————————————————————————————————————————————————————————

pub fn text_format_html(node: impl Into<Node>) -> String {
    let mut buffer = Buffer::default();
    let scope = Scope::default();
    let node: Node = node.into();
    node.apply_formatter(&mut buffer, &scope);
    buffer.finalize()
}
