<script lang="ts">
  import Counter from './lib/Counter.svelte'
  import Homemap from './Homemap.svelte';
    import BottomBar from './BottomBar.svelte';
  async function get_message() {
    let res = await fetch("backend-msg");
    let message = await res.text();
    return message;
  }
  let promise = get_message();
</script>

<main>
        <Homemap />
        <div class = "wrapper">
                <BottomBar />
        </div>
</main>

<style>
        main{
                width: 100%;
                height: 100vh;
        }

        .wrapper{
                height: 100%;
                display:flex;
                justify-content: center;
                align-items: flex-end;
        }
</style>