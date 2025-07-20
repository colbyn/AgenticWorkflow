#[derive(Debug, Clone)]
pub enum MdNode {
    Block(MdBlockNode),
    Inline(MdInlineNode),
}

#[derive(Debug, Clone)]
pub enum MdBlockNode {
    Paragraph(Vec<MdNode>),
    Pre(Vec<MdNode>),
    List(MdListNode),
    BlockQuote(Vec<MdNode>),
}

#[derive(Debug, Clone)]
pub struct MdListItemNode(pub Vec<MdNode>);

#[derive(Debug, Clone)]
pub enum MdListNode {
    Unordered(Vec<MdListItemNode>),
    Ordered(Vec<MdListItemNode>),
}

#[derive(Debug, Clone)]
pub enum MdInlineNode {
    CodeSpan(Vec<MdInlineNode>),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct MarkdownDocument {
    pub nodes: Vec<MdNode>,
}

impl Into<MdNode> for MdBlockNode {
    fn into(self) -> MdNode {
        MdNode::Block(self)
    }
}

impl Into<MdNode> for MdInlineNode {
    fn into(self) -> MdNode {
        MdNode::Inline(self)
    }
}
