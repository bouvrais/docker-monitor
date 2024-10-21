# Docker Monitor

Docker Monitor is a project that allows you to monitor your Docker containers across multiple machines. The vision for the tool is to not only monitor containers but also facilitate keeping them up to date whenever new images become available. It consists of three main components:

1. **Backend**: The backend is responsible for collecting and storing information about the Docker containers running on each machine. It provides an API for the web interface to retrieve this information.

2. **Agent**: The agent runs on each machine that you want to monitor. It communicates with the Docker daemon on the machine and sends container information to the backend.

3. **Web**: The web interface allows you to view the status and details of your Docker containers across all monitored machines. It retrieves the container information from the backend API.

## Setup

To set up and run the Docker Monitor project, follow these steps:

### Backend

1. Navigate to the `backend` directory.
2. Build the backend by running `cargo build`.
3. Configure the backend by modifying the `config.toml` file. Specify the machines you want to monitor and their corresponding addresses.
4. Start the backend server by running `cargo run`.

### Agent

1. Navigate to the `agent` directory.
2. Build the agent by running `cargo build`.
3. Start the agent on each machine you want to monitor by running `cargo run`.

### Web

1. Navigate to the `web` directory.
2. Install the required dependencies by running `npm install`.
3. Build the web interface by running `npm run build`. This will generate the runtime files in the `dist` folder.
4. Open your web browser and navigate to the address where the backend server is running. The backend will serve the static files needed to display the app in the browser and respond to API calls made by the frontend.

## Usage

Once all the components are set up and running, you can access the web interface by opening a web browser and navigating to the address where the web server is running.

The web interface will display a list of all the machines being monitored. Click on a machine to view the details of the Docker containers running on that machine.

## Contributing

If you would like to contribute to the Docker Monitor project, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them with descriptive commit messages.
4. Push your changes to your forked repository.
5. Submit a pull request to the main repository.

## License

This project is licensed under the [MIT License](LICENSE).
