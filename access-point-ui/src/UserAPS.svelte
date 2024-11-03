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
</script>

<ul>
	{#each useraps as ap}
	<li>
		<APCard access_point={ap}/>
	</li>
	{:else}
	<li><p>Loading...</p></li>
	{/each}
</ul>
