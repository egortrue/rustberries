// L1.14

use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

fn get_type<T: Any>(_: T) -> String {
    format!("Тип переменной: {}", std::any::type_name::<T>())
}

fn on_type<T: Any + Debug>(value: T) {
    let type_id = TypeId::of::<T>();

    if type_id == TypeId::of::<i32>() {
        println!("Я целый: {value:?} <<< i32");
    } else if type_id == TypeId::of::<f64>() {
        println!("Я плаваю: {value:?} <<< f64");
    } else if type_id == TypeId::of::<String>() {
        println!("Я говорю: {value:?} <<< String");
    } else if type_id == TypeId::of::<Vec<&str>>() {
        println!("Я содержу: {value:?} <<< vec");
    } else {
        println!("Кто я?");
    }
}

fn main() {
    println!("{}", get_type("aaaa"));
    println!("{}", get_type(10));
    println!("{}", get_type(1123.0));
    println!("{}", get_type(String::from("fasdfasdf")));
    println!("{}", get_type(vec!["asdf", "asdfasdf"]));

    on_type("asdfasdf");
    on_type(10);
    on_type(123123.0);
    on_type(vec!["asdfasdf", "dasfasdf"]);
}
