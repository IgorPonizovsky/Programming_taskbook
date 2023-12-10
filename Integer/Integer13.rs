// Вс 10 дек 2023 12:21:26 MSK
// Дано трёхзначное число. В нём зачеркнули первую слева цифру и переписали её справа. 
// Вывести полученное число. 

use std::io; 
use std::process; 

fn main() { 
    // Получение трёхзначного числа от пользователя: 
    println!("Введите трёхзначное число: "); 
    let mut x = String::new(); 
    io::stdin().read_line(&mut x)
        .expect("Не удалось прочитать строку"); 

    // Приведение полученного значения из строки в число: 
    let mut x: i32 = x.trim().parse()
        .expect("Введите ЦЕЛОЕ трёхзначное число"); 

    // Проверка условия трёхзначности числа: 
    if (x/1000) >= 1 { 
        println!("Введите целое ТРЁХЗНАЧНОЕ число");
        process::exit(0); 
    } 

    // Расчёт и вывод искомых значений: 
    if (x/100) != 0 { 
        x = x/100 + x*10;     // 123 -> 1231 
        println!("Новое число: {}", x%1000); // 1231 -> 231 
    } else { 
        println!("Введите целое ТРЁХЗНАЧНОЕ число"); 
        process::exit(0); 
    }; 
} 
