use http_muncher::*;
use mio::tcp::*;
use std::io::Read;

struct HttpParserHandler;
impl ParserHandler for HttpParserHandler { }

pub struct WebSocketClient {
    pub socket: TcpStream,
    http_parser: Parser
}

impl WebSocketClient {
    pub fn read(&mut self) {
        let mut handler = HttpParserHandler {};

        loop {
            let mut buf = [0; 2048];
            match self.socket.read(&mut buf) {
                Err(e) => {
                    println!("Ошибка чтения сокета: {:?}", e);
                    return
                },
                Ok(len) => {
                    self.http_parser.parse(&mut handler, &buf[0..len]);
                    if self.http_parser.is_upgrade() {
                        // ...
                        break;
                    }
                }
            }
        }
    }

    pub fn new(socket: TcpStream) -> WebSocketClient {
        WebSocketClient {
            socket: socket,
            http_parser: Parser::request()
        }
    }
}