// Вт 12 дек 2023 16:52:25 MSK
// Даны координаты точки, не лежащей на координатных осях OX и OY. 
// Определить номер координатной четверти, в которой находится данная точка. 

use std::io; 

fn main() { 
    // Получение координат x, y от пользователя: 
    println!("Введите координату x: "); 
    let x = read_user_line(); 
    println!("Введите координату y: "); 
    let y = read_user_line(); 

    // Приведение полученных данных из строки в число: 
    let x: i32 = string_to_number(x); 
    let y: i32 = string_to_number(y); 

    // Вывод искомого значения: 
    if x > 0 && y > 0 { 
        println!("Точка лежит в первой координатной четверти"); 
    } else 
    if x < 0 && y > 0 { 
        println!("Точка лежит во второй координатной четверти"); 
    } else 
    if x < 0 && y < 0 { 
        println!("Точка лежит в третьей координатной четверти"); 
    } else 
    if x > 0 && y < 0 { 
        println!("Точка лежит в четвёртой координатной четверти"); 
    } else { 
        println!("Точка лежит где-то на координатных осях"); 
    }; 
} 

// Функция получения строки из стандартного ввода-вывода 
fn read_user_line() -> String { 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалость прочитать строку"); 
    some_string 
} 

// Функция приведения строки в число 
fn string_to_number(some_string: String) -> i32 { 
    let number: i32 = some_string.trim().parse()
        .expect("Введите числовые координаты"); 
    number 
} 
