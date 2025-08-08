//! Used for ai requests.

use ollama_rs::{Ollama, error::OllamaError, generation::completion::request::GenerationRequest};
use regex::Regex;
use string_format::string_format;

use crate::config::CONFIG;

/// Removes all AI's reasoning (the <think> tag containings) using regular expressions (regex)
fn remove_reasoning(text: &str) -> String {
    let re = Regex::new(r"<think\b[^>]*>[\s\S]*?</think>").expect("Failed to remove reasoning");
    re.replace_all(text, "").into()
}

/// Makes a request using Ollama to the AI model and returns the Ok(response) or Err(err)
pub async fn process_ollama(weather: String) -> Result<String, OllamaError> {
    // Note: it is normal to create ollama every function call, because it just has an address to requests
    let ollama = Ollama::default();
    let res = ollama
        .generate(GenerationRequest::new(
            CONFIG.ai_model.clone(),
            string_format(CONFIG.ai_prompt.clone(), weather),
        ))
        .await?
        .response;
    let fmt_res = remove_reasoning(&res);
    println!("{fmt_res}");
    Ok(fmt_res)
}

#[cfg(test)]
mod ai_tests {
    use crate::ai::remove_reasoning;

    #[test]
    fn test_remove_reasoning_no_newline() {
        let text = "<think>My thoughts...</think>Hello world!";
        assert_eq!(remove_reasoning(text), "Hello world!");
        let text = "<think> My thoughts...</think> Hello world!";
        assert_eq!(remove_reasoning(text), " Hello world!");
    }

    #[test]
    fn test_remove_reasoning_with_newline() {
        let text = "<think>My thoughts...\nMore...</think>Hello world!";
        assert_eq!(remove_reasoning(text), "Hello world!");
        let text =
            "<think>My thoughts...\nMore...\nAND MORE!\n\n\n\nThoughts...</think>Hello world!";
        assert_eq!(remove_reasoning(text), "Hello world!");
    }
}
