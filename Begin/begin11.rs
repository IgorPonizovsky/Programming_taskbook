// Вт 05 дек 2023 18:14:56 MSK
// Даны два ненулевых числа. Найти сумму, 
// разность, произведение и частное их модулей. 

use std::io; 

fn main() { 
    // Получение чисел от пользователя 
    println!("Введите число a: "); 
    let a = read_user_line(); 
    println!("Введите число b: "); 
    let b = read_user_line(); 

    // Приведение полученных данных из строк в числа 
    let a: f32 = str_to_num(a); 
    let b: f32 = str_to_num(b); 

    // Вывод искомых значений 
    println!("Сумма модулей чисел: {}", a.abs()+b.abs()); 
    println!("Разность модулей чисел: {}", a.abs()-b.abs()); 
    println!("Произведение модулей чисел: {}", a.abs()*b.abs()); 
    println!("Частное модулей чисел: {}", a.abs()/b.abs()); 
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
    let x: f32 = match x.trim().parse() { 
        Ok(num) => num, 
        Err(_) => 0.0, 
    }; 
    return x; 
} 
