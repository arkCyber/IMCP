use imcp::{new_client, new_server, DefaultHandler, Result};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    // 设置日志
    env_logger::init();

    // 创建服务器
    let handler = DefaultHandler;
    let mut server = new_server(handler)?;

    // 在后台启动服务器
    let server_handle = tokio::spawn(async move {
        server.start("127.0.0.1", 8080).await.unwrap();
    });

    // 等待服务器启动
    sleep(Duration::from_secs(1)).await;

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

    // 等待服务器关闭
    server_handle.abort();

    Ok(())
}
