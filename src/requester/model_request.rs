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
    //context: String,
    chat_history: ChatHistory
}

pub struct ChatHistory {
    messages: Vec<((String, String), (String, String))>
}

impl ModelRequester {
    pub fn new(model_name: &str) -> Self {
        ModelRequester {
            model_name: model_name.to_string(),
            //context: String::new(),
            chat_history: ChatHistory { messages: vec![] }
        }
    }

    pub async fn request(&mut self, prompt: &str) -> Result<String, reqwest::Error> {
        let client = Client::new();
        let context_prompt: String;

        if self.chat_history.messages.is_empty() {
            context_prompt = format!("Context: You are a helpful chatbot focused on science called Paprika! Feel free to include some chilli emojis. Also answer shortly and only elaborate if it is really needed. user question: {}", prompt);
        }
        else {
            let mut chat_history_string: String = String::new();

            for m in &self.chat_history.messages {
                let request_tuple = m.0.clone();
                let response_tuple = m.1.clone();

                chat_history_string = chat_history_string + format!(" user_request: {} model_response: {}", request_tuple.1.as_str(), response_tuple.1.as_str()).as_str();

            }

            context_prompt = format!("Chat history: {} Context: You are a helpful chatbot focused on science called Paprika! Feel free to include some chilli emojis. Also answer shortly and only elaborate if it is really needed. user question: {}", chat_history_string, prompt);
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
                self.chat_history.messages.push(
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