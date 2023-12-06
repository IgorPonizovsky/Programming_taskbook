// Ср 06 дек 2023 10:42:26 MSK
// Поменять местами содержимое переменных A и B 
// и вывести новые значения A и B. 

use std::io; 

fn main() { 
    // Получаем значения переменных от пользователя 
    println!("Введите значение переменной A: "); 
    let a: String = read_user_line(); 
    println!("Введите значение переменной B: "); 
    let b: String = read_user_line(); 

    // Меняем значения переменных местами 
    let c = b; 
    let b = a; 
    let a = c; 

    // Выводим значения переменных на экран 
    println!("A: {}", a.trim()); 
    println!("B: {}", b.trim()); 
} 

// Функция получения строки от пользователя 
fn read_user_line() -> String { 
    let mut x = String::new(); 
    io::stdin().read_line(&mut x)
        .expect("Не удалось прочитать строку"); 
    return x; 
} 
