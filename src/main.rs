use std::io;
fn main() {
    loop {
        imc();
        println!("Deseja recalcular? (S/N)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Falha ao ler a resposta");
        let answer: char = answer.trim().parse().expect("Falha ao converter a resposta para char");
        if answer == 'N' || answer == 'n' {
            break
        }
    }
    
}

fn imc(){
    println!("Olá, bem-vindo ao imc calculator!");
    
    println!("Digite seu peso:");
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("Falha ao ler o peso");
    let weight: f32 = weight.trim().replace(",", ".").parse().expect("Falha ao converter o peso para f32");
    
    println!("Digite sua altura:");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Falha ao ler a altura");
    let height: f32 = height.trim().replace(",", ".").parse().expect("Falha ao converter a altura para f32");
    
    let imc = get_imc(weight, height);
    let imc_indicator = imc_indicator(imc);
    
    println!("Seu IMC é: {imc}");
    println!("Voce esta: {imc_indicator}");
}

fn get_imc(weight: f32, height: f32) -> f32 {
    let imc = weight / (height * height);
    imc
}

fn imc_indicator(imc:f32)->String{

    match imc {
        i if i < 18.5 => String::from("Abaixo do peso"),
        i if i < 25.0 => String::from("Peso normal"),
        i if i < 30.0 => String::from("Sobrepeso"),
        i if i < 35.0 => String::from("Obesidade grau 1"),
        i if i < 40.0 => String::from("Obesidade grau 2"),
        _ => String::from("Obesidade grau 3"),
    }

}