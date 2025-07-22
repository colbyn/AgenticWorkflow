use crate::ast::prompt::PromptNode;

#[derive(Debug, Clone)]
pub enum DocumentChildNode {
    Prompt(PromptNode),
}

impl DocumentChildNode {
    pub fn name(&self) -> String {
        match self {
            Self::Prompt(prompt) => prompt.settings.name.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DocumentNode {
    pub children: Vec<DocumentChildNode>,
}

impl DocumentNode {
    /// TODO: proper error handling.
    pub fn from_str(source: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let html_tree = html_ast::parser::parse_from_fragment(source);
        let html_tree = html_tree.expect("valid html document");
        let document = DocumentNode::from_node(html_tree).expect("should be a valid node");
        Ok(document)
    }
}
