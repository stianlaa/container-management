<script>
    import {url} from '@roxi/routify'
    import {composeInfoStore, containerListStore} from "./_store"
    import {
        requestDockerCompose,
        listContainers,
    } from "../../utils/api";
    import {onInterval} from "../../utils/onInterval";
    import {ContainerState, getContainerState} from "../../utils/container";

    // TODO: it seems like service.name is something a bit different than container names, and that they can't automaticallly be used interchangably

    // TODO view button only makes sense if a container is not down, button should perhaps be disabled in this case
    let composeInfo = null;
    composeInfoStore.subscribe(value => {
        if (value !== null) {
            composeInfo = value;
        }
    });

    let containerList = null;
    containerListStore.subscribe(value => {
        if (value !== null) {
            containerList = value;
        }
    });

    function isRunning(containerName) {
        return getContainerState(containerName, composeInfo, containerList) === ContainerState.Running;
    }

    onInterval(async () => {
        let [composeInfo, containerList] = await Promise.all([requestDockerCompose(), listContainers()]);
        composeInfoStore.set(composeInfo);
        containerListStore.set(containerList);
    }, 5000);
</script>


<style>
    .page {
        display: flex;
        justify-content: center;
        text-align: center;
    }

    .container-entry {
        display: flex;
        width: 75%;
        margin: 20px auto 20px auto;
        padding: 10px 10px 10px 10px;
        border: 2px solid grey;
        border-radius: 5px;

        justify-content: space-between;
    }

    .entry-name {
        display: flex;
        flex-grow: 4;
        margin: auto 0 auto 20px;
    }
</style>

<div class="page flex-container-vertical">
    <h4>Containers</h4>
    {#if composeInfo && containerList}
    {#each Object.entries(composeInfo["services"]) as [containerName, composeServiceObject]}
        <div class="container-entry flex-container-horizontal">
            <h5 class="entry-name">{containerName} ({getContainerState(containerName, composeInfo, containerList)})</h5>

            <button class={`entity-btn btn-large`}>
                Start
            </button>

            <button class={`entity-btn btn-large`}>
                Stop
            </button>

            <a href={$url("./:containerName", { containerName })}
               class={`entity-btn btn-large ${isRunning(containerName) ? "blue-grey" : "yellow darken-4"}`}>
                <i class="material-icons left">{isRunning(containerName) ? "info" : "warning"}</i>
                View
            </a>
            <br/>
        </div>
    {/each}
    {:else}
        <p>Fetching container overview..</p>
    {/if}
</div>
