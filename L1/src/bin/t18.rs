// L1.18

use std::{collections::LinkedList, io::Read};

fn simple_reverse(string: &str) -> String {
    string.chars().rev().collect()
}

// Используем стек - LIFO
fn smart_reverse() {
    let mut stack: LinkedList<char> = LinkedList::new();
    std::io::stdin().lock().bytes().for_each(|char| {
        stack.push_back(char.unwrap() as char);
    });
    stack.pop_back().unwrap(); // убираем лишний 'ENTER' в конце

    while !stack.is_empty() {
        print!("{}", stack.pop_back().unwrap());
    }
    println!("");
}

fn main() {
    let content = "главрыба";
    println!("{}", simple_reverse(content));
    assert_eq!(simple_reverse(content), "абырвалг");

    let content = "$%#%ФЫВСМЙЦadsfgwaeAQW@!#";
    println!("{}", simple_reverse(content));
    assert_eq!(simple_reverse(content), "#!@WQAeawgfsdaЦЙМСВЫФ%#%$");

    smart_reverse();
}
