<script>
    import {
        MapLibre,
        NavigationControl,
        ScaleControl,
        GeoJSONSource,
        LineLayer,
    } from "svelte-maplibre-gl";
    let { date } = $props();
    let datestr = $derived(date.toISOString().split("T")[0]);
</script>

<MapLibre
    class="map"
    style="https://basemaps.cartocdn.com/gl/voyager-gl-style/style.json"
    zoom={9}
    center={{ lng: 9.437489, lat: 47.050207 }}
>
    <NavigationControl />
    <ScaleControl />
    <GeoJSONSource data={`http://127.0.0.1:8083/tracks?date=${datestr}`}>
        <LineLayer
            paint={{
                "line-color": "#ff0000",
                "line-width": 4,
            }}
        />
    </GeoJSONSource>
</MapLibre>

<style>
    :global(.map) {
        width: 100%;
        height: 400px;
    }
</style>
