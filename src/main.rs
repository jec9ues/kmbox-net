pub mod constants;


use std::mem::size_of;
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
    let mut err = KmboxNet::new(ip, port, mac).await;
    err.init().await?;
    err.monitor().await?;
    // err.keyboard_down(keyboard_table::KEY_F1).await?;
    // tokio::time::sleep(Duration::from_millis(200)).await;
    // err.keyboard_down(0).await?;

    // err.mouse_move(Pos2 { x: 100, y: 100 }).await?;
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
    ///
    /// 连接kmboxNet盒子输入参数分别是盒子
    /// ip  ：盒子的IP地址 （显示屏上会有显示,例如：192.168.2.88）
    /// port: 通信端口号   （显示屏上会有显示，例如：6234）
    /// mac : 盒子的mac地址（显示屏幕上有显示，例如：12345）
    ///
    pub async fn new(ip: &str, port: &str, mac: &str) -> Self {
        KmboxNet {
            sock_client: UdpSocket::bind("0.0.0.0:0").await.unwrap(),
            mac: str_to_hex(mac, 4),
            indexpts: 0,
            km_addr: format!("{}:{}", ip, port),
        }
    }
    pub async fn init(&self) -> Result<NetErr, Box<dyn std::error::Error>> {
        let mut send_cache = [0; 1024];
        let mut recv_cache = [0; 1024];

        let tx = CmdHead {
            mac: self.mac,
            rand: random(),
            indexpts: self.indexpts,
            cmd: CONNECT,
        };


        let send_length = bincode::serde::encode_into_slice(&tx, &mut send_cache, bincode::config::legacy())?;
        self.sock_client.send_to(&send_cache[..send_length], &self.km_addr).await?;

        let (recv_length, _remote_addr) = self.sock_client.recv_from(&mut recv_cache).await?;
        let (recv, _bytes_read): (CmdHead, usize) = bincode::serde::decode_from_slice(&recv_cache[..recv_length], bincode::config::legacy())?;
        println!("{:?}", recv);

        Ok(net_rx_return_handle(&recv, &tx))
    }
    /// 鼠标移动x,y个单位。一次性移动。无轨迹模拟，速度最快.
    /// 自己写轨迹移动时使用此函数。
    /// 返回值：0正常执行，其他值异常。
    pub async fn mouse_move(&mut self, pos: Pos2) -> Result<NetErr, Box<dyn std::error::Error>> {
        self.indexpts = self.indexpts.wrapping_add(1);
        let mut send_cache = [0; 1024];
        let mut recv_cache = [0; 1024];

        let tx = CmdMouse {
            head: CmdHead {
                mac: self.mac,
                rand: random(),
                indexpts: self.indexpts,
                cmd: MOUSE_MOVE,
            },
            mouse: SoftMouse {
                button: 0,
                x: pos.x,
                y: pos.y,
                wheel: 0,
                point: [0; 10],
            },
        };


        let send_length = bincode::serde::encode_into_slice(&tx, &mut send_cache, bincode::config::legacy())?;
        self.sock_client.send_to(&send_cache[..send_length], &self.km_addr).await?;

        let (recv_length, _remote_addr) = self.sock_client.recv_from(&mut recv_cache).await?;
        let (recv, _bytes_read): (CmdMouse, usize) = bincode::serde::decode_from_slice(&recv_cache[..recv_length], bincode::config::legacy())?;
        // println!("{:?}", recv);
        Ok(net_rx_return_handle(&recv.head, &tx.head))
    }

    ///鼠标左键控制
    /// isdown :0松开 ，1按下
    /// 返回值：0正常执行，其他值异常。
    pub async fn mouse_button(&mut self, value: i32) -> Result<NetErr, Box<dyn std::error::Error>> {
        self.indexpts = self.indexpts.wrapping_add(1);
        let mut send_cache = [0; 1024];
        let mut recv_cache = [0; 1024];

        let tx = CmdMouse {
            head: CmdHead {
                mac: self.mac,
                rand: random(),
                indexpts: self.indexpts,
                cmd: MOUSE_LEFT,
            },
            mouse: SoftMouse {
                button: value,
                x: 0,
                y: 0,
                wheel: 0,
                point: [0; 10],
            },
        };


        let send_length = bincode::serde::encode_into_slice(&tx, &mut send_cache, bincode::config::legacy())?;
        self.sock_client.send_to(&send_cache[..send_length], &self.km_addr).await?;

        let (recv_length, _remote_addr) = self.sock_client.recv_from(&mut recv_cache).await?;
        let (recv, _bytes_read): (CmdMouse, usize) = bincode::serde::decode_from_slice(&recv_cache[..recv_length], bincode::config::legacy())?;
        // println!("{:?}", recv);
        Ok(net_rx_return_handle(&recv.head, &tx.head))
    }
    ///鼠标滚轮控制
    pub async fn mouse_wheel(&mut self, value: i32) -> Result<NetErr, Box<dyn std::error::Error>> {
        self.indexpts = self.indexpts.wrapping_add(1);
        let mut send_cache = [0; 1024];
        let mut recv_cache = [0; 1024];

        let tx = CmdMouse {
            head: CmdHead {
                mac: self.mac,
                rand: random(),
                indexpts: self.indexpts,
                cmd: MOUSE_WHEEL,
            },
            mouse: SoftMouse {
                button: 0,
                x: 0,
                y: 0,
                wheel: value,
                point: [0; 10],
            },
        };


        let send_length = bincode::serde::encode_into_slice(&tx, &mut send_cache, bincode::config::legacy())?;
        self.sock_client.send_to(&send_cache[..send_length], &self.km_addr).await?;

        let (recv_length, _remote_addr) = self.sock_client.recv_from(&mut recv_cache).await?;
        let (recv, _bytes_read): (CmdMouse, usize) = bincode::serde::decode_from_slice(&recv_cache[..recv_length], bincode::config::legacy())?;
        // println!("{:?}", tx);
        Ok(net_rx_return_handle(&recv.head, &tx.head))
    }

    pub async fn keyboard_down(&mut self, value: u32) -> Result<NetErr, Box<dyn std::error::Error>> {
        self.indexpts = self.indexpts.wrapping_add(1);
        let mut send_cache = [0; 1024];
        let mut recv_cache = [0; 1024];

        let mut tx = CmdKeyboard {
            head: CmdHead {
                mac: self.mac,
                rand: random(),
                indexpts: self.indexpts,
                cmd: KEYBOARD_ALL,
            },
            keyboard: SoftKeyboard {
                ctrl: 0,
                resvel: 0,
                button: [0; 10],
            }
        };
        tx.keyboard.button[0] = value as i8;

        let send_length = bincode::serde::encode_into_slice(&tx, &mut send_cache, bincode::config::legacy())?;
        self.sock_client.send_to(&send_cache[..send_length], &self.km_addr).await?;

        let (recv_length, _remote_addr) = self.sock_client.recv_from(&mut recv_cache).await?;
        let (recv, _bytes_read): (CmdKeyboard, usize) = bincode::serde::decode_from_slice(&recv_cache[..recv_length], bincode::config::legacy())?;
        // println!("{:?}", tx);
        Ok(net_rx_return_handle(&recv.head, &tx.head))
    }
}

