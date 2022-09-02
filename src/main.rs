fn main() {
    another_function(5, 6);
    let y = {
        let x = 3;
        x + 1
    };

    println!("Значение y = {}", y);
    let x = five();
    println!("Значение x = {}", x);

    let y = pluse_one(10);
    println!("Значение y = {}", y);
}

fn another_function(x: i32, y: i32){
    println!("другая функия x = {}, y = {}", x, y);
}

fn five() -> i32 {
    5
}

fn pluse_one(x: i32) -> i32{
    x+1
}
