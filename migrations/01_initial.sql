CREATE SEQUENCE gpslog_id_seq;

CREATE TABLE gpslog (
    id INTEGER PRIMARY KEY DEFAULT NEXTVAL ('gpslog_id_seq'),
    lat REAL NOT NULL,
    lon REAL NOT NULL,
    ts TIMESTAMPTZ NOT NULL
);
