pub mod constants;


use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use rand::{random, Rng};
use serde::{Serialize, Deserialize};
use bincode;
use crate::constants::*;
use crate::constants::cmd::CONNECT;
use tokio::net::UdpSocket;




// 枚举错误代码
#[derive(Debug)]
enum ErrorCode {
    ErrCreateSocket = -9000, // 创建socket失败
    ErrNetVersion,           // socket版本错误
    ErrNetTx,                // socket发送错误
    ErrNetRxTimeout,         // socket接收超时
    ErrNetCmd,               // 命令错误
    ErrNetPts,               // 时间戳错误
    Success = 0,             // 正常执行
    UsbDevTxTimeout,         // USB devic发送失败
}

// 初始化连接盒子
/*pub fn km_net_init(ip: &str, port: &str, mac: &str) {
    // 创建Socket地址
    let addr = format!("{}:{}", ip, port);
    let socket_addr: SocketAddr = addr.parse().map_err(|_| ErrorCode::ErrCreateSocket).expect("");

    // 创建TCP连接
    let stream = UdpStream::connect_timeout(&socket_addr, Duration::from_secs(10))
        .map_err(|_| ErrorCode::ErrCreateSocket).unwrap();

    // 创建命令头
    let cmd_head = CmdHead {
        mac: mac.parse().unwrap(),
        rand: 0, // 你可以设置随机值
        indexpts: 0, // 你可以设置时间戳
        cmd: CMD_CONNECT,
    };



}*/
async fn km_net_init(ip: &str, port: &str, mac: &str) -> Result<(), std::io::Error> {
    // 初始化随机数生成器
    let mut rng = rand::thread_rng();
    // 创建并初始化 CmdHead 结构体
    let tx = CmdHead {
        mac: str_to_hex(mac, 4),
        rand: rng.gen(),
        indexpts: 0,
        cmd: CONNECT
    };

    let mut send_cache = [0; 1024];
    let mut recv_cache = [0; 1024];
    let mut udp_socket = UdpSocket::bind("0.0.0.0:0").await?;

    let send_length = bincode::serde::encode_into_slice(&tx, &mut send_cache, bincode::config::legacy()).unwrap();
    udp_socket.send_to(&send_cache[..send_length], format!("{}:{}", ip, port)).await?;

    let (recv_length, _remote_addr) = udp_socket.recv_from(&mut recv_cache).await?;
    let recv: (ClientTx, usize) = bincode::serde::decode_from_slice(&recv_cache[..recv_length], bincode::config::legacy()).unwrap();
    // println!("{:?}", recv.0);

    net_rx_return_handle(&recv.0, &tx)
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = "192.168.2.188";
    let port = "16582";
    let mac = "75D65054";
    km_net_init(ip, port, mac).await?;

    // 添加您的应用程序逻辑

    Ok(())
}


