<script>
    import { PUBLIC_BASE_URL } from "$env/static/public";
    import {
        addDays,
        isoDateString,
        isToday,
        utcToLocalTime,
    } from "./datetime.js";

    let { date, curTrack, setDate, setCurTrack } = $props();
    let datestr = $derived(isoDateString(date));
    let loader = $derived(load_infos());

    async function load_infos() {
        const res = await fetch(
            `${PUBLIC_BASE_URL}/trackinfos?date=${datestr}`,
        );
        const json = await res.json();
        // Select first track, if there is only one for the first day
        if (
            json.length == 1 ||
            (json.length > 1 &&
                isoDateString(json[0].ts_end) !== isoDateString(json[1].ts_end))
        ) {
            setCurTrack(json[0]);
        } else {
            setCurTrack(null);
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

<div class="tracks-container">
    {#await loader}
        <p>loading track list...</p>
    {:then tracks}
        <table class="tracks-table">
            <tbody>
                {#each tracks as track}
                    <tr
                        class={checkSelected(track) ? "selected" : ""}
                        onclick={() => setCurTrack(track)}
                    >
                        <td>{track.user_id}</td>
                        <td>{track.device}</td>
                        <td>{utcToLocalTime(track.ts_start)}</td>
                        <td>{utcToLocalTime(track.ts_end)}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    {:catch error}
        <p style="color: red">{error.message}</p>
    {/await}
</div>

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

    .tracks-container {
        width: 100%;
        overflow-x: auto;
    }

    .tracks-table {
        width: 100%;
        border-collapse: collapse;
    }

    /*
    .tracks-table th {
        text-align: left;
        padding: 8px;
        background-color: #f2f2f2;
        border-bottom: 1px solid #ddd;
    }
    */

    .tracks-table td {
        padding: 8px;
        border-bottom: 1px solid #eee;
    }

    .tracks-table tr {
        transition: background-color 0.2s;
    }

    .tracks-table tr:hover {
        background-color: #f0f0f0;
        cursor: pointer;
    }

    .tracks-table tr.selected {
        background-color: lightblue;
    }

    .tracks-table tr.selected:hover {
        background-color: #a8d4e6;
    }
</style>
