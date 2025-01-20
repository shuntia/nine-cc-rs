fn main() {
    let args: std::vec::Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("invalid arg count!")
    }
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", args.get(1).unwrap());
    println!("  ret");
    return;
}
