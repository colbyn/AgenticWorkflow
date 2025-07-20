use crate::{MarkdownDocument, MdInlineNode, MdNode};

impl MarkdownDocument {
    pub fn normalize(self) -> Self {
        let nodes = self.nodes
            .into_iter()
            .filter_map(|node| {
                match node {
                    MdNode::Inline(MdInlineNode::Text(text)) => {
                        let text = text.trim().to_owned();
                        if text.is_empty() {
                            return None
                        }
                        Some(MdNode::Inline(MdInlineNode::Text(text)))
                    }
                    node => Some(node),
                }
            })
            .collect::<Vec<_>>();
        Self { nodes }
    }
}
