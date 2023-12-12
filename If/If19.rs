// Вт 12 дек 2023 15:45:12 MSK
// Даны четыре целых числа, одно из которых отлично от трёх других, равных между собой. 
// Определить порядковый номер числа, отличного от остальных. 

use std::io; 

fn main() { 
    // Получение четырёх целых чисел от пользователя: 
    println!("Введите первое целое число: "); 
    let a = read_user_line(); 
    println!("Введите второе целое число: "); 
    let b = read_user_line(); 
    println!("Введите третье целое число: "); 
    let c = read_user_line(); 
    println!("Введите четвёртое целое число: "); 
    let d = read_user_line(); 

    // Приведение полученных строк в числа: 
    let a = string_to_number(a); 
    let b = string_to_number(b); 
    let c = string_to_number(c); 
    let d = string_to_number(d); 

    // Нумерование введёных чисел: 
    let number1 = build_number(1, a); 
    let number2 = build_number(2, b); 
    let number3 = build_number(3, c); 
    let number4 = build_number(4, d); 

    // Вывод искомого значения: 
    if number1.value == number2.value && number2.value == number3.value && number3.value == number4.value { 
        println!("Нет различающихся чисел"); 
    } else 
    if number1.value == number2.value && number2.value == number3.value { 
        println!("Номер отличающегося: {}", number4.index); 
    } else 
    if number2.value == number3.value && number3.value == number4.value { 
        println!("Номер отличающегося: {}", number1.index); 
    } else 
    if number3.value == number4.value && number4.value == number1.value { 
        println!("Номер отличающегося: {}", number2.index); 
    } else 
    if number4.value == number1.value && number1.value == number2.value { 
        println!("Номер отличающегося: {}", number3.index); 
    } else { 
        println!("Нет трёх повторяющихся чисел"); 
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
