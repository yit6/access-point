<script>
import APCard from './APCard.svelte';
import { onMount } from 'svelte';
let useraps = $state([ ]);
let username = "yit";

onMount(async () => {
	let res = await fetch(`/user/${username}`);
	let user = await res.json();
	user.access_points.forEach(async (id) => {
		let res = await fetch(`/ap/${id}`);
		let ap = await res.json();
		console.log(ap);
		useraps.push(ap);
		console.log(useraps);
	});
});

let remove = (ap_remove) => {
	return () => {
		useraps = useraps.filter(ap => ap != ap_remove);
	}
}
</script>

<ul>
	{#each useraps as ap}
	<li>
		<APCard access_point={ap} onclick={remove(ap)}/>
	</li>
	{:else}
	<li><p>Loading...</p></li>
	{/each}
</ul>
