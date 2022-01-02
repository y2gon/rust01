use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

const MAX_LENGTH : usize = 256; // 입력가능한 최대 문자열의 길이

// (server 동작 및 client 연결이 정상적일 때) 입력 값 처리 함수.
fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; MAX_LENGTH]; // 한번에 입력받을 수 있는 최대 문자열 길이를 256 자로 설정
    
    // client 로 부터 입력받은 stream 값을 읽어서 data 에 저장
    while match stream.read(&mut data) {
        
        // data 의 크기가 정상적인 숫자의 값을 가지는 경우 (제대로 입력이 이루어졌을 때)
        Ok(size) => {
            // 해당 data 를 stream 에 binary  값으로 다시 전송
            stream.write(&data[0..size]).unwrap();
            true
        },
        // data 에 정상적으로 입력되지 않았을 경우, error message 를 보내고 서버를 종료.
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // 표준 모듈을 사용하여  Tcp 기반의 소켓을 생성하고, 페킷이 오기를 대기.
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    println!("Server listening on port 3333");

    // TcpListener 가 정상적으로 작동할때의 result 를 incoming 매소드를 사용하여 반복하여 stream 변수에 할당. 
    for stream in listener.incoming() { 

        // stream 값이 정상적일 때와, 문제가 있을 때를 구분
        match stream {

            // stream 이 정상적일 때, 
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                
                // 해당 stream 을 처리할 thread 를 생성하고, 처리함수를 실행.
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }

            // stream 에 문제가 있을 때
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}