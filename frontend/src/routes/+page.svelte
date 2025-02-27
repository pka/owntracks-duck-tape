<script>
    import Tracklist from "./Tracklist.svelte";
    import Map from "./Map.svelte";
    let date = $state(new Date().toISOString().split("T")[0]);
</script>

<Map {date} />
<Tracklist />
