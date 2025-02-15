use std::net::UdpSocket;
use std::thread;
use std::io::{self, Write};

fn main() -> io::Result<()> {    
    // 设置本地监听地址    
    println!("请输入本地监听地址（例如 127.0.0.1:8080）：");    
    let mut local_address = String::new();    
    io::stdin().read_line(&mut local_address).unwrap();    
    let local_address = local_address.trim().to_string();    
    
    // 设置目标地址    
    println!("请输入目标地址（例如 127.0.0.1:8081）：");    
    let mut target_address = String::new();    
    io::stdin().read_line(&mut target_address).unwrap();    
    let target_address = target_address.trim().to_string();    
    
    // 创建 UDP socket 并绑定到本地地址    
    let socket = UdpSocket::bind(&local_address)?;    
    println!("已绑定到本地地址 {}", local_address);    
    
    // 克隆 socket，用于发送和接收线程    
    let send_socket = socket.try_clone()?;    
    let recv_socket = socket;    
    
    // 接收消息线程    
    let recv_thread = thread::spawn(move || {        
        let mut buf = [0; 1024];        
        loop {            
            match recv_socket.recv_from(&mut buf) {                
                Ok((size, src)) => {                    
                    let msg = String::from_utf8_lossy(&buf[..size]);                    
                    println!("\n收到来自 {} 的消息：{}", src, msg);                
                }                
                Err(e) => {                    
                    eprintln!("接收消息时出错：{}", e);                    
                    break;                
                }            
            }        
        }    
    });    
    
    // 发送消息线程    
    let send_thread = thread::spawn(move || {        
        loop {            
            print!("输入消息：");            
            io::stdout().flush().unwrap();            
            let mut input = String::new();            
            io::stdin().read_line(&mut input).unwrap();            
            let msg = input.trim();            
            if msg.is_empty() {                
                continue;            
            }
            print!("1.加入聊天组1\n");            
            print!("2.加入聊天组2\n");            
            print!("3.不加入聊天组\n");            
            io::stdout().flush().unwrap();            
            let mut input = String::new();            
            io::stdin().read_line(&mut input).unwrap();            
            let group = input.trim();
            match group{
                "1" => {
                    if let Err(e) = send_socket.send_to(msg.as_bytes(), "127.0.0.1:8085") {                
                        eprintln!("发送消息时出错：{}", e); 
                    }
                    if let Err(e) = send_socket.send_to(msg.as_bytes(), "127.0.0.1:8086") {                
                        eprintln!("发送消息时出错：{}", e); 
                    }

                }
                "2" => {
                    if let Err(e) = send_socket.send_to(msg.as_bytes(), "127.0.0.1:8087") {                
                        eprintln!("发送消息时出错：{}", e); 
                    }
                    if let Err(e) = send_socket.send_to(msg.as_bytes(), "127.0.0.1:8088") {                
                        eprintln!("发送消息时出错：{}", e); 
                    }
                }
                "3" => {
                    if let Err(e) = send_socket.send_to(msg.as_bytes(), &target_address) {                
                        eprintln!("发送消息时出错：{}", e);  
                    }
                }
                &_ => {
                    println!("输入错误，请重新输入");
                    continue;
                    } 
                }           

        
                        
        }    
    });    
    
    // 等待线程完成    
    recv_thread.join().unwrap();    
    send_thread.join().unwrap();    
    Ok(())
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
