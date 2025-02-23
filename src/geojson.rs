use crate::db::Db;
use geojson::{Feature, FeatureCollection, Geometry, JsonObject, JsonValue};

const MAX_ACCURACY: i32 = 200; // meters

pub fn query_tracks(db: &Db, date: &str) -> duckdb::Result<String> {
    let tracks = db.query_tracks(date)?;

    let features: Vec<Feature> = tracks
        .iter()
        .enumerate()
        .map(|(no, track)| {
            let segments: Vec<Feature> = track
                .points
                .iter()
                .filter(|point| {
                    // keep only points within accuracy
                    point.accuracy < MAX_ACCURACY
                })
                .collect::<Vec<_>>()
                .windows(2)
                .map(|pts| {
                    let line = vec![
                        vec![pts[0].x as f64, pts[0].y as f64],
                        vec![pts[1].x as f64, pts[1].y as f64],
                    ];
                    let geometry = Geometry::new(geojson::Value::LineString(line));
                    let point = pts[0];
                    let properties = JsonObject::from_iter([
                        ("trackno".to_string(), JsonValue::from(no)),
                        ("user".to_string(), JsonValue::from(track.user.clone())),
                        ("device".to_string(), JsonValue::from(track.device.clone())),
                        ("time".to_string(), JsonValue::from(point.ts.to_string())),
                        ("elevation".to_string(), JsonValue::from(point.elevation)),
                        ("speed".to_string(), JsonValue::from(point.speed)),
                    ]);
                    Feature {
                        bbox: None,
                        geometry: Some(geometry),
                        id: None,
                        properties: Some(properties),
                        foreign_members: None,
                    }
                })
                .collect();
            segments
        })
        .flatten()
        .collect();

    let geojson = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };
    Ok(geojson.to_string())
}
