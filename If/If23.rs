// Вт 12 дек 2023 17:46:40 MSK
// Даны целочисленные координаты трёх вершин прямоугольника, стороны которого параллельны координатным осям. 
// Найти координаты его четвёртой вершины. 

use std::io; 

fn main() { 
    // Получение целочисленных координат от пользователя: 
    println!("Введите координату x1: "); 
    let x1 = read_user_line(); 
    println!("Введите координату y1: "); 
    let y1 = read_user_line(); 
    println!("Введите координату x2: "); 
    let x2 = read_user_line(); 
    println!("Введите координату y2: "); 
    let y2 = read_user_line(); 
    println!("Введите координату x3: "); 
    let x3 = read_user_line(); 
    println!("Введите координату y3: "); 
    let y3 = read_user_line(); 

    // Приведение полученных строк в числа: 
    let x1 = string_to_number(x1); 
    let x2 = string_to_number(x2); 
    let x3 = string_to_number(x3); 
    let y1 = string_to_number(y1); 
    let y2 = string_to_number(y2); 
    let y3 = string_to_number(y3); 

    // Расчёт и вывод искомого значения: 
    if ( x1 == x2 ) && ( y2 == y3 ) { 
        println!("Координаты четвёртой вершины: {}, {}", x3, y1); 
    } else 
    if ( x2 == x3 ) && ( y1 == y2 ) { 
        println!("Координаты четвёртой вершины: {}, {}", x1, y3); 
    } else 
    if ( x1 == x3 ) && ( y2 == y3 ) { 
        println!("Координаты четвёртой вершины: {}, {}", x2, y1); 
    } else { 
        println!("Стороны прямоугольника не параллельны осям координат"); 
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
fn string_to_number(some_string: String) -> i32 { 
    let some_number: i32 = some_string.trim().parse()
        .expect("Введите целые числа"); 
    some_number 
} 
