// Вс 10 дек 2023 11:31:39 MSK
// Дано трёхзначное число. Вывести вначале его последнюю цифру(единицы), 
// а затем - среднюю цифру(десятки). 

use std::io; 
use std::process; 

fn main() { 
    // Получение трёхначного числа от пользователя: 
    println!("Введите трёхзначное число: "); 
    let mut x = String::new(); 
    io::stdin().read_line(&mut x)
        .expect("Не удалось прочитать строку"); 

    // Приведение полученного значения из строки в число: 
    let x: i32 = x.trim().parse()
        .expect("Введите ЦЕЛОЕ трёхзначное число"); 

    // Проверка условия трёхзначности числа: 
    if (x/1000) >= 1 { 
        println!("Введите целое ТРЁХЗНАЧНОЕ число");
        process::exit(0); 
    } 

    // Расчёт и вывод искомых значений: 
    println!("Единиц: {}", x%10); 
    println!("Десятков: {}", (x%100)/10); 
} 
