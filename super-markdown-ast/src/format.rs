#![allow(unused)]
use crate::{MarkdownDocument, MdBlockNode, MdInlineNode, MdListItemNode, MdListNode, MdNode};

// ————————————————————————————————————————————————————————————————————————————
// PUBLIC ENTRYPOINT
// ————————————————————————————————————————————————————————————————————————————

pub fn pretty_print_node(markdown: impl Into<MdNode>) -> String {
    let mut buffer = Buffer::default();
    let ref scope = Scope::default();
    let markdown = markdown.into();
    let markdown = markdown.apply_formatter(&mut buffer, scope);
    buffer.finalize()
}

pub fn pretty_print_document(markdown: &MarkdownDocument) -> String {
    let mut buffer = Buffer::default();
    let ref scope = Scope::default();
    markdown.nodes.iter().for_each(|x| x.apply_formatter(&mut buffer, scope));
    let pretty_printed = buffer.finalize();
    buffer.finalize()
}

impl std::fmt::Display for MdNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty_printed = pretty_print_node(self.to_owned());
        write!(f, "{pretty_printed}")
    }
}

impl std::fmt::Display for MdBlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty_printed = pretty_print_node(self.to_owned());
        write!(f, "{pretty_printed}")
    }
}

impl std::fmt::Display for MdInlineNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty_printed = pretty_print_node(self.to_owned());
        write!(f, "{pretty_printed}")
    }
}

impl std::fmt::Display for MarkdownDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty_printed = pretty_print_document(self);
        write!(f, "{pretty_printed}")
    }
}


// ————————————————————————————————————————————————————————————————————————————
// DATA TYPES » BASICS
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
// DATA TYPES » SCOPE
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
// DATA TYPES » BUFFER
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
enum NewlineType {
    Newline,
    EnsureNewline,
    SoftNewline,
    HardNewline,
}

#[derive(Debug, Clone)]
enum TextNode {
    Text(String),
    Newline(NewlineType),
}

#[derive(Debug, Clone, Default)]
struct Buffer {
    pub nodes: Vec<TextNode>,
}

impl Buffer {
    pub fn contains_str(&self, pattern: &str) -> bool {
        self.nodes.iter().any(|x| {
            match x {
                TextNode::Text(text) => {
                    text.contains(pattern)
                }
                TextNode::Newline(_) => false,
            }
        })
    }
    pub fn merge_mut(&mut self, other: Self) {
        self.nodes.extend(other.nodes);
    }
    pub fn push_text_node(&mut self, text_node: TextNode) {
        self.nodes.push(text_node);
    }
    pub fn push_text(&mut self, text: impl AsRef<str>) {
        let text = text.as_ref();
        self.push_text_node(TextNode::Text(text.to_string()));
    }
    // pub fn push_soft_newline(&mut self) {
    //     self.push_text_node(TextNode::Newline(NewlineType::SoftNewline));
    // }
    pub fn push_hard_newline(&mut self) {
        self.push_text_node(TextNode::Newline(NewlineType::HardNewline));
    }
    pub fn push_newline(&mut self) {
        self.push_text_node(TextNode::Newline(NewlineType::Newline));
    }
    pub fn ensure_newline(&mut self) {
        self.push_text_node(TextNode::Newline(NewlineType::EnsureNewline));
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
                TextNode::Newline(NewlineType::Newline) => {
                    output_string.push_str("\n");
                }
                TextNode::Newline(NewlineType::EnsureNewline) => {
                    let mut ends_with_newline = false;
                    if let Some(last_char) = output_string.chars().last() {
                        if last_char == '\n' {
                            ends_with_newline = true;
                        }
                    }
                    if !ends_with_newline {
                        output_string.push_str("\n");
                    }
                }
            }
        }
        output_string
    }
}

// ————————————————————————————————————————————————————————————————————————————
// IMPLEMENTATION
// ————————————————————————————————————————————————————————————————————————————

impl MdNode {
    fn apply_formatter(&self, buffer: &mut Buffer, scope: &Scope) {
        match self {
            Self::Block(block) => block.apply_formatter(buffer, scope),
            Self::Inline(inline) => inline.apply_formatter(buffer, scope),
        }
    }
}

impl MdBlockNode {
    fn apply_formatter(&self, buffer: &mut Buffer, scope: &Scope) {
        match self {
            Self::Paragraph(xs) => {
                let ref scope = scope.with_frame(FormatterFrame::Block(BlockType::Paragraph));
                xs.iter().for_each(|x| x.apply_formatter(buffer, scope));
                buffer.push_newline();
            }
            Self::Pre(xs) => {
                let mut subbuffer = Buffer::default();
                let ref scope = scope.with_frame(FormatterFrame::Block(BlockType::Pre));
                xs.iter().for_each(|x| x.apply_formatter(&mut subbuffer, scope));
                let fence_token = if subbuffer.contains_str("```") {
                    "````"
                } else {
                    "```"
                };
                
                buffer.ensure_newline();
                buffer.push_text(fence_token);
                buffer.push_newline();
                buffer.merge_mut(subbuffer);
                buffer.ensure_newline();
                buffer.push_text(fence_token);
                buffer.push_newline();
            }
            Self::List(xs) => {
                xs.apply_formatter(buffer, scope);
            }
            Self::BlockQuote(xs) => {
                buffer.push_text("> ");
                xs.iter().for_each(|x| x.apply_formatter(buffer, scope));
                buffer.push_newline();
                buffer.push_newline();
            }
        }
    }
}

impl MdInlineNode {
    fn apply_formatter(&self, buffer: &mut Buffer, scope: &Scope) {
        match self {
            Self::CodeSpan(xs) => {
                xs.iter().for_each(|x| x.apply_formatter(buffer, scope));
            }
            Self::Text(text) => {
                buffer.push_text(text);
            }
        }
    }
}

impl MdListNode {
    fn apply_formatter(&self, buffer: &mut Buffer, scope: &Scope) {
        buffer.push_newline();
        match self {
            Self::Ordered(xs) => {
                let ref scope = scope.with_frame(FormatterFrame::Block(BlockType::List(ListType::Ordered)));
                xs.iter().enumerate().for_each(|(ix, x)| x.apply_formatter(buffer, scope, Some(ix)));
            }
            Self::Unordered(xs) => {
                let ref scope = scope.with_frame(FormatterFrame::Block(BlockType::List(ListType::Unordered)));
                xs.iter().for_each(|x| x.apply_formatter(buffer, scope, None));
            }
        }
        buffer.push_newline();
    }
}

impl MdListItemNode {
    fn apply_formatter(&self, buffer: &mut Buffer, scope: &Scope, index: Option<usize>) {
        if let Some(index) = index {
            buffer.push_text(format!("{}. ", index + 1));
        } else {
            buffer.push_text("- ");
        }
        self.0.iter().for_each(|x| x.apply_formatter(buffer, scope));
        buffer.push_newline();
    }
}
