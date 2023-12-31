// Ср 06 дек 2023 19:02:59 MSK
// Решить линейное уравнение A*x+B=0, 
// заданное своими коэффициентами A и B(коэффициент А не равен 0). 

use std::io; 

fn main() { 
    // Получение от пользователя коэффициентов линейного уравнения 
    println!("Введите коэффициент A(!=0) линейного уравнения: "); 
    let a: String = read_user_line(); 
    println!("Введите коэффициент B линейного уравнения: "); 
    let b: String = read_user_line(); 

    // Приведение полученных данных из строк в числа 
    let a: f32 = str_to_num(a); 
    let b: f32 = str_to_num(b); 

    // Вывод искомого значения 
    println!("x={}", (-1.0*b)/a); 
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
