// Ср 06 дек 2023 09:46:08 MSK
// Даны координаты трёх вершин треугольника: (x1,y1), (x2,y2), (x3,y3). 
// Найти его периметр и площадь, используя формулу для 
// нахождения расстояния: ((x2-x1)^2+(y2-y1)^2)^(1/2). 
// Для нахождения площади треугольника со сторонами 'a', 'b', 'c' 
// использовать формулу Герона: S=(p*(p-a)*(p-b)*(p-c))^(1/2) 
// где p=(a+b+c)/2 - полупериметр 

use std::io; 

fn main() { 
    // Получение координат точек треугольника от пользователя 
    println!("Введите x1: "); 
    let x1: String = read_user_line(); 
    println!("Введите y1: "); 
    let y1: String = read_user_line(); 
    println!("Введите x2: "); 
    let x2: String = read_user_line(); 
    println!("Введите y2: "); 
    let y2: String = read_user_line(); 
    println!("Введите x3: "); 
    let x3: String = read_user_line(); 
    println!("Введите y3: "); 
    let y3: String = read_user_line(); 

    // Приведение полученных данных из строк в числа 
    let x1: f32 = str_to_num(x1); 
    let x2: f32 = str_to_num(x2); 
    let x3: f32 = str_to_num(x3); 
    let y1: f32 = str_to_num(y1); 
    let y2: f32 = str_to_num(y2); 
    let y3: f32 = str_to_num(y3); 

    // Расчёт сторон треугольника 
    let ab: f32 = distance_between(x1, x2, y1, y2); 
    let ac: f32 = distance_between(x1, x3, y1, y3); 
    let bc: f32 = distance_between(x2, x3, y2, y3); 

    // Расчёт искомых значений 
    let p2 = (ab+ac+bc)/2.0; 
    let s = (p2*(p2-ab)*(p2-ac)*(p2-bc)).sqrt(); 

    // Вывод искомых значений на экран 
    println!("Периметр треугольника = {}", p2*2.0); 
    println!("Площадь треугольника = {}", s); 
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

// Функция нахождения расстояния между двумя точками 
fn distance_between(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 { 
    let i: f32 = ((x2-x1).powf(2.0)+(y2-y1).powf(2.0)).sqrt(); 
    return i; 
} 
