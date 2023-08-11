use chrono;
use std::fs::File;
use std::io;
use std::io::prelude::*;
fn main() {
    loop {
        println!("Olá, bem-vindo ao imc calculator!");
        imc();
        println!("Deseja recalcular? (S/N)");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Falha ao ler a resposta");
        let answer: char = answer
            .trim()
            .parse()
            .expect("Falha ao converter a resposta para char");
        if answer == 'N' || answer == 'n' {
            break;
        }
    }
}

fn imc() {
    println!("Digite seu nome:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Falha ao ler o peso");
    let name = name.trim();

    println!("Digite seu peso:");
    let mut weight = String::new();
    io::stdin()
        .read_line(&mut weight)
        .expect("Falha ao ler o peso");
    let weight: f32 = weight
        .trim()
        .replace(",", ".")
        .parse()
        .expect("Falha ao converter o peso para f32");

    println!("Digite sua altura:");
    let mut height = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("Falha ao ler a altura");
    let height: f32 = height
        .trim()
        .replace(",", ".")
        .parse()
        .expect("Falha ao converter a altura para f32");

    let imc = get_imc(weight, height);
    let imc_indicator = imc_indicator(imc);

    println!("{name}, segue os resultados:");
    println!("Seu IMC é: {imc}");
    println!("Voce esta: {imc_indicator}");
    write_txt_file(name.to_string(), weight, height, imc, imc_indicator);
}

fn get_imc(weight: f32, height: f32) -> f32 {
    let imc = weight / (height * height);
    imc
}

fn imc_indicator(imc: f32) -> String {
    match imc {
        i if i < 18.5 => String::from("Abaixo do peso"),
        i if i < 25.0 => String::from("Peso normal"),
        i if i < 30.0 => String::from("Sobrepeso"),
        i if i < 35.0 => String::from("Obesidade grau 1"),
        i if i < 40.0 => String::from("Obesidade grau 2"),
        _ => String::from("Obesidade grau 3"),
    }
}

fn write_txt_file(name: String, wight: f32, height: f32, imc: f32, imc_indicator: String) {
    let now = chrono::offset::Local::now();
    let formatted_date = now.format("%Y-%m-%d_%Hh-%Mm-%Ss");

    let mut file = std::fs::File::create(format!("imc_{}_{}.txt", name, formatted_date))
        .expect("Falha ao criar o arquivo");
    file.write_all(
        format!(
            "Nome: {}\nPeso: {:.2}\nAltura: {:.2}\nIMC: {:.2}\nClassificação: {}\n\n\n\nDate: {}",
            name, wight, height, imc, imc_indicator, formatted_date
        )
        .as_bytes(),
    )
    .expect("Falha ao escrever no arquivo");
}
