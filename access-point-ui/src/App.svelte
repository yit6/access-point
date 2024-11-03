<script lang="ts">
  import Counter from './lib/Counter.svelte'
  import Homepage from './Homemap.svelte';
    import Homemap from './Homemap.svelte';
  async function get_message() {
    let res = await fetch("backend-msg");
    let message = await res.text();
    console.log(message);
    return message;
  }
  let promise = get_message();
</script>

<main>
        <p>Svelte frontend</p>
        <Counter />
        {#await promise}
                <p>Loading...</p>
        {:then message}
                <p>{message}</p>
        {/await}
        <Homemap />
</main>
