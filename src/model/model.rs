//Idea is to put model_requester with memories into the Model struct to have one representor for a model and its memories
use crate::{model::request::ModelRequester, model::memories::MemoryManager};
use std::{sync::Arc, vec};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};


pub struct Model {
    pub model_requester: Arc<tokio::sync::Mutex<ModelRequester>>,
    pub memory_manager: MemoryManager,
    pub memories: Arc<tokio::sync::Mutex<Vec<String>>>
}

impl Model {
    pub async fn new() -> Self {
        let memories = Arc::new(Mutex::new(vec![]));
        let model_requester = Arc::new(Mutex::new(ModelRequester::new(
            "gemma4:e4b", 
            "You are a helpful chatbot focused on science and maths/computer science called Paprika! Feel free to include some chilli emojis. Also answer shortly and only elaborate if it is really needed. Default to german answers."
        )));

        let memories_clone = Arc::clone(&memories);
        let chat_history = Arc::clone(&model_requester.lock().await.chat_history);
        let memory_manger_requester = Arc::clone(&model_requester);

        let out = Model {
            model_requester,
            memory_manager: MemoryManager{},
            memories
        };

        

        let _memory_handle = tokio::task::spawn(async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                let c = chat_history.lock().await;
                if !c.messages.is_empty() {
                    let response = memory_manger_requester.lock().await
                        .request_full_text_without_context(format!("Compress the following chat to key memories for yourself to read later on: {} these are the other memories {:?}", c.to_string(), memories_clone.lock().await.clone()).as_str()).await;
                    
                    memories_clone.lock().await.push(response.clone());
                    if memories_clone.lock().await.len() > 1 {
                        memories_clone.lock().await.remove(0);
                    }
                    println!("Compressed from model: {}", response);
                }
                drop(c);  // Explizit Lock freigeben
            }
        });

        out
    }

    fn update_long_term_memory(&mut self) {

    }
}