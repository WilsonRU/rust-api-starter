## Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install)
- [Diesel ORM](https://diesel.rs/)
- [Rocket](https://rocket.rs/)
- [Docker](https://docs.docker.com/install/)

## Recursos

- JWT Authentication
- ORM ( Diesel )

## Configurando

- Instale as dependências necessárias
    ```bash
        cargo build
    ```

- Instale o Diesel CLI para rodar as migration
    ```bash
        cargo install diesel_cli --no-default-features --features postgres
    ```

- Execute as migrations
    ```bash
        diesel migration run
    ```

## Executando a Aplicação

- Para subir o container do PostgreSQL em Ambiente de Desenvolvimento
    ```bash
        docker-compose up -d
    ```

- Levando o servidor de desenvolvimento
    ```bash
        cargo run
    ```

- Criando uma nova migration
    ```bash
        diesel migration generate nome_da_migration
    ```
    
- Rodando os testes
    ```bash
        cargo test
    ```

- Buildar Release para Produção
    ```bash
        cargo build --release
    ```
