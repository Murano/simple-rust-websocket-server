extern crate mio;
use mio::deprecated::{EventLoop, Handler};
use mio::tcp::*;
use std::net::SocketAddr;
use mio::{PollOpt, Ready};
use std::collections::HashMap;

extern crate http_muncher;


mod websocket_server;
use websocket_server::*;

mod websocket_client;
use websocket_client::*;


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
