<script>
	let { access_point, onclick = undefined } = $props();
	console.log(onclick);
</script>

<div>
	<h3>{access_point.name}</h3>
	<p>{access_point.type} that is currently {access_point.status}</p>
	{#if onclick !== undefined}
		<button {onclick}>Remove</button>
	{/if}
</div>
