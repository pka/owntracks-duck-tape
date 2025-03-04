use crate::db::TrackData;
use geojson::{Feature, FeatureCollection, Geometry, JsonObject, JsonValue};

const MAX_ACCURACY: i32 = 200; // meters

/// Build a GeoJSON FeatureCollection with segments containing speed, etc.
pub fn track_with_segments(tracks: &[TrackData]) -> anyhow::Result<String> {
    let features: Vec<Feature> = tracks
        .iter()
        .enumerate()
        .flat_map(|(no, track)| {
            let segments: Vec<Feature> = track
                .points
                .iter()
                .filter(|point| {
                    // keep only points within accuracy
                    point.accuracy.unwrap_or(0) < MAX_ACCURACY
                })
                .collect::<Vec<_>>()
                .windows(2)
                .map(|pts| {
                    let line = vec![
                        vec![pts[0].x, pts[0].y],
                        vec![pts[1].x, pts[1].y],
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
        .collect();

    let geojson = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };
    Ok(geojson.to_string())
}
