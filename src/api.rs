use serde::{Deserialize, Serialize};

use crate::token::TOKEN;

#[derive(Serialize, Debug)]
pub struct ChatReqBody {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: ChatReqRole,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChatReqRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}

impl ChatMessage {
    fn new(content: &str, role: ChatReqRole) -> Self {
        Self {
            role,
            content: content.to_owned(),
        }
    }
}

impl ChatReqBody {
    pub fn new() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_owned(),
            messages: vec![],
        }
    }
    pub fn with_message(&mut self, text: &str, role: ChatReqRole) {
        self.messages.push(ChatMessage::new(text, role));
    }
    pub fn pop_messae(&mut self) {
        self.messages.pop();
    }
    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatRespBody {
    pub id: String,
    pub object: String,
    pub model: String,
    pub usage: ChatRespUsage,
    pub choices: Vec<ChatRespChoice>,
}

#[derive(Debug, Deserialize)]
pub struct ChatRespChoice {
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
    pub index: u64,
}

#[derive(Debug, Deserialize)]
pub struct ChatRespUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}
#[derive(Debug)]
pub struct ChatSession {
    body: ChatReqBody,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResp {
    error: ErrorInfo,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct ErrorInfo {
    message: String,
    r#type: String,
    code: String,
}

impl ChatSession {
    pub fn new() -> Self {
        let mut body = ChatReqBody::new();
        body.with_message("You are a helpful assistant.", ChatReqRole::System);
        Self { body }
    }

    pub async fn question(&mut self, text: &str) -> Result<Vec<String>, ()> {
        let client = reqwest::Client::new();
        let token = TOKEN.lock().unwrap().clone();

        self.body.with_message(text, ChatReqRole::User);
        let resp = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", token.trim()))
            .header("Content-Type", "application/json")
            .body(self.body.serialize())
            .send()
            .await
            .unwrap();

        let resp_text = resp.text().await.unwrap();
        let resp = serde_json::from_str::<ChatRespBody>(&resp_text).unwrap_or_else(|_| {
            let err_info = serde_json::from_str::<ErrorResp>(&resp_text).unwrap();
            panic!("{}", err_info.error.message);
        });

        let mut answers = vec![];

        if resp.choices.len() > 0 {
            resp.choices.iter().for_each(|c| {
                if let Some(_) = c.finish_reason {
                    self.body
                        .with_message(&c.message.content, c.message.role.clone());
                    let answer = c.message.content.to_owned();
                    answers.push(answer);
                }
            });
        } else {
            self.body.pop_messae();
        }

        Ok(answers)
    }
}
