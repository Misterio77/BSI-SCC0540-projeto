DROP TABLE IF EXISTS candidatura, cargo, individuo, partido;
DROP TYPE IF EXISTS tipo_cargo;

CREATE TYPE tipo_cargo AS ENUM (
    'Prefeito',
    'Governador',
    'Presidente',
    'Vereador',
    'DeputadoEstadual',
    'DeputadoFederal',
    'Senador'
);

CREATE TABLE individuo (
    id VARCHAR NOT NULL,
    nome VARCHAR NOT NULL,
    nascimento DATE NOT NULL,
    ficha_limpa BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT individuo_pk PRIMARY KEY (id)
);

CREATE TABLE partido (
    numero SMALLINT NOT NULL,
    nome VARCHAR NOT NULL,
    programa VARCHAR NOT NULL,

    CONSTRAINT partido_pk PRIMARY KEY (numero),
    CONSTRAINT partido_un UNIQUE (nome),
    CONSTRAINT partido_ck CHECK (numero >= 10)
);

CREATE TABLE cargo (
    tipo tipo_cargo NOT NULL,
    local VARCHAR NOT NULL,
    cadeiras SMALLINT NOT NULL DEFAULT 1,

    CONSTRAINT cargo_pk PRIMARY KEY (tipo, local),
    CONSTRAINT cargo_ck CHECK (cadeiras > 0)
);

CREATE TABLE candidatura (
    candidato VARCHAR NOT NULL,
    vice_candidato VARCHAR DEFAULT NULL,
    ano SMALLINT NOT NULL,
    cargo_tipo tipo_cargo NOT NULL,
    cargo_local VARCHAR NOT NULL,
    numero INT NOT NULL,
    partido SMALLINT NOT NULL,
    votos INT DEFAULT NULL,

    CONSTRAINT candidatura_pk PRIMARY KEY (candidato, ano),

    -- Garante que, para cada ano, a pessoa só é vice de uma candidatura
    CONSTRAINT candidatura_un1 UNIQUE (vice_candidato, ano),
    -- Garante que, para cada ano e cargo, só existe 1 candidatura com dado número
    CONSTRAINT candidatura_un2 UNIQUE (cargo_tipo, cargo_local, numero, ano),

    CONSTRAINT candidatura_ck1 CHECK (votos >= 0),
    CONSTRAINT candidatura_ck2 CHECK (numero >= 0),

    -- Referencia o individuo candidato
    CONSTRAINT candidatura_fk1 FOREIGN KEY (candidato)
        REFERENCES individuo (id) ON DELETE CASCADE ON UPDATE CASCADE,
    -- Referencia o individuo vice candidato
    CONSTRAINT candidatura_fk2 FOREIGN KEY (vice_candidato)
        REFERENCES individuo (id) ON DELETE CASCADE ON UPDATE CASCADE,
    -- Referencia o cargo da candidatura
    CONSTRAINT candidatura_fk3 FOREIGN KEY (cargo_tipo, cargo_local)
        REFERENCES cargo (tipo, local) ON DELETE CASCADE ON UPDATE CASCADE,
    -- Referencia o partido
    CONSTRAINT candidatura_fk4 FOREIGN KEY (partido)
        REFERENCES partido (numero) ON DELETE CASCADE ON UPDATE CASCADE
);
