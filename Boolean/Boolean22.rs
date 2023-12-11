// Пн 11 дек 2023 17:48:11 MSK
// Дано трёхзначное число. Проверить истинность высказывания: 
// "Цифры данного числа образуют возрастающую или убывающую последовательность". 

use std::io; 

fn main() { 
    // Получение целого положительного трёхзначного числа от пользователя: 
    println!("Введите целое положительное трёхзначное число: "); 
    let number = read_user_line(); 

    // Приведение полученного от пользователя из строки в число: 
    let number: u32 = string_to_number(number); 

    // Проверка логического условия: 
    let t_f: bool; 
    let number1 = number / 100; 
    let number2 = (number % 100) / 10; 
    let number3 = number % 10; 
    if ((number1 < number2) && (number2 < number3)) | 
       ((number1 > number2) && (number2 > number3)) { 
        t_f = true 
    } else { 
        t_f = false 
    }; 

    // Вывод искомого значения: 
    if t_f == true { 
        println!("Цифры данного числа образуют возрастающую или убывающую последовательность") 
    } else { 
        println!("Цифры данного числа не образуют возрастающую или убывающую последовательность") 
    }; 
} 

// Функция получения строки из стандартного потока ввода-вывода 
fn read_user_line() -> String { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    some_string 
} 

// Функция приведения из строки в число 
fn string_to_number(some_string: String) -> u32 { 
    let number: u32 = some_string.trim().parse()
        .expect("Введите целое положительное трёхзначное число"); 
    number 
} 
