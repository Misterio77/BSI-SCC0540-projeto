BEGIN;

DROP TABLE IF EXISTS doacao, apoio, pleito, candidatura, julgamento, processo, cargo, partido, individuo;
DROP TYPE IF EXISTS tipo_cargo;

-- Representa um indivíduo
CREATE TABLE individuo (
    cpfcnpj VARCHAR NOT NULL,
    nome VARCHAR NOT NULL,
    nascimento DATE NOT NULL,

    CONSTRAINT individuo_pk PRIMARY KEY (cpfcnpj),

    -- CPFs têm 11 dígitos, e CNPJs têm 14. Ambos devem ser numéricos
    CONSTRAINT individuo_ck_cpfcnpj CHECK (cpfcnpj SIMILAR TO '[0-9]{11}' OR cpfcnpj SIMILAR TO '[0-9]{14}')
);


-- Representa um partido
CREATE TABLE partido (
    numero SMALLINT NOT NULL,
    nome VARCHAR NOT NULL,
    programa VARCHAR NOT NULL,

    CONSTRAINT partido_pk PRIMARY KEY (numero),
    CONSTRAINT partido_un UNIQUE (nome),

    CONSTRAINT partido_numero CHECK (numero >= 10)
);


-- Enumera os possíveis tipos de cargo
CREATE TYPE tipo_cargo AS ENUM (
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
    tipo tipo_cargo NOT NULL,
    local VARCHAR NOT NULL,
    cadeiras SMALLINT NOT NULL,
    salario NUMERIC NOT NULL,

    CONSTRAINT cargo_pk PRIMARY KEY (tipo, local),

    CONSTRAINT cargo_ck_cadeiras CHECK (cadeiras > 0),
    CONSTRAINT cargo_ck_salario CHECK (salario > 0)
);


-- Representa um processo judicial
CREATE SEQUENCE processo_id_seq;
CREATE TABLE processo (
    id INTEGER NOT NULL DEFAULT nextval('processo_id_seq'),
    reu VARCHAR NOT NULL,
    crime VARCHAR NOT NULL,

    CONSTRAINT processo_pk PRIMARY KEY (id),

    CONSTRAINT processo_fk
        FOREIGN KEY (reu)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE
);
ALTER SEQUENCE processo_id_seq OWNED BY processo.id;

-- Representa o julgamento de um processo
CREATE TABLE julgamento (
    processo INTEGER NOT NULL,
    -- Qual órgão judiciário julgou
    instancia VARCHAR NOT NULL,
    data DATE NOT NULL,
    procedente BOOLEAN NOT NULL,

    -- Cada processo só pode ser julgado uma vez no órgão, podendo recorrer para outro órgão superior
    CONSTRAINT julgamento_pk PRIMARY KEY (processo, instancia)
);


