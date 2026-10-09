fn a_string() {
    let literal: &str = "olá";               // &str: texto fixo no binário | *imutável*
    let a: String = String::from("olá");     // String a partir de &str
    let b: String = "mundo".to_string();     // outra forma
    let c: String = "!".to_owned();          // outra forma
    let vazia = String::new();       // String vazia

    println!("{literal} {a} {b} {c} [{vazia}]");

    let mut texto = String::from("Rust");

    texto.push(' '); //somente CHAR
    texto.push_str("é legal");
    println!("{texto}");

    texto.insert(0, '>');
    println!("{texto}");

    texto.pop(); // remove o utlimo item
    println!("{texto}");

    texto.clear();
    println!("[{texto}] len={}", texto.len());

    let frase = "  Aprender Rust é divertido  ";

    println!("[{}]", frase.trim());                    // remove espaços das pontas
    println!("{}", frase.to_uppercase());
    println!("{}", frase.to_lowercase());
    println!("{}", frase.contains("Rust"));            // true
    println!("{}", frase.starts_with("  Apr"));        // true
    println!("{}", frase.replace("divertido", "útil"));
    println!("{:?}", frase.find("Rust"));              // Some(10)
    println!("{}", frase.trim().is_empty());           // false

    let csv = "maçã,banana,uva";
    for item in csv.split(',') {
        println!("- {item}");
    }

    let qtd = frase.split_whitespace().count();
    println!("{qtd} palavras");

    let palavra = "coração";

    println!("bytes: {}", palavra.len());            // 9
    println!("chars: {}", palavra.chars().count());  // 7

    for c in palavra.chars() {
        print!("[{c}]");
    }
    println!();

    println!("{:?}", palavra.chars().nth(2));

    println!("{}", &palavra[0..3]);

    let literal = "programação";
    let dono = String::from("Rust é rápido");

    println!("{}", contar_vogais(literal));
    println!("{}", contar_vogais(&dono)); // &String vira &str automaticamente
}

fn contar_vogais(texto: &str) -> usize {
    texto
        .chars()
        .filter(|c| "aeiouáéíóúâêôãõàAEIOU".contains(*c))
        .count()
}