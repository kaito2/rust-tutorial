use std::io;

fn main() {
    let mut guess = String::new();
    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // read_lineがio::Result型（実態はenum）を返し、Result型がErrのときクラッシュさせる
        println!("you guessed: {}", guess);
        let guess: u32 = guess.trim().parse().expect("please type a number");
        println!("you guessed: {}", guess);
    }
}
