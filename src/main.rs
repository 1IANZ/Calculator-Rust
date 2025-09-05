mod calc;
fn main() {
    println!("输入算术表达式，按回车键计算。支持 +, -, *, /, ^, () 符号。");
    println!("输入 q 退出。");
    loop {
        let mut input = String::new();
        println!("请输入算术表达式>");
        std::io::stdin()
            .read_line(&mut input)
            .expect("无法读取输入");
        let input = input.trim();
        if input == "q" {
            println!("程序已退出，欢迎下次使用，再见！");
            break;
        } else if input.is_empty() {
            continue;
        }
        match calc::calculate(&input) {
            Ok(result) => println!("结果: {}", result),
            Err(e) => println!("计算出现错误: {}", e),
        }
    }
}
