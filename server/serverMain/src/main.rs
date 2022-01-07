use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::sync::Arc;
use std::fs::File;

const CODE_LENGTH : usize = 3;   // FTP 통신을 위한 입력값에 대해 입력가능한 최대 문자열의 길이
const FILE_DATA_LENGTH : usize = 256; // 전송된 파일에 대한 Data 에 대해 입력 가능한 최대 문자열의 길이
const CODE : &str = "1";        // FTP 접속 승인 code
 
// (server 동작 및 client 연결이 정상적일 때) 입력 값 처리 함수.
fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; CODE_LENGTH];  // 전송된 CODE 값을 입력받을 array (CODE : 1 , 개행문자 : 2)
    let mut status =[0 as u8; 1];           // client 에 T/F 현황을 전송할 array ("1" == True / "0" == False)
    // client 로 부터 입력받은 stream 값을 읽어서 data 에 저장
    while match stream.read(&mut data) {
        // data 의 크기가 정상적인 숫자의 값을 가지는 경우 (제대로 입력이 이루어졌을 때)
        Ok(_size) => {
            if from_utf8(&mut data[0..(CODE_LENGTH-2)]).unwrap() == CODE{
                status[0] = '1' as u8;              // '1' (accepted) 를 cleint 로 보냄
                stream.write(&status).unwrap();

                // FTP 통신을 위한 준비 및 thread 생성
                let identity = Identity::from_pkcs12(&mut data[0..(CODE_LENGTH-2)], CODE).unwrap();
                let listener = TcpListener::bind("127.0.0.1:8443").unwrap();
                let acceptor = TlsAcceptor::new(identity).unwrap();
                let acceptor = Arc::new(acceptor);
                
                for fileStream in listener.incoming() {
                    match fileStream{
                        Ok(fileStream) => {
                            let acceptor = acceptor.clone();

                            // FTP Data 통신에 사용할 Thread 생성
                            thread::spawn(move||{
                                let fileStream = acceptor.accept(fileStream).unwrap();
                                
                                let mut fileData = [0 as u8; FILE_DATA_LENGTH];
                                while match fileStream.read(&mut fileData){
                                    Ok(size) => {
                                        let title:&str = fileData[0..size].chars();
                                        let mut file = File::create(title);

                                    }
                                    Err(e) => {println!("Error: {}", e)}
                                
                            });
                            
                        }
                        Err(e) => {println!("Error: {}", e)}
                    }
                }
                true
            } else {
                status[0] = '0' as u8;              // client 입력 코드가 server 에서 가지고 있는 code 값과 다를 경우 "0" (False) 를 client 로 전송
                stream.write(&status).unwrap();
                false
            }
            
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
    let mut acceptor:bool = false;

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
                handle_client(stream);
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