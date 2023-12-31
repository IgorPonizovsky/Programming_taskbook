// Вт 05 дек 2023 19:58:41 MSK
// Дана длина окружности 'L'. Найти её радиус 'R' и площадь 'S' круга, 
// ограниченного этой окружностью, учитывая, что 'L=2*П*R', 'S=П*R^2'. 
// В качестве значения П испльзовать 3.14 

use std::io; 

const PI: f32 = 3.14; 

fn main() { 
    // Получение длины окружности от пользователя 
    println!("Введите длину окружности L: "); 
    let mut l = String::new(); 
    io::stdin().read_line(&mut l)
        .expect("Не удалось прочитать строку"); 

    // Приведение полученных данных из строки в число 
    let l: f32 = match l.trim().parse() { 
        Ok(num) => num, 
        Err(_) => 0.0, 
    }; 

    // Расчёт требуемых величин 
    let r: f32 = l/(2.0*PI); 
    let s: f32 = PI*r.powf(2.0); 

    // Вывод искомого значения на экран 
    println!("Радиус окружности R={}", r); 
    println!("Площадь круга S={}", s); 
} 
