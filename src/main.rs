extern crate mio;
use mio::deprecated::{EventLoop, Handler};
use mio::tcp::*;
use std::net::SocketAddr;
use std::collections::HashMap;
use mio::{Token, PollOpt, Ready};

struct WebSocketServer {
    socket: TcpListener,
    clients: HashMap<Token, TcpStream>,
    token_counter: usize
}

const SERVER_TOKEN: Token = Token(0);

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<WebSocketServer>,
             token: Token, events: Ready)
    {
        match token {
            SERVER_TOKEN => {
                let client_socket = match self.socket.accept() { //TODO вроде пашет, разобраться и делать дальше
                    Err(e) => {
                        println!("Ошибка установления подключения: {}", e);
                        return;
                    },

                    Ok((sock, _)) => sock
                };

                self.token_counter += 1;
                let new_token = Token(self.token_counter);

                self.clients.insert(new_token, client_socket);
                event_loop.register(&self.clients[&new_token],
                                    new_token, Ready::readable(),
                                    PollOpt::edge() | PollOpt::oneshot()).unwrap();
            },
            Token(vall) => {
//                panic!("Errr");
                println!("Token not srv: {}", vall);
                return;
            }
        }
    }
}

fn main() {
    let mut event_loop = EventLoop::new().unwrap();


    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    let mut server = WebSocketServer {
        token_counter: 1,        // Начинаем отсчет токенов с 1
        clients: HashMap::new(), // Создаем пустую хеш-таблицу, HashMap
        socket: server_socket    // Передаем владение серверным сокетом в структуру
    };

    event_loop.register(&server.socket, SERVER_TOKEN, Ready::readable(), PollOpt::edge()).unwrap();

    event_loop.run(&mut server).unwrap();
}
