pub trait Autenticavel {
    fn login(&mut self, usuario: String, senha: String) -> bool;
    fn logout(&mut self) -> bool;
}