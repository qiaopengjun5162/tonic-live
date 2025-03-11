mod abi;

pub use abi::*;
use chrono::Utc;

impl Token {
    pub fn new(data: impl Into<String>) -> Self {
        Self { data: data.into() }
    }

    // TODO: use jwt token instead of username and password
    pub fn into_username(&self) -> String {
        self.data.clone()
    }

    pub fn is_valid(&self) -> bool {
        !self.data.is_empty()
    }
}

impl LoginRequest {
    // impl Into<String>的使用使得函数的参数更加灵活，可以接受任何实现了Into<String>
    // trait的类型。
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            // Self表示当前结构体的类型，这里用于返回一个新的结构体实例
            username: username.into(), // into()方法用于将参数转换为String类型
            password: password.into(),
        }
    }

    // TODO: use jwt token instead of username and password
    pub fn into_token(&self) -> Token {
        Token::new(&self.username)
    }
}

impl NewChatMessage {
    pub fn new(room: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            room: room.into(),
            content: content.into(),
        }
    }

    pub fn into_chat_message(self, sender: impl Into<String>) -> ChatMessage {
        ChatMessage::new(sender, self.room, self.content)
    }
}

impl ChatMessage {
    pub fn new(
        sender: impl Into<String>,
        room: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        let timestamp = Utc::now().timestamp(); // 获取当前时间戳
        Self {
            sender: sender.into(),
            room: room.into(),
            content: content.into(),
            timestamp, // 将时间戳赋值给timestamp字段
        }
    }
}

impl GetMessagesRequest {
    pub fn new() -> Self {
        Self {}
    }
}
