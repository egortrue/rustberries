// L1.11

use std::collections::HashMap;

const INTERVAL: i64 = 10;

fn main() {
    let values = vec![
        -25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5, 20.0, 39.9,
    ];
    let mut groups: HashMap<String, Vec<f64>> = HashMap::new();

    for value in values {
        // Формируем ключ по интервалу
        let div: i64 = value as i64 / INTERVAL;
        let key = if div < 0 {
            format!("[{},{})", (div - 1) * INTERVAL, div * INTERVAL)
        } else {
            format!("[{},{})", div * INTERVAL, (div + 1) * INTERVAL)
        };

        // Создаем вектор если новый интервал
        if !groups.contains_key(&key) {
            groups.insert(key.clone(), Vec::new());
        }

        // Добавляем значения в нужный интервал
        groups.get_mut(&key).unwrap().push(value);
    }

    println!("{:#?}", groups);

    /*

    {
        "[-30,-20)": [
            -25.4,
            -27.0,
            -21.0,
        ],
        "[10,20)": [
            13.0,
            19.0,
            15.5,
        ],
        "[30,40)": [
            32.5,
            39.9,
        ],
        "[20,30)": [
            24.5,
            20.0,
        ],
    }

    */
}
