fn main() {
    let mut idd: i32 = 20;
    //println!("Tenho {} anos de idade.", idd);
    idd = 21;
    //println!("Eu irei fazer {} anos de idade em 4 de Janeiro.", idd);

    // Shadowing
    let x: i32 = 5;
    let x = x + 1;
    let x = x* 2;
    //println!("x = {}", x);

    // Shadowing
    let espacos = "   ";
    let espacos = espacos.len();
    //println!("espacos = {}", espacos);

    // Constante
    const PI_APROX: f64 = 3.14159;
    //println!("PI = {}", PI_APROX);

    // Tipos primitivos
    let inteiro: i64 = 10_000_000_000;
    let sem_sinal: u8 = 255;
    let decimal: f64 = 2.5;
    let verdadeiro: bool = true;
    let letra: char = 'R';

    //println!("{} {} {} {} {}", inteiro, sem_sinal, decimal, verdadeiro, letra);

    // Formatação com println!
    let nome = "Erick";
    let nota = 9.5;

    println!("Nome: {}, Nota: {}", nome, nota);
    println!("Nome: {nome}, Nota: {nota}");
    println!("Nota formatada: {:.1}", nota);
    println!("Debug: {:?}", nome);
    println!("Largura: [{:>8}]", nome);  // alinhado a direita (< esquerda, ^ centro)
}
