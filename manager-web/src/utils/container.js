export const ContainerState = {
    Created: "Created",
    Running: "Running",
    Restarting: "Restarting", // A running container that is in the process of restarting
    Exited: "Exited", // A running container that is stopped.
    Paused: "Paused", // A running container that is paused
    Dead: "Dead",
    Unknown: "Unknown",
    Down: "Down", // An entry exists docker-compose, but no container exists
}

export function getContainerState(containerName, containerInfo) {
    if (containerInfo?.state == null) {
        return ContainerState.Down;
    }
    return containerInfo?.state;
}

export function isRunning(containerName, containerInfo) {
    return getContainerState(containerName, containerInfo) === ContainerState.Running;
}
