// Вс 10 дек 2023 16:11:52 MSK
// С начала суток прошло N секунд(N-целое). 
// Найти количество секунд, прошедших с начала последней минуты. 

use std::io; 

fn main() { 
    // Получение количества прошедших секунд от пользователя: 
    println!("Введите количество прошедних секунд: "); 
    let mut seconds = String::new(); 
    io::stdin().read_line(&mut seconds)
        .expect("Не удалось прочитать строку"); 

    // Приведение полученного значения из строки в число: 
    let seconds: u32 = seconds.trim().parse()
        .expect("Введите числовое значение"); 

    // Расчёт и вывод искомого значения: 
    println!("С начала текущей минуты прошло {} секунд", (seconds%60)); 
} 
