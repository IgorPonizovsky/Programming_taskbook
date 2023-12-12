// Вт 12 дек 2023 12:09:45 MSK
// Даны три числа. Найти наименьшее из них. 

use std::io; 

fn main() { 
    // Получение трёх чисел от пользователя: 
    println!("Введите первое число: "); 
    let a = read_user_line(); 
    println!("Введите второе число: "); 
    let b = read_user_line(); 
    println!("Введите третье число: "); 
    let c = read_user_line(); 

    // Приведение полученных строк в числа: 
    let mut a = string_to_number(a); 
    let b = string_to_number(b); 
    let c = string_to_number(c); 

    // Проверка логического условия: 
    if a > b { 
        a = b; 
    }; 
    if a > c { 
        a = c; 
    }; 
    // Вывод искомых значений: 
    println!("Наименьшее: {}", a); 
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
