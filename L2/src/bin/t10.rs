// L2.10
// https://linux.die.net/man/1/telnet
// Протестировано с помощью https://hub.docker.com/r/istio/tcp-echo-server
// Завершение работы: Ctrl-D (unix) / Ctrl-Z (windows)

// Сервер: docker run -d -p 23:9000 istio/tcp-echo-server
// Клиент: cargo run --bin t10 -- --timeout 30 localhost 23

/*

Usage: t10.exe [OPTIONS] <HOST> [PORT]

Arguments:
  <HOST>  IP или доменное имя
  [PORT]  Порт подключения [default: 23]

Options:
      --timeout <TIMEOUT>  Время ожидания подключения (в секундах) [default: 10]
  -h, --help               Print help

*/

use clap::Parser;
use core::str;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::process::exit;
use std::thread;
use std::time;

const BUFFER_SIZE: usize = 1024;

#[derive(Parser)]
struct Args {
    /// IP или доменное имя
    host: String,

    /// Порт подключения
    #[clap(default_value_t = 23)]
    port: u16,

    /// Время ожидания подключения (в секундах)
    #[clap(long, default_value_t = 10)]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Обработка аргументов
    let args = Args::parse();
    let socket = format!("{}:{}", &args.host, &args.port)
        .to_socket_addrs()? // Умеет обрабатывать доменные имена
        .next()
        .expect("No valid IP addresses were found");
    let timeout = time::Duration::from_secs(args.timeout);

    // Настройка подключения к хосту
    let stream = TcpStream::connect_timeout(&socket, timeout)?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;

    // Поток вывода с сервера
    let mut stream_output = stream.try_clone()?;
    let thread_output = thread::spawn(move || {
        let mut buffer = [0; BUFFER_SIZE];
        loop {
            match stream_output.read(&mut buffer) {
                Ok(0) => {
                    eprintln!("Connection closed by the server!");
                    break;
                }
                Ok(bytes) => {
                    println!("{}", String::from_utf8_lossy(&buffer[..bytes]));
                    continue;
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                    eprintln!("Connection closed because of timeout!");
                    break;
                }
                Err(e) => {
                    eprintln!("Undefined error: {}", e);
                    break;
                }
            }
        }
    });

    // Поток ввода на сервер
    let mut stream_input = stream.try_clone()?;
    let thread_input = thread::spawn(move || {
        let mut stdin = io::stdin().lock();
        let mut buffer = [0; BUFFER_SIZE];
        loop {
            // Чтение данных с клавиатуры
            match stdin.read(&mut buffer) {
                Ok(0) => {
                    eprintln!("Connection closed by the client!");
                    break;
                }
                Ok(bytes) => {
                    // Передача данных
                    match stream_input.write(&buffer[..bytes]) {
                        Ok(_) => continue,
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                            eprintln!("Connection closed because of timeout!");
                            break;
                        }
                        Err(e) => {
                            eprintln!("Undefined error: {}", e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Undefined error: {}", e);
                    break;
                }
            }
        }
    });

    drop(stream);
    loop {
        if thread_input.is_finished() || thread_output.is_finished() {
            exit(0);
        }
    }
}
