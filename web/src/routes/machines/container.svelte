<script lang="ts">
  class Port {
    IP?: string;
    PublicPort?: number;
    PrivatePort?: number;
    Type?: string;
  }
  export let container: {
    id: string;
    name: string;
    image: string;
    status: string;
    created: Date;
    ports: Port[];
  };
</script>

<div class="container-details">
  <h2>{container.name.replace(/^\//g, '')}</h2>
  <div class="detail-row">
    <span class="label">ID:</span>
    <span>{container.id}</span>
  </div>
  <div class="detail-row">
    <span class="label">Image:</span>
    <span>{container.image}</span>
  </div>
  <div class="detail-row">
    <span class="label">Status:</span>
    <span class={`status ${container.status.toLowerCase()}`}>{container.status}</span>
  </div>
  <div class="detail-row">
    <span class="label">Created:</span>
    <span>{new Date(container.created).toLocaleString()}</span>
  </div>
  <div class="detail-row">
    <span class="label">Ports:</span>
    <span>
      {container.ports.map(port => {
        let portString = '';
        if (port.IP) portString += `${port.IP} - `;
        if (port.PublicPort) portString += `${port.PublicPort}:`;
        portString += `${port.PrivatePort}`;
        if (port.Type) portString += ` (${port.Type})`;
        return portString;
      }).join(', ') || 'None'}
    </span>
  </div>
</div>

<style>
  .container-details {
    background-color: #111115;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    margin: 10px;
  }

  h2 {
    margin-top: 0;
    margin-bottom: 20px;
    color: #858585;
  }

  .detail-row {
    display: flex;
    margin-bottom: 10px;
  }

  .label {
    font-weight: bold;
    min-width: 100px;
  }

  .status {
    padding: 4px 8px;
    border-radius: 4px;
    font-weight: bold;
  }

  .status.running {
    background-color: #4caf50;
    color: white;
  }

  .status.exited {
    background-color: #f44336;
    color: white;
  }

  .status.paused {
    background-color: #ff9800;
    color: white;
  }
</style>
