// Пн 11 дек 2023 17:56:55 MSK
// Дано четырёхзначное число. Проверить истинность высказывания: 
// "Данное число читается одинаково слева направо и справа налево". 

use std::io; 

fn main() { 
    // Получение целого положительного четырёхзначного числа от пользователя: 
    println!("Введите целое положительное трёхзначное число: "); 
    let number = read_user_line(); 

    // Приведение полученного от пользователя из строки в число: 
    let number: u32 = string_to_number(number); 

    // Разбиение числа на цифры: 
    let n1 = number / 1_000; 
    let n2 = (number % 1_000) / 100; 
    let n3 = (number % 100) / 10; 
    let n4 = number % 10; 

    // Проверка логического условия: 
    let t_f: bool; 
    if (n1 == n4) && (n2 == n3) { 
        t_f = true 
    } else { 
        t_f = false 
    }; 

    // Вывод искомого значения: 
    if t_f == true { 
        println!("Данное число читается одинаково слева направо и справа налево") 
    } else { 
        println!("Данное число не читается одинаково слева направо и справа налево") 
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
        .expect("Введите целое положительное четырёхзначное число"); 
    number 
} 