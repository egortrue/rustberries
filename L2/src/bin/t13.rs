/* Вывод

1
4
5
6
wrap 8
8
0
3
2


*/

// структура Example, которая содержит целое число i32
struct Example(i32);

// У структуры Example есть реализация trait Drop
// который автоматически вызывается при уничтожении экземпляров этой структуры
impl Drop for Example {
    // Функция drop() печатает значение, хранящееся в структуре
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

// Обертка-структура
struct ExampleWrap(Example);

// Реализация для trait Drop для структуры ExampleWrap
// который автоматически вызывается при уничтожении экземпляров этой структуры
impl Drop for ExampleWrap {
    fn drop(&mut self) {
        // сохранить текущий экземпляр структуры Example перед заменой его на новый
        let e = std::mem::replace(&mut self.0, Example(0));

        // печатает старое значение, удаленное из структуры
        println!("wrap {}", e.0);
    }
}

// Функция main: В основной функции происходит создание
// и использование различных экземпляров структур Example и ExampleWrap
fn main() {
    // Создается временный объект Example(1), который сразу покидает область видимости
    // поскольку объект временный, он не получает имени, но метод `drop` все равно вызывается
    Example(1);

    // Создается объект Example(2) и получает имя "_e2"
    // когда объект покинет область видимости, метод `drop` будет вызван.
    let _e2 = Example(2);

    // Аналогично создается объект Example(3) и получает имя "_e3"
    let _e3 = Example(3);

    // Создается временный объект Example(4), который сразу покидает область видимости.
    let _ = Example(4);

    // Создается неинициализированная переменная
    let mut _e5;

    // Переменной _e5 присваивается значение Some(Example(5))
    _e5 = Some(Example(5));

    // Значение переменной _e5 изменяется на None.
    // Структура Example внутри Option уничтожается, вызывая метод `drop`
    _e5 = None;

    // Создается объект Example(6) и получает имя "e6"
    let e6 = Example(6);
    // Явно вызывается метод `drop` для объекта e6
    drop(e6);

    // Создается объект Example(7) и получает имя "e7"
    let e7 = Example(7);
    // Объект e7 удаляется из памяти без вызова метода `drop`
    std::mem::forget(e7);

    // Создается объект ExampleWrap(Example(8)), который имеет вложенную структуру Example
    ExampleWrap(Example(8));

    // Все созданные объекты выходят из области видимости и вызывают свои методы `drop`.
    // из стека обратный порядок
    // ExampleWrap(Example(8)) -> wrap 8 -> 8 -> 0
    // Example(3) -> 3
    // Example(2) -> 2
}
