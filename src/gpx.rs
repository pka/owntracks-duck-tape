use crate::db::Db;
use geo_types::Point;
use gpx::{Gpx, GpxVersion, Track, TrackSegment, Waypoint};

pub fn query_tracks(db: &Db, date: &str) -> duckdb::Result<String> {
    let tracks = db.query_tracks(date)?;

    let tracks = tracks
        .iter()
        .map(|track| {
            let track_segment = TrackSegment {
                points: track
                    .points
                    .iter()
                    .filter_map(|point| {
                        // keep only points within 200 meters accuracy
                        if point.accuracy < 200 {
                            let mut wpt = Waypoint::new(Point::new(point.x as f64, point.y as f64));
                            wpt.time = Some(point.ts.into());
                            wpt.elevation = Some(point.elevation as f64);
                            wpt.speed = Some(point.speed as f64);
                            Some(wpt)
                        } else {
                            None
                        }
                    })
                    .collect(),
            };
            Track {
                name: Some(format!(
                    "Track {date}-{user}-{device}",
                    date = track.date,
                    user = track.user,
                    device = track.device
                )),
                comment: None,
                description: None,
                source: None,
                links: vec![],
                type_: None,
                number: None,
                segments: vec![track_segment],
            }
        })
        .collect();
    let gpx = Gpx {
        version: GpxVersion::Gpx11,
        creator: None,
        metadata: None,
        waypoints: vec![],
        tracks,
        routes: vec![],
    };

    let mut vec = Vec::new();
    gpx::write(&gpx, &mut vec).unwrap();
    let gpxstr = String::from_utf8(vec).unwrap();
    Ok(gpxstr)
}
