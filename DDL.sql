DROP TABLE candidatura, cargo;

CREATE TABLE cargo (
    tipo VARCHAR NOT NULL,
    local VARCHAR NOT NULL,
    cadeiras SMALLINT NOT NULL,

    CONSTRAINT cargo_pk PRIMARY KEY (tipo, local),
    CONSTRAINT cargo_ck CHECK (cadeiras > 0)
);

CREATE TABLE candidatura (
    numero INT NOT NULL,
    ano SMALLINT NOT NULL,
    cargo_tipo VARCHAR NOT NULL,
    cargo_local VARCHAR NOT NULL,
    votos INT DEFAULT NULL,

    CONSTRAINT candidatura_pk PRIMARY KEY (numero, ano, cargo_local),
    CONSTRAINT candidatura_ck CHECK (votos >= 0),
    CONSTRAINT candidatura_fk FOREIGN KEY (cargo_tipo, cargo_local)
        REFERENCES cargo (tipo, local) ON DELETE CASCADE ON UPDATE CASCADE
);
