// Вт 12 дек 2023 16:02:25 MSK
// На числовой оси расположены три точки: A, B, C. Определить, какая из двух последних точке(B или C) 
// расположена ближе к A, и вывести эту точку и её расстояние от точки A. 

use std::io; 

fn main() { 
    // Получение трёх чисел от пользователя: 
    println!("Введите точку A: "); 
    let a = read_user_line(); 
    println!("Введите точку B: "); 
    let b = read_user_line(); 
    println!("Введите точку C: "); 
    let c = read_user_line(); 

    // Приведение полученных строк в числа: 
    let a = string_to_number(a); 
    let b = string_to_number(b); 
    let c = string_to_number(c); 

    // Расчёт расстояний между точками и точкой A: 
    let ab = (a - b).abs(); 
    let ac = (a - c).abs(); 

    // Вывод искомого значения: 
    if ab < ac { 
        println!("Точка В({}) удалена от точки А на: {}", b, ab); 
    } else 
    if ab > ac { 
        println!("Точка С({}) удалена от точки А на: {}", c, ac); 
    } else { 
        println!("Точки В({}) и С({}) равноудалены от точки А на: {}", b, c, ab); 
    }; 
} 

// Функция получени строки от пользователя 
fn read_user_line() -> String { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    some_string 
} 

// Функция приведения из строки в число 
fn string_to_number(some_string: String) -> f64 { 
    let some_number: f64 = some_string.trim().parse()
        .expect("Введите целые числа"); 
    some_number 
} 
