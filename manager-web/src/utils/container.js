export const ContainerStatus = {
    Created: "Created",
    Running: "Running",
    Restarting: "Restarting", // A running container that is in the process of restarting
    Exited: "Exited", // A running container that is stopped.
    Paused: "Paused", // A running container that is paused
    Dead: "Dead",
    Unknown: "Unknown",
    Down: "Down", // An entry exists in compose.yml, but no container exists
}

export function getContainerStatus(containerName, containerInfo) {
    if (containerInfo?.status == null) {
        return ContainerStatus.Down;
    }
    return containerInfo?.status;
}

export function isRunning(containerName, containerInfo) {
    return getContainerStatus(containerName, containerInfo) === ContainerStatus.Running;
}
