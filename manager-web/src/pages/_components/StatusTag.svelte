<script>
    import {ContainerState, getContainerState} from "$utils/container";

    export let containerName;
    export let containerInfo;

    function statusTag(name, containerInfo) {
        switch (getContainerState(name, containerInfo)) {
            case ContainerState.Running:
                return {icon: "thumb_up", color: "green lighten-3"};
            case ContainerState.Created:
            case ContainerState.Unknown:
            case ContainerState.Restarting:
                return {icon: "live_help", color: "grey lighten-3"};
            default:
                return {icon: "thumb_down", color: "red lighten-3"};
        }
    }
</script>


<style>
    .status-tag {
        margin: auto auto auto 20px;
        border-radius: 15px;
        padding: 8px;
        color: white;
    }
</style>

<div class={`status-tag ${statusTag(containerName, containerInfo).color}`}>
    <i class="material-icons left">{statusTag(containerName, containerInfo).icon}</i>
    <h6 style="display: inline">({getContainerState(containerName, containerInfo)})</h6>
</div>