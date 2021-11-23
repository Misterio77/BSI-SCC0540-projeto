# Projeto BD

Todo o código do projeto está disponível no [meu repositório](https://git.sr.ht/~misterio/BSI-SCC0540-projeto)

Caso queira experimentar, existe uma versão live rodando na minha Raspberry Pi, [disponível publicamente](https://bd.misterio.me) (nota: caso tenha problemas em acessar, certifique-se de que está conectando por HTTPS e não HTTP)

## Ferramentas usadas
- Banco de dados: PostgreSQL (Versão 13)
- Linguagem: Rust
- Framework: [Rocket](https://github.com/SergioBenitez/Rocket/tree/master)
- Templating Engine: [Tera](https://github.com/Keats/tera)

## Como rodar

### Compilar

Compile com `cargo build --release`. Caso queira compilar no modo desenvolvimento (compila mais rápido, mas é menos otimizado em tamanho do binário), use apenas `cargo build`.

### Banco de dados

Crie um banco de dados no PostgreSQL (de preferência a versão 13, mas a 12 deve funcionar também), e configure um usuário corretamente para ter acesso à ele.

Feito isso, execute `DDL.sql` para criar a schema, e `DML.sql` para preencher com dados fictícios que geramos.

Você pode fazer isso usando o `psql`:
```
cat DDL.sql DML.sql | psql
```

### Configuração

Edite o arquivo `Rocket.toml`, colocando em `url` (na seção `default.databases.database`) a string de conexão que você usará para conectar ao seu banco. A padrão usa um socket unix com meu usuário (misterio), mas você pode usar TCP facilmente também. [Mais info sobre connection string](https://stackoverflow.com/questions/3582552).

Caso queira, você pode alterar a porta adicionando `port = 1234` (na seção `default`), ou IP adicionando `address`.

Caso você rode a partir de uma pasta que não seja a raiz do projeto, lembre de levar as pastas `assets` e `templates` junto (estas contem o CSS e os templates HTML), ou altere as opções `template_dir` e `assets_dir`.

Você também pode alterar essas opções usando variáveis de ambiente (exemplo: `ROCKET_PORT`), caso prefira.

### Executar

Basta usar `cargo run --release` (ou `cargo run` para modo desenvolvimento).

A aplicação deve ficar acessível no IP e porta configurados, (padrão é `http://127.0.0.1:8000`).

## Estrutura do projeto

A aplicação é dividida em três camadas lógicas:

### Modelo

Localizado em `src/schema`.

Modela classes de acordo com o banco de dados.

Cada entidade tem sua `struct`, métodos para `obter` a entidade partindo da sua chave, `listar` (esses recebem um filtro, e às vezes podem se tornar queries especiais), e a `remover` do banco. Inclui também um filtro para listagem personalizada, bem como ordenação.

### Rotas

Localizadas em `src/routes`.

Define as rotas que serão expostas no servidor. Essas consomem as classes do modelo.

Cada entidade tem uma rota `get`, uma rota `list`, e uma rota delete.

As funções de rota especificam qual o caminho (URI) ela estará disponível, e quais os parâmetros pedidos ao usuário (exemplo: `/individuos/<cpfcnpj>`), e o que respondem (exemplo: Template renderizado (tela HTML com contexto passado à ela), ou redirecionamento (no caso do delete)).


### Templates

Localizados em `templates`.

Os templates são arquivos HTML definindo o layout de cada tela, de forma parametrizada (a serem preenchidos pela rota que o renderiza). A templating engine que usamos é o [Tera](https://github.com/Keats/tera).

Temos basicamente os templates para tela de listagem (esses tem nome no plural, como `candidaturas.html.tera`), e para a tela de informação sobre 1 fileira (nome no singular, como `candidatura.html.tera`). Ambos localizados em `templates/routes`.

As telas de informação são simples. Apenas contém elementos para exibir informações sobre a entidade obtida. As chaves estrangeiras são exibidas como links para outras entidades. Também há links para listar entidades que referenciam a entidade sendo vista.

As telas de listagem são um pouco mais complexas. Elas contém uma tabela, que renderiza cada resultado como uma fileira. Além disso, há um formulário utilizado para definir filtragem e ordenação.

Também temos alguns outros templates:

- `base.html.tera` - barra de navegação, rodapé, e meta-dados, usado por todos os outros
- `error.html.tera` - exibe um erro que pode ter ocorrido
- `flash.html.tera` - aviso que aparece após um redirecionamento (por exemplo, remoção)
- `paginas.html.tera` - botões para navegar em páginas de listagem

### Outros arquivos fonte

O Rocket, framework que usamos, é bastante barebones e minimalista. Não contendo facilidades prontas encontradas comumente em frameworks maiores (como o Django). Então algumas funcionalidades nescessárias para manter o modelo e as rotas limpas foram implementadas por nós e colocadas na pasta `src/common`. Essas incluem:

- `error.rs` - Erro personalizado, que facilita levantar "exceções" sem muito código adicional.
- `pagination.rs` - Permite determinar a página atual, e cria link baseado nas adjacentes, sem alterar as outras query strings presentes.
- `assets.rs` - Implementa um fairing (middleware) que permite servir assets estáticos (como CSS) com uma política de cachê.
- `database.rs` - Boilerplate para se conectar ao banco de forma assíncrona utilizando o `tokio-postgres`.
- `post_as_delete.rs` - Como formulários HTML só suportam GET e POST, esse fairing permite reescrever requests do tipo POST `blabla/delete` em DELETE `blabla`.
