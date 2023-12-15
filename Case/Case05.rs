// Пт 15 дек 2023 17:58:47 MSK
// Арифметические действия над числами пронумерованы следующим образом: 1-сложение, 2-вычитание, 3-умножение, 4-деление. 
// Дан номер действия N(целое число от 1 до 4) и вещественные числа A и B(B не равно 0). 
// Выполнить над числами указанное действие и вывести результат. 

use std::io; 

fn main() { 
    // Получение чисел от пользователя: 
    println!( "(1)сложение, (2)вычитание, (3)умножение, (4)деление" ); 
    println!( "Введите номер действия: " ); 
    let n: i8 = get_integer_number(); 
    println!( "Введите число A: " ); 
    let a: f64 = get_float_number(); 
    println!( "Введите число B: " ); 
    let b: f64 = get_float_number(); 

    // Вывод искомого значения: 
    match n { 
        1 => println!( "{}", a + b ), 
        2 => println!( "{}", a - b ), 
        3 => println!( "{}", a * b ), 
        4 => println!( "{}", a / b ), 
        _ => println!( "Выберите действие(1-4)" ), 
    } 
} 

// Функция получения целого числа 
fn get_integer_number() -> i8 { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    let some_number: i8 = some_string.trim().parse()
        .expect("Введите целое число(1-4)"); 
    some_number 
} 

// Функция получения вещественного числа 
fn get_float_number() -> f64 { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    let some_number: f64 = some_string.trim().parse()
        .expect("Введите числа"); 
    some_number 
} 
