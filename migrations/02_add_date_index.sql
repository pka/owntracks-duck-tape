CREATE INDEX gpslog_date_device_idx ON gpslog (date(ts, 'unixepoch'), device_id);
