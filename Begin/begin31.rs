// Ср 06 дек 2023 16:45:26 MSK
// Дано значение температуры Т в градусах Фаренгейта. 
// Определить значение этой же температуры в градусах Цельсия. 
// Температура по Цельсию и температура по Фаренгейту связаны 
// следующим соотношением: 'Tc=(Tf-32)*(5/9)' 

use std::io; 

fn main() { 
    // Получение значения температуры по Фаренгейту от пользователя 
    println!("Введите температуру по Фаренгейту: "); 
    let mut f = String::new(); 
    io::stdin().read_line(&mut f)
        .expect("Не удалось прочитать строку"); 

    // Приведение полученных данных из строки в число 
    let f: f32 = f.trim().parse()
        .expect("Введите числовое значение"); 

    // Расчёт и вывод искомого значения 
    println!("Температура по Цельсию: {}", (f-32.0)*(5.0/9.0)); 
} 
