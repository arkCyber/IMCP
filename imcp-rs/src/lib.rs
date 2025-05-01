pub mod client;
pub mod protocol;
pub mod server;

pub use client::IMCPClient;
pub use protocol::{
    IMCPError as Error, IMCPHandler, Message, MessageBody, MessageHeader, MessageType,
};
pub use server::IMCPServer;

pub type Result<T> = std::result::Result<T, Error>;

/// 创建一个新的IMCP客户端实例
pub fn new_client() -> Result<IMCPClient> {
    IMCPClient::new()
}

/// 创建一个新的IMCP服务器实例
pub fn new_server<H: IMCPHandler + Send + Sync + 'static>(handler: H) -> Result<IMCPServer> {
    IMCPServer::new(std::sync::Arc::new(handler))
}

/// 默认的消息处理器实现
#[derive(Default)]
pub struct DefaultHandler;

#[async_trait::async_trait]
impl IMCPHandler for DefaultHandler {
    async fn handle_message(&self, message: Message, _client_public_key: &[u8]) -> Result<Message> {
        Ok(message)
    }
}
