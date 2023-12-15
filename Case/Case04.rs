// Пт 15 дек 2023 17:41:02 MSK
// Дан номер месяца - целое число в диапазоне 1-12(1-январь, 2-февраль и т.д.). 
// Определить количество дней в этом месяце для невисокосного года. 

use std::io; 

fn main() { 
    // Получение целого числа от пользователя: 
    println!("Введите номер месяца(1-12): "); 
    let estimation: i8 = get_integer_number(); 

    // Вывод искомого значения: 
    match estimation { 
        2 => println!( "28" ), 
        1 | 3 | 5 | 7..=8 | 10 | 12 => println!( "31" ), 
        4 | 6 | 9 | 11 => println!( "30" ), 
        _ => println!( "Введите число от 1 до 12" ), 
    } 
} 

//Функция получения целого числа 
fn get_integer_number() -> i8 { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 
    let some_number: i8 = some_string.trim().parse()
        .expect("Введите целое число(1-12)"); 
    some_number 
} 
