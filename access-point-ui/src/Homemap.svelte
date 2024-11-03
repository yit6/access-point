{#if true}
    <Infocard />
{/if}
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
import Infocard from "./Infocard.svelte";

let mapElement;
let canvasElement;
let map = null;
let deck = null;
let hover = {
  x: 0,
  y: 0,
  hoveredObject: null
};
let viewState = {
	longitude: -77.671,
	latitude: 43.0849,
	zoom: 15,
	pitch: 0,
	bearing: 0,
};



onMount(() => {
	createMap();
	createDeck();

fetch("/ap/").then(promise => {
		promise.json().then(aps => {
			renderLayers({ data: Object.values(aps) });
      console.log(Object.values(aps))
		});
	});
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
		controller: {dragRotate: false},
		// Change the map's viewState whenever the view state of deck.gl changes.
		onViewStateChange: ({ viewState }) => {
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
	if (!deck) { console.error("deck is null"); return; }

	deck.setProps({
		layers: createDataLayers(props)
	});
}

function createDataLayers(props) {
	const {data} = props;
	return new ScatterplotLayer({
		id: "scatterplot",
		data: data,
		getPosition: d => [d.location.long, d.location.lat],
		getFillColor: d => [0, 128, 255],
		getRadius: d => 5,
		radiusScale: 6,
		opacity: 0.5,
		pickable: true,
		radiusMinPixels: 0.25,
		radiusMaxPixels: 30,
    		onClick: (hoverProps) => handleHover("scatterplotLayer", hoverProps),
	});
}

function handleHover(layerType, hoverProps) {
  let name, status;
  if (layerType === "scatterplotLayer") {
    console.log(hoverProps.object)
    name = hoverProps.object.name;
    status = hoverProps.object.status;
  }
  // Set the coordinates for the tooltip.
  hover.x = hoverProps.x;
  hover.y = hoverProps.y;
  hover.hoveredObject = hoverProps.object;
  hover.name = name;
  hover.status = status;
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

#deck-canvas {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
}


.UI-overlay{
  height: 100%;
  position: absolute;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
}

p{
  color:rgb(0,0,0)
}

.status{
  color:rgb(0,0,0)
}

</style>
