// Пн 11 дек 2023 19:26:07 MSK
// Даны числа x, y. Проверить истинность высказывания: 
// "Точка с координатами (x,y) лежит во второй или четвёртой координатной четверти". 

use std::io; 

fn main() { 
    // Получение координат x, y от пользователя: 
    println!("Введите координату x: "); 
    let x = read_user_line(); 
    println!("Введите координату y: "); 
    let y = read_user_line(); 

    // Приведение полученных данных из строки в число: 
    let x: i32 = string_to_number(x); 
    let y: i32 = string_to_number(y); 

    // Вывод искомого значения: 
    if ((x < 0) && (y < 0)) | ((x > 0) && (y < 0)) { 
        println!("Точка лежит во второй или четвёртой координатной четверти"); 
    } else { 
        println!("Точка не лежит во второй или четвёртой координатной четверти"); 
    }; 
} 

// Функция получения строки из стандартного ввода-вывода 
fn read_user_line() -> String { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалость прочитать строку"); 
    some_string 
} 

// Функция приведения строки в число 
fn string_to_number(some_string: String) -> i32 { 
    let number: i32 = some_string.trim().parse()
        .expect("Введите числовые координаты"); 
    number 
} 
