// Вт 12 дек 2023 11:15:06 MSK
// Даны две переменные вещественного типа: A, B. 
// Перераспределить значения переменных так, чтобы в A оказалось меньшее из значений, а в B - большее. 
// Вывести новые значения переменных A и B. 

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
        let c = a; 
        let a = b; 
        let b = c; 
        printing(a, b); 
    } else { 
        printing(a, b); 
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
fn string_to_number(some_string: String) -> f64 { 
    let some_number: f64 = some_string.trim().parse()
        .expect("Введите целые числа"); 
    some_number 
} 

// Процедура вывода значений 
fn printing(a: f64, b: f64) { 
    println!("Наименьшее(A): {}", a); 
    println!("Наибольшее(B): {}", b); 
} 
