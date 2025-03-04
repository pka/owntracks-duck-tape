use crate::db::TrackData;
use geo_types::Point;
use gpx::{Gpx, GpxVersion, Track, TrackSegment, Waypoint};
use time::{macros::format_description, OffsetDateTime};

/// Build a GPX track from track data.
pub fn tracks(tracks: &[TrackData]) -> anyhow::Result<String> {
    let ot_format =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second][offset_hour]");
    let tracks = tracks
        .iter()
        .map(|track| {
            let track_segment = TrackSegment {
                points: track
                    .points
                    .iter()
                    .filter_map(|point| {
                        // keep only points within 200 meters accuracy
                        if point.accuracy.unwrap_or(0) < 200 {
                            let time: Option<gpx::Time> =
                                OffsetDateTime::parse(&point.ts, &ot_format)
                                    .map(|dt| dt.into())
                                    .ok();
                            let mut wpt = Waypoint::new(Point::new(point.x, point.y));
                            wpt.time = time;
                            wpt.elevation = point.elevation.map(|val| val as f64);
                            wpt.speed = point.speed.map(|val| val as f64 / 3.6);
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
        version: GpxVersion::Gpx10, // Speed is only included in GPX 1.0. Will be in next georust/gpx release.
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
