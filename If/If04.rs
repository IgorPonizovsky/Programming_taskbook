// Вт 12 дек 2023 09:39:05 MSK
// Даны три целых числа. Найти количество положительных чисел в исходном наборе. 

use std::io; 

fn main() { 
    // Получение трёх целых чисел от пользователя: 
    println!("Введите первое целое число: "); 
    let a = read_user_line(); 
    println!("Введите второе целое число: "); 
    let b = read_user_line(); 
    println!("Введите третье целое число: "); 
    let c = read_user_line(); 

    // Приведение полученных строк в числа: 
    let a = string_to_number(a); 
    let b = string_to_number(b); 
    let c = string_to_number(c); 

    // Подсчёт положительных чисел: 
    let mut counter: i32 = 0; 
    counter += even_number(a); 
    counter += even_number(b); 
    counter += even_number(c); 

    // Вывод искомого значения: 
    println!("{} положительных чисел.", counter); 
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

// Функция проверки положительности  
fn even_number(some_number: i32) -> i32 { 
    if some_number > 0 { 
        return 1; 
    } else { 
        return 0; 
    }; 
} 
