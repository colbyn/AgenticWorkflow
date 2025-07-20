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
