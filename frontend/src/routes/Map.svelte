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
        Popup,
    } from "svelte-maplibre-gl";
    import maplibregl from "maplibre-gl";
    import { PUBLIC_BASE_URL } from "$env/static/public";

    let hoveredPositionFeat = $state.raw();
    let hoveredPointFeat = $state.raw();
    let lnglat = $state.raw(new maplibregl.LngLat(0, 0));
    let { curTrack, trackpoints, positionsSelector, setCurTrack } = $props();

    function postitionToTrack(pos) {
        return { ...pos, ts_start: pos.time, ts_end: pos.time };
    }
    function objToHtml(obj) {
        var str = "";
        for (var p in obj) {
            if (Object.prototype.hasOwnProperty.call(obj, p)) {
                str += p + ": " + obj[p] + "<br/>";
            }
        }
        return str;
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
    {#if trackpoints}
        <GeoJSONSource data={trackpoints}>
            <CircleLayer
                paint={{
                    "circle-color": "#ff0000",
                    "circle-radius": 5,
                }}
                onmousemove={(ev) => {
                    hoveredPointFeat = ev.features[0];
                    console.log(hoveredPointFeat.properties);
                    lnglat = ev.lngLat; // cursor location
                    console.log(lnglat);
                }}
                onmouseout={() => {
                    hoveredPointFeat = undefined;
                }}
                minzoom={11}
            />
            {#if hoveredPointFeat}
                <FeatureState
                    id={hoveredPointFeat.id}
                    state={{ hover: true }}
                />
                <Popup {lnglat} closeButton={false}>
                    {@html objToHtml(hoveredPointFeat.properties)}
                </Popup>
            {/if}
        </GeoJSONSource>
    {/if}
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
