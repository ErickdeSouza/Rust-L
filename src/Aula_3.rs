fn fluxo() {
    let temp = 28;

    if temp > 30 {
        println!("Muito quente");
    } else if temp > 20 {
        println!("Agradável");
    } else {
        println!("Frio");
    }


    let idade = 20;

    //sempre o mesmo type no if e else
    let cart = if idade >= 28 { "adulto" } else { "menor" }; 
    println!("{cart}");

    let desconto = if idade < 12 {
        50
    } else if idade >= 60 {
        40
    } else {
        0
    };
    println!("Desconto: {desconto}%");


    let mut contador = 0;

    let resultado = loop {
        contador += 1;
        println!("contador = {contador}");

        if contador == 3 {
            break contador * 10;
        }
    };
    println!("Resultado: {resultado}");


    let mut n = 3;

    while n > 0 {
        println!("{n}...");
        n -= 1;
    }
    println!("Decolar!");


    // Intervalo exclusivo: 0, 1, 2, 3, 4
    for i in 0..5 {
        print!("{i} ");
    }
    println!();

    // Intervalo inclusivo: 1, 2, 3, 4, 5
    for i in 1..=5 {
        println!("{i} ");
    }
    println!();

    // Invertido
    for i in (1..=5).rev() {
        println!("{i} ");
    }
    println!();

    // De 2 em 2
    for i in (0..=10).step_by(2) {
        print!("{i} ");
    }
    println!();

    let frutas = ["maçã", "banana", "uva"];
    for fruta in frutas {
        println!("Fruta: {fruta}");
    }

    for (pos, fruta) in frutas.iter().enumerate() {
        println!("{pos}: {fruta}");
    }


    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // ignora os pares
        }
        print!("{i} ");
    }
    println!();

    // Labels: controla loops juntos/aninhados
    'externo: for i in 1..=3 {
        for j in 1..=3 {
            if i * j == 4 {
                println!("achou i={i}, j={j}");
                break 'externo; // sai dos DOIS loops
            }
        }
    }


    let dia = 10;

    let nome = match dia {
        1 => "Domingo",
        2 => "Segunda",
        3 => "Terça",
        4 => "Quarta",
        5 => "Quinta",
        6 => "Sexta",
        7 => "Sábado",
        _ => "Dia inválido",
    };
    println!("{nome}");

    
    let nota = 85;
    
    let conceito = match nota {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        0..=69 => "D",
        _ => "Nota inválida",
    };
    println!("Conceito: {conceito}");

    let numero = 7;
    match numero {
        1 | 3 | 5 | 7 | 9 => println!("ímpar pequeno"),
        2 | 4 | 6 | 8 => println!("par pequeno"),
        _ => println!("fora de 1 a 9"),
    }

    let temperatura = -3;
    match temperatura {
        t if t < 0 => println!("Congelante: {t}°C"),
        0 => println!("Ponto de congelamento"),
        t @ 1..=25 => println!("Ameno: {t}°C"),
        t => println!("Quente: {t}°C"),
    }
}

fn lab() {
    let secreto: u32 = 73;
    let mut baixo: u32 = 1;
    let mut alto: u32 = 100;
    let mut tentativas: u32 = 0;

    let total_tentativas = loop {
        let palpite = (baixo + alto) / 2;
        tentativas += 1;
        println!("Tentativa {tentativas}: {palpite}");

        if palpite == secreto {
            break tentativas;
        } else if palpite < secreto {
            baixo = palpite + 1;
        } else {
            alto = palpite - 1;
        }
    };

    let avaliacao = match total_tentativas {
        1 => "Sorte absurda!",
        2..=5 => "Muito bom!",
        6..=7 => "Dentro do esperado para busca binária.",
        _ => "Algo está errado...",
    };
    println!("Descoberto em {total_tentativas} tentativas. {avaliacao}");

    println!("\nPrimos até 50:");
    let mut quantidade = 0;

    'candidatos: for n in 2..=50u32 {
        for divisor in 2..n {
            if n % divisor == 0 {
                continue 'candidatos;
            }
        }
        print!("{n} ");
        quantidade += 1;
    }
    println!("\nTotal: {quantidade}");

    let mut n: u64 = 27;
    let mut passos = 0;

    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        passos += 1;
    }
    println!("\nCollatz(27) chegou a 1 em {passos} passos");
}