/* 
Пт 15 дек 2023 17:12:21 MSK
Дано целое число K. Вывести строку-описани оценки, соответствующей числу K 
(1-"плохо", 2-"неудовлетворительно", 3-"удовлетворительно", 4-"хорошо", 5-"отлично"). 
Если K не лежит в диапазоне 1-5, то вывести строку "ошибка". 
*/ 

use std::io; 

fn main() { 
    // Получение целого числа от пользователя: 
    println!("Введите оценку(1-5): "); 
    let estimation: i32 = get_integer_number(); 

    // Вывод искомого значения: 
    match estimation { 
        1 => println!( "Плохо" ), 
        2 => println!( "Неудовлетворительно" ), 
        3 => println!( "Удовлетворительно" ), 
        4 => println!( "Хорошо" ), 
        5 => println!( "Отлично" ), 
        _ => println!( "Ошибка" ), 
    } 
} 

//Функция получения целого числа 
fn get_integer_number() -> i32 { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    let some_number: i32 = some_string.trim().parse()
        .expect("Введите целое число(1-5)"); 
    some_number 
} 
