use crate::db::{GpsPoint, Position, TrackData};
use geojson::{Feature, FeatureCollection, Geometry, JsonObject, JsonValue};

const MAX_ACCURACY: i32 = 200; // meters
const ANNOTATIONS_SKIP_LIST: &[&str] = &["_id", "m", "BSSID", "SSID", "created_at"];

fn point_properties(pt: &GpsPoint) -> JsonObject {
    let mut json = JsonObject::from_iter([
        ("time".to_string(), JsonValue::from(pt.ts.to_string())),
        ("tid".to_string(), JsonValue::from(pt.tid.clone())),
        ("speed".to_string(), JsonValue::from(pt.speed)),
        ("elevation".to_string(), JsonValue::from(pt.elevation)),
        ("accuracy".to_string(), JsonValue::from(pt.accuracy)),
        ("v_accuracy".to_string(), JsonValue::from(pt.v_accuracy)),
        ("cog".to_string(), JsonValue::from(pt.cog)),
    ]);
    let annotations: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(pt.annotations.as_str()).unwrap();
    json.extend(
        annotations
            .into_iter()
            .filter(|(key, _)| !ANNOTATIONS_SKIP_LIST.contains(&key.as_str())),
    );
    json
}

/// Build a GeoJSON LineString FeatureCollection
pub fn track(tracks: &[TrackData]) -> anyhow::Result<String> {
    let features: Vec<Feature> = tracks
        .iter()
        .map(|track| {
            let points = track.points.iter().filter(|point| {
                // keep only points within accuracy
                point.accuracy.unwrap_or(0) < MAX_ACCURACY
            });
            let geometry = Geometry::new(geojson::Value::LineString(
                points.clone().map(|pt| vec![pt.x, pt.y]).collect(),
            ));
            // Use properties of last point
            let properties = points.last().map(point_properties);
            Feature {
                bbox: None,
                geometry: Some(geometry),
                id: None,
                properties,
                foreign_members: None,
            }
        })
        .collect();

    let geojson = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };
    Ok(geojson.to_string())
}

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
                    let line = vec![vec![pts[0].x, pts[0].y], vec![pts[1].x, pts[1].y]];
                    let geometry = Geometry::new(geojson::Value::LineString(line));
                    let mut properties = point_properties(pts[0]);
                    properties.extend([("trackno".to_string(), JsonValue::from(no))]);
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

/// Build a GeoJSON Point FeatureCollection
pub fn positions(points: &[Position]) -> anyhow::Result<String> {
    let features = points
        .iter()
        // .filter(|point| {
        //     // keep only points within accuracy
        //     point.accuracy.unwrap_or(0) < MAX_ACCURACY
        // })
        .map(|pt| {
            let geometry = Geometry::new(geojson::Value::Point(vec![pt.x, pt.y]));
            let properties = JsonObject::from_iter([
                ("device_id".to_string(), JsonValue::from(pt.device_id)),
                ("time".to_string(), JsonValue::from(pt.ts.to_string())),
                ("tid".to_string(), JsonValue::from(pt.tid.clone())),
                ("speed".to_string(), JsonValue::from(pt.speed)),
                ("elevation".to_string(), JsonValue::from(pt.elevation)),
                ("accuracy".to_string(), JsonValue::from(pt.accuracy)),
                ("v_accuracy".to_string(), JsonValue::from(pt.v_accuracy)),
                ("cog".to_string(), JsonValue::from(pt.cog)),
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

    let geojson = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };
    Ok(geojson.to_string())
}
