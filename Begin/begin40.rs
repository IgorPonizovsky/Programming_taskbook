// Ср 06 дек 2023 19:36:51 MSK
// Найти решение системы линейных уравнений вида 
//      a1*x+b1*y=c1, 
//      a2*x+b2*y=c2, 
// заданной своими коэффициентами a1, b1, c1, a2, b2, c2, 
// если известно, что данная система имеет единственное решение. 
// Воспользоваться формулами 
// x=(c1*b2-c2*b1)/d, y=(a1*c2-a2*c1)/d, d=a1*b2-a2*b1 

use std::io; 

fn main() { 
    // Получение коэффициентов системы линейных уравнений от пользователя 
    println!("a1:"); 
    let a1: String = read_user_line(); 
    println!("b1:"); 
    let b1: String = read_user_line(); 
    println!("c1:"); 
    let c1: String = read_user_line(); 
    println!("a2:"); 
    let a2: String = read_user_line(); 
    println!("b2:"); 
    let b2: String = read_user_line(); 
    println!("c2:"); 
    let c2: String = read_user_line(); 

    // Приведение полученных данных из строк в числа 
    let a1: f32 = str_to_num(a1); 
    let a2: f32 = str_to_num(a2); 
    let b1: f32 = str_to_num(b1); 
    let b2: f32 = str_to_num(b2); 
    let c1: f32 = str_to_num(c1); 
    let c2: f32 = str_to_num(c2); 

    // Вывод искомых значений 
    println!("x={}", (c1*b2-c2*b1)/(a1*b2-a2*b1)); 
    println!("y={}", (a1*c2-a2*c1)/(a1*b2-a2*b1)); 
} 

// Функция получения строки от пользователя 
fn read_user_line() -> String { 
    let mut x = String::new(); 
    io::stdin().read_line(&mut x)
        .expect("Не удалось прочитать строку"); 
    return x; 
} 

// Функция перевода из строки в число 
fn str_to_num(x: String) -> f32 { 
    let x: f32 = x.trim().parse()
        .expect("Введите числовое значение"); 
    return x; 
} 
