use std::io;

fn main() {
    println!("Choose conversion type: ");
    println!("\t 1. Fahrenheight to Celsius");
    println!("\t 2. Celcius to Fahrenheight");
    let mut conversion_option = String::new();
    let mut conversion_num = String::new();

    io::stdin().read_line(&mut conversion_option).expect("Failed to read line");
    let conversion_option:i32 = conversion_option.trim().parse().unwrap();

    println!("Enter the number to be converted");
    io::stdin().read_line(&mut conversion_num).expect("Failed to read line");
    let mut conversion_num:f64 = conversion_num.trim().parse().unwrap();
    
    let temp_f = conversion_num;

    // 1. Fahrenheight to Celsius
    if conversion_option == 1 {
        conversion_num = (conversion_num - 32.0) / 1.8;
        println!("The temperature {}F = {}C" ,temp_f,conversion_num);
  
        //  2. Celcius to Fahrenheight
    } else if  conversion_option == 2  {
       conversion_num = (conversion_num * 1.8) + 32.0;
       println!("The temperature {}C = {}F" ,temp_f,conversion_num);
    } 


}
