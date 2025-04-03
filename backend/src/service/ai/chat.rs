// use axum::Json;
// use rig::{completion::Chat, message::Message, streaming::StreamingChat};

// use crate::{
//     model::ai::AiChat,
//     util::{AppResult, ai, error::AppError, response::AppResponse},
// };

// pub async fn chat(Json(ai_chat): Json<AiChat>) -> AppResult<String> {
//     let agent = ai::new();
//     let mut history: Vec<Message> = Vec::new();
//     for line in ai_chat.history {
//         match line.role.as_str() {
//             "user" => {
//                 history.push(Message::user(line.content));
//             }
//             "assistant" => {
//                 history.push(Message::assistant(line.content));
//             }
//             _ => {
//                 return Err(AppError::Other);
//             }
//         }
//     }
//     let response = agent.stream_chat(&ai_chat.prompt, history).await.unwrap();
//     Ok(AppResponse::success(Some(response)))
// }

use axum::{
    Json,
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::{self, StreamExt};
use rig::{
    message::Message,
    streaming::{StreamingChat, StreamingChoice},
};
use std::{convert::Infallible, time::Duration};

use crate::{model::ai::AiChat, util::ai};

use futures::stream::BoxStream; // 需要 futures = "0.3"

pub async fn chat(
    Json(ai_chat): Json<AiChat>,
) -> Sse<BoxStream<'static, Result<Event, Infallible>>> {
    let agent = ai::new();
    let mut history: Vec<Message> = Vec::new();
    for line in ai_chat.history {
        match line.role.as_str() {
            "user" => {
                history.push(Message::user(line.content));
            }
            "assistant" => {
                history.push(Message::assistant(line.content));
            }
            _ => {
                break;
            }
        }
    }

    let response = match agent.stream_chat(&ai_chat.prompt, history).await {
        Ok(res) => res,
        Err(e) => {
            let error_stream =
                stream::once(async move { Ok(Event::default().data(format!("Error: {}", e))) });
            return Sse::new(error_stream.boxed());
        }
    };

    let chat_stream = response
        .map(|chunk| match chunk {
            Ok(StreamingChoice::Message(text)) => Ok(Event::default().data(text)),
            Ok(StreamingChoice::ToolCall(name, _, params)) => {
                Ok(Event::default().data(format!("Tool call: {}", name)))
            }
            Err(e) => Ok(Event::default().data(format!("Error: {}", e))),
        })
        .boxed(); // 将流转换为 BoxStream

    Sse::new(chat_stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}
