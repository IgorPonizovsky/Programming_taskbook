// Вт 05 дек 2023 10:51:58 MSK
// Дана сторона квадрата 'a'. Найти его периметр 'P=4*a' 

use std::io; 

fn main() { 
    //Получение длины от пользователя 
    println!("Введите длину стороны квадрата: "); 
    let mut a = String::new(); 
    io::stdin().read_line(&mut a)
        .expect("Не удалось прочитать строку");

    //Привидение полученного из строки в число 
    let a: f32 = match a.trim().parse() { 
        Ok(num) => num, 
        Err(_) => 0.0, 
    }; 

    //Вывод искомой величины 
    println!("Периметр квадрата: P={}", 4.0*a);
} 
// Вт 05 дек 2023 11:51:21 MSK
// Добавил возможность подсчёта периметра для квадрата с нецелочисленной длиной сторон 
