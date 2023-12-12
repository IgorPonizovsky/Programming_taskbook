// Вт 12 дек 2023 13:13:01 MSK
// Даны три числа. Вывести вначале наименьшее, а затем наибольшее из чисел. 

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

    // Создание массива из чисел: 
    let array: [f64; 3] = [a, b, c]; 

    // Ищем максимальное и минимальное в цикле: 
    let mut min: f64 = a; 
    let mut max: f64 = a; 
    for i in 0..3 { 
        if min > array[i] { 
            min = array[i]; 
        }; 
        if max < array[i] { 
            max = array[i]; 
        }; 
    }; 
    
    // Вывод искомых значений: 
    printing(min, max); 
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

// Процедура вывода искомых значений 
fn printing(min: f64, max: f64) { 
    println!("Наименьшее: {}", min); 
    println!("Наибольшее: {}", max); 
} 
