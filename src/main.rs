pub mod constants;


use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use rand::{random, Rng};
use serde::{Serialize, Deserialize};
use bincode;
use tokio::net::UdpSocket;
use crate::constants::cmd::*;
use crate::constants::*;



/*async fn km_net_init(ip: &str, port: &str, mac: &str) -> Result<NetErr, std::io::Error> {
    // 初始化随机数生成器
    let mut rng = rand::thread_rng();
    // 创建并初始化 CmdHead 结构体
    let tx = CmdHead {
            mac: str_to_hex(mac, 4),
            rand: rng.gen(),
            indexpts: 0,
            cmd: CONNECT,
    };

    let mut send_cache = [0; 1024];
    let mut recv_cache = [0; 1024];
    let mut udp_socket = UdpSocket::bind("0.0.0.0:0").await?;

    let send_length = bincode::serde::encode_into_slice(&tx, &mut send_cache, bincode::config::legacy()).unwrap();
    udp_socket.send_to(&send_cache[..send_length], format!("{}:{}", ip, port)).await?;

    let (recv_length, _remote_addr) = udp_socket.recv_from(&mut recv_cache).await?;
    let recv: (CmdHead, usize) = bincode::serde::decode_from_slice(&recv_cache[..recv_length], bincode::config::legacy()).unwrap();
    println!("{:?}", recv.0);

    Ok(net_rx_return_handle(&recv.0, &tx))
}*/



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = "192.168.2.188";
    let port = "16582";
    let mac = "75D65054";
    let err = KmboxNet::new(ip, port, mac).await;
    err.init().await?;
    // println!("{:?}", err);
    // 添加您的应用程序逻辑

    Ok(())
}
#[derive(Debug)]
pub struct KmboxNet {
    sock_client: UdpSocket,
    mac: u32,
    indexpts: u32,
    km_addr: String,
}

impl KmboxNet {
        pub async fn new(ip: &str, port: &str, mac: &str) -> Self {
            KmboxNet {
                sock_client: UdpSocket::bind("0.0.0.0:0").await.unwrap(),
                mac: str_to_hex(mac, 4),
                indexpts: 0,
                km_addr: format!("{}:{}", ip, port),
            }
        }
        pub async fn init(&self) -> Result<NetErr, Box<dyn std::error::Error>> {

        let mut rng = rand::thread_rng();
        let mut send_cache = [0; 1024];
        let mut recv_cache = [0; 1024];

        let tx = CmdHead {
            mac: self.mac,
            rand: rng.gen(),
            indexpts: self.indexpts,
            cmd: CONNECT,
        };


        let send_length = bincode::serde::encode_into_slice(&tx, &mut send_cache, bincode::config::legacy())?;
        self.sock_client.send_to(&send_cache[..send_length], &self.km_addr).await?;

        let (recv_length, _remote_addr) = self.sock_client.recv_from(&mut recv_cache).await?;
        let (recv, _bytes_read) = bincode::serde::decode_from_slice(&recv_cache[..recv_length], bincode::config::legacy())?;
        // println!("{:?}", recv);

        Ok(net_rx_return_handle(&recv, &tx))
    }
}

