use std::io::{stdin, stdout, Write};

fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

fn main() {
    println!("Please input 4-digits User ID: ");
    let userid = Some(get_user_input());

    println!("Please input 8-digits Product ID: ");
    let productid = Some(get_user_input());

    let plain_serial = format!("{}{}", userid.unwrap(), productid.unwrap());
    println!("Plain serial: {}", plain_serial); // 암호화 전 시리얼 출력

    let verify_userid = &plain_serial[0..4];
    let verify_productid = &plain_serial[4..12];
    println!("Verify User ID: {}", verify_userid);
    println!("Verify Product ID: {}", verify_productid);
}
