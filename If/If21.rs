// Вт 12 дек 2023 16:26:48 MSK
// Даны целочисленные координаты точки на плоскости. Если точка совпадает с началом координат, то вывести '0'. 
// Если точка не совпадает с началом координат, но лежит на оси OX или OY, то вывести соответственно '1' или '2'. 
// Если точка не лежит на координатных осях, то вывести '3'. 

use std::io; 

fn main() { 
    // Получение координат точки от пользователя: 
    println!("Введите координату x: "); 
    let x = read_user_line(); 
    println!("Введите координату y: "); 
    let y = read_user_line(); 

    // Приведение полученных строк в числа: 
    let x = string_to_number(x); 
    let y = string_to_number(y); 

    // Вывод искомого значения: 
    if x == 0 && y == 0 { 
        println!("0"); 
    } else 
    if x == 0 && y != 0 { 
        println!("2"); 
    } else 
    if x != 0 && y == 0 { 
        println!("1"); 
    } else { 
        println!("3"); 
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
