// Вт 05 дек 2023 15:29:05 MSK
// Найти длину окружности 'L' и площадь круга 'S' 
// заданного радиуса 'R': 'L=2*П*R', 'S=П*R^2'. 
// В качестве значения 'П' использовать 3.14 

use std::io; 

const PI: f32 = 3.14; 

fn main() { 
    // Получение радиуса окружности от пользователя 
    println!("Введите радиус окружности r: "); 
    let mut r = String::new(); 
    io::stdin().read_line(&mut r)
        .expect("Не удалось прочитать строку"); 

    // Приведение полученных данных из строки в число 
    let r: f32 = match r.trim().parse() { 
        Ok(num) => num, 
        Err(_) => 0.0, 
    }; 

    // Вывод искомых значений на экран 
    println!("Длина окружности L={}", 2.0*PI*r); 
    println!("Площадь круга S={}", PI*r.powf(2.0)); 
} 
