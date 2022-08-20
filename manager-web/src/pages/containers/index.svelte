<script>
    import {composeInfoStore, containerListStore} from "./_store"
    import {
        requestDockerCompose,
        listContainers,
    } from "$utils/api";
    import {onMount} from "svelte";
    import {onInterval} from "$utils/onInterval";
    import ContainerRow from "../_components/ContainerRow.svelte";

    let dockerComposeInfo = null;
    composeInfoStore.subscribe(value => {
        if (value !== null) {
            dockerComposeInfo = value;
        }
    });

    // TODO note: containerList[containerName] is not the same as containerInfo, the two should be merged or separated clearly, so there is no confusion
    let containerList = null;
    containerListStore.subscribe(value => {
        if (value !== null) {
            containerList = value;
        }
    });

    onInterval(async () => await updateInfo(), 5000);

    onMount(async () => {
        let composeInfo = await requestDockerCompose();
        composeInfoStore.set(composeInfo);
    })

    async function updateInfo() {
        let containerList = await listContainers();
        containerListStore.set(containerList);
    }
</script>


<style>
    .page {
        display: flex;
        justify-content: center;
        text-align: center;
    }
</style>

<div class="page flex-container-vertical">
    <h4>Containers</h4>
    {#if dockerComposeInfo && containerList}
    {#each Object.entries(dockerComposeInfo["services"]) as [containerName, composeInfo]}
        <ContainerRow
                containerName={containerName}
                containerInfo={containerList[containerName]}
                updateInfo={updateInfo}
        />
    {/each}
    {:else}
        <p>Fetching container overview..</p>
    {/if}
</div>
