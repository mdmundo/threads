use std::thread;

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let data: [u32; 6] = [24, 12, 0, 1, 8, 3];

    let mut children = vec![];

    for number in data {
        println!("Looking for number {} in Fibonacci sequence", number);

        children.push(thread::spawn(move || -> u32 {
            let result = fibonacci(number);

            println!("Number {} of the Fibonacci sequence is {}", number, result);

            result
        }));
    }

    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);
}
