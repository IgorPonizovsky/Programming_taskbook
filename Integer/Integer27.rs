// Вс 10 дек 2023 18:00:33 MSK
// Дни недели пронумерованы следующим образом: 1-Понедельник, 2-Вторник, ..., 6-Суббота, 7-Воскресенье. 
// Дано целое число К, лежащее в диапазоне 1-365. Определить номер дня недели для К-го дня года, 
// если известно, что в этом году 1 янваля было субботой. 

use std::io; 

fn main() { 
    // Получение номера дня года от пользователя: 
    println!("Введите номер дня года(1..365): "); 
    let mut k = String::new(); 
    io::stdin().read_line(&mut k)
        .expect("Не удалось прочитать строку"); 

    // Приведение полученных данных из строки в число: 
    let k: u32 = k.trim().parse()
        .expect("Введите числовое значение"); 

    // Расчёт и вывод искомого значения: 
    println!("Номер дня недели: {}", (k+5)%7); 
} 
