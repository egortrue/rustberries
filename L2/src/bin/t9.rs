// L2.9
// https://linux.die.net/man/1/wget
// reqwest + scraper + tokio

// cargo run --bin t9 -- -l 4 https://rust-lang.org

/*

Usage: t9.exe [OPTIONS] <URL>

Arguments:
  <URL>  Страница для загрузки

Options:
  -l, --level <LEVEL>
          Определяет максимальную глубину вложенности страниц [default: 1]
  -o, --output-directory <OUTPUT_DIRECTORY>
          Директория для сохранения файлов [default: out]
  -h, --help
          Print help

*/

/* Пример вывода

https://rust-lang.org
https://rust-lang.org/what/embedded
https://rust-lang.org/learn
https://rust-lang.org/what/wasm
https://rust-lang.org/what/cli
https://rust-lang.org/policies/security
https://rust-lang.org/tools/install
https://rust-lang.org/production
https://rust-lang.org/governance
https://rust-lang.org/policies/code-of-conduct
https://rust-lang.org/policies
https://rust-lang.org/learn/get-started
https://rust-lang.org/policies/licenses
https://rust-lang.org/tools
https://rust-lang.org/what/networking
https://rust-lang.org/community
https://rust-lang.org/static/keys/rust-security-team-key.gpg.ascii
https://rust-lang.org/what/wasm#production
https://rust-lang.org/what/cli#production
https://rust-lang.org/what/embedded#production
https://rust-lang.org/static/pdfs/Rust-Tilde-Whitepaper.pdf
https://rust-lang.org/static/pdfs/Rust-npm-Whitepaper.pdf
https://rust-lang.org/what/networking#production
https://rust-lang.org/governance/wgs/wg-security-response
https://rust-lang.org/governance/wgs/gamedev
https://rust-lang.org/governance/wgs/wg-rust-by-example
https://rust-lang.org/governance/teams/leadership-council
https://rust-lang.org/governance/teams/lang
...

*/

use clap::Parser;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::{
    collections::{HashSet, VecDeque},
    env,
};

#[derive(Parser)]
struct Args {
    /// Страница для загрузки
    url: String,

    /// Определяет максимальную глубину вложенности страниц
    #[clap(short, long, default_value_t = 1)]
    level: usize,

    /// Директория для сохранения файлов
    #[clap(short, long, default_value = "out")]
    output_directory: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let workdir = if args.output_directory.starts_with("/") {
        args.output_directory
    } else {
        env::current_dir().unwrap().display().to_string() + "/" + &args.output_directory
    };

    let mut level = 0;
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();

    queue.push_back(args.url.clone());
    while !queue.is_empty() {
        let urls: Vec<String> = queue.drain(..).collect();

        for url in urls {
            if visited.contains(&url) {
                continue;
            }
            println!("{}", url);
            if let Some(mut content) = get_page(&url).await {
                let dir = workdir.clone() + url.strip_prefix(&args.url).unwrap_or_default();

                // Парсинг страницы
                let (links, resources) = parse_page(&content);

                // Обработка ссылок
                for original_link in links {
                    // Фильтрация пустых ссылок и внешних ссылок
                    if original_link.is_empty() || original_link == "/" {
                        continue;
                    }
                    if !original_link.starts_with("/") && !original_link.starts_with(&args.url) {
                        continue;
                    }

                    // Формирование очереди скачивания
                    let external_link = if original_link.starts_with("/") {
                        args.url.clone() + original_link.as_str()
                    } else {
                        original_link.clone()
                    };
                    queue.push_back(external_link);

                    // Изменение контента на локальную ссылку
                    let local_link = workdir.clone() + &original_link + "/index.html";
                    content = content.replace(&original_link, &local_link);
                }

                // Обработка ассетов
                for original_resource in resources {
                    if !original_resource.starts_with("/")
                        && !original_resource.starts_with(&args.url)
                    {
                        continue;
                    }

                    let external_resource = if original_resource.starts_with("/") {
                        args.url.clone() + original_resource.as_str()
                    } else {
                        original_resource.clone()
                    };

                    let resource_name = original_resource.split("/").last().unwrap();
                    let resource_content = get_page(&external_resource).await.unwrap();
                    save_data(&dir, &resource_name, &resource_content).await;

                    // Изменение контента на локальную ссылку
                    let local_resource = "./".to_string() + &resource_name;
                    content = content.replace(&original_resource, &local_resource);
                }

                save_data(&dir, "index.html", &content).await;
            }
            visited.insert(url);
        }

        level += 1;
        if level == args.level {
            break;
        }
    }
}

/// Получение страницы
async fn get_page(url: &String) -> Option<String> {
    if let Ok(response) = reqwest::get(url).await {
        if response.status() == StatusCode::OK {
            if let Ok(content) = response.text().await {
                return Some(content);
            }
        }
    }

    None
}

/// Парсинг ссылок и ассетов
fn parse_page(page: &String) -> (HashSet<String>, HashSet<String>) {
    let html = Html::parse_document(page);

    // Парсинг ссылок
    let mut links = HashSet::new();
    let links_selector = Selector::parse("a[href]").unwrap();
    for element in html.select(&links_selector) {
        if let Some(link) = element.value().attr("href") {
            links.insert(link.to_string());
        }
    }

    // Парсинг ресурсов
    let mut resources = HashSet::new();
    let resources_selector = Selector::parse("[src]").unwrap();
    for element in html.select(&resources_selector) {
        if let Some(resource) = element.value().attr("src") {
            resources.insert(resource.to_string());
        }
    }

    (links, resources)
}

async fn save_data(directory: &str, name: &str, content: &str) {
    tokio::fs::create_dir_all(&directory)
        .await
        .expect("Couldn't create output directory");

    tokio::fs::write(format!("{directory}/{name}"), content)
        .await
        .expect("Couldn't write to the file")
}
