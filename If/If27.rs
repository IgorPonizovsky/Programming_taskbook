// Вт 12 дек 2023 21:10:19 MSK
// Для данного вещественного x найти значение следующей f, принимающей значения целого типа: 
// f(x) = 0 , x<0 
//      = 1 , x принадлежит [0,1), [2,3), ... , 
//      =-1 , x принадлежит [1,2), [3,4), ... . 


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
    if x < 0.0 { 
        println!("x<0: f(x)=0"); 
    }; 
    let integer_part = to_i32(x); 
    let integer_part = to_f64(integer_part); 
    if integer_part % 2.0 == 0.0 { 
        if ((0.0 <= x ) && ( x < 1.0)) | ((integer_part <= x ) && ( x < (integer_part + 1.0))) { 
            println!("f(x)=1"); 
        }; 
    } else { 
        println!("f(x)=-1"); 
    }; 
} 

// Получение целой части вещественного числа 
fn to_i32(x: f64) -> i32 { 
    x as i32 
}

// Перевод i32 в f64 
fn to_f64(x: i32) -> f64 { 
    x as f64 
} 
