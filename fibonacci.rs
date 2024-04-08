fn fibo(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        return fibo(n-1) + fibo(n-2);
    }
}

fn main() {
    let n = 20;
    println!("fibo {n} = {}", fibo(n));
    
}
