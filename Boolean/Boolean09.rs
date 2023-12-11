// Пн 11 дек 2023 14:19:01 MSK
// Даны два целых числа: A, B. Проверить истинность высказывания: 
// "Хотя бы одно из чисел A и B нечетное". 

use std::io; 

fn main() { 
    // Получение чисел A и B от пользователя: 
    println!("Введите целое число А: "); 
    let a = read_user_line(); 
    println!("Введите целое число B: "); 
    let b = read_user_line(); 

    // Приведение полученных значений из строк в числа: 
    let a: i32 = string_to_number(a); 
    let b: i32 = string_to_number(b); 

    // Выведение логического значения: 
    let t_f: bool; 
    if (a % 2 != 0) | (b % 2 != 0) { 
        t_f = true; 
    } else { 
        t_f = false; 
    }; 

    // Выведение искомого результата: 
    if t_f == true { 
        println!("Хотя бы одно из чисел A и B нечетное"); 
    } else { 
        println!("Оба числа A и B чётные"); 
    }; 
} 

// Функция получения строки от пользователя 
fn read_user_line() -> String { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    some_string 
} 

// Функция приведения строки в число 
fn string_to_number(some_string: String) -> i32 { 
    let number: i32 = some_string.trim().parse()
        .expect("Введите целые числа"); 
    number 
} 
