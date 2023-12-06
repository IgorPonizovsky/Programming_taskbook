// Ср 06 дек 2023 12:50:25 MSK
// Даны переменный A, B, C. Изменить из значения, переместив 
// содержимое A в B, B в C, C в A, 
// и вывести новые значения переменных A, B, C. 

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
    let c = b; 
    let b = a; 
    let a = x; 

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
