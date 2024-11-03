<script lang="ts">
import Counter from './lib/Counter.svelte'
import Homemap from './Homemap.svelte';
import BottomBar from './BottomBar.svelte';

let name = document.cookie.split(";").find((row)=>row.trim().startsWith("username="));
console.log(name);
if (name == undefined || name.split('=').length == 1) {
	window.location.href="/login/index.html";
} else {
	name = name.split('=')[1];
}

async function get_message() {
	let res = await fetch("backend-msg");
	let message = await res.text();
	return message;
}
let promise = get_message();
</script>

<main>  
        <Homemap />    
        <BottomBar />
</main>

<style>
        main{
                width: 100%;
                height: 100vh;
                display: flex;
                flex-direction: column;
                justify-content: flex-end;
                align-items: center;
        }

        .wrapper{
                height: 100%;
                display:flex;
                justify-content: center;
                align-items: flex-end;
        }
</style>
