// Пт 15 дек 2023 16:08:27 MSK
// Дано целое число в диапазоне 1-7. Вывести строку - название дня недели, 
// соответствующее данному числу(1-"понедельник", 2-"вторник" и т.д.). 

use std::io; 

fn main() { 
    // Получение номера дня недели от пользователя: 
    println!("Введите номер дня недели(1-7): "); 
    let day: i32 = get_integer_number(); 

    // Вывод искомого значения: 
    match day { 
        1 => println!( "Понедельник" ), 
        2 => println!( "Вторник" ), 
        3 => println!( "Среда" ), 
        4 => println!( "Четверг" ), 
        5 => println!( "Пятница" ), 
        6 => println!( "Суббота" ), 
        7 => println!( "Воскресенье" ), 
        _ => println!( "Введите число от 1 до 7" ), 
    } 
} 

//Функция получения целого числа 
fn get_integer_number() -> i32 { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    let some_number: i32 = some_string.trim().parse()
        .expect("Введите целое число(1-7)"); 
    some_number 
} 
