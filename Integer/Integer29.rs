// Вс 10 дек 2023 19:22:08 MSK
// Даны целые положительные числа A, B, C. На примоугольнике размера AxB размещено максимально возможное количество квадаратов 
// со стророной C (без наложений). Найти количество квадратов, размещённых на прямоугольнике. а так же площадь незанятой части прямоугольника. 

use std::io; 

fn main() { 
    // Получение сторон фигур от пользователя: 
    println!("Введите сторону прямоугольника A: "); 
    let a = read_user_line();
    println!("Введите сторону прямоугольника B: "); 
    let b = read_user_line();
    println!("Введите сторону квадрата С: "); 
    let c = read_user_line();

    // Преобразование полученных значений из строки в число: 
    let a: u32 = string_to_number(a); 
    let b: u32 = string_to_number(b); 
    let c: u32 = string_to_number(c); 

    // Расчёт площадей фигур: 
    let s_ab: u32 = a*b; 
    let s_c: u32 = c*c; 

    // Вывод искомых значений: 
    println!("В прямоугольнике размещено {} квадратов", (s_ab/s_c)); 
    println!("Свободная площадь: {}", s_ab-(s_ab/s_c)*s_c); 
} 

// Функция получения строки от пользователя 
fn read_user_line() -> String { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    some_string 
} 

// Функция перевода строки в число 
fn string_to_number(some_string: String) -> u32 { 
    let number: u32 = some_string.trim().parse()
        .expect("Введите числовое значение"); 
    number 
} 
