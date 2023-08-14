# Lite Server

## Descrição

O Lite Server é um projeto em Rust que implementa um servidor web leve e eficiente. Ele foi desenvolvido para fornecer uma solução simples e de alto desempenho para lidar com solicitações HTTP.

## Funcionalidades

O Lite Server possui as seguintes funcionalidades:

- Roteamento de solicitações HTTP para diferentes endpoints.
- Manipulação de solicitações GET, POST, DELETE e PATCH.
- Suporte a parâmetros de consulta (query parameters) e cabeçalhos personalizados.
- Respostas personalizadas para diferentes códigos de status HTTP.
- Servir arquivos estáticos.

## Rotas

O Lite Server suporta as seguintes rotas:

- GET: `/` - Retorna a página inicial.
- GET: `/publisher` - Retorna todos os publishers.
- GET: `/publisher/:id` - Retorna os detalhes de um publisher específico.
- POST: `/publisher` - Cria um novo publisher. (Corpo da solicitação: `{ "name":STRING,"type":U8,"gender":STRING }`)
- DELETE: `/publisher/:id` - Remove um publisher específico.
- PATCH: `/publisher/:id` - Atualiza um publisher específico. (Corpo da solicitação: `{ "name":STRING,"type":U8,"gender":STRING, "active": BOOLEAN }`)
- POST: `/presentations` - Cria uma nova apresentação.  (Corpo da solicitação: `{ "length":U8, "gender":STRING }`)

## Pré-requisitos

Antes de executar o Lite Server, certifique-se de ter as seguintes dependências instaladas:

- Rust (versão 1.70.0): [https://www.rust-lang.org/](https://www.rust-lang.org/)
- SQLite (versão 2.3.0): [https://www.sqlite.org/index.html](https://www.sqlite.org/index.html)

## Instalação

Siga os passos abaixo para executar o Lite Server em sua máquina:

1. Clone este repositório: `git clone https://github.com/seu-usuario/lite-server.git`
2. Acesse o diretório do projeto: `cd lite-server`
3. Execute o comando de compilação: `cargo build`
4. Crie um arquivo `.env` na raiz do projeto.
5. Dentro do arquivo `.env`, defina a variável `DB_PATH` com o caminho do banco de dados SQLite.
Exemplo: `DB_PATH=/caminho/para/o/banco.db`

## Uso

Após a instalação, siga os passos abaixo para utilizar o Lite Server:

1. Execute o comando: `cargo run`
2. O servidor estará em execução na porta 8000 por padrão.
3. Acesse `http://localhost:8000` em seu navegador para interagir com o servidor.

## Contribuição

Contribuições são bem-vindas! Se você deseja contribuir para o projeto Lite Server, siga as etapas abaixo:

1. Fork este repositório.
2. Crie uma nova branch: `git checkout -b minha-branch`
3. Faça as alterações desejadas.
4. Commit suas alterações: `git commit -m 'Adiciona nova funcionalidade'`
5. Push para a branch: `git push origin minha-branch`
6. Abra um pull request.

## Autor

- Nome: Deivid Almeida
- GitHub: [DeividAlmeida](https://github.com/DeividAlmeida)

## Licença

Este projeto é licenciado sob a [MIT License](https://opensource.org/licenses/MIT). Consulte o arquivo `LICENSE` para obter mais informações.
