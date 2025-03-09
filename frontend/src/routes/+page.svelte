<script>
    import Tracklist from "./Tracklist.svelte";
    import Trackinfo from "./Trackinfo.svelte";
    import Tracktitle from "./Tracktitle.svelte";
    import Map from "./Map.svelte";
    import { isoDateString, isToday } from "./datetime.js";

    let date = $state(new Date());
    let curTrack = $state();
    let positionsSelector = $derived.by(() => {
        // Show positions if date is today
        if (isToday(date)) {
            return `date=${isoDateString(date)}`;
        } else {
            return null;
        }
    });

    function setDate(newDate) {
        date = newDate;
    }

    function setCurTrack(newTrackId) {
        curTrack = newTrackId;
    }
</script>

<div class="container">
    <div class="top-section">
        <div class="left-column">
            <div class="map-section">
                <div class="map-title">
                    <Tracktitle {curTrack} />
                </div>
                <div class="map-container">
                    <Map {curTrack} {positionsSelector} {setCurTrack} />
                </div>
            </div>

            <div class="trackinfo-container">
                <Trackinfo {curTrack} />
            </div>
        </div>

        <div class="tracklist-container">
            <Tracklist {date} {curTrack} {setDate} {setCurTrack} />
        </div>
    </div>
</div>

<style>
    :global {
        /* https://fontsource.org/ */
        /* barlow-latin-400-normal */
        @font-face {
            font-family: "Barlow";
            font-style: normal;
            font-display: swap;
            font-weight: 400;
            src:
                url(@fontsource/barlow/files/barlow-latin-400-normal.woff2)
                    format("woff2"),
                url(@fontsource/barlow/files/barlow-latin-400-normal.woff)
                    format("woff");
            unicode-range:
                U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA,
                U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+20AC, U+2122,
                U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
        }

        /* barlow-latin-500-normal */
        @font-face {
            font-family: "Barlow";
            font-style: normal;
            font-display: swap;
            font-weight: 500;
            src:
                url(@fontsource/barlow/files/barlow-latin-500-normal.woff2)
                    format("woff2"),
                url(@fontsource/barlow/files/barlow-latin-500-normal.woff)
                    format("woff");
            unicode-range:
                U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA,
                U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+20AC, U+2122,
                U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
        }

        html {
            font-family: "Barlow", sans-serif;
        }

        body {
            font-family: "Barlow", sans-serif;
        }
    }

    * {
        box-sizing: border-box;
        margin: 0;
        padding: 0;
    }

    .container {
        width: 100%;
        margin: 0 auto;
        display: flex;
        flex-direction: column;
    }

    .top-section {
        display: flex;
        flex-wrap: wrap;
        width: 100%;
    }

    .left-column {
        flex: 1;
        min-width: 300px;
        padding-right: 20px;
        display: flex;
        flex-direction: column;
    }

    .map-title {
        margin-bottom: 6px;
    }

    .map-section {
        width: 100%;
        position: relative;
        margin-bottom: 20px;
    }

    .map-container {
        width: 100%;
    }

    .trackinfo-container {
        width: 100%;
        margin-bottom: 20px;
    }

    .tracklist-container {
        flex: 1;
        min-width: 300px;
        max-height: calc(
            100vh - 40px
        ); /* Limit to screen height minus some padding */
        overflow-y: auto; /* Make it scrollable */
        padding-right: 10px; /* Add some padding to accommodate scrollbar */
    }

    @media (max-width: 768px) {
        .container {
            display: flex;
            flex-direction: column;
        }

        .top-section {
            flex-direction: column;
        }

        .left-column {
            width: 100%;
            padding-right: 0;
            margin-bottom: 20px;
        }

        .map-section {
            width: 100%;
            margin-bottom: 20px;
        }

        .tracklist-container {
            width: 100%;
            margin-bottom: 20px;
            max-height: 400px; /* Limit height on mobile */
            overflow-y: auto;
        }
    }
</style>
