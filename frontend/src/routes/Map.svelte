<script>
    // https://svelte-maplibre-gl.mierune.dev/
    import {
        MapLibre,
        NavigationControl,
        ScaleControl,
        GeoJSONSource,
        FeatureState,
        LineLayer,
        CircleLayer,
        SymbolLayer,
    } from "svelte-maplibre-gl";
    import { PUBLIC_BASE_URL } from "$env/static/public";

    let hoveredPositionFeat = $state.raw();
    let { curTrack, positionsSelector, setCurTrack } = $props();

    function postitionToTrack(pos) {
        return { ...pos, ts_start: pos.time, ts_end: pos.time };
    }
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
            data={`${PUBLIC_BASE_URL}/track?device_id=${curTrack.device_id}&ts_start=${curTrack.ts_start}`}
        >
            <LineLayer
                paint={{
                    "line-color": "#ff0000",
                    "line-width": 4,
                }}
            />
        </GeoJSONSource>
    {/if}
    {#if positionsSelector}
        <GeoJSONSource
            data={`${PUBLIC_BASE_URL}/positions?${positionsSelector}`}
        >
            <CircleLayer
                paint={{
                    "circle-color": [
                        "case",
                        ["boolean", ["feature-state", "hover"], false],
                        "lightblue",
                        "#0000ff",
                    ],
                    "circle-radius": 20,
                }}
                onmousemove={(ev) => {
                    hoveredPositionFeat = ev.features[0];
                }}
                onmouseout={() => {
                    hoveredPositionFeat = undefined;
                }}
                onclick={(ev) => {
                    setCurTrack(postitionToTrack(ev.features[0].properties));
                }}
            />
            <SymbolLayer
                layout={{
                    "text-field": ["get", "tid"],
                    "text-size": 15,
                }}
                paint={{
                    "text-color": "#ffffff",
                }}
            />
            {#if hoveredPositionFeat}
                <!-- Set the hover state on the source for the hovered feature -->
                <FeatureState
                    id={hoveredPositionFeat.id}
                    state={{ hover: true }}
                />
            {/if}
        </GeoJSONSource>
    {/if}
</MapLibre>

<style>
    :global(.map) {
        width: 100%;
        height: 400px;
        border: 1px solid #ddd;
    }
</style>
