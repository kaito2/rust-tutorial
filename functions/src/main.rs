fn main() {
    let x = plus_one(1);

    let y = {
        let x = 3;
        x + 1
    };

    println!("{}", x);
    println!("{}", y);

    // if の条件部分は bool である必要がある
    let number = 3;
    if number < 5 {
        println!("condition was true"); // 条件は真でした
    } else {
        println!("condition was false"); // 条件は偽でした
    }

    // if式は値を返すので let の右辺に持ってこれる
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // numberの値は、{}です
    println!("The value of number is: {}", number);

    for number in 1..=4 {
        println!("{}!", number);
    }
}

fn plus_one(i: i32) -> i32 {
    i + 1
}
