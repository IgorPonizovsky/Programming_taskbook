// Ср 06 дек 2023 06:12:46 MSK
// Найти расстояние между двумя точками с заданными координатами 
// 'x1' и 'x2' на числовой оси: |x2-x1| 

use std::io; 

fn main() { 
    // Получение координат точек от пользователя 
    println!("Введите координату х1: "); 
    let x1: String = read_user_line(); 
    println!("Введите координату х2: "); 
    let x2: String = read_user_line(); 

    // Приведение полученных данных из строк в числа 
    let x1: f32 = str_to_num(x1); 
    let x2: f32 = str_to_num(x2); 

    // Вывод искомой величины на экран 
    println!("Расстояние между точками равно {}", (x2-x1).abs()); 
} 

// Функция получения строки от пользователя 
fn read_user_line() -> String { 
    let mut x = String::new(); 
    io::stdin().read_line(&mut x)
        .expect("Не удалось прочитать строку"); 
    return x; 
} 

// Функция перевода из строки в число 
fn str_to_num(x: String) -> f32 { 
    let x: f32 = match x.trim().parse() { 
        Ok(num) => num, 
        Err(_) => 0.0, 
    }; 
    return x; 
} 
