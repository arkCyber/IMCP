
##
IMCP是一个用于工业模型上下文通信的协议实现，提供了安全、可靠的消息交换机制。

## 特性

- 安全的WebSocket通信
- 基于Ed25519的数字签名
- 会话管理和状态跟踪
- 消息序列化和反序列化
- 异步操作支持

## 安装

在`Cargo.toml`中添加依赖：

```toml
[dependencies]
imcp = "0.1.0"
```

## 快速开始

### 创建客户端

```rust
use imcp::{new_client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 创建客户端
    let mut client = new_client()?;
    
    // 连接到服务器
    client.connect("ws://127.0.0.1:8080").await?;
    
    // 发送数据
    let data = b"Hello, IMCP!".to_vec();
    client.send(data).await?;
    
    // 接收响应
    let response = client.receive().await?;
    println!("Received response: {:?}", response);
    
    // 关闭连接
    client.close().await?;
    
    Ok(())
}
```

### 创建服务器

```rust
use imcp::{new_server, DefaultHandler, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 创建服务器
    let handler = DefaultHandler;
    let mut server = new_server(handler)?;
    
    // 启动服务器
    server.start("127.0.0.1", 8080).await?;
    
    Ok(())
}
```

### 自定义消息处理器

```rust
use imcp::{IMCPHandler, Message, Result};
use std::collections::HashMap;

struct CustomHandler;

#[async_trait::async_trait]
impl IMCPHandler for CustomHandler {
    async fn handle_message(
        &self,
        message: Message,
        client_public_key: &[u8],
    ) -> Result<Message> {
        // 处理消息并返回响应
        Ok(message)
    }
}
```

## 示例

运行示例：

```bash
cargo run --example basic
```

## 文档

详细的API文档可以通过以下命令生成：

```bash
cargo doc --open
```

## 许可证

MIT 