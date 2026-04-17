use reqwest::Client;
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct GenerateRequest {
    model: String,
    prompt: String,
}

pub struct ModelRequester {
    model_name: String,

}

impl ModelRequester {
    pub fn new(model_name: &str) -> Self {
        ModelRequester {
            model_name: model_name.to_string()
        }
    }

    pub async fn request(&mut self, prompt: &str) -> Result<String, reqwest::Error> {
        let client = Client::new();

        let context_prompt: String = format!("Context: You are a helpful chatbot focused on science called Paprika! Occasionally include some jokes with vegetables and some paprika/chilli emojis. Not too often though and only short. user question: {}", prompt);

        let payload = GenerateRequest {
            model: self.model_name.clone(),
            prompt: context_prompt
        };
        let payload_json = serde_json::to_string(&payload).unwrap();

        let r = client.post("http://127.0.0.1:11434/api/generate").body(payload_json).send().await?;
        return r.text().await;
    } 

    pub async fn request_full_text(&mut self, prompt: &str) -> String {
        let mut out: String = "".to_string();
        match self.request(prompt).await {
            Ok(r) => {
                let lines: Vec<&str> = r.split("\n").collect();
                for line in lines {
                    //println!("Line: {}", line);
                    //let j: serde_json::Value = serde_json::from_str(line).unwrap();
                    match serde_json::from_str::<serde_json::Value>(line) {
                        Ok(j) => {
                            if let Some(response) = j.get("response") {
                                out.push_str(response.as_str().unwrap_or(""));
                            }
                        }
                        Err(e) => {
                            
                        }
                    }
                    
                }

                out
            },
            Err(e) => {
                log::error!("");
                e.to_string()
            }
        }
    }
}