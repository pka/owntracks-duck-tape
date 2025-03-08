<script>
    // https://svelte-ionicons.codewithshin.com/icons
    import Paw from "svelte-ionicons/Paw.svelte";

    let { curTrack } = $props();

    function utcToLocalDate(utcTimeString) {
        // input example: '2025-03-01 08:21:16+00'
        const date = new Date(utcTimeString);
        return date.toLocaleDateString("de-CH");
    }
</script>

{#if curTrack}
    <Paw size="15" />&nbsp;Track from {utcToLocalDate(curTrack.ts_end)}
{/if}
