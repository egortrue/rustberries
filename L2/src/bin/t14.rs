fn main() {
    // Создание канала: Мы создаем канал связи с помощью std::sync::mpsc::channel, который возвращает пару (tx, rx)
    // Здесь tx — это передающая сторона, а rx — принимающая
    // Шаблонный параметр i32 - указывает на данные который будут транислироваться через поток
    let (tx, rv) = std::sync::mpsc::channel::<i32>();

    // Запуск нового потока: новый поток запускается с помощью std::thread::spawn
    // Функция принимает замвыкание, которое перемещается (borrow) в новый поток со всеми переменными
    // В данном случае замыкание отправляет числа от 0 до 9 через канал tx
    let handle = std::thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });

    // Ждем завершения потока
    // После того как поток запустился, мы ждем его завершения с помощью handle.join()
    // чтобы гарантировать, что все сообщения были отправлены
    handle.join().unwrap();

    // Читаем данные из канала
    // Используя rx.iter(), мы получаем последовательность значений, которую можем прочитать и вывести на экран
    for i in rv.iter() {
        println!("{i:?}");
    }
}
