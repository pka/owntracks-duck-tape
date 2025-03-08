<script>
    let { curTrack } = $props();

    function utcToLocalTime(utcTimeString) {
        // input example: '2025-03-01 08:21:16+00'
        const date = new Date(utcTimeString);
        return date.toLocaleTimeString("de-CH");
    }
</script>

{#if curTrack}
    <ul>
        <li>tid: <b>{curTrack.tid}</b></li>
        <li>User: {curTrack.user_id}</li>
        <li>Device: {curTrack.device}</li>
        <li>
            Time: {utcToLocalTime(curTrack.ts_start)} - {utcToLocalTime(
                curTrack.ts_end,
            )}
        </li>
        <li>Speed: {curTrack.speed_min}-{curTrack.speed_max} km/h</li>
        <li>Altitude: {curTrack.elevation_min}-{curTrack.elevation_max} müM</li>
        <li><pre>{curTrack.annotations}</pre></li>
    </ul>
{/if}
