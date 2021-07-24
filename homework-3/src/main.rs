use std::io::prelude::*;
use std::net::TcpListener;


fn main() {
    // 创建TCP连接 监听本地7878端口
    let listener = TcpListener::bind("localhost:8080").unwrap();

    //循环获取流数据
    for stream in listener.incoming() {
        //创建1024字节缓冲区
        let mut buffer = [0; 1024];
        //获取输入内容
        let resut = stream.unwrap().read(&mut buffer);
        //打印输入内容
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        //模式匹配读取的结果
        match resut {
            //读取正常
            Ok(_) => {
                println!("input stream is  ok!");
            }
            //读取有误，panic退出
            Err(error) => {
                println!("input stream is error!");
                panic!("Problem opening the file: {:?}", error)
            }
        }
    }
}
