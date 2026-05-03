//Idea is to put model_requester with memories into the Model struct to have one representor for a model and its memories
use crate::{model::request::ModelRequester, model::memories::MemoryManager};
use std::{sync::Arc, vec};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};


pub struct Model {
    pub model_requester: ModelRequester,
    pub memory_manager: MemoryManager,
    pub memories: Arc<tokio::sync::Mutex<Vec<String>>>
}

impl Model {
    pub async fn new() -> Self {
        let memories = Arc::new(Mutex::new(vec![]));
        let memories_clone = Arc::clone(&memories);
        let out = Model {
            model_requester: ModelRequester::new(
                "gemma4:e4b", 
                "You are a helpful chatbot focused on science and maths/computer science called Paprika! Feel free to include some chilli emojis. Also answer shortly and only elaborate if it is really needed. Default to german answers."
            ),
            memory_manager: MemoryManager{},
            memories
        };

        

        let _memory_handle = tokio::task::spawn(async move {
            loop {
                let h = memories_clone.lock().await;
                println!("{}", h.len());
                sleep(Duration::from_secs(1)).await;
            }
        });

        out
    }

    fn update_long_term_memory(&mut self) {

    }
}