use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient, SubscribeEntriesRequest,
};
use std::sync::Arc;
use tokio::sync::Semaphore;
use clap::Parser;

#[derive(Parser)]
#[command(name = "deshred")]
#[command(about = "连接到 shredstream 代理服务器并处理 entries")]
struct Args {
    /// 服务器地址 (例如: 172.245.211.10:8002)
    #[arg(long, default_value = "172.245.211.10:8002")]
    host: String,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let server_url = format!("http://{}", args.host);
    
    println!("正在连接到服务器 {}...", server_url);
    
    let mut client = match ShredstreamProxyClient::connect(server_url).await {
        Ok(client) => {
            println!("成功连接到服务器");
            client
        }
        Err(e) => {
            eprintln!("连接服务器失败: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, e));
        }
    };
    
    println!("正在订阅 entries 流...");
    let mut stream = match client.subscribe_entries(SubscribeEntriesRequest {}).await {
        Ok(response) => {
            println!("成功订阅 entries 流");
            response.into_inner()
        }
        Err(e) => {
            eprintln!("订阅 entries 流失败: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, e));
        }
    };

    println!("开始接收数据...");
    
    // 创建一个信号量来限制并发任务数量
    let semaphore = Arc::new(Semaphore::new(10));
    
    loop {
        match stream.message().await {
            Ok(Some(slot_entry)) => {
                let entries =
                    match bincode::deserialize::<Vec<solana_entry::entry::Entry>>(&slot_entry.entries) {
                        Ok(e) => e,
                        Err(e) => {
                            eprintln!("反序列化失败，错误: {}", e);
                            continue;
                        }
                    };
                
                println!(
                    "slot {}, entries: {}, transactions: {}",
                    slot_entry.slot,
                    entries.len(),
                    entries.iter().map(|e| e.transactions.len()).sum::<usize>()
                );
                
                // 为每个 entry 创建异步任务来处理交易哈希
                for (entry_index, entry) in entries.into_iter().enumerate() {
                    let semaphore_clone = semaphore.clone();
                    let slot = slot_entry.slot;
                    
                    tokio::spawn(async move {
                        // 获取信号量许可
                        let _permit = semaphore_clone.acquire().await.unwrap();
                        
                        // 异步处理每个交易
                        for (tx_index, transaction) in entry.transactions.iter().enumerate() {
                            if let Some(signature) = transaction.signatures.get(0) {
                                println!(
                                    "Slot: {}, Entry: {}, Tx: {}, Hash: {}",
                                    slot,
                                    entry_index,
                                    tx_index,
                                    signature
                                );
                            } else {
                                eprintln!(
                                    "Slot: {}, Entry: {}, Tx: {} - 无法获取签名",
                                    slot,
                                    entry_index,
                                    tx_index
                                );
                            }
                        }
                    });
                }
            }
            Ok(None) => {
                println!("流已结束");
                break;
            }
            Err(e) => {
                eprintln!("接收消息时出错: {}", e);
                break;
            }
        }
    }
    Ok(())
}
