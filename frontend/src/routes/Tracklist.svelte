<script>
    let date = $state(new Date());
    let datestr = $derived(date.toISOString().split("T")[0]);
    let loader = $derived(load_infos());
    let track_id = $state();

    async function load_infos() {
        const res = await fetch(
            "http://127.0.0.1:8083/trackinfos?date=" + datestr,
        );
        const json = await res.json();
        return json;
    }
    function addDays(date, days) {
        const newDate = new Date(date);
        newDate.setDate(date.getDate() + days);
        return newDate;
    }
</script>

<button onclick={() => (date = addDays(date, -1))}> &lt; </button>
{datestr}
<button onclick={() => (date = addDays(date, 1))}> &gt; </button>

{#await loader}
    <p>loading track list...</p>
{:then tracks}
    <ul id="tags">
        {#each tracks as track}
            <li>
                <button onclick={() => (track_id = track.date)}> Show </button>
                {track.user} - {track.device} - {track.ts_start} - {track.ts_end}
            </li>
        {/each}
    </ul>
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}
