ALTER TABLE gpslog ADD tid VARCHAR DEFAULT 'pi' NOT NULL,
ADD velocity SMALLINT, -- USMALLINT
ADD alt SMALLINT, -- USMALLINT
ADD accuracy INTEGER, -- UINTEGER
ADD v_accuracy SMALLINT,
ADD batt_level SMALLINT, -- UTINYINT
ADD batt_status SMALLINT NOT NULL DEFAULT -1, -- UTINYINT
ADD cog SMALLINT, -- TINYINT
ADD rad INTEGER, -- UINTEGER
ADD trigger CHAR,
ADD pressure REAL,
ADD poi VARCHAR,
ADD conn_status CHAR,
ADD tag VARCHAR,
ADD topic VARCHAR,
ADD inregions TEXT,
ADD inrids TEXT,
ADD ssid VARCHAR,
ADD bssid VARCHAR,
ADD created_at TIMESTAMPTZ,
ADD mmode SMALLINT -- UTINYINT
-- ADD msg_id VARCHAR NOT NULL
;
