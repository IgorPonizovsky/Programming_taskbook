// Ср 06 дек 2023 16:53:04 MSK
// Дано значение температуры Т в градусах Цельсия. 
// Определить значение этой же температуры в градусах Фаренгейта. 
// Температура по Цельсию и температура по Фаренгейту связаны 
// следующим соотношением: 'Tc=(Tf-32)*(5/9)' 

use std::io; 

fn main() { 
    // Получение значения температуры по Цельсию от пользователя 
    println!("Введите температуру по Цельсию: "); 
    let mut c = String::new(); 
    io::stdin().read_line(&mut c)
        .expect("Не удалось прочитать строку"); 

    // Приведение полученных данных из строки в число 
    let c: f32 = c.trim().parse()
        .expect("Введите числовое значение"); 

    // Расчёт и вывод искомого значения 
    println!("Температура по Фаренгейту: {}", c/(5.0/9.0)+32.0); 
} 
