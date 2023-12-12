// Вт 12 дек 2023 09:21:58 MSK
// Дано целое число. Если оно является положительным, то прибавить к нему 1; в противном случае не изменять его. 
// Вывести полученное число. 

use std::io; 

fn main() { 
    // Получение числа от пользователя: 
    println!("Введите целое число: "); 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 

    // Приведение полученноего от пользователя из строки в число: 
    let number: i32 = some_string.trim().parse()
        .expect("Введите целое число"); 
    some_string.clear(); 

    // Вывод искомого значения: 
    if number > 0 { 
        println!("{}", number + 1); 
    } else { 
        println!("{}", number); 
    }; 
} 
