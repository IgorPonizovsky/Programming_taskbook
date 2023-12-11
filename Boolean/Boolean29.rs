// Пн 11 дек 2023 19:38:29 MSK
// Даны числа x, y, x1, y1, x2, y2. Проверить истинность высказывания: "Точка с координатами (x, y) лежит внутри 
// прямоугольника, левая верхняя вершина которого имеет координаты (x1, y1), правая нижняя - (x2, y2), 
// а стороны параллельны координатным осям". 

use std::io; 

fn main() { 
    // Получение координат точек от пользователя: 
    println!("Введите координату x точки: "); 
    let x = read_user_line(); 
    println!("Введите координату y точки: "); 
    let y = read_user_line(); 
    println!("Введите координату x1 точки: "); 
    let x1 = read_user_line(); 
    println!("Введите координату y1 точки: "); 
    let y1 = read_user_line(); 
    println!("Введите координату x2 точки: "); 
    let x2 = read_user_line(); 
    println!("Введите координату y2 точки: "); 
    let y2 = read_user_line(); 

    // Приведение полученных координат из строк в числа: 
    let x = string_to_number(x); 
    let y = string_to_number(y); 
    let x1 = string_to_number(x1); 
    let y1 = string_to_number(y1); 
    let x2 = string_to_number(x2); 
    let y2 = string_to_number(y2); 

    // Вывод искомого значения: 
    if ((x < x1) && (x > x2)) && ((y > y1) && (y < y2)) { 
        println!("Точка с координатами({}, {}) лежит внутри прямоугольника", x, y); 
    } else { 
        println!("Точка с координатами({}, {}) не лежит внутри прямоугольника", x, y); 
    }; 
} 

// Функция получения строки из стандартного потока ввода-вывода 
fn read_user_line() -> String { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    some_string 
} 

// Функция перевода строки в число 
fn string_to_number(some_string: String) -> i32 { 
    let number: i32 = some_string.trim().parse()
        .expect("Введите числа"); 
    number 
} 
