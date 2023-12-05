// Вт 05 дек 2023 19:11:54 MSK
// Даны катеты прямоугольного треугольника 'a' и 'b'. 
// Найти его гипотенузу 'c' и периметр 'P': 
// 'c=(a^2+b^2)^(1/2)', 'P=a+b+c' 

use std::io; 

fn main() { 
    // Получение значений катетов прямоугольного треугольника 
    println!("Введите катет a: ");
    let a = read_user_line(); 
    println!("Введите катет b: ");
    let b = read_user_line(); 

    // Преобразование полученных данных из строк в числа 
    let a: f32 = str_to_num(a); 
    let b: f32 = str_to_num(b); 

    // Расчёт гипотенузы 
    let c: f32 = (a.powf(2.0)+b.powf(2.0)).sqrt();

    // Вывод искомых величин на экран 
    println!("Гипотенуза c={}", c);
    println!("Периметр P={}", a+b+c); 
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
