<script>
    import { utcToLocalTime } from "./datetime.js";

    let { curTrack } = $props();
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
