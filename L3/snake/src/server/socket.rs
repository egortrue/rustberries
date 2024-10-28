use app::domain::world::World;
use dashmap::DashMap;
use std::{
    env,
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
    ops::Deref,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

pub fn run(world: Arc<RwLock<World>>) {
    let socket = env::var("SOCKET_UPDATE_WORLD").expect("ENV not declared: SOCKET_UPDATE_WORLD");
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
    loop {
        let mut disconnected = vec![];

        // Отправка данных о мире
        for user in connections.iter() {
            let world = serde_json::to_string(world.read().unwrap().deref()).unwrap();
            let address = user.key();
            let mut stream = user.value();

            match stream.write(world.as_bytes()) {
                Ok(bytes) => log::info!("Send {bytes} bytes to {address}"),
                Err(error) => {
                    log::warn!("Disconnected {address}: {error}");
                    disconnected.push(address.clone());
                }
            }
        }

        // Удаление отключившихся пользователей
        for user in disconnected {
            connections.remove(&user);
        }

        // Обновление мира
        thread::sleep(Duration::from_millis(world.read().unwrap().update_time));
        world.write().unwrap().spawn_apple();
    }
}
