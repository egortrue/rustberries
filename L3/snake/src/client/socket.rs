use app::domain::{snake::Snake, world::World};
use std::{
    env,
    io::{Read, Write},
    net::TcpStream,
    ops::{Deref, DerefMut},
    process::exit,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

pub fn run(snake: Arc<RwLock<Snake>>, world: Arc<RwLock<World>>) {
    let timeout = Duration::from_millis(
        env::var("WORLD_UPDATE_TIME")
            .expect("ENV not found: WORLD_UPDATE_TIME")
            .parse()
            .unwrap(),
    );

    // Скачивание мира
    let mut buffer = [0; 100 * 1024];
    let mut stream_world = get_stream_world();
    recieve_world(&mut stream_world, &mut buffer, &world); // Предварительная загрузка для формирования GUI
    thread::spawn(move || loop {
        recieve_world(&mut stream_world, &mut buffer, &world);
    });

    // Загрузка змейки
    let mut stream_snake = get_stream_snake();
    thread::spawn(move || loop {
        thread::sleep(timeout);
        send_snake(&mut stream_snake, &snake);
    });
}

pub fn get_stream_world() -> TcpStream {
    let socket = env::var("SOCKET_BROADCAST_WORLD").expect("ENV not found: SOCKET_BROADCAST_WORLD");
    match TcpStream::connect(socket) {
        Ok(s) => s,
        Err(_) => {
            log::error!("Server not started!");
            exit(1)
        }
    }
}

pub fn get_stream_snake() -> TcpStream {
    let socket = env::var("SOCKET_UPDATE_SNAKE").expect("ENV not found: SOCKET_UPDATE_SNAKE");
    match TcpStream::connect(socket) {
        Ok(s) => s,
        Err(_) => {
            log::error!("Server not started!");
            exit(1);
        }
    }
}

pub fn recieve_world(stream: &mut TcpStream, buffer: &mut [u8], world: &Arc<RwLock<World>>) {
    let address = stream.peer_addr().unwrap();
    match stream.read(buffer) {
        Ok(0) => {
            log::error!("Disconnected {address}");
            exit(1);
        }
        Ok(bytes) => {
            log::info!("Received {bytes} bytes from {address}");
            if let Ok(new_world) = serde_json::from_slice(&buffer[..bytes]) {
                let _ = std::mem::replace(world.write().unwrap().deref_mut(), new_world);
            };
        }
        Err(error) => {
            log::error!("Disconnected {address}: {error}");
            exit(1);
        }
    }
}

pub fn send_snake(stream: &mut TcpStream, snake: &Arc<RwLock<Snake>>) {
    let address = stream.peer_addr().unwrap();
    let serialized_snake = serde_json::to_string(snake.read().unwrap().deref()).unwrap();
    match stream.write(serialized_snake.as_bytes()) {
        Ok(0) => {
            log::info!("Disconnected {address}");
            exit(1);
        }
        Ok(bytes) => log::info!("Send {bytes} bytes to {address}"),
        Err(error) => {
            log::error!("Disconnected {address}: {error}");
            exit(1);
        }
    }
}
