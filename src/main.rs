//Celsius 攝氏度
//Fahrenheit 華氏
//Kelvin 凱氏溫度
//Temperature conversion 溫度轉換
use std::io::{self, Write}; //等效下面兩行
//use std::io;
//use std::io::Write //防止文字卡在buffer（緩衝）

enum TemperatureType {
    Celsius,
    Fahrenheit,
    Kelvin,
}

fn main() {
    println!("1.攝氏溫度 2.華氏溫度 3.凱氏溫度");
    print_new("請輸入要被轉的溫度單位: ");
    let choice1 = read_choice_i32("要被轉的溫度單位: ");
    let choice1 = limit_number(choice1, 1, 3, "要被轉的溫度單位: ");

    println!("1.攝氏溫度 2.華氏溫度 3.凱氏溫度");
    print_new("請輸入要轉成的溫度單位: ");
    let choice2 = read_choice_i32("要轉成的溫度單位: ");
    let choice2 = limit_number(choice2, 1, 3, "要轉成的溫度單位: ");

    let choice1 = choice_to_unit(choice1);
    let choice2 = choice_to_unit(choice2);
    let choice1_name = choice_name(&choice1);
    let choice2_name = choice_name(&choice2);

    print!("請輸入{} = ", choice1_name);
    io::stdout().flush().unwrap(); //防止文字卡在buffer（緩衝）
    let mut temp_num = read_choice_f64(choice1_name);

    temp_num = to_kelvin(temp_num, choice1);
    temp_num = from_kelvin(temp_num, choice2);

    println!("{}= {}", choice2_name, temp_num);
}

fn read_choice_f64(print_string: &str) -> f64 {
    loop {
        let mut string1 = String::new();
        io::stdin().read_line(&mut string1).expect("Error");

        match string1.trim().parse::<f64>() {
            Ok(n) => {
                return n;
            }
            Err(_) => {
                let string1 = String::from("請重新輸入") + print_string;
                print_new(&string1);
            }
        };
    }
}

fn read_choice_i32(print_string: &str) -> i32 {
    loop {
        let mut string1 = String::new();
        io::stdin().read_line(&mut string1).expect("Error");

        match string1.trim().parse::<i32>() {
            Ok(n) => {
                return n;
            }
            Err(_) => {
                let string1 = String::from("請重新輸入") + print_string;
                print_new(&string1);
            }
        };
    }
}

fn limit_number(mut choice: i32, min: i32, max: i32, str1: &str) -> i32 {
    if !((choice >= min) && (choice <= max)) {
        choice = read_choice_i32(str1);
        return limit_number(choice, min, max, str1);
    }
    choice
}

fn print_new(str1: &str) {
    print!("{}", str1);
    io::stdout().flush().unwrap(); //防止文字卡在buffer（緩衝）
}

fn to_kelvin(num: f64, unit: TemperatureType) -> f64 {
    match unit {
        TemperatureType::Celsius => num + 273.15,
        TemperatureType::Fahrenheit => (num - 32.0) * 5.0 / 9.0 + 273.15,
        TemperatureType::Kelvin => num,
    }
}

fn from_kelvin(num: f64, unit: TemperatureType) -> f64 {
    match unit {
        TemperatureType::Celsius => num - 273.15,
        TemperatureType::Fahrenheit => (num - 273.15) * 9.0 / 5.0 + 32.0,
        TemperatureType::Kelvin => num,
    }
}

fn choice_to_unit(num: i32) -> TemperatureType {
    match num {
        1 => TemperatureType::Celsius,
        2 => TemperatureType::Fahrenheit,
        3 => TemperatureType::Kelvin,
        _ => unreachable!(),
    }
}

fn choice_name(unit: &TemperatureType) -> &'static str {
    match unit {
        TemperatureType::Celsius => "攝氏溫度",
        TemperatureType::Fahrenheit => "華氏溫度",
        TemperatureType::Kelvin => "凱氏溫度",
    }
}
