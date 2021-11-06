BEGIN;

DROP TABLE IF EXISTS doacao, membro_equipe, candidatura, processo, cargo, partido, individuo;
DROP TYPE IF EXISTS TIPO_CARGO;

-- Tabelas --

-- Representa um indivíduo
CREATE TABLE individuo (
    cpfcnpj VARCHAR NOT NULL,
    nome VARCHAR NOT NULL,
    nascimento DATE NOT NULL,
    ficha_limpa BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT individuo_pk PRIMARY KEY (cpfcnpj)
);


-- Representa um partido
CREATE TABLE partido (
    numero SMALLINT NOT NULL,
    nome VARCHAR NOT NULL,
    programa VARCHAR NOT NULL,

    CONSTRAINT partido_pk PRIMARY KEY (numero),
    CONSTRAINT partido_un UNIQUE (nome),
    CONSTRAINT partido_ck CHECK (numero >= 10)
);


-- Enumera os possíveis tipos de cargo
CREATE TYPE TIPO_CARGO AS ENUM (
    -- Esses tem vice
    'Prefeito',
    'Governador',
    'Presidente',
    -- Esses não tem vice
    'Vereador',
    'DeputadoEstadual',
    'DeputadoFederal',
    'Senador'
);
-- Representa um cargo a ser pleiteado
CREATE TABLE cargo (
    tipo TIPO_CARGO NOT NULL,
    local VARCHAR NOT NULL,
    cadeiras SMALLINT NOT NULL DEFAULT 1,
    salario NUMERIC NOT NULL,

    CONSTRAINT cargo_pk PRIMARY KEY (tipo, local),
    CONSTRAINT cargo_ck1 CHECK (cadeiras > 0),
    CONSTRAINT cargo_ck2 CHECK (salario > 0)
);


-- Representa um processo judicial
CREATE SEQUENCE processo_id_seq;
CREATE TABLE processo (
    id INTEGER NOT NULL DEFAULT nextval('processo_id_seq'),
    reu VARCHAR NOT NULL,
    crime VARCHAR NOT NULL,
    julgado BOOLEAN NOT NULL DEFAULT false,
    data_julgamento DATE,
    procedente BOOLEAN,
    pena VARCHAR,

    CONSTRAINT processo_pk PRIMARY KEY (id),

    CONSTRAINT processo_fk
        FOREIGN KEY (reu)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    -- Caso o processo já tenha sido julgado,
    -- ele PRECISA ter data_julgamento e procedente NOT NULLs.
    CONSTRAINT processo_ck_data_e_procedente CHECK (
        julgado = false OR (
            data_julgamento IS NOT NULL AND
            procedente IS NOT NULL
        )
    ),
    -- Caso o processo tenha procedente culpado (true)
    -- ele PRECISA ter pena NOT NULL
    CONSTRAINT processo_ck_pena CHECK (
        procedente = false OR pena IS NOT NULL
    )
);
ALTER SEQUENCE processo_id_seq OWNED BY processo.id;


-- Representa uma candidatura
CREATE TABLE candidatura (
    candidato VARCHAR NOT NULL,
    vice_candidato VARCHAR DEFAULT NULL,
    ano SMALLINT NOT NULL,
    cargo_tipo TIPO_CARGO NOT NULL,
    cargo_local VARCHAR NOT NULL,
    numero INTEGER NOT NULL,
    partido SMALLINT NOT NULL,
    votos INTEGER DEFAULT NULL,

    CONSTRAINT candidatura_pk PRIMARY KEY (candidato, ano),

    -- Garante que, para cada ano, a pessoa só é vice de uma candidatura
    CONSTRAINT candidatura_un1
        UNIQUE (vice_candidato, ano),
    -- Garante que, para cada ano e cargo, só existe 1 candidatura com dado número
    CONSTRAINT candidatura_un2
        UNIQUE (cargo_tipo, cargo_local, numero, ano),

    CONSTRAINT candidatura_ck1
        CHECK (votos >= 0),
    CONSTRAINT candidatura_ck2
        CHECK (numero >= 0),

    CONSTRAINT candidatura_fk1
        FOREIGN KEY (candidato)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT candidatura_fk2
        FOREIGN KEY (vice_candidato)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT candidatura_fk3
        FOREIGN KEY (cargo_tipo, cargo_local)
        REFERENCES cargo (tipo, local)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT candidatura_fk4
        FOREIGN KEY (partido)
        REFERENCES partido (numero)
        ON DELETE CASCADE ON UPDATE CASCADE
);


-- Representa cada membro da equipe
CREATE TABLE membro_equipe (
    membro VARCHAR NOT NULL,
    candidato VARCHAR NOT NULL,
    ano SMALLINT NOT NULL,

    CONSTRAINT membro_equipe_pk PRIMARY KEY (membro, ano),

    CONSTRAINT membro_equipe_fk1
        FOREIGN KEY (membro)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT membro_equipe_fk2
        FOREIGN KEY (candidato)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT membro_equipe_fk3
        FOREIGN KEY (candidato, ano)
        REFERENCES candidatura (candidato, ano)
        ON DELETE CASCADE ON UPDATE CASCADE
);


-- Representa as doações de campanha
CREATE SEQUENCE doacao_id_seq;
CREATE TABLE doacao (
    id INTEGER NOT NULL DEFAULT nextval('doacao_id_seq'),
    valor NUMERIC NOT NULL,
    doador VARCHAR NOT NULL,
    candidato VARCHAR NOT NULL,
    ano SMALLINT NOT NULL,

    CONSTRAINT doacao_pk PRIMARY KEY (id),
    CONSTRAINT doacao_ck CHECK (valor > 0),

    CONSTRAINT doacao_fk1
        FOREIGN KEY (doador)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT doacao_fk2
        FOREIGN KEY (candidato)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT doacao_fk3
        FOREIGN KEY (candidato, ano)
        REFERENCES candidatura (candidato, ano)
        ON DELETE CASCADE ON UPDATE CASCADE

);
ALTER SEQUENCE doacao_id_seq OWNED BY doacao.id;

-- Triggers --

COMMIT;
