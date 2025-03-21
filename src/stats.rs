use crate::db::GpsPoint;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};
use geo::algorithm::vincenty_distance::VincentyDistance;
use geojson::{JsonObject, JsonValue};
use stats::{MinMax, OnlineStats};

#[derive(Default)]
pub struct TrackStats {
    ts: MinMax<i64>,
    speed: MinMax<i16>,
    speed_stats: OnlineStats,
    elevation: MinMax<i16>,
    elevation_stats: OnlineStats,
}

impl TrackStats {
    pub fn from_iter<'a>(iter: impl Iterator<Item = &'a GpsPoint>) -> Self {
        let mut stats = Self::default();
        let tz = *Local::now().offset();
        for pt in iter {
            // SQLite stores in UTC without TZ
            let dt = DateTime::<FixedOffset>::parse_from_str(&pt.ts, "%F %T%#z")
                .or(NaiveDateTime::parse_from_str(&pt.ts, "%F %T")
                    .map(|utcts| utcts.and_local_timezone(tz).unwrap()));
            // log::debug!("Timestamp `{}` -> `{dt:?}`", &pt.ts);
            match dt {
                Ok(dt) => stats.ts.add(dt.timestamp()),
                Err(e) => {
                    log::info!("Ignoring invalid timestamp `{}`: {e}", &pt.ts);
                }
            }
            if let Some(speed) = pt.speed {
                stats.speed.add(speed);
                stats.speed_stats.add(speed);
            }
            if let Some(elevation) = pt.elevation {
                stats.elevation.add(elevation);
                stats.elevation_stats.add(elevation);
            }
        }
        stats
    }
    pub fn as_properties(&self) -> JsonObject {
        let (ts_start, ts_end) = (
            self.ts
                .min()
                .and_then(|ts| DateTime::from_timestamp(*ts, 0)),
            self.ts
                .max()
                .and_then(|ts| DateTime::from_timestamp(*ts, 0)),
        );
        let duration = if let (Some(ts_start), Some(ts_end)) = (ts_start, ts_end) {
            Some(ts_end - ts_start)
        } else {
            None
        };

        JsonObject::from_iter([
            (
                "min_speed".to_string(),
                JsonValue::from(self.speed.min().copied()),
            ),
            (
                "max_speed".to_string(),
                JsonValue::from(self.speed.max().copied()),
            ),
            (
                "mean_speed".to_string(),
                JsonValue::from(self.speed_stats.mean()),
            ),
            (
                "min_elevation".to_string(),
                JsonValue::from(self.elevation.min().copied()),
            ),
            (
                "max_elevation".to_string(),
                JsonValue::from(self.elevation.max().copied()),
            ),
            (
                "mean_elevation".to_string(),
                JsonValue::from(self.elevation_stats.mean()),
            ),
            (
                "ts_start".to_string(),
                JsonValue::from(ts_start.map(|ts| ts.format("%F %T%z").to_string())),
            ),
            (
                "ts_end".to_string(),
                JsonValue::from(ts_end.map(|ts| ts.format("%F %T%z").to_string())),
            ),
            (
                "duration".to_string(),
                JsonValue::from(duration.map(|v| v.num_seconds())),
            ),
        ])
    }
}

#[derive(Default)]
pub struct BboxStats {
    x: MinMax<f64>,
    y: MinMax<f64>,
}

impl BboxStats {
    pub fn from_xy_iter(iter: impl Iterator<Item = (f64, f64)>) -> Self {
        let mut stats = Self::default();
        for (x, y) in iter {
            stats.x.add(x);
            stats.y.add(y);
        }
        stats
    }
    pub fn bbox(&self) -> Option<Vec<f64>> {
        if let (Some(xmin), Some(ymin), Some(xmax), Some(ymax)) =
            (self.x.min(), self.y.min(), self.x.max(), self.y.max())
        {
            Some(vec![*xmin, *ymin, *xmax, *ymax])
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct DistanceStats {
    distance: f64,
}

impl DistanceStats {
    pub fn from_xy_iter(iter: impl Iterator<Item = (f64, f64)>) -> Self {
        let mut stats = Self::default();
        let points = iter.collect::<Vec<_>>();
        points.windows(2).for_each(|pair| {
            let p1 = geo::Point::from(pair[0]);
            let p2 = geo::Point::from(pair[1]);
            let distance = p1.vincenty_distance(&p2).unwrap();
            stats.distance += distance;
        });
        stats
    }
    pub fn as_properties(&self) -> JsonObject {
        JsonObject::from_iter([("distance".to_string(), JsonValue::from(self.distance))])
    }
}

#[derive(Default)]
pub struct ElevationDiffStats {
    elevation_up: i64,
    elevation_down: i64,
}

impl ElevationDiffStats {
    pub fn from_iter(iter: impl Iterator<Item = i16>) -> Self {
        let mut stats = Self::default();
        let points = iter.collect::<Vec<_>>();
        points.windows(2).for_each(|pair| {
            let (elev0, elev1) = (pair[0], pair[1]);
            if elev1 > elev0 {
                stats.elevation_up += (elev1 - elev0) as i64;
            } else {
                stats.elevation_down += (elev0 - elev1) as i64;
            }
        });
        stats
    }
    pub fn as_properties(&self) -> JsonObject {
        JsonObject::from_iter([
            (
                "elevation_up".to_string(),
                JsonValue::from(self.elevation_up),
            ),
            (
                "elevation_down".to_string(),
                JsonValue::from(self.elevation_down),
            ),
        ])
    }
}
