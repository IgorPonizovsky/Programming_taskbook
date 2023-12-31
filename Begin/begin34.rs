// Ср 06 дек 2023 17:15:53 MSK
// Известно, что Х кг шоколадных конфет стоит А рублей, 
// а Y кг ирисок стоит B рублей. 
// Определить, сколько стоит 1кг шоколадных конфет, 
// 1кг ирисок, а так же во сколько раз шоколадные конфеты дороже ирисок. 

use std::io; 

fn main() { 
    // Получение данных цены и веса от пользователя 
    println!("Укажите вес упаковки шоколадных конфет(кг): "); 
    let x: String = read_user_line(); 
    println!("Укажите цену этой упаковки шоколадных конфет: "); 
    let a: String = read_user_line(); 
    println!("Укажите вес упаковки ирисок(кг): "); 
    let y: String = read_user_line(); 
    println!("Укажите цену этой упаковки ирисок: "); 
    let b: String = read_user_line(); 

    // Приведение полученных данных из строк в числа 
    let x: f32 = str_to_num(x); 
    let a: f32 = str_to_num(a); 
    let y: f32 = str_to_num(y); 
    let b: f32 = str_to_num(b); 

    // Вывод искомых значений 
    println!("1кг шоколадных конфет стоит {}", a/x); 
    println!("1кг ирисок стоит {}", b/y); 
    println!("Шоколадные конфеты дороже ирисок в {} раз", (a/x)/(b/y)); 
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
