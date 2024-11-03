<script>
import APCard from './APCard.svelte';
let aps = [
	{name:"Wheelchair ramp",type:"ramp",status:"working"},
	{name:"Elevator",type:"elevator",status:"not working"},
];
</script>

<ul>
	{#each aps as ap}
	<li>
		<APCard access_point={ap}/>
	</li>
	{/each}
</ul>
