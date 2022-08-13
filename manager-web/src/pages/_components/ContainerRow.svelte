<script>
    import {url} from '@roxi/routify'
    import {
        tryActivateContainer,
        tryDeactivateContainer, tryCreateContainer,
    } from "../../utils/api";
    import {ContainerState, createContainerArgs, getContainerState} from "../../utils/container";
    import {Circle} from "svelte-loading-spinners";

    export let name;
    export let composeInfo;
    export let containerInfo;
    export let updateInfo;
    let requestInProgress = false;

    function isRunning(name, composeInfo, containerInfo) {
        return getContainerState(name, composeInfo, containerInfo) === ContainerState.Running;
    }

    function onActivateContainerClick(name) {
        requestInProgress = true;
        if (getContainerState(name, composeInfo, containerInfo) === ContainerState.Down) {
            tryCreateContainer(createContainerArgs(name, composeInfo))
                .then(() => {requestInProgress = false; updateInfo()})
        } else {
            tryActivateContainer(containerInfo.id)
                .then(() => {requestInProgress = false; updateInfo()})
        }
    }

    function onDeactivateContainerClick(containerId) {
        requestInProgress = true;
        tryDeactivateContainer(containerId)
            .then(() => {requestInProgress = false; updateInfo()})
    }

    function statusTag(name, composeInfo, containerInfo) {
        switch (getContainerState(name, composeInfo, containerInfo)) {
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
    .container-row {
        display: flex;
        width: 75%;
        margin: 20px auto 20px auto;
        padding: 10px 10px 10px 10px;
        border: 2px solid grey;
        border-radius: 5px;

        justify-content: space-between;
    }

    .row-name {
        display: flex;
        margin: auto 0 auto 20px;
    }

    .entity-btn {
        margin-left: 20px;
    }

    .status-tag {
        margin: auto auto auto 20px;
        border-radius: 15px;
        padding: 8px;
        color: white;
    }
</style>

<div class="page flex-container-vertical">
    <div class="container-row flex-container-horizontal">
        <h5 class="row-name">
            {name}
        </h5>
        {#if requestInProgress}
            <Circle size="50" color="#607d8b"/>
        {:else}
            <div class={`status-tag ${statusTag(name, composeInfo, containerInfo).color}`}>
                <i class="material-icons left">{statusTag(name, composeInfo, containerInfo).icon}</i>
                <h6 style="display: inline">({getContainerState(name, composeInfo, containerInfo)})</h6>
            </div>
        {/if}

        {#if isRunning(name, composeInfo, containerInfo)}
            <button class="entity-btn btn-large blue-grey"
                    on:click={onDeactivateContainerClick(containerInfo.id)}>
                <i class="material-icons left">{"remove_circle_outline"}</i>
                Deactivate
            </button>
        {:else}
            <button class="entity-btn btn-large green darken-1"
                    on:click={onActivateContainerClick(name)}>
                <i class="material-icons left">{"add_circle_outline"}</i>
                {getContainerState(name, composeInfo, containerInfo) === ContainerState.Down ? "Create" : "Activate"}
            </button>
        {/if}

        <a href={$url("./:name", { name })}
           disabled={!isRunning(name, composeInfo, containerInfo) || null}
           class="entity-btn btn-large blue-grey">
            View
        </a>
        <br/>
    </div>
</div>
