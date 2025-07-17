use std::path::PathBuf;
use clap::{Parser, Subcommand};
use xml_ai_core::runtime::{DocumentInvocation, RuntimeEnvironment};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineInterface {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Run(RunCli),
}

#[derive(Parser, Debug)]
struct RunCli {
    /// Path to the prompt file.
    // #[arg(long)]
    pub file: PathBuf,
    /// The name of the prompt.
    #[arg(short, long)]
    pub name: String,
    /// API key file path.
    #[arg(short, long)]
    pub key_file: PathBuf,
    /// Path to the output log file.
    #[arg(short, long)]
    pub output: PathBuf,
}

impl CommandLineInterface {
    pub fn load() -> Self {
        Self::parse()
    }
    pub async fn execute(self) {
        match self.command {
            SubCommand::Run(run) => run.execute().await,
        }
    }
}

impl RunCli {
    pub async fn execute(self) {
        let source = std::fs::read_to_string(&self.file).expect("path to given prompt template file");
        let html_tree = html_ast::parser::parse_from_fragment(source).expect("valid html content");
        let document = xml_ai_core::ast::document::DocumentNode::from_node(html_tree).expect("should be a valid node");
        let api_key = std::fs::read_to_string(&self.key_file).expect("API key file");
        let document_invocation = DocumentInvocation {
            runtime_environment: RuntimeEnvironment {
                api_key,
            },
            target_prompt: String::from(&self.name),
        };
        let prompt_context = document.invoke(&document_invocation).await.unwrap();
        let conversation_snapshot = prompt_context.to_snapshot();
        std::fs::create_dir_all(self.output.parent().unwrap()).unwrap();
        match self.output.extension().unwrap().to_str().unwrap() {
            "json" => {
                let json_snapshot = serde_json::to_string_pretty(&conversation_snapshot).unwrap();
                std::fs::write(&self.output, &json_snapshot).unwrap();
            },
            "toml" => {
                let toml_snapshot = toml::to_string_pretty(&conversation_snapshot).unwrap();
                std::fs::write(&self.output, &toml_snapshot).unwrap();
            },
            _ => panic!("NOT A VALID OUTPUT FILE"),
        }
        println!("DONE:");
        println!("{:#?}", conversation_snapshot);
    }
}

// #[derive(Debug, Clone)]
// enum OutputFormat {
//     Json,
//     Toml,
// }

// impl OutputFormat {
//     pub fn from_file_ext(ext: impl AsRef<Path>) -> Option<Self> {
//         match ext.as_ref().extension()?.to_str()? {
//             "json" => Some(Self::Json),
//             "toml" => Some(Self::Toml),
//             _ => None,
//         }
//     }
//     pub fn write_to()
// }


