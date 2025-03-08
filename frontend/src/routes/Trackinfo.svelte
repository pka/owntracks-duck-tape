<script>
    let { curTrack } = $props();

    function utcToLocalTime(utcTimeString) {
        // input example: '2025-03-01 08:21:16+00'
        const date = new Date(utcTimeString);
        return date.toLocaleTimeString("de-CH");
    }
</script>

{#if curTrack}
    {curTrack.user}
    {curTrack.device}
    {utcToLocalTime(curTrack.ts_start)}
    - {utcToLocalTime(curTrack.ts_end)}
{/if}
