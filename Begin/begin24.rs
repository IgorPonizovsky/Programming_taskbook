// Ср 06 дек 2023 14:51:43 MSK
// Даны переменные A, B, C. Изменить их значения, перемести содержимое 
// A в C, C в B, B в A, и вывести новые значени переменных A, B, C. 

use std::io; 

fn main() { 
    // Получаем значения переменных от пользователя 
    println!("Введите значение переменной A: "); 
    let a: String = read_user_line(); 
    println!("Введите значение переменной B: "); 
    let b: String = read_user_line(); 
    println!("Введите значение переменной C: "); 
    let c: String = read_user_line(); 

    // Меняем значения переменных местами 
    let x = c; 
    let c = a; 
    let a = b; 
    let b = x; 

    // Выводим значения переменных на экран 
    println!("A: {}", a.trim()); 
    println!("B: {}", b.trim()); 
    println!("C: {}", c.trim()); 
} 

// Функция получения строки от пользователя 
fn read_user_line() -> String { 
    let mut x = String::new(); 
    io::stdin().read_line(&mut x)
        .expect("Не удалось прочитать строку"); 
    return x; 
} 
