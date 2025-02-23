<script>
    import {
        MapLibre,
        NavigationControl,
        ScaleControl,
    } from "svelte-maplibre-gl";
</script>

<MapLibre
    class="map"
    style="https://basemaps.cartocdn.com/gl/voyager-gl-style/style.json"
    zoom={8}
    center={{ lng: 9.437489, lat: 47.050207 }}
>
    <NavigationControl />
    <ScaleControl />
</MapLibre>

<style>
    :global(.map) {
        width: 100%;
        height: 400px;
    }
</style>
