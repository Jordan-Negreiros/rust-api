## Projeto Rust com Arquitetura Limpa, Rocket.rs e Diesel.rs

Este é um projeto Rust que utiliza a arquitetura limpa para criar uma aplicação web usando o framework Rocket.rs para o backend e Diesel.rs para interação com um banco de dados PostgreSQL.

### Estrutura do Projeto

O projeto está organizado seguindo os princípios da arquitetura limpa, com divisão em camadas:

- **api:** Contém as interfaces da aplicação, como a interface web usando Rocket.rs.
- **application:**  Implementa os casos de uso da aplicação.
- **domain:** Contém as entidades e regras de negócio.
- **infrastructure:**  Contém a implementação detalhada de detalhes técnicos, como conexões com o banco de dados.

### Requisitos

- Rust (instalado usando rustup)
- PostgreSQL
- Diesel CLI (`cargo install diesel_cli`)
- Outras dependências do projeto (veja o arquivo `Cargo.toml`)

### Configuração do Banco de Dados

1. Crie um banco de dados PostgreSQL.
2. Configure as variáveis de ambiente no arquivo `.env`:

    ```env
    DATABASE_URL=postgres://seu_usuario:senha@localhost/seu_banco_de_dados
    ```

### Executando o Projeto

1. Execute as migrações do banco de dados usando Diesel:

    ```bash
    diesel migration run
    ```

2. Inicie o servidor Rocket:

    ```bash
    cargo run
    ```

A aplicação estará disponível em [http://localhost:8000](http://localhost:8000).

### Endpoints

- `GET /temperatura/<city>`: Retorna a temperatura max e mínima de uma cidade.
- `POST /cities`: Adiciona uma nova cidade para pesquisa.

  Exemplo de payload:
  ```json
  {
    "name": "Brasília",
    "state": "DF",
    "country": "BR",
    "latitude": -15.7797,
    "longitude": -47.9297
  }
  ```


## Projeto de estudo, Ainda em Construção !!!