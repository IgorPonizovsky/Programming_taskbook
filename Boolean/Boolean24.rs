// Пн 11 дек 2023 18:23:36 MSK
// Даны числа A, B, C(число А != 0). Рассмотрев дискриминант 'D=B^2-4*A*C', проверить истинность высказывания: 
// "Квадратное уравнение Ax^2+Bx+C=0 имеет вещественные корни". 

use std::io; 

fn main() { 
    // Получение чисел A, B и C от пользователя: 
    println!("Введите целое положительное число А: "); 
    let a = read_user_line(); 
    println!("Введите целое положительное число B: "); 
    let b = read_user_line(); 
    println!("Введите целое положительное число C: "); 
    let c = read_user_line(); 

    // Приведение полученных значений из строк в числа: 
    let a: u32 = string_to_number(a); 
    let b: u32 = string_to_number(b); 
    let c: u32 = string_to_number(c); 

    // Проверка логического условия: 
    let t_f: bool; 
    if (b*b-4*a*c) > 0 { 
        t_f = true; 
    } else { 
        t_f = false; 
    }; 

    // Вывод искомого результата: 
    if t_f == true { 
        println!("Квадратное уравнение имеет вещественные корни"); 
    } else { 
        println!("Квадратное уравнение не имеет вещественных корней"); 
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
fn string_to_number(some_string: String) -> u32 { 
    let number: u32 = some_string.trim().parse()
        .expect("Введите целые числа"); 
    number 
} 
