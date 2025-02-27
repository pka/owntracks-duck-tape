<script>
    import Tracklist from "./Tracklist.svelte";
    import Map from "./Map.svelte";
    let date = $state(new Date());

    function setDate(newDate) {
        date = newDate;
    }
</script>

<Map {date} />
<Tracklist {date} {setDate} />
