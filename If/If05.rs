// Вт 12 дек 2023 09:58:26 MSK
// Даны три целых числа. Найти количество положительных и количество отрицательных чисел в исходном наборе. 

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

    // Подсчёт положительных и отрицательных чисел: 
    let mut positive: i8 = 0; 
    let mut negative: i8 = 0; 
    even_number(a, &mut positive, &mut negative); 
    even_number(b, &mut positive, &mut negative); 
    even_number(c, &mut positive, &mut negative); 

    // Вывод искомых значений: 
    println!("{} положительных чисел.", positive); 
    println!("{} отрицательных чисел.", negative); 
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

// Функция проверки на положительность/отрицательность 
fn even_number(some_number: i32, positive: &mut i8, negative: &mut i8) { 
    if some_number > 0 { 
        *positive = *positive + 1; 
    } else { 
        *negative = *negative + 1; 
    }; 
} 
