<script>
    import { PUBLIC_BASE_URL } from "$env/static/public";
    import { addDays, utcToLocalTime } from "./datetime.js";

    let { date, curTrack, setDate, setCurTrack } = $props();
    let datestr = $derived(date.toISOString().split("T")[0]);
    let loader = $derived(load_infos());

    async function load_infos() {
        const res = await fetch(
            `${PUBLIC_BASE_URL}/trackinfos?date=${datestr}`,
        );
        const json = await res.json();
        // Select first track
        if (json.length > 0) {
            setCurTrack(json[0]);
        }
        return json;
    }

    function checkSelected(track) {
        return (
            curTrack &&
            curTrack.device_id === track.device_id &&
            curTrack.ts_start === track.ts_start
        );
    }
</script>

<div class="header">
    <div class="title">Owntrack-rs</div>
    <div class="date-selector">
        <button onclick={() => setDate(addDays(date, -1))}> &lt; </button>
        {datestr}
        <button onclick={() => setDate(addDays(date, 1))}> &gt; </button>
    </div>
</div>

{#await loader}
    <p>loading track list...</p>
{:then tracks}
    <ul id="tags">
        {#each tracks as track}
            <li class={checkSelected(track) ? "selected" : ""}>
                <button onclick={() => setCurTrack(track)}> Show </button>
                {track.user_id}
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
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 10px;
    }

    .title {
        font-weight: bold;
        font-size: 1.2em;
    }

    .date-selector {
        display: flex;
        align-items: center;
        gap: 5px;
    }

    li.selected {
        background-color: lightblue;
    }

    ul {
        list-style-type: none;
        padding: 0;
    }

    li {
        padding: 5px;
        margin-bottom: 2px;
        border-radius: 3px;
    }
</style>
