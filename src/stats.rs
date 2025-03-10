use crate::db::GpsPoint;
use geojson::{JsonObject, JsonValue};
use stats::{MinMax, OnlineStats};

#[derive(Default)]
pub struct TrackStats {
    x: MinMax<f64>,
    y: MinMax<f64>,
    speed: MinMax<i16>,
    speed_stats: OnlineStats,
    elevation: MinMax<i16>,
    elevation_stats: OnlineStats,
    elevation_up: i64,
    elevation_down: i64,
    // TODO: time_begin, time_end, duration, distance (!)
}

impl TrackStats {
    pub fn iter_points<'a>(&mut self, iter: impl Iterator<Item = &'a GpsPoint>) {
        for pt in iter {
            self.x.add(pt.x);
            self.y.add(pt.y);
            if let Some(speed) = pt.speed {
                self.speed.add(speed);
                self.speed_stats.add(speed);
            }
            if let Some(elevation) = pt.elevation {
                self.elevation.add(elevation);
                self.elevation_stats.add(elevation);
            }
        }
    }
    pub fn iter_pairs<'a>(&mut self, iter: impl Iterator<Item = &'a GpsPoint>) {
        let points = iter.collect::<Vec<_>>();
        points.windows(2).for_each(|pair| {
            if let (Some(elev0), Some(elev1)) = (pair[0].elevation, pair[1].elevation) {
                if elev1 > elev0 {
                    self.elevation_up += (elev1 - elev0) as i64;
                } else {
                    self.elevation_down += (elev0 - elev1) as i64;
                }
            }
        });
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
    pub fn as_properties(&self) -> JsonObject {
        JsonObject::from_iter([
            (
                "min_speed".to_string(),
                JsonValue::from(*self.speed.min().unwrap_or(&0)),
            ),
            (
                "max_speed".to_string(),
                JsonValue::from(*self.speed.max().unwrap_or(&0)),
            ),
            (
                "mean_speed".to_string(),
                JsonValue::from(self.speed_stats.mean()),
            ),
            (
                "min_elevation".to_string(),
                JsonValue::from(*self.elevation.min().unwrap_or(&0)),
            ),
            (
                "max_elevation".to_string(),
                JsonValue::from(*self.elevation.max().unwrap_or(&0)),
            ),
            (
                "mean_elevation".to_string(),
                JsonValue::from(self.elevation_stats.mean()),
            ),
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
