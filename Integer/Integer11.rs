// Вс 10 дек 2023 11:37:58 MSK
// Дано трёхзначное число. Найти сумму и произведение его цифр. 

use std::io; 
use std::process; 

fn main() { 
    // Получение трёхзначного числа от пользователя: 
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
    if (x/100) == 0 { 
        println!("Введите целое ТРЁХЗНАЧНОЕ число"); 
        process::exit(0); 
    } else { 
        println!("Сумма цифр числа: {}", (x/100)+((x%100)/10)+(x%10)); 
        println!("Произведение цифр числа: {}", (x/100)*((x%100)/10)*(x%10)); 
    }; 
} 
