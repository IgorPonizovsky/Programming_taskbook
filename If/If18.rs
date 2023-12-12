// Вт 12 дек 2023 14:51:47 MSK
// Даны три целых числа, одно из которых отлично от двух других, равных между собой. 
// Определить порядковый номер числа, отличного от остальных. 

use std::io; 

fn main() { 
    // Получение трёх целых чисел от пользователя: 
    println!("Введите первое целое число: "); 
    let a = read_user_line(); 
    println!("Введите второе целое число: "); 
    let b = read_user_line(); 
    println!("Введите третье целое число: "); 
    let c = read_user_line(); 

    // Приведение полученных строк в числа: 
    let a = string_to_number(a); 
    let b = string_to_number(b); 
    let c = string_to_number(c); 

    // Нумерование введёных чисел: 
    let number1 = build_number(1, a); 
    let number2 = build_number(2, b); 
    let number3 = build_number(3, c); 

    // Вывод искомого значения: 
    if number1.value == number2.value && number2.value == number3.value { 
        println!("Нет различающихся чисел"); 
    } else 
    if number1.value == number2.value { 
        println!("Порядковый номер отличающегося числа: {}", number3.index); 
    } else 
    if number2.value == number3.value { 
        println!("Порядковый номер отличающегося числа: {}", number1.index); 
    } else 
    if number3.value == number1.value { 
        println!("Порядковый номер отличающегося числа: {}", number2.index); 
    } else { 
        println!("Нет повторяющихся чисел"); 
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

// Функция нумерования чисел 
fn build_number(index: i8, value: i32) -> Number {
    Number { 
        index: index, 
        value: value, 
    } 
} 

// Структура последовательности чисел 
struct Number { 
    index: i8, 
    value: i32, 
} 
