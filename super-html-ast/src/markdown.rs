use markdown_ast::{MarkdownDocument, MdNode};

use crate::{Element, Fragment, Node, TagBuf};

enum BlockType {
    Paragraph,
    Pre,
    OrderedList,
    UnorderedList,
    ListItem,
    BlockQuote,
}

impl BlockType {
    pub fn from_tag(tag: &TagBuf) -> Option<Self> {
        match tag.as_normalized() {
            "p" => Some(Self::Paragraph),
            "pre" => Some(Self::Pre),
            "ul" => Some(Self::UnorderedList),
            "ol" => Some(Self::OrderedList),
            "li" => Some(Self::ListItem),
            "blockquote" => Some(Self::BlockQuote),
            _ => None,
        }
    }
}

enum InlineType {
    CodeSpan,
}

impl InlineType {
    pub fn from_tag(tag: &TagBuf) -> Option<Self> {
        match tag.as_normalized() {
            "code" => Some(Self::CodeSpan),
            _ => None,
        }
    }
}

pub fn to_markdown_document(nodes: &[Node]) -> MarkdownDocument {
    let nodes = nodes.iter().flat_map(|x| x.to_md_nodes()).collect::<Vec<_>>();
    MarkdownDocument { nodes }
}

impl Node {
    fn to_md_nodes(&self) -> Vec<markdown_ast::MdNode> {
        match self {
            Self::Element(element) => element.to_md_nodes(),
            Self::Fragment(fragment) => fragment.to_md_nodes(),
            Self::Text(text) => {
                vec![markdown_ast::MdNode::Inline(markdown_ast::MdInlineNode::Text(text.to_string()))]
            }
        }
    }
    fn to_md_inline_nodes(&self) -> Vec<markdown_ast::MdInlineNode> {
        match self {
            Self::Element(element) => element.to_md_inline_nodes(),
            Self::Fragment(fragment) => fragment.to_md_inline_nodes(),
            Self::Text(text) => {
                let md = markdown_ast::MdInlineNode::Text(text.to_string());
                vec![md]
            }
        }
    }
    fn md_list_items(&self) -> Vec<markdown_ast::MdListItemNode> {
        match self {
            Self::Element(element) => vec![element.md_list_item()],
            Self::Fragment(fragment) => fragment.md_list_items(),
            Self::Text(_) => Vec::new(),
        }
    }
}

impl Element {
    fn to_md_nodes(&self) -> Vec<markdown_ast::MdNode> {
        if let Some(block_type) = BlockType::from_tag(&self.tag) {
            return match block_type {
                BlockType::Paragraph => {
                    let children = self.children.to_md_nodes();
                    let md = markdown_ast::MdNode::Block(markdown_ast::MdBlockNode::Paragraph(children));
                    vec![md]
                }
                BlockType::Pre => {
                    let children = self.children.to_md_nodes();
                    let md = markdown_ast::MdNode::Block(markdown_ast::MdBlockNode::Pre(children));
                    vec![md]
                }
                BlockType::UnorderedList => {
                    let children = self.children.md_list_items();
                    let md = markdown_ast::MdListNode::Unordered(children);
                    let md = markdown_ast::MdBlockNode::List(md);
                    let md = markdown_ast::MdNode::Block(md);
                    vec![md]
                }
                BlockType::OrderedList => {
                    let children = self.children.md_list_items();
                    let md = markdown_ast::MdListNode::Ordered(children);
                    let md = markdown_ast::MdBlockNode::List(md);
                    let md = markdown_ast::MdNode::Block(md);
                    vec![md]
                }
                BlockType::ListItem => {
                    let msg = vec![
                        "TODO: IS THIS EVEN POSSIBLE?",
                        "Client code is probably invalid.",
                        "This API is intentionally struct (although erroneous case handling could be better).",
                    ].join(" ");
                    unimplemented!("{msg}")
                }
                BlockType::BlockQuote => {
                    let children = self.children.to_md_nodes();
                    let md = markdown_ast::MdNode::Block(markdown_ast::MdBlockNode::BlockQuote(children));
                    vec![md]
                }
            }
        }
        if let Some(_) = InlineType::from_tag(&self.tag) {
            let nodes = self
                .to_md_inline_nodes()
                .into_iter()
                .map(MdNode::Inline)
                .collect::<Vec<_>>();
            return nodes
        }
        unimplemented!("TODO: {:?}", self.tag.as_normalized())
    }
    fn to_md_inline_nodes(&self) -> Vec<markdown_ast::MdInlineNode> {
        let children = self.children.to_md_inline_nodes();
        match self.tag.as_normalized() {
            "code" => {
                let md = markdown_ast::MdInlineNode::CodeSpan(children);
                vec![md]
            }
            tag => {
                unimplemented!("TODO: {tag:?}")
            }
        }
    }
    fn md_list_item(&self) -> markdown_ast::MdListItemNode {
        let children = self.children.to_md_nodes();
        match self.tag.as_normalized() {
            "li" => {
                markdown_ast::MdListItemNode(children)
            }
            tag => {
                unimplemented!("TODO: {tag:?}")
            }
        }
    }
}

impl Fragment {
    fn to_md_nodes(&self) -> Vec<markdown_ast::MdNode> {
        self.iter().flat_map(|x| x.to_md_nodes()).collect::<Vec<_>>()
    }
    fn to_md_inline_nodes(&self) -> Vec<markdown_ast::MdInlineNode> {
        self.iter().flat_map(|x| x.to_md_inline_nodes()).collect::<Vec<_>>()
    }
    fn md_list_items(&self) -> Vec<markdown_ast::MdListItemNode> {
        self.iter().flat_map(|x| x.md_list_items()).collect::<Vec<_>>()
    }
}
