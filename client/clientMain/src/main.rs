use native_tls::TlsConnector;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::fs::File;

const CODE_LENGTH : usize = 3; // 1 글자만 입력 허용 (2 는 개행시 입력되는 코드를 받기 위한 buffer)
const CONTENTS_LENGTH : usize = 256; // 파일 내용의 글자수를 전송할 수있는 최대 크기를 256 자 이내로 제한.

fn main() {
    // localhost:3333 에 연결이 정상적으로 될 경우와 실패할 경우를 구분
    match TcpStream::connect("localhost:3333"){
        // 정상적으로 연결이 된 경우,
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            let mut msg_length  = CODE_LENGTH + 1;
            let mut input = String::new();

            // 표준 입력 받기 (1자를 넘게 입력할 경우, 글자수 초과 메세지를 남기고 재입력 요청)
            while msg_length > CODE_LENGTH {
                
                println!("Please input the access code (Maxium length : {}): ", CODE_LENGTH-2);
                msg_length = std::io::stdin().read_line(&mut input).unwrap();

                if msg_length > CODE_LENGTH {
                    println!("Please input one chararter.");
                    input.clear();
                }
            }
            
            // 표준 입력 string 을 ASCII code 값의 array로 전환
            let msg = input.as_bytes();
            
            // binary 로 변환된 입력값을 전송
            stream.write(&msg).unwrap();
            println!("Sent the code, awaiting reply...");

            // Server로부터 전송된 stream 값을 data array 에 입력
            let mut fromServer:[u8;1] =[0];
            match stream.read_exact(&mut fromServer) {
                Ok(_) => {
                    // client 로 재전송된 값이 기존에 server 로 보낸 message 와 동일한지 확인하여 T("1")/F(_) 를 분기하여 처리.
                    if from_utf8(&mut fromServer).unwrap() == "1" {
                        //FTP protocol 을 사용한 통신 진행
                        let connector = TlsConnector::new().unwrap();
                        let stream = TcpStream::connect("127.0.0.1:8443").unwrap();
                        let mut stream = connector.connect("127.0.0.1:8443", stream).unwrap();

                        println!("Accepted! start loading a file. Please input the file name. (.txt file only)");

                        // 파일명을 표준입럭 받아서 server 에 전송
                        std::io::stdin().read_line(&mut input).unwrap();

                        match File::open(&mut input) {
                            // 해당 파일이 존재 할때 
                            Ok(mut file) => { 
                                let mut data :[u8;CONTENTS_LENGTH] = [0 as u8; 256];
                                file.read(&mut data).unwrap();

                                let mut sendingData = data.as_bytes();
                                stream.write(&sendingData).unwrap();
                                println!("Transmission complete!");
                            }
                            Err(_) => {
                                println!("The file is not exist.")
                            }
                        }
                    } else {
                        println!("Denied to upload a file");
                    }
                }
                Err(e) => {
                    println!("Failed to receive data : {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect : {}", e);
        }
    }
    println!("Terminated.");
}