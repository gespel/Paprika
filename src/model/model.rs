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
            "ministral-3:3b", 
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
                let mut c = chat_history.lock().await;
                if !c.messages.is_empty() {
                    let memories_old = memories_clone.lock().await.clone();
                    let persona = memory_manger_requester.lock().await.context.clone();
                    let response = memory_manger_requester.lock().await
                        .request_full_text_without_context(format!("Compress the following chat between you and the user to key memories: {} these are the other memories {:?} and this is your identity: {}", c.to_string(), memories_old, persona).as_str()).await;
                    
                    log::debug!("Waiting for memories lock to add response...");
                    memories_clone.lock().await.push(response.clone());
                    log::debug!("Waiting for memories lock to remove old memories...");
                    if memories_clone.lock().await.len() > 1 {
                        memories_clone.lock().await.remove(0);
                    }
                    log::debug!("Waiting for chat_history log to delete old chat messages...");
                    c.messages.clear();
                    println!("Compressed from model: {}", response);
                }
                //log::debug!("Dropping chat_history lock");
                drop(c);  // Explizit Lock freigeben
            }
        });

        out
    }

    fn update_long_term_memory(&mut self) {

    }
}