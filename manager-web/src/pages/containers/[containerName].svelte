<script lang="ts">
    import {requestContainerInfoByName} from "../../utils/api.ts";
    import {onMount} from "svelte";
    import {onInterval} from "../../utils/onInterval.js";
    import {getContainerStatus, ContainerStatus} from "../../utils/container.js"
    import {composeInfoStore} from "./_store.js"
    import ContainerRow from "../_components/ContainerRow.svelte";
    import ContainerLog from "../_components/ContainerLog.svelte";
    import ContainerArgField from "../_components/ContainerArgField.svelte";

    const UPDATE_INTERVAL_MS = 5000;

    export let containerName = null;

    let containerInfo = null;

    let dockerComposeInfo = null;
    composeInfoStore.subscribe(value => {
        if (value !== null) {
            dockerComposeInfo = value;
        }
    });

    onMount(initialUpdate);

    onInterval(updateInfo, UPDATE_INTERVAL_MS);

    async function initialUpdate() {
        let containerInfoResponse = await requestContainerInfoByName(containerName);
        if (getContainerStatus(containerName, containerInfoResponse) !== ContainerStatus.Down) {
            containerInfo = containerInfoResponse;
        }
    }

    async function updateInfo() {
        containerInfo = await requestContainerInfoByName(containerName);
    }
</script>

<style>
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

    <div>
        <p>
            <b>Image: </b>{dockerComposeInfo?.services[containerName] === null ? "unknown" : dockerComposeInfo.services[containerName]?.image}
        </p>
        <br/>
        <p>
            <b>Dependency: </b>{dockerComposeInfo?.services[containerName] === null ? "none" : dockerComposeInfo.services[containerName]?.depends_on}
        </p>
    </div>

    <ContainerArgField containerName={containerName} containerInfo={containerInfo}/>

    <ContainerLog
            containerId={containerInfo?.id}
            containerStatus={containerInfo?.status}
    />
</div>
