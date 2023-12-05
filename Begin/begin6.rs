// Вт 05 дек 2023 15:13:30 MSK
// Даны длны рёбер 'a', 'b', 'c' прямоугольного параллелепипеда. 
// Найти его объём 'V=a*b*c' 
// и площадь его поверхности 'S=2*(a*b+b*c+a*c)'. 

use std::io; 

fn main() { 
    // Получение значений длинн рёбер прямоугольного параллелепипеда от пользователя 
    println!("Введите ребро a: ");
    let a = read_user_line(); 
    println!("Введите ребро b: ");
    let b = read_user_line(); 
    println!("Введите ребро c: ");
    let c = read_user_line(); 

    // Преобразование полученных данных из строк в числа 
    let a: f32 = str_to_num(a); 
    let b: f32 = str_to_num(b); 
    let c: f32 = str_to_num(c); 

    // Вывод искомых величин на экран 
    println!("Объём V={}", a*b*c); 
    println!("Площадь поверхности S={}", 2.0*(a*b+b*c+a*c)); 
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
