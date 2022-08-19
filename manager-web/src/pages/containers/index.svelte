<script>
    import {composeInfoStore, containerListStore} from "./_store"
    import {
        requestDockerCompose,
        listContainers,
    } from "../../utils/api";
    import {onInterval} from "../../utils/onInterval";
    import ContainerRow from "../_components/ContainerRow.svelte";

    let dockerComposeInfo = null;
    composeInfoStore.subscribe(value => {
        if (value !== null) {
            dockerComposeInfo = value;
        }
    });

    let containerList = null;
    containerListStore.subscribe(value => {
        if (value !== null) {
            containerList = value;
        }
    });

    onInterval(async () => await updateInfo(), 5000);

    // TODO consider removing compose from regular update and keep in onMount instead
    async function updateInfo() {
        let [composeInfo, containerList] = await Promise.all([requestDockerCompose(), listContainers()]);
        composeInfoStore.set(composeInfo);
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
                name={containerName}
                containerInfo={containerList[containerName]}
                updateInfo={updateInfo}
        />
    {/each}
    {:else}
        <p>Fetching container overview..</p>
    {/if}
</div>
