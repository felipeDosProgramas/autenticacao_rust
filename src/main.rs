use std::io::{stdin, Stdin};
use crate::autenticavel::Autenticavel;
use crate::sistema_de_seguranca::SistemaDeSeguranca;

mod sistema_de_seguranca;
mod autenticavel;

fn get_entrada(buffer: &Stdin) -> String {
    let mut str_entrada = String::new();
    let _ = buffer.read_line(&mut str_entrada)
        .expect("erro ao ler o terminal");
    str_entrada.trim().to_string()
}
fn get_dados_login(buffer: &Stdin) -> Option<(String, String)> {
    println!("insira seu login e sua senha, 'q' para cancelar");
    let primeira_entrada = get_entrada(&buffer);
    if primeira_entrada.eq_ignore_ascii_case("q") {
        None
    } else {
        Some((primeira_entrada, get_entrada(&buffer)))
    }
}
fn menu_interno(sti: &Stdin){
    println!("para sair, insira a letra 's'");
    if get_entrada(&sti).eq_ignore_ascii_case("s")
        { println!("até mais!"); }
    else
        { menu_interno(sti) }
}
fn main() {
    let mut sds = SistemaDeSeguranca::new();
    let sti = stdin();
    if let Some((l, s)) = get_dados_login(&sti){
        if sds.login(l,s) {
            println!("bem vindo!");
            menu_interno(&sti);
        }
        else {
            println!("usuário incorreto!");
            return main();
        }
        println!("até mais!");
        return;
    }
}
