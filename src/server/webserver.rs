use rocket::{State, serde::json::Json, fs::FileServer, get, post, routes, response::content};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, vec};
use tokio::sync::Mutex;
use crate::model::{model::Model};
use pulldown_cmark::{Parser, html};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub id: String,
    pub role: String, // "user" oder "assistant"
    pub content: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub message: String,
    pub role: String,
}

pub struct ChatState {
    pub model: Mutex<Model>,
}

pub struct PaprikaFrontendServer;

impl PaprikaFrontendServer {
    pub fn new() -> Self {
        PaprikaFrontendServer
    }

    pub async fn start() {
        let chat_state = Arc::new(
            ChatState {
                model: Mutex::new(
                    Model::new().await
                )
            }
        );

        let _ = rocket::build()
            .manage(chat_state)
            .mount("/api", routes![
                send_message,
            ])
            .mount("/", routes![root, index_file])
            .mount("/static", FileServer::from("static"))
            .launch()
            .await;
    }
}

#[get("/")]
fn root() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("../../static/index.html"))
}

#[get("/index.html")]
fn index_file() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("../../static/index.html"))
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[post("/chat/send", format = "json", data = "<request>")]
async fn send_message(
    request: Json<ChatRequest>,
    state: &State<Arc<ChatState>>,
) -> Json<ChatResponse> {
    // Nutze den ModelRequester um eine echte Antwort zu generieren
    let assistant_response = {
        let model = state.model.lock().await;
        log::debug!("[SERVER] Sende Anfrage zum Modell: {}", request.message);
        let response = model.model_requester.lock().await.request_full_text(&request.message, model.memories.lock().await.clone().join(",").as_str()).await;
        log::debug!("[SERVER] Antwort vom Modell erhalten: {}", response);
        response
    };

    let assistant_message = ChatMessage {
        id: uuid::Uuid::new_v4().to_string(),
        role: "assistant".to_string(),
        content: if assistant_response.is_empty() {
            "Entschuldigung, ich konnte keine Antwort generieren. Bitte versuche es später erneut.".to_string()
        } else {
            markdown_to_html(&assistant_response)
        },
        timestamp: chrono::Local::now().to_rfc3339(),
    };

    Json(ChatResponse {
        id: assistant_message.id,
        message: assistant_message.content,
        role: assistant_message.role,
    })
}
