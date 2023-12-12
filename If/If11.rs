// Вт 12 дек 2023 11:51:59 MSK
// Даны две переменные целого типа: A, B. 
// Если их значения не равны, то присвоить каждой переменной большее из этих значений, 
// а если равны, то присвоить переменным нулевые значения. 
// Вывести новые значения переменных A и B. 

use std::io; 

fn main() { 
    // Получение двух целых чисел от пользователя: 
    println!("Введите первое целое число: "); 
    let a = read_user_line(); 
    println!("Введите второе целое число: "); 
    let b = read_user_line(); 

    // Приведение полученных строк в числа: 
    let mut a = string_to_number(a); 
    let mut b = string_to_number(b); 

    // Вывод искомых значений:  
    if a != b { 
        if a < b { 
            a = b; 
        } else { 
            b = a; 
        }; 
        printing(a, b); 
    } else { 
        a = 0; 
        b = 0; 
        printing(a, b); 
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

// Процедура вывода значений 
fn printing(a: i32, b: i32) { 
    println!("A: {}", a); 
    println!("B: {}", b); 
} 
