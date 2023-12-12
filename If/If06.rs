// Вт 12 дек 2023 10:28:50 MSK
// Даны два числа. Вывести большее из них. 

use std::io; 

fn main() { 
    // Получение двух целых чисел от пользователя: 
    println!("Введите первое целое число: "); 
    let a = read_user_line(); 
    println!("Введите второе целое число: "); 
    let b = read_user_line(); 

    // Приведение полученных строк в числа: 
    let a = string_to_number(a); 
    let b = string_to_number(b); 

    // Вывод искомого значения: 
    if a > b { 
        println!("Наибольшее {}", a); 
    } else { 
        println!("Наибольшее {}", b); 
    }; 
} 

// Функция получени строки от пользователя 
fn read_user_line() -> String { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    some_string 
} 

// Функция приведения из строки в число 
fn string_to_number(some_string: String) -> i32 { 
    let some_number: i32 = some_string.trim().parse()
        .expect("Введите целые числа"); 
    some_number 
} 
