BEGIN;

-- Limpar tabelas antes de criar
/*
DROP VIEW IF EXISTS candidatura_eleita;
DROP TABLE IF EXISTS doacao, apoio, pleito, candidatura, julgamento, processo, cargo, partido, individuo;
DROP TYPE IF EXISTS tipo_cargo;
DROP FUNCTION IF EXISTS individuo_ficha_suja, check_ficha_limpa;
*/

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

    -- Número eleitoral é chave primária
    CONSTRAINT partido_pk PRIMARY KEY (numero),
    -- Nenhum partido pode ter nome igual, também
    CONSTRAINT partido_un UNIQUE (nome),
    -- E o número eleitoral tem sempre 2 dígitos, começando em 10
    CONSTRAINT partido_numero CHECK (numero >= 10 AND numero <= 99)
);


-- Enumera os possíveis tipos de cargo político
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

    -- Tem como pk uma chave composta do tipo do cargo e o local
    -- onde é disputado (não a esfera!)
    -- Por exemplo, um deputado federal representando o estado de
    -- São Saulo deve ser listado como ('DeputadoFederal', 'São Paulo').
    CONSTRAINT cargo_pk PRIMARY KEY (tipo, local),

    -- As cadeiras e os salários devem ser maiores que 0
    CONSTRAINT cargo_ck_cadeiras CHECK (cadeiras > 0),
    CONSTRAINT cargo_ck_salario CHECK (salario >= 0)
);


-- Representa um processo judicial
CREATE TABLE processo (
    id INTEGER NOT NULL GENERATED ALWAYS AS IDENTITY,
    reu VARCHAR NOT NULL,
    crime VARCHAR NOT NULL,

    CONSTRAINT processo_pk PRIMARY KEY (id),

    CONSTRAINT processo_fk
        FOREIGN KEY (reu)
        REFERENCES individuo (cpfcnpj)
        ON DELETE CASCADE ON UPDATE CASCADE
);

-- Representa o julgamento de um processo
CREATE TABLE julgamento (
    processo INTEGER NOT NULL,
    -- Qual órgão judiciário julgou
    instancia VARCHAR NOT NULL,
    data DATE NOT NULL,
    procedente BOOLEAN NOT NULL,

    -- Cada processo só pode ser julgado uma vez no órgão, podendo recorrer para outro órgão superior
    CONSTRAINT julgamento_pk PRIMARY KEY (processo, instancia),

    CONSTRAINT julgamento_fk
        FOREIGN KEY (processo)
        REFERENCES processo (id)
        ON DELETE CASCADE ON UPDATE CASCADE
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
        UNIQUE (cargo_local, ano, numero),

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
    CONSTRAINT candidatura_ck_candidato_fisico CHECK (candidato SIMILAR TO '[0-9]{11}'),
    CONSTRAINT candidatura_ck_vice_fisico CHECK (candidato SIMILAR TO '[0-9]{11}'),

    -- Verifica se a candidatura cumpre requisito de vice q o cargo pede
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


-- Representa cada apoiador da candidatura
-- Na minha visão, esses são os membros partidários
-- e contratados que ajudam na campanha (podendo incluir
-- empresas).
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
CREATE TABLE doacao (
    id INTEGER NOT NULL GENERATED ALWAYS AS IDENTITY,
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
-- 1 doação por indivíduo com cnpj por candidatura
CREATE UNIQUE INDEX doacao_un_juridica ON doacao (doador, candidato, ano) WHERE (doador SIMILAR TO '[0-9]{14}');


/*
    Views
*/

-- Essa função toma como argumento uma data, e retorna uma "view" de indivíduos
-- que tem ficha suja naquela data (julgamentos procedentes até 5 anos atrás).
CREATE OR REPLACE FUNCTION individuo_ficha_suja(data DATE)
    RETURNS TABLE (cpfcnpj VARCHAR, nome VARCHAR, nascimento DATE)
    LANGUAGE plpgsql AS
$individuo_ficha_suja$
BEGIN
    RETURN QUERY
    SELECT individuo.cpfcnpj, individuo.nome, individuo.nascimento
    FROM individuo
    INNER JOIN processo
        ON processo.reu = individuo.cpfcnpj
    INNER JOIN julgamento
        ON julgamento.processo = processo.id
    WHERE
        julgamento.procedente IS true AND
        julgamento.data <= ($1) AND
        julgamento.data >= ($1 - interval '5 years');
END
$individuo_ficha_suja$;

-- View com apenas os candidatos eleitos
CREATE VIEW candidatura_eleita AS
    SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido
    FROM (
        SELECT
            c.candidato, c.vice_candidato, c.ano, c.cargo_tipo, c.cargo_local, c.numero, c.partido,
            -- Particiona por cargo e ano, e ordena por turno e votos
            -- Adicionando um numero baseado nesses criterios, que poderemos usar
            -- no WHERE para filtrar.
            row_number() OVER(
                PARTITION BY c.cargo_tipo, c.cargo_local, c.ano
                ORDER BY p.turno DESC, p.votos DESC
            ) AS rownum
        FROM
            candidatura c
        INNER JOIN pleito p
            ON p.candidato = c.candidato
            AND p.ano = c.ano
    )
    AS eleicao
    INNER JOIN cargo
        ON cargo.local = eleicao.cargo_local
        AND cargo.tipo = eleicao.cargo_tipo
    WHERE rownum <= cargo.cadeiras;

/*
    Triggers
*/

-- Permite que apenas indivíduos ficha limpa (no ano da candidatura) se candidatem
CREATE OR REPLACE FUNCTION check_ficha_limpa()
    RETURNS TRIGGER
    LANGUAGE plpgsql AS
$check_ficha_limpa$
DECLARE
    ficha_suja VARCHAR;
    vice_ficha_suja VARCHAR;
BEGIN
    -- Procura o cpf do candidato na view de ficha suja
    SELECT cpfcnpj INTO ficha_suja
    FROM individuo_ficha_suja(MAKE_DATE(NEW.ano, 12, 31))
    WHERE cpfcnpj = NEW.candidato;

    IF (ficha_suja IS NOT NULL) THEN
        RAISE EXCEPTION 'O candidato deve ser ficha limpa.';
    END IF;

    -- Verificar o mesmo para o vice candidato
    -- (Caso o vice candidato seja NULL, essa query não
    -- retornará fileira alguma, logo não levantará erro)
    SELECT cpfcnpj INTO vice_ficha_suja
    FROM individuo_ficha_suja(MAKE_DATE(NEW.ano, 12, 31))
    WHERE cpfcnpj = NEW.vice_candidato;

    IF (vice_ficha_suja IS NOT NULL) THEN
        RAISE EXCEPTION 'O vice candidato deve ser ficha limpa.';
    END IF;

    RETURN NEW;
END
$check_ficha_limpa$;

CREATE TRIGGER check_ficha_limpa
    BEFORE INSERT ON candidatura
    FOR EACH ROW EXECUTE PROCEDURE check_ficha_limpa();

COMMIT;
