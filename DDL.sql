BEGIN;

DROP TABLE IF EXISTS doacao, membro_equipe, candidatura, processo, cargo, partido, individuo;
DROP TYPE IF EXISTS tipo_cargo;

-- Representa um indivíduo
CREATE TABLE individuo (
    cpfcnpj VARCHAR NOT NULL,
    nome VARCHAR NOT NULL,
    nascimento DATE NOT NULL,
    -- TODO: trigger para manter atualizado
    ficha_limpa BOOLEAN NOT NULL DEFAULT true,

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
    cargo_tipo tipo_cargo NOT NULL,
    cargo_local VARCHAR NOT NULL,
    numero INTEGER NOT NULL,
    partido SMALLINT NOT NULL,
    votos INTEGER DEFAULT NULL,

    CONSTRAINT candidatura_pk PRIMARY KEY (candidato, ano),

    -- Garante que, para cada ano, a pessoa só é vice de uma candidatura
    CONSTRAINT candidatura_un_vice_ano
        UNIQUE (vice_candidato, ano),
    -- Garante que, para cada ano e cargo, só existe 1 candidatura com dado número
    CONSTRAINT candidatura_un_numero_ano_cargo
        UNIQUE (cargo_tipo, cargo_local, numero, ano),

    -- Verifica que o número na urna é positivo, e que o numero de digitos é correto
    CONSTRAINT candidatura_ck_numero
        CHECK (numero > 0 AND
            CASE
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
            END
        ),

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

    -- Verifica que votos não são negativos
    CONSTRAINT candidatura_ck_votos
        CHECK (votos >= 0),

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


-- Representa cada membro da equipe
CREATE TABLE membro_equipe (
    membro VARCHAR NOT NULL,
    candidato VARCHAR NOT NULL,
    ano SMALLINT NOT NULL,

    CONSTRAINT membro_equipe_pk PRIMARY KEY (membro, ano),

    CONSTRAINT membro_equipe_fk_membro
        FOREIGN KEY (membro)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT membro_equipe_fk_candidato
        FOREIGN KEY (candidato)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT membro_equipe_fk_candidatura
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

    CONSTRAINT doacao_fk_candidato
        FOREIGN KEY (candidato)
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
