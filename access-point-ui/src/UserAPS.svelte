<script>
import APCard from './APCard.svelte';
import BottomBar from './BottomBar.svelte';
import Logout from './Logout.svelte';
import { onMount } from 'svelte';
let useraps = $state([ ]);

let name = document.cookie.split(";").find((row)=>row.trim().startsWith("username="));
console.log(name);
if (name == undefined || name.split('=').length == 1) {
	window.location.href="/login/index.html";
} else {
	name = name.split('=')[1];
}

onMount(async () => {
	let res = await fetch(`/user/${name}`);
	let user = await res.json();
	user.access_points.forEach(async (id) => {
		let res = await fetch(`/ap/${id}`);
		let ap = await res.json();
		useraps.push(ap);
		useraps.sort((a, b) => {
			a = a.status;
			b = b.status;

			if (a == "NotWorking") { a = 0; }
			if (a ==   "InRepair") { a = 1; }
			if (a ==    "Working") { a = 2; }

			if (b == "NotWorking") { b = 0; }
			if (b ==   "InRepair") { b = 1; }
			if (b ==    "Working") { b = 2; }

			return a-b;
		});
		console.log(useraps);
	});
});

let remove = (ap_remove) => {
	return async () => {
		fetch(`/user/remove/${name}/${ap_remove.id}`, {method: 'DELETE'}).then(() => {
			useraps = useraps.filter(ap => ap != ap_remove);
		});
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
<Logout />
<BottomBar />
