<script>
    import { PUBLIC_BASE_URL } from "$env/static/public";

    let { date, curTrack, setDate, setTrackId } = $props();
    let datestr = $derived(date.toISOString().split("T")[0]);
    let loader = $derived(load_infos());

    async function load_infos() {
        const res = await fetch(
            `${PUBLIC_BASE_URL}/trackinfos?date=${datestr}`,
        );
        const json = await res.json();
        // Select first track
        if (json.length > 0) {
            setTrackId(json[0]);
        }
        return json;
    }
    function addDays(date, days) {
        const newDate = new Date(date);
        newDate.setDate(date.getDate() + days);
        return newDate;
    }
    function utcToLocalTime(utcTimeString) {
        // input example: '2025-03-01 08:21:16+00'
        const date = new Date(utcTimeString);
        return date.toLocaleTimeString("de-CH");
    }

    function checkSelected(track) {
        return (
            curTrack &&
            curTrack.user === track.user &&
            curTrack.device === track.device &&
            curTrack.ts_start === track.ts_start
        );
    }
</script>

<button onclick={() => setDate(addDays(date, -1))}> &lt; </button>
{datestr}
<button onclick={() => setDate(addDays(date, 1))}> &gt; </button>

{#await loader}
    <p>loading track list...</p>
{:then tracks}
    <ul id="tags">
        {#each tracks as track}
            <li class={checkSelected(track) ? "selected" : ""}>
                <button onclick={() => setTrackId(track)}> Show </button>
                {track.user}
                {track.device}
                {utcToLocalTime(track.ts_start)}
                - {utcToLocalTime(track.ts_end)}
            </li>
        {/each}
    </ul>
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}

<style>
    li.selected {
        background-color: lightblue;
    }
</style>
