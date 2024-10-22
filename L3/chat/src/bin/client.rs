use rand::Rng;
use reqwest::Client;
use serde_json::json;
use std::{
    error::Error,
    io::{stdout, Write},
    net::{SocketAddr, ToSocketAddrs},
};
use tokio::io::AsyncReadExt;

const SERVER: &str = "http://localhost:80";

enum Command {
    HELP,
    LIST,
    CREATE,
    JOIN,
    SEND,
    LEAVE,
    EXIT,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let port: u16 = rand::thread_rng().gen();
    let address = format!("127.0.0.1:{port}")
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    // Прослушивание сокета - прием входящих сообщений
    let listener = tokio::net::TcpListener::bind(&address).await?;
    tokio::spawn(async move {
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0; 1024];
            loop {
                match stream.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(bytes) => {
                        println!(
                            "NEW MESSAGE: {}\n",
                            String::from_utf8_lossy(&buffer[..bytes])
                        )
                    }
                    _ => break,
                }
            }
        }
    });

    // Логин в систему чата
    let mut user_id = None;
    while user_id.is_none() {
        match login(&address, &client).await {
            Ok(id) => user_id = Some(id),
            Err(e) => eprintln!("{}", e.to_string()),
        };
    }
    let user = user_id.unwrap();
    let mut chat = String::new();

    // Взаимодействие с системой чата
    loop {
        // Чтение и парсинг ввода
        print!("\nCHAT: ");
        std::io::stdout().flush().unwrap();
        let mut buffer: String = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let (command, argument) = parse_command(&buffer.trim());

        // Обработка команд
        match command {
            Command::HELP => print_help(),
            Command::LIST => list(&client).await,
            Command::CREATE => create(&argument.to_string(), &client).await,
            Command::JOIN => {
                chat = argument.to_string();
                join(&user, &chat, &client).await;
            }
            Command::SEND => send(&user, &chat, &argument.to_string(), &client).await,
            Command::LEAVE => leave(&user, &chat, &client).await,
            Command::EXIT => break,
        };
    }

    Ok(())
}

fn parse_command(input: &str) -> (Command, &str) {
    // Простейшие команды
    match input {
        "list" => return (Command::LIST, ""),
        "leave" => return (Command::LEAVE, ""),
        "exit" => return (Command::EXIT, ""),
        _ => (),
    };

    // Параметризированные команды
    let (command, argument) = input.split_once(" ").unwrap_or_default();
    match command {
        "create" => return (Command::CREATE, argument),
        "join" => return (Command::JOIN, argument),
        "send" => return (Command::SEND, argument),
        _ => (),
    }

    (Command::HELP, "")
}

fn print_help() {
    let help = "Available commands:\n  \
       help        - Print this helper\n  \
       list        - List all chats\n  \
       join <name> - Join chat with [NAME]\n  \
       send <text> - Send message to current chat\n  \
       leave       - Leave current chat\n  \
       exit        - Exit the program\n";
    println!("{help}");
}

async fn login(address: &SocketAddr, client: &Client) -> Result<String, String> {
    // Ввод с клавиатуры
    let mut username = String::new();
    print!("Please enter your username: ");
    let _ = stdout().flush();
    std::io::stdin()
        .read_line(&mut username)
        .expect("Did not enter a correct string");

    // Формируем тело запроса
    let entrypoint = format!("{SERVER}/login");
    let body = json!({
        "username": username.strip_suffix("\r\n").unwrap(),
        "address": address,
    });

    // Запрос
    let response = match client.post(entrypoint).json(&body).send().await {
        Ok(r) => r,
        Err(_) => return Err("Failed to connect to server!".to_string()),
    };
    let raw = response.text().await.unwrap();

    // Валидация ответа
    let result: serde_json::Value = match serde_json::from_str(&raw) {
        Ok(r) => r,
        Err(_) => return Err(raw),
    };
    let user_id = result.as_object().unwrap().get("id").unwrap();
    Ok(user_id.as_str().unwrap().to_string())
}

async fn list(client: &Client) {
    let entrypoint = format!("{SERVER}/list");

    // Запрос
    match client.get(entrypoint).send().await {
        Ok(r) => {
            println!("{}", r.text().await.unwrap());
        }
        Err(_) => {
            eprintln!("Failed to connect to server!");
        }
    };
}

async fn create(name: &String, client: &Client) {
    // Формируем тело запроса
    let entrypoint = format!("{SERVER}/create");
    let body = json!({
        "name": name
    });
    println!("{}", body.to_string());

    // Запрос
    match client.post(entrypoint).json(&body).send().await {
        Ok(r) => {
            println!("{}", r.text().await.unwrap());
        }
        Err(_) => {
            eprintln!("Failed to connect to server!");
        }
    };
}

async fn join(user: &String, chat: &String, client: &Client) {
    // Формируем тело запроса
    let entrypoint = format!("{SERVER}/join");
    let body = json!({
        "user": user,
        "chat": chat,
    });
    println!("{}", body.to_string());

    // Присоединение к чату
    match client.post(entrypoint).json(&body).send().await {
        Ok(r) => {
            println!("{}", r.text().await.unwrap());
        }
        Err(_) => {
            eprintln!("Failed to connect to server!");
        }
    };

    // Дополнительно получаем все сообщения при входе в чат
    let entrypoint = format!("{SERVER}/messages");
    match client.get(entrypoint).json(&body).send().await {
        Ok(r) => {
            println!("{}", r.text().await.unwrap());
        }
        Err(_) => {
            eprintln!("Failed to connect to server!");
        }
    };
}

async fn send(user: &String, chat: &String, text: &String, client: &Client) {
    // Формируем тело запроса
    let entrypoint = format!("{SERVER}/send");
    let body = json!({
        "user": user,
        "chat": chat,
        "text": text,
    });
    println!("{}", body.to_string());

    // Запрос
    match client.post(entrypoint).json(&body).send().await {
        Ok(r) => {
            println!("{}", r.text().await.unwrap());
        }
        Err(_) => {
            eprintln!("Failed to connect to server!");
        }
    };
}

async fn leave(user: &String, chat: &String, client: &Client) {
    // Формируем тело запроса
    let entrypoint = format!("{SERVER}/leave");
    let body = json!({
        "user": user,
        "chat": chat,
    });
    println!("{}", body.to_string());

    // Запрос
    match client.post(entrypoint).json(&body).send().await {
        Ok(r) => {
            println!("{}", r.text().await.unwrap());
        }
        Err(_) => {
            eprintln!("Failed to connect to server!");
        }
    };
}
