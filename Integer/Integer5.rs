// Вс 10 дек 2023 09:24:30 MSK
// Даны целые положительные числа А и В(А>В). На отрезке длины А размещено максимальное возможное количество отрезков длины В(без наложений). 
// Используя операцию взятия остатка от деления нацело, найти длину незанятой части отрезка А. 

use std::io; 

fn main() { 
    // Получение длин отрезков А и В от пользователя: 
    println!("Введите длину отрезка А(целое число): "); 
    let a = read_user_line(); 
    println!("Введите длину отрезка В(целое число < A): "); 
    let b = read_user_line(); 

    // Приведение полученных значений из строк в числа: 
    let a: u32 = string_to_number(a);
    let b: u32 = string_to_number(b);

    // Расчёт и вывод искомого значения: 
    println!("Длина незанятой части отрезка А: {}", a % b); 
} 

// Функция получения строки от пользователя 
fn read_user_line() -> String { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку");
    some_string
} 

// Функция перевода из строки в число 
fn string_to_number(some_string: String) -> u32 { 
    let number: u32 = some_string.trim().parse()
        .expect("Введите ЦЕЛОЕ положительное число"); 
    number
} 
