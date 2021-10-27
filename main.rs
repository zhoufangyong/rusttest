//使用了io库，用于读写
use std::io::{Error,Read,Write};
//使用了网络库用于监听Tcp流
use std::net::{TcpListener, TcpStream};
use std::thread;        //线程
use std::time;

//TCP客户处理，供主函数调用
fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    //申明变量，512大小的缓冲
    let mut buf = [0; 512]; 
    //循环读缓冲，直到读完
    for _ in 0..1000 { 
        let bytes_read = stream.read(&mut buf)?; 
        if bytes_read == 0 { 
        return Ok(()); 
    }
    //写入读取的数据
    stream.write(&buf[..bytes_read])?; 
    thread::sleep(time::Duration::from_secs(1 as u64)); 
    }
    Ok(()) 
}

//main为主函数入口
fn main() -> std::io::Result<()> { 
    //监听IP127.0.0.1的8080端口
    let listener = TcpListener::bind("127.0.0.1:8080")?; 
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
            let stream = stream.expect("failed!"); 
            let handle = thread::spawn(move || {
                //调用客户端处理程序
                handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error)); 
            }); 
            
            thread_vec.push(handle); 
    } 
    for handle in thread_vec {
        handle.join().unwrap(); 
    }                 
    
    Ok(()) 
}



    
