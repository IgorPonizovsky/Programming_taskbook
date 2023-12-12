// Вт 12 дек 2023 12:19:55 MSK
// Даны три числа. Найти среднее из них(т.е. число, расположенное между наименьшим и наибольшим). 

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
    let a = string_to_number(a); 
    let b = string_to_number(b); 
    let c = string_to_number(c); 

    // Вывод искомых значений: 
    if ( a < b && b < c ) | ( a > b && b > c ) { 
        println!("Среднее: {}", b); 
    } else if ( a < c && c < b ) | ( a > c && c > b ) { 
        println!("Среднее: {}", c); 
    } else { 
        println!("Среднее: {}", a); 
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
