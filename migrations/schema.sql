CREATE SEQUENCE gpslog_id_seq;

CREATE TABLE gpslog(
    id INTEGER PRIMARY KEY DEFAULT nextval('gpslog_id_seq'),
    "user" VARCHAR NOT NULL,
    device VARCHAR NOT NULL,
    lat FLOAT NOT NULL,
    lon FLOAT NOT NULL,
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    tid VARCHAR NOT NULL,
    velocity SMALLINT,
    alt SMALLINT,
    accuracy INTEGER,
    v_accuracy SMALLINT,
    batt_level SMALLINT,
    batt_status SMALLINT NOT NULL,
    cog SMALLINT,
    rad INTEGER,
    "trigger" VARCHAR,
    pressure FLOAT,
    poi VARCHAR,
    conn_status VARCHAR,
    tag VARCHAR,
    topic VARCHAR,
    inregions VARCHAR,
    inrids VARCHAR,
    ssid VARCHAR,
    bssid VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE,
    mmode SMALLINT
);

CREATE INDEX gpslog_pkey ON gpslog(id);
