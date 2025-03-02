<script>
    import {
        MapLibre,
        NavigationControl,
        ScaleControl,
        GeoJSONSource,
        LineLayer,
    } from "svelte-maplibre-gl";
    import { PUBLIC_BASE_URL } from "$env/static/public";

    let { curTrack } = $props();
</script>

<MapLibre
    class="map"
    style="https://basemaps.cartocdn.com/gl/voyager-gl-style/style.json"
    zoom={9}
    center={{ lng: 9.437489, lat: 47.050207 }}
>
    <NavigationControl />
    <ScaleControl />
    {#if curTrack}
        <GeoJSONSource
            data={`${PUBLIC_BASE_URL}/track?user=${curTrack.user}&device=${curTrack.device}&ts_start=${curTrack.ts_start}`}
        >
            <LineLayer
                paint={{
                    "line-color": "#ff0000",
                    "line-width": 4,
                }}
            />
        </GeoJSONSource>
    {/if}
</MapLibre>

<style>
    :global(.map) {
        width: 100%;
        height: 400px;
    }
</style>
