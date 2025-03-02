<script>
    import Tracklist from "./Tracklist.svelte";
    import Map from "./Map.svelte";
    let date = $state(new Date());
    let curTrack = $state();

    function setDate(newDate) {
        date = newDate;
    }

    function setTrackId(newTrackId) {
        curTrack = newTrackId;
    }
</script>

<Map {curTrack} />
<Tracklist {date} {curTrack} {setDate} {setTrackId} />
