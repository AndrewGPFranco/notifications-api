/// Handlers recebem a requisição HTTP e montam a resposta.
///
/// Em Java, são parecidos com métodos de um `@RestController`.
pub async fn index() -> &'static str {
    "Home"
}

pub async fn about() -> &'static str {
    "About"
}
