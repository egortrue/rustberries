use app::domain::{snake::Snake, world::World};
use std::{
    env,
    io::Read,
    net::TcpStream,
    ops::DerefMut,
    process::exit,
    sync::{Arc, RwLock},
    thread,
};

pub fn run(snake: Arc<RwLock<Snake>>, world: Arc<RwLock<World>>) {
    // Подключение к серверу
    let socket = env::var("SOCKET_UPDATE_WORLD").expect("ENV not declared: SOCKET_UPDATE_WORLD");
    let mut stream = match TcpStream::connect(socket) {
        Ok(s) => s,
        Err(_) => {
            log::error!("Server not started!");
            exit(1)
        }
    };

    // Получение начального состояния и постоянное обновление мира в отдельном потоке
    let mut world_buffer = [0; 100 * 1024];
    update_world(&mut stream, &mut world_buffer, &world);
    thread::spawn(move || loop {
        update_world(&mut stream, &mut world_buffer, &world);
    });
}

fn update_world(stream: &mut TcpStream, buffer: &mut [u8], world: &Arc<RwLock<World>>) {
    match stream.read(buffer) {
        Ok(0) => {
            log::error!("Connection closed by remote server!");
            exit(1);
        }
        Ok(bytes) => {
            log::info!("Received {bytes} bytes");
            let new_world: World = serde_json::from_slice(&buffer[..bytes]).unwrap();
            let _ = std::mem::replace(world.write().unwrap().deref_mut(), new_world);
        }
        Err(error) => {
            log::error!("Connection closed: {error}");
            exit(1);
        }
    }
}
