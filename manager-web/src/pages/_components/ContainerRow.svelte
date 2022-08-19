<script>
    import {url} from '@roxi/routify'
    import {
        requestDefaultContainerOptions,
        tryStartContainer,
        tryStopContainer, tryCreateContainer,
    } from "$utils/api";
    import {ContainerState, getContainerState, isRunning} from "$utils/container";
    import {Circle} from "svelte-loading-spinners";
    import StatusTag from "./StatusTag.svelte";

    export let name;
    export let containerInfo;
    export let updateInfo;
    let requestInProgress = false;

    async function onStartContainerClick(name, containerInfo) {
        requestInProgress = true;
        if (getContainerState(name, containerInfo) === ContainerState.Down) {
            let default_options = await requestDefaultContainerOptions(name);
            await tryCreateContainer(default_options);
        } else {
            await tryStartContainer(containerInfo.id);
        }
        requestInProgress = false;
        updateInfo();
    }

    async function onStopContainerClick(containerInfo) {
        requestInProgress = true;
        await tryStopContainer(containerInfo.id);
        requestInProgress = false;
        updateInfo();
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
</style>

<div class="page flex-container-vertical">
    <div class="container-row flex-container-horizontal">
        <h5 class="row-name">
            {name}
        </h5>
        {#if requestInProgress}
            <Circle size="50" color="#607d8b"/>
        {:else}
            <StatusTag containerName={name} containerInfo={containerInfo}/>
        {/if}

        {#if isRunning(name, containerInfo)}
            <button class="entity-btn btn-large blue-grey" disabled={requestInProgress}
                    on:click={onStopContainerClick(containerInfo)}>
                <i class="material-icons left">{"remove_circle_outline"}</i>
                Stop
            </button>
        {:else}
            <button class="entity-btn btn-large green darken-1" disabled={requestInProgress}
                    on:click={onStartContainerClick(name, containerInfo)}>
                <i class="material-icons left">{"add_circle_outline"}</i>
                {getContainerState(name, containerInfo) === ContainerState.Down ? "Create" : "Start"}
            </button>
        {/if}

        <a href={$url("./:name", { name })}
           disabled={getContainerState(name, containerInfo) === ContainerState.Down || null}
           class="entity-btn btn-large blue-grey">
            View
        </a>
        <br/>
    </div>
</div>
