# Sistema de Busca Otimizado para Catálogo de Produtos – MegaStore

## Descrição do projeto
Sistema de **busca** e **recomendação** para catálogos de produtos, com foco em rapidez e baixo consumo de recursos. A busca utiliza **tabelas hash** para indexação de nomes e recuperação por id; as recomendações são geradas a partir de um **grafo de similaridade** entre produtos (categoria/tags) com travessia limitada por profundidade.

## Tecnologias utilizadas
- **Linguagem:** Rust (edição 2021)
- **Crates:** `serde`, `serde_json`, `clap` (CLI), `anyhow`
- **Testes:** `cargo test` (unitários e integração)

## Como executar o sistema de busca
1. Instale o Rust (via `rustup`).

2. No diretório do projeto:
    ```bash
    cargo run
    cargo run --release
    ```
3. Executar
    ```bash
    cargo run -- --query "gaming keyboard" --id 3 --k 3
    ```
    ```bash
    cargo run -- --input products.json --query "smartphone samsung" --id 42 --k 5
    ```

### Parâmetros de CLI
- `--input <arquivo>`: JSON com a lista de produtos (opcional; se ausente, usa dados de exemplo).
- `--query <texto>`: termo de busca (case-insensitive).
- `--id <n>`: id do produto de referência para recomendações.
- `--depth <n>`: profundidade máxima de travessia no grafo (>= 1).

## Como executar os testes
```bash
cargo test
cargo test nome_do_teste
```

## Exemplos de uso
- Busca simples:
  ```bash
  ./target/release/megastore_search --query "wireless"
  ```
- Recomendação com JSON externo:
  ```bash
  ./target/release/megastore_search --input products.json --id 1 --depth 2
  ```
- Busca + recomendação:
  ```bash
  ./target/release/megastore_search --input products.json --query "pro" --id 1 --depth 1
  ```

## Arquitetura do sistema
- **Biblioteca (`src/lib.rs`)**
  - `Product`: id, name, category, tags.
  - `ProductGraph`: grafo não direcionado de similaridade.
  - `Catalogue`: índice em memória com:
    - `HashMap<ProductId, Product>` para acesso por id
    - `HashMap<String, HashSet<ProductId>>` para índice de tokens de nome
    - construção/consulta do grafo para recomendações
- **Binário (`src/main.rs`)**
  - CLI com `clap`, leitura opcional de `products.json`, execução de busca e recomendações.
- **Testes (`tests/test_catalogue.rs`)**
  - Verificam busca por termo e recomendação por similaridade.

## Algoritmos e estruturas de dados utilizados
- **Tabelas hash (`HashMap`)** para:
  - índice por id
  - índice de tokens de nome (tokenização simples e normalização para minúsculas)
- **Grafo de similaridade**:
  - vértices: `ProductId`
  - arestas: produtos que compartilham categoria ou tags
  - recomendação via **BFS** limitada por `depth`, retornando vizinhos alcançáveis
- **Busca**:
  - consulta por termo → normalização → lookup no índice de tokens → coleta de produtos

## Considerações sobre desempenho e escalabilidade
- **Tempo médio de acesso** em índices hash é próximo de **O(1)**, garantindo latência estável em catálogos grandes.
- **Recomendação** com **BFS limitada**: custo proporcional ao tamanho da fronteira por nível; limitar `depth` (1–2) mantém resposta rápida.
- **Escalabilidade**:
  - particionamento do índice por prefixo de token
  - particionamento do grafo por categoria
  - cache de vizinhanças populares
- **Diretrizes de benchmark**:
  - gerar datasets sintéticos (10k, 100k, 1M produtos)
  - medir: construção do índice/grafo, buscas de termos comuns/raros, recomendações com `depth` 1–3
  - usar `cargo run --release` e/ou `hyperfine` para medir o binário; considerar `criterion` para benchmarks formais

## Contribuições
- Abra *issues* com casos reais (múltiplos termos, filtros por categoria, pesos de aresta).
- Envie *PRs* com testes cobrindo novas rotas do grafo, mantendo SRP/SoC.
- Use `clippy` e `rustfmt` antes de abrir PR.

## Licença
Distribuído sob a **MIT License**. Adicione o arquivo `LICENSE` ao repositório conforme o texto padrão da licença MIT.

---

### Exemplo de `products.json`
```json
[
  { "id": 1, "name": "Smartphone X", "category": "Electronics", "tags": ["phone","camera"] },
  { "id": 2, "name": "Laptop Pro 15", "category": "Computers", "tags": ["laptop","pro"] },
  { "id": 3, "name": "USB-C Charger", "category": "Accessories", "tags": ["charger","usb-c"] },
  { "id": 4, "name": "Wireless Earbuds", "category": "Audio", "tags": ["earbuds","wireless"] }
]
```