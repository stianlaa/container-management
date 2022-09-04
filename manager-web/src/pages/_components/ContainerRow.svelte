<script>
    import { url } from "@roxi/routify";
    import {
        requestDefaultContainerOptions,
        tryStartContainer,
        tryStopContainer,
        tryCreateContainer,
    } from "$utils/api";
    import {
        ContainerStatus,
        getContainerStatus,
        isRunning,
    } from "$utils/container";
    import { Circle } from "svelte-loading-spinners";
    import StatusTag from "./StatusTag.svelte";

    export let containerName = null;
    export let containerInfo = null;
    export let viewButton = true;
    export let updateInfo = () => {};
    let requestInProgress = false;

    async function onStartContainerClick(containerName, containerInfo) {
        requestInProgress = true;
        if (
            getContainerStatus(containerName, containerInfo) ===
            ContainerStatus.Down
        ) {
            let default_options = await requestDefaultContainerOptions(
                containerName
            );
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

<div class="flex-container-vertical">
    <div class="container-row flex-container-horizontal">
        <h5 class="row-name">
            {containerName}
        </h5>
        <p>{JSON.stringify(containerInfo)}</p>
        {#if requestInProgress}
            <Circle size="50" color="#607d8b" />
        {:else}
            <StatusTag {containerName} {containerInfo} />
        {/if}

        {#if isRunning(containerName, containerInfo)}
            <button
                class="entity-btn btn-large blue-grey"
                disabled={requestInProgress}
                on:click={onStopContainerClick(containerInfo)}
            >
                <i class="material-icons left">{"remove_circle_outline"}</i>
                Stop
            </button>
        {:else}
            <button
                class="entity-btn btn-large green darken-1"
                disabled={requestInProgress}
                on:click={onStartContainerClick(containerName, containerInfo)}
            >
                <i class="material-icons left">{"add_circle_outline"}</i>
                {getContainerStatus(containerName, containerInfo) ===
                ContainerStatus.Down
                    ? "Create default"
                    : "Start"}
            </button>
        {/if}

        {#if viewButton}
            <a
                href={$url("./:containerName", { containerName })}
                class="entity-btn btn-large blue-grey"
            >
                View
            </a>
        {/if}
    </div>
</div>

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
