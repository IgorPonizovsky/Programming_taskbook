// Вт 12 дек 2023 14:38:58 MSK
// Даны три переменные вещественного типа: A, B, C. Если их значения упорядочены по возрастанию, то удвоить их; 
// в противном случае заменить значение каждой переменной на противоположное. Вывести новые значения переменных A, B, C. 

use std::io; 

fn main() { 
    // Получение трёх вещественных переменных от пользователя: 
    println!("Введите переменную A: "); 
    let a = read_user_line(); 
    println!("Введите переменную B: "); 
    let b = read_user_line(); 
    println!("Введите переменную C: "); 
    let c = read_user_line(); 

    // Приведение полученных строк в числа: 
    let mut a = string_to_number(a); 
    let mut b = string_to_number(b); 
    let mut c = string_to_number(c); 

    // Проверка логического условия: 
    if a < b && b < c { 
        a += a; 
        b += b; 
        c += c;
    } else { 
        a *= -1.0; 
        b *= -1.0; 
        c *= -1.0; 
    }; 

    // Вывод искомого значения: 
    println!("Новое значение переменной A: {}", a); 
    println!("Новое значение переменной B: {}", b); 
    println!("Новое значение переменной C: {}", c); 
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