-- Representa uma candidatura
-- TODO: só permitir adicionar candidatura caso no ano da eleição seja ficha limpa
CREATE TABLE candidatura (
    candidato VARCHAR NOT NULL,
    vice_candidato VARCHAR DEFAULT NULL,
    ano SMALLINT NOT NULL,
    cargo_tipo tipo_cargo NOT NULL,
    cargo_local VARCHAR NOT NULL,
    numero INTEGER NOT NULL,
    partido SMALLINT NOT NULL,

    -- Cada pessoa só pode ser candidata a um cargo por eleição
    CONSTRAINT candidatura_pk PRIMARY KEY (candidato, ano),

    -- Garante que, para cada ano, a pessoa só é vice de uma candidatura
    CONSTRAINT candidatura_un_vice_ano
        UNIQUE (vice_candidato, ano),
    -- Garante que, para cada ano e cargo, só existe 1 candidatura com dado número
    CONSTRAINT candidatura_un_numero_ano_cargo
        UNIQUE (cargo_tipo, cargo_local, numero, ano),

    -- Verifica que os dois primeiros dígitos do número representam o partido
    CONSTRAINT candidatura_ck_partido
    CHECK ((LEFT((numero::VARCHAR), 2)::INTEGER ) = partido),

    -- Verifica que o número tem o numero de digitos correto
    CONSTRAINT candidatura_ck_numero
        CHECK (CASE
            WHEN (cargo_tipo = 'Vereador' OR cargo_tipo = 'DeputadoEstadual')
                -- 5 dígitos
                THEN (FLOOR(LOG(numero)+1) = 5)
            WHEN (cargo_tipo = 'DeputadoFederal') THEN
                -- 4 dígitos
                (FLOOR(LOG(numero)+1) = 4)
            WHEN (cargo_tipo = 'Senador') THEN
                -- 3 dígitos
                (FLOOR(LOG(numero)+1) = 3)
            ELSE
                -- 2 dígitos
                (FLOOR(LOG(numero)+1) = 2)
        END),

    -- Verifica que o candidato é pessoa física
    CONSTRAINT candidatura_ck_candidato CHECK (candidato SIMILAR TO '[0-9]{11}'),

    -- Verifica se a candidatura cumpre requisito de vice do cargo
    -- E também que o candidato e vice são distintos
    CONSTRAINT candidatura_ck_vice_candidato
        CHECK (CASE
            -- Cargos executives requerem um vice na chapa
            WHEN (cargo_tipo = 'Presidente' OR cargo_tipo = 'Governador' OR cargo_tipo = 'Prefeito')
                THEN (vice_candidato IS NOT NULL)
            -- Legislativos não
            ELSE (vice_candidato IS NULL)
        END
        AND vice_candidato != candidato),

    CONSTRAINT candidatura_fk_candidato
        FOREIGN KEY (candidato)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT candidatura_fk_vice_candidato
        FOREIGN KEY (vice_candidato)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT candidatura_fk_cargo
        FOREIGN KEY (cargo_tipo, cargo_local)
        REFERENCES cargo (tipo, local)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT candidatura_fk_partido
        FOREIGN KEY (partido)
        REFERENCES partido (numero)
        ON DELETE CASCADE ON UPDATE CASCADE
);


-- Representa um pleito
CREATE TABLE pleito (
    candidato VARCHAR NOT NULL,
    ano SMALLINT NOT NULL,
    turno SMALLINT NOT NULL DEFAULT 1,
    votos INTEGER NOT NULL,

    CONSTRAINT pleito_pk PRIMARY KEY (candidato, ano, turno),

    CONSTRAINT pleito_ck_turno CHECK (turno > 0),
    CONSTRAINT pleito_ck_votos CHECK (votos >= 0),

    CONSTRAINT pleito_fk_candidatura
        FOREIGN KEY (candidato, ano)
        REFERENCES candidatura (candidato, ano)
        ON DELETE CASCADE ON UPDATE CASCADE
);


-- Representa cada apoiador da campanha
CREATE TABLE apoio (
    apoiador VARCHAR NOT NULL,
    candidato VARCHAR NOT NULL,
    ano SMALLINT NOT NULL,
    funcao VARCHAR NOT NULL,

    CONSTRAINT apoio_pk PRIMARY KEY (apoiador, ano),

    CONSTRAINT apoio_fk_apoiador
        FOREIGN KEY (apoiador)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT apoio_fk_candidatura
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
    CONSTRAINT doacao_ck_valor CHECK (valor > 0),

    CONSTRAINT doacao_fk_doador
        FOREIGN KEY (doador)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT doacao_fk_candidatura
        FOREIGN KEY (candidato, ano)
        REFERENCES candidatura (candidato, ano)
        ON DELETE CASCADE ON UPDATE CASCADE

);
ALTER SEQUENCE doacao_id_seq OWNED BY doacao.id;
-- 1 doação por indivíduo com cnpj por candidatura
CREATE UNIQUE INDEX doacao_un_juridica ON doacao (doador, candidato, ano) WHERE (doador SIMILAR TO '[0-9]{14}');

COMMIT;
