<script>
    let {name,status,apProps} = $props();
    async function reportProblem(props) {
        try {
            const url = `/ap/issue/${props.object.id}`;
            const response = await fetch(url,{method:"PUT"});
            if (!response.ok) {
            console.log(response.status);
            }
            // ...
        } catch (error) {
            console.error(error.message);
        }
    }
</script>

<div class="tooltip">
    <span>
      <p>{name}</p>
      <p class = status>{status}</p>
    </span>
    <span>
      <button>Add to my routine</button>
      <button onclick={()=>console.log("clicked")}>Report as broken</button>
    </span>
    
  </div>

  <style>
    .tooltip{
        width: 90%;
        height: 15%;
        background: #ffffff;
        border-radius: 20px;
        margin-bottom: 1em;
        color: #fff;
        font-size: 16px;
        z-index: 9;
    }

    button{
        background-color: aqua;
    }
  </style>