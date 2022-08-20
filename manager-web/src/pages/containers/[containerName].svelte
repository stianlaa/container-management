<script lang="ts">
    import {
        requestContainerInfoByName,
        requestDefaultContainerOptions,
    } from "../../utils/api.ts";
    import {onMount} from "svelte";
    import {onInterval} from "../../utils/onInterval.js";
    import {getContainerState, ContainerState} from "../../utils/container.js"
    import {composeInfoStore} from "./_store.js"
    import ContainerRow from "../_components/ContainerRow.svelte";
    import ContainerLog from "../_components/ContainerLog.svelte";

    const UPDATE_INTERVAL_MS = 5000;

    export let containerName = null;

    let containerInfo = null;
    let commandLineArgsInput = null;
    let commandLineArgsButtonDisabled = true;

    let dockerComposeInfo = null;
    composeInfoStore.subscribe(value => {
        if (value !== null) {
            dockerComposeInfo = value;
        }
    });

    onMount(initialUpdate);

    onInterval(updateInfo, UPDATE_INTERVAL_MS);

    async function initialUpdate() {
        let [containerInfoResponse, defaultContainerOptions] = await Promise.all([requestContainerInfoByName(containerName), requestDefaultContainerOptions(containerName)]);
        if (getContainerState(containerName, containerInfoResponse) === ContainerState.Down) {
            commandLineArgsInput = defaultContainerOptions["entrypoint"];
        } else {
            containerInfo = containerInfoResponse;
            commandLineArgsInput = containerInfo["args"];
        }
    }

    async function updateInfo() {
        containerInfo = await requestContainerInfoByName(containerName);
    }

    async function modifyContainerCommandLineArgs(containerName, commandLineArgsInput) {
        // TODO first, remove the existing container, if it exists
        // TODO second, create new container, but with entrypoint overridden
    }
</script>

<style>
    .button-row > .btn {
        margin-left: 5px;
    }

    .page {
        display: flex;
        justify-content: center;
        text-align: center;
        margin: 20px
    }
</style>

<div class="page flex-container-vertical">
    <ContainerRow
            containerName={containerName}
            containerInfo={containerInfo}
            updateInfo={updateInfo}
            viewButton={false}
    />

    <div class="container-info">
        <p>
            <b>Image: </b>{dockerComposeInfo?.services[containerName] === null ? "unknown" : dockerComposeInfo.services[containerName]?.image}
        </p>
        <p>
            <b>Dependency: </b>{dockerComposeInfo?.services[containerName] === null ? "none" : dockerComposeInfo.services[containerName]?.depends_on}
        </p>
        <p><b>Command-line:</b></p>
        <input bind:value={commandLineArgsInput} on:input={() => commandLineArgsButtonDisabled = false}/>
        <a class={`btn blue-grey ${commandLineArgsButtonDisabled ? "disabled" : ""}`}
           on:click={modifyContainerCommandLineArgs(containerName, commandLineArgsInput)}>Save and
            recreate</a>
    </div>

    <ContainerLog
            containerId={containerInfo?.id}
            containerState={containerInfo?.state}
    />
</div>
