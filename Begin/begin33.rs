// Ср 06 дек 2023 17:00:22 MSK
// Известно, что Х кг конфет стоит А рублей. 
// Определить, сколько стоит 1кг и Yкг этих же конфет. 

use std::io; 

fn main() { 
    // Получение данных цены и веса от пользователя 
    println!("Вес упаковки(кг) конфет: "); 
    let x: String = read_user_line(); 
    println!("Цена этой упаковки конфет: "); 
    let a: String = read_user_line(); 
    println!("Сколько хочешь таких конфет(кг): "); 
    let y: String = read_user_line(); 

    // Приведение полученных данных из строк в числа 
    let x: f32 = str_to_num(x); 
    let a: f32 = str_to_num(a); 
    let y: f32 = str_to_num(y); 

    // Вывод искомых значений 
    println!("1кг конфет стоит {}", a/x); 
    println!("{}кг конфет будут стоить {}", y, (a/x)*y); 
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
    let x: f32 = x.trim().parse()
        .expect("Введите числовое значение"); 
    return x; 
} 
