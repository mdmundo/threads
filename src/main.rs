// Esse programa recebe um vetor de números ordinais para a sequência de Fibonacci
// e retorna a soma dos respectivos números utilizando threads.

use std::thread;

// Essa função vai receber um número e retorna o respectivo número na sequência de Fibonacci.
// Por exemplo se recebe como entrada o número 1, retorna o primeiro número da sequência de Fibonacci.
// Se recebe como entrada o número 3, retorna o terceiro número da sequência de Fibonacci que é 2.
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    // Retorna o número da sequência de Fibonacci que corresponde à posição n.
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    // Declaração do vetor que serve como parâmetro para função fibonacci(n: u32)
    // Obs.: O número 24 ocupa essa posição para mostrar que mesmo sendo chamado primeiro
    // seu resultado vem por último por ser a operação que custa mais processamento.
    let input: [u32; 6] = [24, 12, 0, 1, 8, 3];

    // Declaração do vetor que receberá o resultado das chamadas para função fibonacci(n: u32)
    let mut output = vec![];

    // Laço de repetição que percorre o vetor input
    for number in input {
        println!("Looking for number {} in Fibonacci sequence", number);

        // output.push Adiciona parâmetro ao final do vetor
        // thread::spawn Inicia um novo thread
        // move É utilizado para dar propriedade de valores para o thread
        output.push(thread::spawn(move || -> u32 {
            let result = fibonacci(number);

            println!("Number {} of the Fibonacci sequence is {}", number, result);

            // Retorna o resultado da chamada à função fibonacci(n: u32)
            result
        }));
    }

    // Realiza a soma de todos os valores resultantes da chamada à função fibonacci(n: u32)
    let final_result = output.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);
}
