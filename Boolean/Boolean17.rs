// Пн 11 дек 2023 16:27:33 MSK
// Дано целое положительное число. Проверить истинность высказывания: 
// "Данное число является нечётным трёхзначным". 

use std::io; 

fn main() { 
    // Получение числа от пользователя: 
    println!("Введите целое положительное число: "); 
    let number  = read_user_line(); 

    // Приведение полученного значения из строки в число: 
    let number = string_to_number(number); 

    // Проверка логического значения: 
    let t_f: bool; 
    if (number % 2 != 0) && (number / 1_000 == 0) && (number / 100 >= 1) { 
        t_f = true; 
    } else { 
        t_f = false; 
    }; 

    // Выведение искомого значения: 
    if t_f == true { 
        println!("Данное число является нечётным трёхзначным"); 
    } else { 
        println!("Данное число не является нечётным трёхзначным"); 
    }; 
} 

// Функция получения строки из стандартного потока ввода 
fn read_user_line() -> String { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалость прочитать строку"); 
    some_string 
} 

// Функция приведения строки в число 
fn string_to_number(some_string: String) -> i32 {
    let number: i32 = some_string.trim().parse()
        .expect("Введите число"); 
    number 
} 
