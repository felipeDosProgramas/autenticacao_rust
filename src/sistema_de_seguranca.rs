use crate::autenticavel::Autenticavel;

static DADOS_CORRETOS: (&str, &str) = ("adm", "s");
pub struct SistemaDeSeguranca {
    autenticado: bool
}
impl SistemaDeSeguranca {
    pub fn new() -> Self {
        SistemaDeSeguranca{
            autenticado: false
        }
    }
}
impl Autenticavel for SistemaDeSeguranca {
    fn login(&mut self, usuario: String, senha: String) -> bool {
        self.autenticado = (usuario.as_str(), senha.as_str()) == DADOS_CORRETOS;
        self.autenticado
    }

    fn logout(&mut self) -> bool {
        if self.autenticado {
            self.autenticado = false;
            true
        } else
            { false }
    }
}