// Вт 12 дек 2023 19:02:38 MSK
// Для данного вещественного x найти значение следующей функции f, 
// принимающей вещественные значения: 
// f(x) = 2*Sin(x) , x>0 
//      = 6 - x    , x<=0 

use std::io; 

fn main() { 
    // Получение числа от пользователя: 
    println!("Введите вещественное число: "); 
    let mut some_string = String::new(); 
    io::stdin().read_line(&mut some_string)
        .expect("Не удалось прочитать строку"); 

    // Приведение полученноего от пользователя из строки в число: 
    let x: f64 = some_string.trim().parse()
        .expect("Введите вещественное число"); 
    some_string.clear(); 

    // Вывод искомого значения: 
    if x > 0.0 { 
        println!("x>0: f(x)={}", x.sin() * 2.0 ); 
    } else { 
        println!("x<=0: f(x)={}", 6.0 - x ); 
    }; 
} 
