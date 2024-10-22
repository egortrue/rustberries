use super::message::Message;
use std::net::SocketAddr;
use tokio::{io::AsyncWriteExt, sync::broadcast::Receiver};
use tokio_util::sync::CancellationToken;

pub struct User {
    name: String,
    address: SocketAddr,

    // Токен подключения
    token: Option<CancellationToken>,
}

impl User {
    pub fn new(name: String, address: SocketAddr) -> Self {
        Self {
            name,
            address,
            token: None,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn is_subscribed(&self) -> bool {
        self.token.is_some()
    }

    pub fn subscribe(&mut self, receiver: Receiver<Message>) -> Result<(), String> {
        if self.is_subscribed() {
            return Err("User already subscribed!".to_string());
        }

        // Подготовка данных
        let token = CancellationToken::new();
        let address = self.address.clone();
        let mut receiver = Box::new(receiver);
        self.token = Some(token.clone());

        // Асинхронный поток отправки данных клиенту в режиме реального времени
        // Данные отправляются по тригеру broadcast::Receiver
        // Сам ресивер живет только в этом потоке и удалиться вместе остановкой потока по токену
        // Пользователь на данный момент уже должен прослушивать указнный сокет
        tokio::spawn(async move {
            if let Ok(mut stream) = tokio::net::TcpStream::connect(address).await {
                loop {
                    tokio::select! {
                        _ = token.cancelled() => {
                            break;
                        }
                        answer = receiver.recv() => {
                            match answer {
                                Ok(message) => {
                                    let data = serde_json::to_string(&message).unwrap();
                                    match stream.write(data.as_bytes()).await {
                                        Ok(bytes) => log::trace!("Send {bytes}b to {address}"),
                                        Err(_) => break,
                                    }

                                }
                                Err(_) => break,
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    pub fn unsubscribe(&mut self) -> Result<(), String> {
        if !self.is_subscribed() {
            return Err("User already unsubscribed!".to_string());
        }

        self.token.as_ref().unwrap().cancel();
        self.token = None;
        Ok(())
    }
}
