fn oper() {
    let a = 17;
    let b = 5;

    println!("Soma: {}", a + b);
    println!("Subtração: {}", a - b);
    println!("Multiplicação: {}", a * b);
    println!("Divisão: {}", a / b);
    println!("Resto: {}", a % b);

    let x = 17.0;
    let y = 5.0;
    println!("Divisão Decimal: {}", x / y);

    let mut n = 10;
    n += 5;  // n = n + 5  → 15
    n -= 3;  // n = n - 3  → 12
    n *= 2;  // n = n * 2  → 24
    n /= 4;  // n = n / 4  → 6
    n %= 4;  // n = n % 4  → 2
    println!("{}", n);

    let inteiro: i32 = 10;
    let decimal: f64 = 2.5;

    let resultado = inteiro as f64 * decimal;
    println!("Resultado: {}", resultado);

    let pi = 3.99_f64;
    print!("{}", pi as i32);

    // Inteiro maior → menor: pode perder dados
    let grande: i32 = 300;
    println!("{}", grande as u8);

    // Negativo → sem sinal
    let negativo: i32 = -1;
    println!("{}", negativo as u32);

    let pequeno: i32 = 100;
    let ampliado: i64 = i64::from(pequeno);
    println!("{}", ampliado);

    
    let resultado = u8::try_from(grande);
    println!("{:?}", resultado);

    let ok = u8::try_from(200);
    println!("{:?}", ok);


    let x: u8 = 250;

    // "x + 10"
    println!("{}", x.wrapping_add(10));
    println!("{}", x.saturating_add(10));
    println!("{:?}", x.checked_add(10));
    println!("{:?}", x.checked_add(5));
    
    let (valor, estorou) = x.overflowing_add(10);
    println!("{} {}", valor, estorou);

    let idade = 25;
    let tem_c = true;

    println!("{}", idade >= 10);
    println!("{}", idade >= 18 && tem_c);
    println!("{}", idade < 18 || !tem_c);
    println!("{}", idade != 25);
    println!("{} ... {}", 0.1 + 0.2 == 0.3, 0.1 + 0.2); // false!

    let diferenca = ((0.1_f64 + 0.2) - 0.3).abs();
    println!("{}", diferenca < 1e-9); // 1e-9 = (0,000000001)

    // Operadores bit a bit
    println!("{}", 0b1100 & 0b1010); // 8  (0b1000)
    println!("{}", 1 << 3);          // 8

    let pessoa: (&str, i32, f64) = ("Erick", 20, 1.78);

    println!("Nome: {}", pessoa.0);
    println!("Idade: {}", pessoa.1);

    let (nome, idade, altura) = pessoa;
    println!("{nome} tem {idade} anos e {altura} m");

    let mut ponto = (10, 20);
    ponto.0 += 5;
    println!("{:?}", ponto);

    let nada: () = ();
    println!("{:?}", nada);

    let notas: [i32; 5] = [7, 8, 6, 9, 10];
    println!("Primeira: {}", notas[0]);
    println!("Tamanho: {}", notas.len());
    println!("{:?}", notas);

    // Todos os elementos iguais
    let zeros = [0; 4]; // [0, 0, 0, 0]
    println!("{:?}", zeros);

    let mut placar = [0; 3];
    placar[1] = 50;
    println!("{:?}", placar); // [0, 50, 0]

    for nota in notas {
        print!("{} ", nota);
    }
    println!();

    let matriz = [[1, 2, 3], [4, 5, 6]];
    println!("{}", matriz[1][2]); // 6
}

/// EXERCICIO N. 2
const N_AP: f64 = 7.0;
const F_MIN: f64 = 75.0;

fn notas() {
    let aluno: (&str, u32, u32) = ("Erick", 42, 50);
    let notas: [f64; 5] = [8.5, 6.0, 7.5, 9.0, 5.5];

    let (nome, ass, total) = aluno;

    let mut soma = 0.0;
    let mut maior = notas[0];
    let mut menor = notas[0];

    for nota in notas {
        soma += nota;
        if nota > maior {
            maior = nota;
        }
        if nota < menor {
            menor = nota;
        }
    }

    let media = soma / notas.len() as f64;
    let frequencia = ass as f64 / total as f64 * 100.0;

    let n_ok = media >= N_AP;
    let f_ok = frequencia >= F_MIN;
    let aprovado = n_ok && f_ok;

    println!("Nome: {nome}");
    println!("Notas: {:?}", notas);
    println!("Soma: {soma:.1}");
    println!("Média: {media:.2}");
    println!("Maior: {maior} | Menor: {menor}");
    println!("Frequência: {frequencia:.1}%");
    println!("Média suficiente? {n_ok}");
    println!("Frequência suficiente? {f_ok}");
    println!("Situação: {}", if aprovado { "APROVADO" } else { "REPROVADO" });
}