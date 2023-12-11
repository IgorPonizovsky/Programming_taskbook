// Пн 11 дек 2023 12:12:31 MSK
// Дано целое число А. Проверить истинность высказывания: 
//"Число А является чётным". 

use std::io; 

fn main() { 
    // Получение числа от пользователя: 
    println!("Введите целое число А: "); 
    let a = read_user_line(); 

    // Приведение полученного значения из строки в число: 
    let a: i32 = string_to_number(a); 

    // Выведение логического значения: 
    let t_f: bool; 
    if a%2 == 0 { 
        t_f = true; 
    } else { 
        t_f = false; 
    }; 

    // Выведение искомого результата: 
    if t_f == true { 
        println!("Число {} является чётным", a); 
    } else { 
        println!("Число {} является нечётным", a); 
    }; 
} 

// Функция получения строки от пользователя 
fn read_user_line() -> String { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    some_string 
} 

// Функция приведения строки в число 
fn string_to_number(some_string: String) -> i32 { 
    let number: i32 = some_string.trim().parse()
        .expect("Введите целое число"); 
    number 
} 
