ALTER TABLE gpslog ADD "user" VARCHAR DEFAULT 'pi' NOT NULL,
ADD device VARCHAR DEFAULT 'pimobile' NOT NULL;

-- DuckDB syntax:
-- ALTER TABLE gpslog ADD "user" VARCHAR DEFAULT 'pi';
-- ALTER TABLE gpslog ADD device VARCHAR DEFAULT 'pimobile';
-- Binder Error: Unsupported ALTER TABLE type - Postgres tables only support RENAME TABLE, RENAME COLUMN, ADD COLUMN and DROP COLUMN
-- ALTER TABLE gpslog ALTER COLUMN "user" SET NOT NULL;
-- ALTER TABLE gpslog ALTER COLUMN device SET NOT NULL;
