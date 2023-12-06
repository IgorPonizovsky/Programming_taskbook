// Ср 06 дек 2023 06:26:18 MSK
// Даны три точки 'a', 'b', 'c' на числовой оси. 
// Найти длины отрезков 'ac' и 'bc' и их сумму. 

use std::io; 

fn main() { 
    // Получение координат точек от пользователя 
    println!("Введите координаты точки a: "); 
    let a: f32 = write_user_num(); 
    println!("Введите координаты точки b: "); 
    let b: f32 = write_user_num(); 
    println!("Введите координаты точки c: "); 
    let c: f32 = write_user_num(); 

    // Вывод искомых значений на экран 
    println!("Длина отрезка 'ac'={}", (a-c).abs()); 
    println!("Длина отрезка 'bc'={}", (b-c).abs()); 
    println!("Длина 'ac'+'bc'={}", (a-c).abs()+(b-c).abs()); 
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

// Функция записи значения пользователя 
fn write_user_num() -> f32 { 
    let x: String = read_user_line(); 
    let x: f32 = str_to_num(x); 
    return x; 
} 
