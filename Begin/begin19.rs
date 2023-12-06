// Ср 06 дек 2023 07:23:00 MSK
// Даны координаты двух противоположных вершин прямоугольника: 
// (x1,y1), (x2,y2). Стороны прямоугольника параллельны осям координат. 
// Найти периметр и площаль данного прямоугольника. 

use std::io; 

fn main() { 
    // Получение координат вершин прямоугольника от пользователя 
    println!("Введите координату 'x' первой вершины: "); 
    let x1: String = read_user_line(); 
    println!("Введите координату 'y' первой вершины: "); 
    let y1: String = read_user_line(); 
    println!("Введите координату 'x' второй вершины: "); 
    let x2: String = read_user_line(); 
    println!("Введите координату 'y' второй вершины: "); 
    let y2: String = read_user_line(); 

    // Приведение полученных данных из строк в числа 
    let x1: f32 = str_to_num(x1); 
    let x2: f32 = str_to_num(x2); 
    let y1: f32 = str_to_num(y1); 
    let y2: f32 = str_to_num(y2); 

    // Расчёт искомых значений 
    let p: f32 = 2.0*(distance_between(x1, x2)+distance_between(y1, y2)); 
    let s: f32 = distance_between(x1, x2)*distance_between(y1, y2); 

    // Вывод искомых значений на экран 
    println!("Периметр прямоугольника P={}", p); 
    println!("Площадь прямоугольника S={}", s); 
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

// Функция расчёта расстояния между двумя точками(возвращает модуль) 
fn distance_between(a: f32, b: f32) -> f32 { 
    let num: f32 = (a-b).abs(); 
    return num; 
} 
