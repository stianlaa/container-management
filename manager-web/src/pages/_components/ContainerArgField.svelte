<script lang="ts">
    import {
        requestDefaultContainerOptions,
        tryCreateContainer,
        tryRemoveContainer, tryStartContainer,
        tryStopContainer
    } from "../../utils/api.ts";
    import {onMount} from "svelte";
    import {getContainerStatus, ContainerStatus} from "../../utils/container.js"
    import {Circle} from "svelte-loading-spinners";

    export let containerName = null;
    export let containerInfo = null;
    let commandLineArgsInput = null;
    let commandLineArgsButtonDisabled = true;
    let defaultContainerOptions = null;
    let requestInProgress = false;

    onMount(initialUpdate);

    async function initialUpdate() {
        defaultContainerOptions = await requestDefaultContainerOptions(containerName);
        if (getContainerStatus(containerName, containerInfo) === ContainerStatus.Down) {
            commandLineArgsInput = defaultContainerOptions["entrypoint"].join(", ");
        } else {
            commandLineArgsInput = [`${containerName}`, ...containerInfo["args"]].join(", ");
        }
    }

    async function modifyContainerCommandLineArgs(defaultOptions, containerId) {
        let options = defaultOptions;
        options["entrypoint"] = parseArgString(commandLineArgsInput)
        requestInProgress = true;
        await tryStopContainer(containerId)
            .then(() => tryRemoveContainer(containerId))
            .then(() => {
                return tryCreateContainer(options)
            })
            .then((response) => tryStartContainer(response.id));

        requestInProgress = false;
    }

    function parseArgString(argString) {
        return argString?.split(",")?.map((arg) => arg.trim());
    }
</script>

<style>
    .outer-container {
        display: flex;
        justify-content: center;
    }

    .btn {
        width: 25%;
    }

    .btn-container {
        display: flex;
        margin: 5px auto 5px auto;
        width: 100%;
        justify-content: center;
    }
</style>

<div class="outer-container flex-container-vertical">
    <p><b>Arguments:</b></p>
    <input bind:value={commandLineArgsInput} on:input={() => commandLineArgsButtonDisabled = false}/>

    <div class="btn-container">
        {#if requestInProgress}
            <Circle size="50" color="#607d8b"/>
        {:else}
            <button class={`btn blue-grey ${commandLineArgsButtonDisabled ? "disabled" : ""}`}
                    on:click={modifyContainerCommandLineArgs(defaultContainerOptions, containerInfo.id)}>
                Save and recreate
            </button>
        {/if}

    </div>
</div>
