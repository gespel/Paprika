use reqwest::Client;
use serde::{Serialize};
use std::{sync::Arc, vec};
use tokio::sync::Mutex;

#[derive(Serialize, Debug)]
pub struct GenerateRequest {
    model: String,
    prompt: String,
}

pub struct ModelRequester {
    model_name: String,
    context: String,
    pub chat_history: Arc<tokio::sync::Mutex<ChatHistory>>
}

pub struct ChatHistory {
    messages: Vec<((String, String), (String, String))>
}

impl ChatHistory {
    pub fn to_string(&self) -> String {
        let mut chat_history_string: String = String::new();

        for m in &self.messages {
            let request_tuple = m.0.clone();
            let response_tuple = m.1.clone();

            chat_history_string = chat_history_string + format!(" user_request: {} model_response: {}", request_tuple.1.as_str(), response_tuple.1.as_str()).as_str();

        }

        chat_history_string
    }
}

impl ModelRequester {
    pub fn new(model_name: &str, context: &str) -> Self {
        ModelRequester {
            model_name: model_name.to_string(),
            context: context.to_string(),
            chat_history: Arc::new(Mutex::new(ChatHistory { messages: vec![] }))
        }
    }

    pub async fn request(&mut self, prompt: &str) -> Result<String, reqwest::Error> {
        let client = Client::new();
        let context_prompt: String;

        if self.chat_history.lock().await.messages.is_empty() {
            context_prompt = format!("Context: {} user question: {}", self.context, prompt);
        }
        else {
            context_prompt = format!("Chat history: {} Context: {} user question: {}", &self.chat_history.lock().await.to_string(), self.context, prompt);
        }
        
        log::info!("Request to model: {}", context_prompt);

        let payload = GenerateRequest {
            model: self.model_name.clone(),
            prompt: context_prompt
        };
        let payload_json = serde_json::to_string(&payload).unwrap();

        let r = client.post("http://127.0.0.1:11434/api/generate").body(payload_json).send().await?;
        return r.text().await;
    }

    pub async fn request_full_text_without_context(&mut self, prompt: &str) -> String {
        let client = Client::new();
        
        log::info!("Request to model: {}", prompt);

        let payload = GenerateRequest {
            model: self.model_name.clone(),
            prompt: prompt.to_string()
        };
        let payload_json = serde_json::to_string(&payload).unwrap();

        let r = client.post("http://127.0.0.1:11434/api/generate").body(payload_json).send().await;
        let response = r.expect("").text().await;

        let mut out: String = String::new();
        match response {
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
                out
            },
            Err(e) => {
                log::error!("{:?}", e);
                e.to_string()
            }
        }
    }

    pub async fn request_full_text(&mut self, prompt: &str) -> String {
        let mut out: String = String::new();
        match self.request(prompt).await {
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
                //self.context = format!("{} model_answer: {}", self.context, out.to_string());
                self.chat_history.lock().await.messages.push(
                    (
                        ("user_prompt".to_string(), prompt.to_string()),
                        ("model_answer".to_string(), out.to_string())
                    )
                );
                out
            },
            Err(e) => {
                log::error!("{:?}", e);
                e.to_string()
            }
        }
    }
}