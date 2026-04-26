use reqwest::Client;
use serde::{Serialize};
use uuid::timestamp::context;

#[derive(Serialize, Debug)]
pub struct GenerateRequest {
    model: String,
    prompt: String,
}

pub struct ModelRequester {
    model_name: String,
    context: String,
}

pub struct ChatHistory {
    messages: (String, String)
}

impl ModelRequester {
    pub fn new(model_name: &str) -> Self {
        ModelRequester {
            model_name: model_name.to_string(),
            context: String::new()
        }
    }

    pub async fn request(&mut self, prompt: &str, chat_history: &str) -> Result<String, reqwest::Error> {
        let client = Client::new();
        let context_prompt: String;

        if chat_history.is_empty() {
            context_prompt = format!("Context: You are a helpful chatbot focused on science called Paprika! Feel free to include some chilli emojis. Also answer shortly and only elaborate if it is really needed. user question: {}", prompt);
        }
        else {
            context_prompt = format!("Chat history: {} Context: You are a helpful chatbot focused on science called Paprika! Feel free to include some chilli emojis. Also answer shortly and only elaborate if it is really needed. user question: {}", chat_history, prompt);
        }

        self.context = format!("{} user_prompt: {}", self.context.clone(), prompt.to_string());
        
        log::info!("Request to model: {}", context_prompt);

        let payload = GenerateRequest {
            model: self.model_name.clone(),
            prompt: context_prompt
        };
        let payload_json = serde_json::to_string(&payload).unwrap();

        let r = client.post("http://127.0.0.1:11434/api/generate").body(payload_json).send().await?;
        return r.text().await;
    } 

    pub async fn request_full_text(&mut self, prompt: &str) -> String {
        let mut out: String = String::new();
        match self.request(prompt, self.context.clone().as_str()).await {
            Ok(r) => {
                let lines: Vec<&str> = r.split("\n").collect();
                for line in lines {    
                    match serde_json::from_str::<serde_json::Value>(line) {
                        Ok(j) => {
                            if let Some(response) = j.get("response") {
                                out.push_str(response.as_str().unwrap_or(""));
                            }
                        }
                        Err(e) => {
                            if !line.is_empty() {
                                log::warn!("{}, {:?}", line, e);
                            }
                        }
                    }
                }
                self.context = format!("{} model_answer: {}", self.context, out.to_string());
                out
            },
            Err(e) => {
                log::error!("{:?}", e);
                e.to_string()
            }
        }
    }
}