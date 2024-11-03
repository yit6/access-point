{#if hover.hoveredObject}
  <div class="tooltip">
    <span>
      <p>{hover.hoveredObject.name}</p>
      <p class = status>{hover.hoveredObject.status}</p>
    </span>
    <span>
      <button onclick={()=>add_access_point(hover.hoveredObject.id)}>Add to my routine</button>
      <button onclick={()=>reportProblem(hover.hoveredObject.id)}>Report as broken</button>
    </span> 
</div>
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

async function reportProblem(APprops) {
        try {
            const url = `/ap/issue/${APprops.object.id}`;
            const response = await fetch(url,{method:"PUT"});
            if (!response.ok) {
            console.log(response.status);
            }
        } catch (error) {
            console.error(error.message);
        }
    }

async function add_access_point(id) {
  let uname = document.cookie.split(";").find((row)=>row.trim().startsWith("username="));
  if (uname == undefined || uname.split('=').length == 1) {
	window.location.href="/login/index.html";
  } else {
	  uname = uname.split('=')[1];
  }
  try {
      const response = await fetch("/user/add",{method:"POST",
        body: JSON.stringify({username : uname, access_point : id})
      });
  } catch (error) {
    console.error(error.message);
  }
}    


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

.tooltip{
  width: 90%;
  height: 15%;
  background: #ffffff;
  border-radius: 20px;
  margin-bottom: 1em;
  color: #fff;
  font-size: 16px;
  z-index: 9;        
}

p{
  color:rgb(0,0,0)
}

.status{
  color:rgb(0,0,0)
}

</style>
