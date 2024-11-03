<script>
	let { access_point } = $props();
</script>

<div>
	<h3>{access_point.name}</h3>
	<p>{access_point.type} that is currently {access_point.status}</p>
</div>
