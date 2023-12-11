// Пн 11 дек 2023 14:54:13 MSK
// Даны три целых числа: A, B, C. Проверить истинность высказывания: 
// "Ровно одно из чисел A, B, C положительное". 

use std::io; 

fn main() { 
    // Получение чисел A, B и C от пользователя: 
    println!("Введите целое число А: "); 
    let a = read_user_line(); 
    println!("Введите целое число B: "); 
    let b = read_user_line(); 
    println!("Введите целое число C: "); 
    let c = read_user_line(); 

    // Приведение полученных значений из строк в числа: 
    let a: i32 = string_to_number(a); 
    let b: i32 = string_to_number(b); 
    let c: i32 = string_to_number(c); 

    // Выведение логического значения: 
    let t_f: bool; 
    if ((a > 0) && (b < 0) && (c < 0)) | 
        ((a < 0) && (b > 0) && (c < 0)) | 
        ((a < 0) && (b < 0) && (c > 0)) { 
        t_f = true; 
    } else { 
        t_f = false; 
    }; 

    // Выведение искомого результата: 
    if t_f == true { 
        println!("Ровно одно из чисел A, B, C положительное"); 
    } else { 
        println!("Все числа A, B, C положительные или отрицательные"); 
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
