<div class="deck-container">
	<div id="map" bind:this={mapElement}></div>
	<canvas id="deck-canvas" bind:this={canvasElement}></canvas>
</div>

<script>
import { onMount } from "svelte";
import { ScatterplotLayer } from "@deck.gl/layers";
import { Deck } from "@deck.gl/core";
import maplibregl from 'maplibre-gl';
import 'maplibre-gl/dist/maplibre-gl.css';

let mapElement;
let canvasElement;
let map = null;
let deck = null;
let viewState = {
	longitude: -118.2443409,
	latitude: 34.0544779,
	zoom: 2,
	pitch: 0,
	bearing: 0,
};

onMount(() => {
	createMap();
	createDeck();
	
	//make the layer reactive to changes, rerender each time layer data is changed
	let inputAPs = [
		{
			name: "myfakedata",
			position: [-77.67, 43.08]
		}
	];

	console.log(inputAPs[0].position)
	$: renderLayers({ data: inputAPs });
});

function createMap() {
	map = new maplibregl.Map({
		container: 'map',
		style: 'https://basemaps.cartocdn.com/gl/positron-gl-style/style.json',
		zoom: 7,
		bearing: 0,
		pitch: 0
	});
}

function createDeck() {
	deck = new Deck({
		canvas: canvasElement,
		width: "100%",
		height: "100%",
		initialViewState: viewState,
		controller: true,
		// Change the map's viewState whenever the view state of deck.gl changes.
		onViewStateChange: ({ viewState }) => {
			console.info("State change!");
			map.jumpTo({
				center: [viewState.longitude, viewState.latitude],
				zoom: viewState.zoom,
				bearing: viewState.bearing,
				pitch: viewState.pitch,
			});
		},
	});
}

// Use the `deck.setProps()` method to set the layers in deck.gl.
// See https://deck.gl/docs/api-reference/core/deck#layers.
function renderLayers(props) {
	// If `deck` is null then return early to prevent errors.
	if (!deck) {console.error("deck is null"); return; }
	console.info("HEREEEE!!!!!!!!!!!!");

	deck.setProps({
		layers: createDataLayers(props)
	});
}

function createDataLayers(props) {
	const {data} = props;
	return new ScatterplotLayer({
		id: "scatterplot",
		getPosition: d => d.coordinates,
		getFillColor: d => [0, 128, 255],
		getRadius: d => 5000,
		radiusScale: 6,
		opacity: 0.5,
		pickable: true,
		radiusMinPixels: 0.25,
		radiusMaxPixels: 30,
	});
}
</script>

<style>
#map {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
	background: #e5e9ec;
	overflow: hidden;
}
</style>
