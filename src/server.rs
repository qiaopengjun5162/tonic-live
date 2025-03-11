use std::pin::Pin;

use crate::pb::chat_server::ChatServer;
use crate::pb::{
    chat_server::Chat, ChatMessage, GetMessagesRequest, LoginRequest, NewChatMessage,
    SendMessageResponse, Token,
};
use std::result::Result;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::codegen::tokio_stream::Stream;
use tonic::transport::Server;
use tonic::{Extensions, Request, Response, Status};
use tracing::{info, warn};

const MAX_MESSAGES: usize = 1024;

pub struct ChatService {
    tx: broadcast::Sender<ChatMessage>,
}

pub type ChatResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Chat for ChatService {
    async fn login(&self, request: Request<LoginRequest>) -> ChatResult<Token> {
        // 将request对象中的LoginRequest提取出来，并赋值给info变量。
        // into_inner方法通常用于将请求体中的数据提取出来。
        let info = request.into_inner();
        info!("login: {:#?}", info);
        let token = info.into_token();
        Ok(Response::new(token))
    }

    async fn send_message(
        &self,
        request: Request<NewChatMessage>,
    ) -> ChatResult<SendMessageResponse> {
        let sender = get_username(request.extensions())?;
        let info = request.into_inner();
        info!("send_message: {:#?}", info);
        let message = info.into_chat_message(sender);
        // store it to the server storage
        // publish message to everyone who interested in it
        self.tx.send(message).unwrap();
        Ok(Response::new(SendMessageResponse {}))
    }

    type GetMessagesStream =
        Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send + 'static>>;

    async fn get_messages(
        &self,
        request: Request<GetMessagesRequest>,
    ) -> ChatResult<Self::GetMessagesStream> {
        let info = request.into_inner();
        info!("subscribe to messages: {info:#?}");
        let mut rx = self.tx.subscribe();
        let (sender, receiver) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            // TODO: filter out messages I'm not interested in
            while let Ok(msg) = rx.recv().await {
                if let Err(e) = sender.send(Ok(msg)) {
                    info!("failed to send message: {e}");
                    warn!("Failed to send message. Sender might be closed.");
                    return;
                }
            }
        });
        let stream = UnboundedReceiverStream::new(receiver);
        Ok(Response::new(Box::pin(stream)))
    }
}

impl Default for ChatService {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(MAX_MESSAGES);
        Self { tx }
    }
}

pub async fn start() {
    let svc = ChatServer::with_interceptor(ChatService::default(), check_auth);
    let addr = "0.0.0.0:8080".parse().unwrap();
    info!("Listening on http://{addr}");
    Server::builder().add_service(svc).serve(addr).await.unwrap();
}

#[allow(clippy::result_large_err)]
fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    let token = match req.metadata().get("authorization") {
        Some(v) => {
            let data = v.to_str().map_err(|_| {
                Status::new(tonic::Code::Unauthenticated, "Invalid authorization token")
            })?;
            let stripped = data.strip_prefix("Bearer ").ok_or_else(|| {
                Status::new(tonic::Code::Unauthenticated, "Missing Bearer prefix")
            })?;
            Token::new(stripped)
        }
        None => Token::default(),
    };
    req.extensions_mut().insert(token);
    Ok(req)
}

#[allow(clippy::result_large_err)]
fn get_username(ext: &Extensions) -> Result<String, Status> {
    let token = ext.get::<Token>().ok_or(Status::unauthenticated("No token"))?;
    if token.is_valid() {
        Ok(token.into_username())
    } else {
        Err(Status::unauthenticated("Invalid token"))
    }
}
