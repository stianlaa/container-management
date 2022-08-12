export function getContainerState(containerName, composeInfo, containerList) {
    if (composeInfo["services"][containerName] && containerList[containerName]?.state == null) {
        return "Down";
    }
    return containerList[containerName]?.state;
}
