use ollama_rs::{Ollama, error::OllamaError, generation::completion::request::GenerationRequest};
use regex::Regex;
use string_format::string_format;

use crate::{infra::ai::AiProvider, models::types::Res, tools::config::CONFIG};

pub struct OllamaAi;

impl OllamaAi {
    fn remove_reasoning(text: &str) -> String {
        let re = Regex::new(r"<think\b[^>]*>[\s\S]*?</think>").expect("Failed to remove reasoning");
        re.replace_all(text, "").into()
    }
}

impl AiProvider for OllamaAi {
    async fn process(weather: String) -> Res<String> {
        // Note: it is normal to create ollama every function call, because it just has an address to requests
        log::info!("Generating response using `{}` model", CONFIG.ai_model);
        let ollama = Ollama::default();
        let res = ollama
            .generate(GenerationRequest::new(
                CONFIG.ai_model.clone(),
                string_format(CONFIG.ai_prompt.clone(), weather),
            ))
            .await?
            .response;
        println!("Removing reasoning part from response...");
        let fmt_res = Self::remove_reasoning(&res);
        Ok(fmt_res)
    }
}

#[cfg(test)]
mod ai_tests {
    use crate::infra::ollama::OllamaAi;

    #[test]
    fn test_remove_reasoning_no_newline() {
        let text = "<think>My thoughts...</think>Hello world!";
        assert_eq!(OllamaAi::remove_reasoning(text), "Hello world!");
        let text = "<think> My thoughts...</think> Hello world!";
        assert_eq!(OllamaAi::remove_reasoning(text), " Hello world!");
    }

    #[test]
    fn test_remove_reasoning_with_newline() {
        let text = "<think>My thoughts...\nMore...</think>Hello world!";
        assert_eq!(OllamaAi::remove_reasoning(text), "Hello world!");
        let text =
            "<think>My thoughts...\nMore...\nAND MORE!\n\n\n\nThoughts...</think>Hello world!";
        assert_eq!(OllamaAi::remove_reasoning(text), "Hello world!");
    }
}
