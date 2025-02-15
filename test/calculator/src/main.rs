use std::io;

fn main() {
    println!("请输入操作(+,-,*,/):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("读取失败!");
    let operation: &str = operation.trim();
    println!("请输入两个数字（用空格分隔）:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取失败!");
    let numbers: Vec<&str> = input.trim().split_whitespace().collect();
    if numbers.len() != 2 {
        println!("输入格式错误，请确保输入了两个数字！");
        return;
    }
    let num1 = numbers[0].parse::<f64>().expect("无法解析第一个数字");
    let num2 = numbers[1].parse::<f64>().expect("无法解析第二个数字");
    let result = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("除数不能为0!");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("不支持的操作!");
            return;
        }
    };
    println!("结果是: {}", result);
}
