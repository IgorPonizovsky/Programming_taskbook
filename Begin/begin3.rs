// Вт 05 дек 2023 13:27:56 MSK
// Даны стороны прямоугольника 'a' и 'b'. Найти его площадь 'S=a*b' и периметр 'P=2*(a+b)' 

use std::io; 

fn main() { 
    // Получение от пользователя длинн сторон 
    println!("Введите сторону a: "); 
    let mut a = String::new(); 
    io::stdin().read_line(&mut a)
        .expect("Не удалось прочитать строку"); 
    println!("Введите строну b: "); 
    let mut b = String::new(); 
    io::stdin().read_line(&mut b)
        .expect("Не удалось прочитать строку"); 

    // Приведение введённых пользователем данных из строк в числа 
    let a: f32 = str_to_num(a); 
    let b: f32 = str_to_num(b); 

    // Вывод искомых величин на экран 
    println!("Площадь S={}", a*b); 
    println!("Периметр P={}", 2.0*(a+b)); 
} 

// Функция перевода из строки в число 
fn str_to_num(x: String) -> f32 { 
    let x: f32 = match x.trim().parse() { 
        Ok(num) => num, 
        Err(_) => 0.0, 
    }; 
    return x;
}
