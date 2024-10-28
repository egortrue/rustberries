use app::domain::{snake::Snake, world::World};
use dashmap::DashMap;
use std::{
    env,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    ops::Deref,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

pub fn run(world: Arc<RwLock<World>>) {
    recieve_snakes(world.clone());
    broadcast_world(world.clone());
}

fn broadcast_world(world: Arc<RwLock<World>>) {
    let socket = env::var("SOCKET_BROADCAST_WORLD").expect("ENV not found: SOCKET_BROADCAST_WORLD");
    let connections: Arc<DashMap<SocketAddr, TcpStream>> = Arc::new(DashMap::new());
    let listener = TcpListener::bind(socket).expect("Failed to create listener");

    // Подключение пользователей
    let connections_accepter = connections.clone();
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let user_address = stream.peer_addr().unwrap();
                    log::info!("New connection from: {:?}", user_address);
                    connections_accepter.insert(user_address, stream);
                }
                Err(error) => log::error!("Failed to connect user: {error}"),
            }
        }
    });

    log::info!("Server started successfully");

    let timeout = Duration::from_millis(env::var("WORLD_UPDATE_TIME").unwrap().parse().unwrap());
    let mut disconnected = vec![];
    loop {
        thread::sleep(timeout);

        // Отправка данных о мире
        for user in connections.iter() {
            let serialized_world = serde_json::to_string(world.read().unwrap().deref()).unwrap();
            let address = user.key();
            let mut stream = user.value();

            match stream.write(serialized_world.as_bytes()) {
                Ok(0) => {
                    log::info!("Disconnected {address}");
                    disconnected.push(address.clone());
                }
                Ok(bytes) => log::info!("Send {bytes} bytes to {address}"),
                Err(error) => {
                    log::warn!("Disconnected {address}: {error}");
                    disconnected.push(address.clone());
                }
            }
        }

        // Удаление отключившихся пользователей
        for user in &disconnected {
            connections.remove(&user);
        }
        disconnected.clear();
    }
}

fn recieve_snakes(world: Arc<RwLock<World>>) {
    let socket = env::var("SOCKET_UPDATE_SNAKE").expect("ENV not found: SOCKET_UPDATE_SNAKE");
    let connections: Arc<DashMap<SocketAddr, TcpStream>> = Arc::new(DashMap::new());
    let listener = TcpListener::bind(socket).expect("Failed to create listener");

    // Подключение пользователей
    let connections_accepter = connections.clone();
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let user_address = stream.peer_addr().unwrap();
                    log::info!("New connection from: {:?}", user_address);
                    connections_accepter.insert(user_address, stream);
                }
                Err(error) => log::error!("Failed to connect user: {error}"),
            }
        }
    });

    let timeout = Duration::from_millis(env::var("WORLD_UPDATE_TIME").unwrap().parse().unwrap());
    let mut disconnected = vec![];

    thread::spawn(move || loop {
        thread::sleep(timeout);

        // Получение новых данных о змеях
        let mut buffer = [0u8; 10 * 1024];
        let mut world = world.write().unwrap();
        for user in connections.iter() {
            let mut stream = user.value();
            let address = stream.peer_addr().unwrap();

            match stream.read(&mut buffer) {
                Ok(0) => {
                    log::warn!("Disconnected {address}");
                    disconnected.push(address);
                }
                Ok(bytes) => {
                    log::info!("Received {bytes} bytes from {address}");

                    let new_snake: Snake = serde_json::from_slice(&buffer[..bytes]).unwrap();
                    let old_snake = world
                        .snakes
                        .iter_mut()
                        .enumerate()
                        .find(|(_, el)| el.username == new_snake.username);

                    // Обновление данных о змейке (если существует и жива)
                    if let Some((index, snake)) = old_snake {
                        if new_snake.is_alive() {
                            let _ = std::mem::replace(snake, new_snake);
                        } else {
                            world.snakes.swap_remove(index);
                        }
                    }
                    // Добавление новой змейки
                    else {
                        if new_snake.is_alive() {
                            world.snakes.push(new_snake);
                        }
                    }
                }
                Err(error) => {
                    log::warn!("Disconnected {address}: {error}");
                    disconnected.push(address);
                }
            }
        }

        // Удаление отключившихся пользователей
        for user in &disconnected {
            connections.remove(&user);
        }
        disconnected.clear();
    });
}
