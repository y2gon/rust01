use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

const MAX_LENGTH : usize = 256; // 입력가능한 최대 문자열의 길이

fn main() {
    // localhost:3333 에 연결이 정상적으로 될 경우와 실패할 경우를 구분
    match TcpStream::connect("localhost:3333"){
        // 정상적으로 연결이 된 경우,
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            let mut msg_length  = MAX_LENGTH + 1;
            let mut input = String::new();

            // 표준 입력 받기 (256자를 넘게 입력할 경우, 글자수 초과 메세지를 남기고 재입력 요청)
            while msg_length > MAX_LENGTH {
                
                println!("Message input (Maxium length = 256): ");
                msg_length = std::io::stdin().read_line(&mut input).unwrap();

                if msg_length > MAX_LENGTH {
                    println!("Please input message under 256 characters.");
                    input.clear();
                }
            }

            let mut msg = [0; MAX_LENGTH];  
            
            // 표준 입력 string 을 ASCII code 값의 array로 전환
            for (idx, b_char ) in input.chars().enumerate(){
                msg[idx] = b_char as u8;  
            }
            
            // 최대 256 크기의 array에서 실제 문자값이 포함된 부분까지 slice 하여 stream 에 binary 로 전송
            stream.write(&msg[0..(msg_length-2)]).unwrap();
            println!("Sent Hello, awaiting reply...");
            
            // 서버에서 전송한 stream 을 처리
            let mut data = [0 as u8; MAX_LENGTH]; 

            // 전송된 stream 값을 data array 에 입력 하고, 해당 작업이 정상적으로 진행되었는지에 따라 처리
            match stream.read_exact(&mut data[0..(msg_length-2)]) {
                Ok(_) => {
                    // client 로 재전송된 값이 기존에 server 로 보낸 message 와 동일한지 확인하여 T/F 를 분기하여 처리.
                    if &data[0..(msg_length-2)] == &msg[0..(msg_length-2)]{
                        println!("Receiving Message : {}" , from_utf8(&data).unwrap());
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data : {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect : {}", e);
        }
    }
    println!("Terminated.");
}
