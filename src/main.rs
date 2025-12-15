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
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Error");
        let a: i32 = match a.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("請重新輸入");
                continue 'loop1;
            }
        };
        if !(a == 1 || a == 2) {
            println!("請重新輸入");
            continue 'loop1;
        }
        let name1 = ["華氏", "攝氏"];

        let temper: f64 = 'loop2: loop {
            let mut temper = String::new();
            print!("請輸入{}溫度 = ", name1[(a - 1) as usize]);
            io::stdout().flush().unwrap(); //防止文字卡在buffer（緩衝）
            io::stdin().read_line(&mut temper).expect("Error");
            println!("");
            let temper = match temper.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("請重新輸入");
                    continue 'loop2;
                }
            };
            break temper;
        };

        let temper1 = if a == 1 {
            (temper * 9.0 / 5.0) + 32.0
        } else {
            (temper - 32.0) * 5.0 / 9.0
        };
        println!(
            "{}度{} = {}度{}",
            temper,
            name1[(a - 1) as usize],
            temper1,
            name1[(a % 2) as usize]
        );
        break;
    }
}
