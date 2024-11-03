<script>
    //name and status are the name and status of the access point
    //hoverPoint is the object of the access point being hovered over on
    let {info} = $props();

    
</script>

<div class="tooltip">
    <span>
      <p>{name}</p>
      <p class = status>{status}</p>
    </span>
    <span>
      <button>Add to my routine</button>
      <button onclick={()=>reportProblem(hoverPoint)}>Report as broken</button>
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