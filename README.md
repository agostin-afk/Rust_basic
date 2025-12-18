# Rust Basic
![GitHub repo size](https://img.shields.io/github/repo-size/agostin-afk/Rust_basic?style=for-the-badge)
![GitHub language count](https://img.shields.io/github/languages/count/agostin-afk/Rust_basic?style=for-the-badge)
![GitHub forks](https://img.shields.io/github/forks/agostin-afk/Rust_basic?style=for-the-badge)
![Bitbucket open issues](https://img.shields.io/bitbucket/issues/agostin-afk/Rust_basic?style=for-the-badge)
![Bitbucket open pull requests](https://img.shields.io/bitbucket/pr-raw/agostin-afk/Rust_basic?style=for-the-badge)
> Repositório de estudos e exemplos da linguagem Rust em níveis básico, intermediário e avançado

## 🚀 Funcionalidades
- 📚 **Básico**: Variáveis, tipos de dados, operadores, controle de fluxo
- 🔄 **Intermediário**: Ownership, borrowing, structs, enums, funções avançadas
- 🏗️ **Avançado**: Traits, generics, lifetimes, concurrency, macros
- 🧪 **Testes**: Exemplos de testes unitários com Cargo
- ⚡ **Projetos**: Exemplos práticos e padrões de Rust

## ⚙️ Requisitos
- [Rust](https://www.rust-lang.org/tools/install) (instalado via rustup)
- Conhecimento básico de programação (recomendado)

## 🛠️ Instalação e Execução
1. Clone o repositório:
```bash
git clone https://github.com/agostin-afk/Rust_basic.git
cd Rust_basic
```
2. Para projetos Cargo (se aplicável):
```bash
cargo run --bin exemplo
cargo test
```
3. Para arquivos únicos:
```bash
rustc exemplo.rs
./exemplo
```
Explore por pastas: basic/, intermediary/, advanced/

## 💻 Exemplos de Código
### Variáveis e Tipos Básicos
```rust
fn main() {
    let nome = "Rust"; // String imutável
    let idade: u8 = 25;
    let mut saldo: f32 = 100.50; // Mutável
    saldo += 10.0;
    println!("Nome: {}, Idade: {}, Saldo: {:.2}", nome, idade, saldo);
}
```

### Controle de Fluxo
```rust
fn main() {
    for i in 1..6 {
        print!("{} ", i); // 1 2 3 4 5
    }
    let mut i = 0;
    while i < 5 {
        print!("{} ", i);
        i += 1;
    }
}
```

### Funções
```rust
fn fatorial(n: u32) -> u32 {
    if n <= 1 { 1 } else { n * fatorial(n - 1) }
}
fn main() {
    let res = fatorial(5); // 120
    println!("{}", res);
}
```

### Structs
```rust
struct Pessoa {
    nome: String,
    idade: u8,
}
impl Pessoa {
    fn apresentar(&self) {
        println!("Olá, sou {} e tenho {} anos", self.nome, self.idade);
    }
}
fn main() {
    let p = Pessoa { nome: "Ana".to_string(), idade: 30 };
    p.apresentar();
}
```

## 🧪 Testes Unitários
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn soma_basica() {
        assert_eq!(soma(2, 3), 5);
    }
}
fn soma(a: i32, b: i32) -> i32 { a + b }
```
Execute com: `cargo test` ou `rustc --test arquivo.rs && ./arquivo`

## 📁 Estrutura do Projeto
```
Rust_basic/
├── basic/        # Exemplos básicos: variáveis, loops, funções simples
├── intermediary/ # Exemplos intermediários: ownership, structs, enums
├── advanced/     # Exemplos avançados: traits, concurrency, macros
├── LICENSE       # Licença do projeto (se existir)
└── README.md     # Este arquivo
```

## 🧑‍💻 Tecnologias Utilizadas
- **Rust**: Linguagem segura, concorrente e performática
- **Cargo**: Gerenciador de pacotes e build
- **Testes Nativos**: Suporte integrado a testes

## 💡 Dicas para Estudo
1. Comece pela pasta basic/
2. Avance para ownership e borrowing em intermediary/
3. Explore concurrency em advanced/
4. Execute e modifique os exemplos
5. Use `cargo test` para praticar testes

## 📌 Observações
- Focado em exemplos didáticos para aprendizes de Rust
- Exemplos executáveis com Rust estável
- Para dúvidas, abra uma issue

## 🤝 Como Contribuir
1. Fork o repositório
2. Crie branch: `git checkout -b minha-feature`
3. Adicione exemplos comentados
4. Teste: `cargo build` ou `rustc`
5. Push e abra PR

---
**Nota**: Consulte a [documentação oficial do Rust](https://doc.rust-lang.org/book/) para detalhes atualizados.
