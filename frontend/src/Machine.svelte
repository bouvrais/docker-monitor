<script>
  export let machine;
</script>

<div class="machine">
  <h2>{machine.name}</h2>
  <p>Address: {machine.address}</p>
  <h3>Containers:</h3>
  <ul>
    {#each machine.containers as container}
      <li>
        {container.name} ({container.image}) - {container.status}
      </li>
    {/each}
  </ul>
</div>

<style>
  .machine {
    border: 1px solid #ccc;
    border-radius: 5px;
    padding: 1em;
    margin-bottom: 1em;
  }

  ul {
    list-style-type: none;
    padding: 0;
  }

  li {
    margin-bottom: 0.5em;
  }
</style>
