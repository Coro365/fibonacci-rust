use std::io;

fn main() {
    println!("Please input fibonacci index number!(1-93)");
    
    let mut request = String::new();
    
    io::stdin()
        .read_line(&mut request)
        .expect("Failed!");

    println!("You request: {}", request);
    
    let request: usize = request.trim().parse()
        .expect("not number");

    fibonacci(request);

}

fn fibonacci(request: usize) {
    let mut memo = vec![0usize, 1];
    let mut cunt: usize = 2;
    println!("{:?}", memo);

    while request >= cunt {
        memo.push(memo[cunt - 1] + memo[cunt -2]);
        println!("[{}]: {}", cunt, memo[cunt]);
        cunt += 1;
    }
    println!("{:?}", memo);
}

