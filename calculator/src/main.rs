use std::env::{args};

fn main() {
    let mut args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();
    let first_num = first.parse::<f32>().unwrap();
    let second_num = second.parse::<f32>().unwrap();
    let result = get_result(operator, first_num, second_num);
    let formatted_result = get_formatted_result(first_num, operator, second_num, result);
    println!("{}", formatted_result);
}

fn get_result(operator: char, first_num: f32, second_num: f32) -> f32 {
    match operator {
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        'x' => first_num * second_num,
        '/' => first_num / second_num,
        _ => panic!("Invalid operator used"),
    }
}

fn get_formatted_result(first_num: f32, operator: char, second_num: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_num, operator, second_num, result)
}


// fn get_type<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }
