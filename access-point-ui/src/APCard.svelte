<script>
	import "./tw.css";
	let { access_point, onclick = undefined } = $props();
	console.log(onclick);
</script>

<div class="bg-blue-300 border-blue-600 p-4 m-4 rounded-lg">
	<h3>{access_point.name}</h3>
	<p>{access_point.nice_kind} that is currently {access_point.nice_status}</p>
	{#if onclick !== undefined}
		<button class="flex bg-blue-400 border-blue-600 p-4 m-4 justify-center rounded-lg" {onclick}>Remove</button>
	{/if}
</div>
