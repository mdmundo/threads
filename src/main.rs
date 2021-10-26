// Para rodar este código acesse https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=5c338fd5d0e0d40ad1f9dc534f654308
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
    let input: [u32; 6] = [24, 12, 5, 1, 8, 3];

    // Declaração do vetor que receberá o resultado das chamadas para função fibonacci(n: u32)
    let mut output = vec![];

    // Laço de repetição que percorre o vetor input
    for number in input {
        println!(
            "Procurando número que está na posição {} na sequência de Fibonacci",
            number
        );

        // output.push Adiciona parâmetro ao final do vetor
        // thread::spawn Inicia um novo thread
        // move É utilizado para dar propriedade de valores para o thread
        output.push(thread::spawn(move || -> u32 {
            let result = fibonacci(number);

            println!("Posição {} da sequência de Fibonacci é {}", number, result);

            // Retorna o resultado da chamada à função fibonacci(n: u32)
            result
        }));
    }

    // Realiza a soma de todos os valores resultantes da chamada à função fibonacci(n: u32)
    let final_result = output.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("A soma dos valores encontrados é {}", final_result);
}
