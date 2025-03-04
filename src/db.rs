use crate::owntracks::Location;
//use chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};
use sqlx::migrate::{MigrateDatabase, Migrator};
use sqlx::{AnyPool, Sqlite};

static MIGRATOR: Migrator = sqlx::migrate!();

/// Track identification
#[derive(sqlx::FromRow, Deserialize, Debug)]
pub struct TrackId {
    pub user: String,
    pub device: String,
    pub ts_start: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct TrackData {
    pub user: String,
    pub device: String,
    pub date: String, // time::Date,
    pub points: Vec<GpsPoint>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct GpsPoint {
    pub y: f64,
    pub x: f64,
    /// Timestamp in format 2025-02-19 06:46:54+00
    pub ts: String, // DateTime<FixedOffset> is not supported by Any driver
    pub speed: Option<i16>,
    pub elevation: Option<i16>,
    /// Accuracy in meters
    pub accuracy: Option<i32>, // owntracks: u32
    /// Vertical accuracy in meters
    pub v_accuracy: Option<i16>,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct TrackInfo {
    pub user: String,
    pub device: String,
    pub date: String,     // time::Date is not supported by Any driver
    pub ts_start: String, // DateTime<FixedOffset> is not supported by Any driver
    pub ts_end: String,   // DateTime<FixedOffset> is not supported by Any driver
    pub speed_min: Option<i16>,
    pub speed_max: Option<i16>,
    pub elevation_min: Option<i16>,
    pub elevation_max: Option<i16>,
}

impl TrackId {
    pub fn date(&self) -> String {
        // from timestamp in format 2025-02-19 06:46:54+00
        self.ts_start.split(' ').next().unwrap().to_string()
    }
}

#[derive(Clone)]
pub struct Db {
    pool: AnyPool,
}

impl Db {
    pub async fn connect() -> anyhow::Result<Self> {
        let conn_str =
            dotenvy::var("DB_CONNECTION").unwrap_or("sqlite://tracking.sqlite".to_string());
        if conn_str.starts_with("sqlite:")
            && !Sqlite::database_exists(&conn_str).await.unwrap_or(false)
        {
            log::info!("Creating database {conn_str}");
            Sqlite::create_database(&conn_str).await?;
        }
        sqlx::any::install_default_drivers();
        log::info!("Connecting to database...");
        let pool = AnyPool::connect(&conn_str).await?;
        Ok(Db { pool })
    }

    pub async fn run_migrations(&self) -> anyhow::Result<()> {
        log::info!("Running database migrations...");
        MIGRATOR.run(&self.pool).await?;
        let is_pg = self.pool.acquire().await?.backend_name() == "PostgreSQL";
        if is_pg {
            let _result = sqlx::raw_sql(
                r#"
                CREATE SEQUENCE IF NOT EXISTS gpslog_id_seq;
                ALTER TABLE gpslog ALTER COLUMN id SET DEFAULT NEXTVAL ('gpslog_id_seq');
                -- SQLite comaptible date/time functions
                CREATE OR REPLACE FUNCTION unixepoch(bigint, varchar(20)) RETURNS TIMESTAMPTZ
                    AS 'select to_timestamp($1);'
                    LANGUAGE SQL
                    IMMUTABLE;
                CREATE OR REPLACE FUNCTION date(TIMESTAMPTZ, varchar(20)) RETURNS VARCHAR
                    AS 'select $1::DATE::VARCHAR;'
                    LANGUAGE SQL
                    IMMUTABLE;
                CREATE OR REPLACE FUNCTION datetime(TIMESTAMPTZ, varchar(20)) RETURNS VARCHAR
                    AS 'select $1::VARCHAR;'
                    LANGUAGE SQL
                    IMMUTABLE;
                "#,
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn insert_location(
        &self,
        user: &str,
        device: &str,
        loc: &Location,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO gpslog
             ("user", device, tid, ts, velocity, lat, lon, alt, accuracy, v_accuracy, batt_level, batt_status,
              cog, rad, trigger, pressure, poi, conn_status, tag, topic, inregions, inrids, ssid, bssid,
              created_at, mmode)
              VALUES ($1, $2, $3, unixepoch($4, 'unixepoch'), $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24,
                      unixepoch($25, 'unixepoch'), $26)"#
        )
        .bind(user)
        .bind(device)
        .bind(&loc.tid)
        .bind(loc.ts)
        .bind(loc.velocity.map(|val| val as i32)) // u16 is not supported by Any driver
        .bind(loc.lat)
        .bind(loc.lon)
        .bind(loc.alt.map(|val| val as i32)) // u16 is not supported by Any driver
        .bind(loc.accuracy.map(|val| val as i64)) // u32 is not supported by Any driver
        .bind(loc.v_accuracy)
        .bind(loc.batt_level.map(|val| val as i16)) // u8 is not supported by Any driver
        .bind(loc.batt_status as i16) // u8 is not supported by Any driver
        .bind(loc.cog)
        .bind(loc.rad.map(|val| val as i64)) // u32 is not supported by Any driver
        .bind(&loc.trigger)
        .bind(loc.pressure)
        .bind(&loc.poi)
        .bind(&loc.conn_status)
        .bind(&loc.tag)
        .bind(&loc.topic)
        .bind(&loc.inregions)
        .bind(&loc.inrids)
        .bind(&loc.ssid)
        .bind(&loc.bssid)
        .bind(loc.created_at)
        .bind(loc.mmode.map(|val| val as i16)) // u8 is not supported by Any driver
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Return track infos of a given date
    pub async fn query_tracks_info(&self, date: &str) -> anyhow::Result<Vec<TrackInfo>> {
        let mut tracks: Vec<TrackInfo> = sqlx::query_as(
            r#"SELECT
                "user",
                device,
                date(ts, 'unixepoch') as date,
                datetime(min(ts), 'unixepoch') as ts_start,
                datetime(max(ts), 'unixepoch') as ts_end,
                min(velocity) as speed_min,
                max(velocity) as speed_max,
                min(alt) as elevation_min,
                max(alt) as elevation_max
            FROM gpslog
            WHERE date(ts, 'unixepoch') = $1
            GROUP BY "user", device, date(ts, 'unixepoch')"#,
        )
        .bind(date)
        .fetch_all(&self.pool)
        .await?;

        // Sort the tracks by ts_end in descending order
        tracks.sort_by(|a, b| b.ts_end.cmp(&a.ts_end));

        Ok(tracks)
    }

    /// Query a single track by its ID.
    pub async fn query_track(&self, track_id: &TrackId) -> anyhow::Result<TrackData> {
        let date = track_id.date();
        let points: Vec<GpsPoint> = sqlx::query_as(
            r#"
                SELECT
                    lat as y,
                    lon as x,
                    datetime(ts, 'unixepoch') AS ts,
                    velocity as speed,
                    alt as elevation,
                    accuracy,
                    v_accuracy
                FROM gpslog
                WHERE date(ts, 'unixepoch') = $1
                AND "user" = $2
                AND device = $3
                ORDER BY id
                "#,
        )
        .bind(&date)
        .bind(&track_id.user)
        .bind(&track_id.device)
        .fetch_all(&self.pool)
        .await?;

        let gps_points = points
            .into_iter()
            .map(|p| GpsPoint {
                y: p.y,
                x: p.x,
                ts: p.ts,
                speed: p.speed,
                elevation: p.elevation,
                accuracy: p.accuracy,
                v_accuracy: p.v_accuracy,
            })
            .collect();

        let track = TrackData {
            user: track_id.user.clone(),
            device: track_id.device.clone(),
            date,
            points: gps_points,
        };

        Ok(track)
    }

    /// Return tracks of a given date
    pub async fn query_tracks(&self, date: &str) -> anyhow::Result<Vec<TrackData>> {
        // First get the unique user/device combinations for the date
        // Alternative as single query:
        // SELECT user, device, ts::date, array_agg((lat, lon, ts, velocity, alt, accuracy, v_accuracy) ORDER BY id) AS points
        // WHERE ts::date = ?                                                                                                                                                                                                                                                                                                                                           ║
        // GROUP BY user, device, ts::date
        let user_devices: Vec<TrackId> = sqlx::query_as(
            r#"
            SELECT DISTINCT "user", device, datetime(min(ts), 'unixepoch') AS ts_start
            FROM gpslog
            WHERE date(ts, 'unixepoch') = date($1)
            GROUP BY "user", device
            "#,
        )
        .bind(date)
        .fetch_all(&self.pool)
        .await?;

        let mut tracks = Vec::new();

        for track_id in user_devices {
            let track = self.query_track(&track_id).await?;
            if !track.points.is_empty() {
                tracks.push(track);
            }
        }

        Ok(tracks)
    }
}
