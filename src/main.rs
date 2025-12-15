//Celsius 攝氏度
//Fahrenheit 華氏
//Temperature conversion 溫度轉換
use std::io::{self, Write}; //等效下面兩行
//use std::io;
//use std::io::Write //防止文字卡在buffer（緩衝）

fn main() {
    println!("1.攝氏溫度轉華氏溫度");
    println!("2.華氏溫度轉攝氏溫度");
    println!("請輸入1或2");

    'loop1: loop {
        let mut choice_input = String::new();
        io::stdin().read_line(&mut choice_input).expect("Error");

        let choice: i32 = match choice_input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("請重新輸入");
                continue 'loop1;
            }
        };

        if !(choice == 1 || choice == 2) {
            println!("請重新輸入");
            continue 'loop1;
        }

        let temperature_names = ["華氏", "攝氏"];

        let input_temperature: f64 = 'loop2: loop {
            let mut temperature_input = String::new();

            print!("請輸入{}溫度 = ", temperature_names[(choice - 1) as usize]);
            io::stdout().flush().unwrap(); //防止文字卡在buffer（緩衝）

            io::stdin()
                .read_line(&mut temperature_input)
                .expect("Error");

            let temperature = match temperature_input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("請重新輸入");
                    continue 'loop2;
                }
            };

            break temperature;
        };

        let converted_temperature = if choice == 1 {
            (input_temperature * 9.0 / 5.0) + 32.0
        } else {
            (input_temperature - 32.0) * 5.0 / 9.0
        };

        println!(
            "{}度{} = {}度{}",
            input_temperature,
            temperature_names[(choice - 1) as usize],
            converted_temperature,
            temperature_names[(choice % 2) as usize]
        );

        break;
    }
}
