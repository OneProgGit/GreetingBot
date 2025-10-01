use meowbot_proto::generated::ai::{AiRequest, AiResponse, ai_server::Ai};
use ollama_rs::generation::completion::request::GenerationRequest;
use regex::Regex;
use tonic::{Request, Response, Status};

use string_format::string_format;

pub struct Ollama;

impl Ollama {
    fn remove_reasoning(text: &str) -> String {
        let re = Regex::new(r"<think\b[^>]*>[\s\S]*?</think>").expect("Failed to remove reasoning");
        re.replace_all(text, "").into()
    }
}

#[cfg(test)]
mod remove_reasoning_tests {
    use crate::ollama::Ollama;

    #[test]
    fn test_remove_reasoning_no_newline() {
        let text = "<think>My thoughts...</think>Hello world!";
        assert_eq!(Ollama::remove_reasoning(text), "Hello world!");
        let text = "<think> My thoughts...</think> Hello world!";
        assert_eq!(Ollama::remove_reasoning(text), " Hello world!");
    }

    #[test]
    fn test_remove_reasoning_with_newline() {
        let text = "<think>My thoughts...\nMore...</think>Hello world!";
        assert_eq!(Ollama::remove_reasoning(text), "Hello world!");
        let text =
            "<think>My thoughts...\nMore...\nAND MORE!\n\n\n\nThoughts...</think>Hello world!";
        assert_eq!(Ollama::remove_reasoning(text), "Hello world!");
    }
}

#[tonic::async_trait]
impl Ai for Ollama {
    async fn get_response(
        &self,
        request: Request<AiRequest>,
    ) -> Result<Response<AiResponse>, Status> {
        let request = request.get_ref().to_owned();

        let weather = request.weather;
        let prompt = request.prompt;
        let model = request.model;

        let ollama = ollama_rs::Ollama::default();
        let response = ollama
            .generate(GenerationRequest::new(
                model,
                string_format(prompt, weather),
            ))
            .await
            .expect("Failed to get AI response")
            .response;
        let fmt_response = Self::remove_reasoning(&response);

        let response = AiResponse {
            response: fmt_response,
        };

        Ok(Response::new(response))
    }
}
