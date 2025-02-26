<script>
    import {
        MapLibre,
        NavigationControl,
        ScaleControl,
        GeoJSONSource,
        LineLayer,
    } from "svelte-maplibre-gl";
    import Tracklist from "./Tracklist.svelte";
    let date = $state(new Date().toISOString().split("T")[0]);
</script>

<MapLibre
    class="map"
    style="https://basemaps.cartocdn.com/gl/voyager-gl-style/style.json"
    zoom={9}
    center={{ lng: 9.437489, lat: 47.050207 }}
>
    <NavigationControl />
    <ScaleControl />
    <GeoJSONSource data={`http://127.0.0.1:8083/tracks?date=${date}`}>
        <LineLayer
            paint={{
                "line-color": "#ff0000",
                "line-width": 4,
            }}
        />
    </GeoJSONSource>
</MapLibre>
<Tracklist />

<style>
    :global(.map) {
        width: 100%;
        height: 400px;
    }
</style>
