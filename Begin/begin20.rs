// Ср 06 дек 2023 09:23:13 MSK
// Найти расстояние между двумя точками с заданными координатами 
// (x1, y1) и (x2, y2) на плоскости. 
// Расстояние вычисляется по формуле: ((x2-x1)^2+(y2-y1)^2)^(1/2) 

use std::io; 

fn main() { 
    // Получение координат точек на плоскости от пользователя 
    println!("Введите координату x первой точки: "); 
    let x1: String = read_user_line(); 
    println!("Введите координату y первой точки: "); 
    let y1: String = read_user_line(); 
    println!("Введите координату x второй точки: "); 
    let x2: String = read_user_line(); 
    println!("Введите координату y второй точки: "); 
    let y2: String = read_user_line(); 

    // Приведение полученных данных из строк в числа 
    let x1: f32 = str_to_num(x1); 
    let x2: f32 = str_to_num(x2); 
    let y1: f32 = str_to_num(y1); 
    let y2: f32 = str_to_num(y2); 

    // Вывод искомого значения на экран 
    println!("Расстояние между точками ={}", ((x2-x1).powf(2.0)+(y2-y1).powf(2.0)).sqrt()); 
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
