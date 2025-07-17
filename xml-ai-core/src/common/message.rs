use std::str::FromStr;

// ————————————————————————————————————————————————————————————————————————————
// MESSAGE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageRole {
    System,
    Assistant,
    User,
}

impl MessageRole {
    pub fn is_system(&self) -> bool {
        match self {
            Self::System => true,
            _ => false,
        }
    }
    pub fn is_assistant(&self) -> bool {
        match self {
            Self::Assistant => true,
            _ => false,
        }
    }
    pub fn is_user(&self) -> bool {
        match self {
            Self::User => true,
            _ => false,
        }
    }
}

impl FromStr for MessageRole {
    type Err = ParseRoleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim();
        match normalized {
            "system" => Ok(MessageRole::System),
            "assistant" => Ok(MessageRole::Assistant),
            "user" => Ok(MessageRole::User),
            _ => Err(ParseRoleError { given: s.to_string() })
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseRoleError {
    pub given: String,
}

impl std::fmt::Display for ParseRoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unrecognized role {:?}", self.given)
    }
}

impl std::error::Error for ParseRoleError {}
