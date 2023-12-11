// Пн 11 дек 2023 22:36:05 MSK
// Даны координаты двух различных полей шахматной доски x1, y1, x2, y2(целые числа, лежащие в диапазоне 1-8). 
// Проверить истинность высказывания: "Конь за один ход может перейти с одного поля на другое". 

use std::io; 

fn main() { 
    // Получение координат x1, y1, x2, y2 от пользователя: 
    println!("Введите координату x1(1..8): "); 
    let x1 = read_user_line(); 
    println!("Введите координату y1(1..8): "); 
    let y1 = read_user_line(); 
    println!("Введите координату x2(1..8): "); 
    let x2 = read_user_line(); 
    println!("Введите координату y2(1..8): "); 
    let y2 = read_user_line(); 

    // Приведение полученных данных из строки в число: 
    let x1: i32 = string_to_number(x1); 
    let y1: i32 = string_to_number(y1); 
    let x2: i32 = string_to_number(x2); 
    let y2: i32 = string_to_number(y2); 

    // Вывод искомого значения: 
    if ((x1 - 2) == x2 && (y1 - 1) == y2) | 
       ((x1 + 2) == x2 && (y1 - 1) == y2) | 
       ((x1 - 2) == x2 && (y1 + 1) == y2) | 
       ((x1 + 2) == x2 && (y1 + 1) == y2) | 
       ((y1 - 2) == y2 && (x1 - 1) == x2) | 
       ((y1 + 2) == y2 && (x1 - 1) == x2) | 
       ((y1 - 2) == y2 && (x1 + 1) == x2) | 
       ((y1 + 2) == y2 && (x1 + 1) == x2) { 
        println!("Конь за один ход может перейти с одного поля на другое"); 
    } else { 
        println!("Конь за один ход не может перейти с одного поля на другое"); 
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
