// Пн 11 дек 2023 20:12:58 MSK
// Даны целые числа a, b, c, являющиеся сторонами некоторого треугольника. Проверить истинность высказывания: 
// "Треугольник со сторонами a, b, c является равносторонним". 

use std::io; 

fn main() { 
    // Получение длин сторон треугольника от пользователя: 
    println!("Введите сторону a: "); 
    let a = read_user_line(); 
    println!("Введите сторону b: "); 
    let b = read_user_line(); 
    println!("Введите сторону c: "); 
    let c = read_user_line(); 

    // Приведение полученных координат из строк в числа: 
    let a = string_to_number(a); 
    let b = string_to_number(b); 
    let c = string_to_number(c); 

    // Вывод искомого значения: 
    if (a == b) && (a == c) { 
        println!("Треугольник равносторонний"); 
    } else { 
        println!("Треугольник не равносторонний"); 
    }; 
} 

// Функция получения строки из стандартного потока ввода-вывода 
fn read_user_line() -> String { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    some_string 
} 

// Функция перевода строки в число 
fn string_to_number(some_string: String) -> i32 { 
    let number: i32 = some_string.trim().parse()
        .expect("Введите числа"); 
    number 
} 
