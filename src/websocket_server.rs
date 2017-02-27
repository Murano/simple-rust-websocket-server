extern crate mio;
use mio::tcp::*;
use std::collections::HashMap;
use mio::{Token, PollOpt, Ready};
use mio::deprecated::{EventLoop, Handler};


pub struct WebSocketServer {
    pub socket: TcpListener,
    pub clients: HashMap<Token, TcpStream>,
    pub token_counter: usize
}

pub const SERVER_TOKEN: Token = Token(0);

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<WebSocketServer>,
             token: Token, events: Ready)
    {
        match token {
            SERVER_TOKEN => {
                let client_socket = match self.socket.accept() { //TODO further chapter 9
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
                println!("New client added");
            },
            Token(vall) => {
                //                panic!("Errr");
                println!("Token not srv: {}", vall);

                return;
            }
        }
    }
}