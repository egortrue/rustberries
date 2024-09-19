// L1.1

/*
Сделать трейт Action с методом say,
который должен выводить сообщение в консоль.
*/
pub trait Action {
    fn say(&self);
}

/*
Сделать структуру Person, которая содержит строковое имя.
*/
pub struct Person {
    pub name: String,
}

/*
Сделать реализацию трейта Action для структуры Person,
в котором метод say будет выводить в консоль
текст “Hello, NAME” (где NAME - имя, хранимое в структуре).
*/
impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    let person = Person {
        name: String::from("Egor"),
    };
    person.say();
    person.say();
}
