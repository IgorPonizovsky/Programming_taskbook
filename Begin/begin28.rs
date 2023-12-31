// Ср 06 дек 2023 15:38:34 MSK
// Дано число A. Вычислить A^15, используя две вспомогательные 
// переменные и пять операция умножения. Для этого последовательно 
// находить A^2, A^3, A^5, A^10, A^15. 
// Вывести все найденные степени числа A. 

use std::io; 

fn main() { 
    // Получение значения A от пользователя 
    println!("Введите число A: "); 
    let mut a = String::new(); 
    io::stdin().read_line(&mut a)
        .expect("Не удалось прочитать строку"); 

    // Приведение полученных данных из строки в число 
    let a: f32 = a.trim().parse()
        .expect("Введите числовое значение"); 
    
    // Расчёт и вывод искомых значений 
    let x: f32 = a*a; 
    println!("A^2={}", x); 
    let y: f32 = a*x; 
    println!("A^3={}", y); 
    let y: f32 = x*y; 
    println!("A^5={}", y); 
    let x: f32 = y*y; 
    println!("A^10={}", x); 
    let a: f32 = x*y; 
    println!("A^15={}", a); 
} 
