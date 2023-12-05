// Вт 05 дек 2023 16:13:43 MSK
// Даны два неотрицательных числа 'a' и 'b'. 
// Найти их среднее геометрическое, т.е. квадратный корень 
// из их произведения: (a*b)^(1/2) 

use std::io; 

fn main() { 
    // Получение чисел от пользователя 
    println!("Введите число a: "); 
    let mut a = String::new(); 
    io::stdin().read_line(&mut a)
        .expect("Не удалось прочитать строку"); 
    println!("Введите число b: "); 
    let mut b = String::new(); 
    io::stdin().read_line(&mut b)
        .expect("Не удалось прочитать строку"); 

    // Приведение полученных данных из строк в числа 
    let a: f32 = str_to_num(a); 
    let b: f32 = str_to_num(b); 

    // Вывод искомого значения 
    println!("Среднее геометрическое равно {}", (a*b).sqrt()); 
}

// Функция перевода из строки в число 
fn str_to_num(x: String) -> f32 { 
    let x: f32 = match x.trim().parse() { 
        Ok(num) => num, 
        Err(_) => 0.0, 
    }; 
    return x; 
} 
