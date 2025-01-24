fn main() {
    let args: std::vec::Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("invalid arg count!")
    }
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    let target = args[1][..].to_owned();
    let mut it = target.chars();
    let mut buf = "".to_owned();
    while let c = it.next() {
        match c {
            Some(ch) => match ch {
                '+' => break,
                '-' => break,
                cha => buf.push(cha),
            },
            None => panic!("unexpected error while reading"),
        }
    }
    println!("  mov rax, {}", buf);
    while let Some(c) = it.next() {
        match c {
            '+' => println!("  add rax, {}", parseint(&mut it).unwrap()),
            '-' => println!("  sub rax, {}", parseint(&mut it).unwrap()),
            _ => panic!("unexpected char"),
        }
    }
    println!("  ret");
    return;
}
fn parseint(s: &mut std::str::Chars) -> Result<u64, std::num::ParseIntError> {
    let mut ret: String = Default::default();
    while let Some(c) = s.clone().next() {
        if c.is_digit(10) {
            ret.push(c);
            s.next();
        } else {
            break;
        }
    }
    ret.parse()
}
