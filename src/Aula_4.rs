fn func() {
    saudar();
    saudar_pessoa("Erick");

    let r = somar(3, 4);
    println!("{r}");
    println!("{}", quadrado(5));
    println!("{}", eh_par(10));

    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}"); // 4

    println!("{}", dividir(10.0, 4.0));
    println!("{}", dividir(1.0, 0.0));
    println!("{}", classificar(-3));

    let (menor, maior) = min_max([4, 9, -2, 7, 1]); 
    println!("menor={menor}, maior={maior}");

    let a = [1, 2, 3];
    let b = [10, 20, 30, 40, 50];
    println!("{}", soma(&a));
    println!("{}", soma(&b));
    println!("{}", soma(&b[1..3])); // 20 + 30

    let a = 5;
    println!("{}", incrementar(a)); // 6
    println!("{a}");                // 5, nao muda variavel original

    let mut b = 9;
    zerar(&mut b);
    println!("{b}");   // aqui muda a variavel original

    fn arredondar(x: f64) -> f64 {
        (x * 10.0).round() / 10.0
    }

    let m = media(&[7.5, 8.0, 6.25]);
    println!("{}", arredondar(m));
}

fn saudar() {
    println!("Olá!");
}

fn saudar_pessoa(nome: &str) {
    println!("Olá, {nome}!");
}

fn somar(a: i32, b: i32) -> i32 {
    a + b
}

fn quadrado(n: i32) -> i32 {
    return n * n;
}

fn eh_par(n: i32) -> bool {
    n % 2 == 0
}

fn dividir(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return 0.0;
    }
    a / b
}

fn classificar(n: i32) -> &'static str {
    if n < 0 {
        return "negativo";
    }
    if n == 0 {
        return "zero";
    }
    "positivo"
}

fn min_max(valores: [i32; 5]) -> (i32, i32) {
    let mut menor = valores[0];
    let mut maior = valores[0];
    for v in valores {
        if v < menor { menor = v; }
        if v > maior { maior = v; }
    }
    (menor, maior)
}

fn soma(valores: &[i32]) -> i32 {
    let mut total = 0;
    for v in valores {
        total += v;
    }
    total
}

fn incrementar(mut n: i32) -> i32 {
    n += 1;
    n
}

fn zerar(valor: &mut i32) {
    *valor = 0;
}

fn media(notas: &[f64]) -> f64 {
    if notas.is_empty() {
        return 0.0;
    }
    let soma: f64 = notas.iter().sum();
    soma / notas.len() as f64
}