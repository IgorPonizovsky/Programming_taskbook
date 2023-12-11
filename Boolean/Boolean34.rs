// Пн 11 дек 2023 21:20:54 MSK
// Даны координаты поля шахматной доски x, y(целые числа, лежащие в диапазоне 1-8). 
// Учитывая, что левое нижнее поле доски(1,1) является чёрным, проверить истинность высказывания: 
// "Данное поле является белым". 

use std::io; 

fn main() { 
    // Получение координат x, y от пользователя: 
    println!("Введите координату x(1..8): "); 
    let x = read_user_line(); 
    println!("Введите координату y(1..8): "); 
    let y = read_user_line(); 

    // Приведение полученных данных из строки в число: 
    let x: i32 = string_to_number(x); 
    let y: i32 = string_to_number(y); 

    // Вывод искомого значения: 
    if x % 2 != y % 2 { 
        println!("Данное поле является белым"); 
    } else { 
        println!("Данное поле не является белым"); 
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
