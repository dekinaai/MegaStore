# Sistema de Busca e Recomendação - MegaStore

**Título:** Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição
Projeto didático em Rust que implementa:  
- Busca simples e eficiente em catálogo de produtos usando *HashMap* para indexação.  
- Sistema de recomendações baseado em **grafos** (produtos conectados por relações de co-compra/similaridade).  
- Estrutura de projeto pronta para testes e documentação.  

## Estrutura do Repositório
```
src/
  main.rs
  lib.rs
  data.rs
  search.rs
  graph.rs
tests/
  integration_test.rs
Cargo.toml
README.md
DOCS.pdf
```

## Tecnologias utilizadas
- Rust (cargo)  
- Crates: `serde`, `serde_json`, `petgraph`  
- Ferramentas: `cargo test`, `cargo run`  

## Como executar
1. Instale o Rust (https://rustup.rs).  
2. No diretório do projeto, rode:  
```
cargo build
cargo run --release
```
O binário executará uma demo: carregará produtos de exemplo, permitirá buscas por nome/marca/categoria e mostrará recomendações por grafo.

## Como executar os testes
```
cargo test
```

## Exemplos de uso
- Buscar produtos:
  - `search smartphone`  
- Buscar por marca:
  - `search MegaPhone`  
- Recomendar produtos:
  - `recommend 1 5`  

## Arquitetura
- `data.rs`: Estruturas e carregamento de produtos.  
- `search.rs`: Indexação (HashMap) e busca.  
- `graph.rs`: Funções auxiliares de grafo.  
- `main.rs`: CLI interativa.  
- `integration_test.rs`: Testes de integração.  

## Licença
MIT
