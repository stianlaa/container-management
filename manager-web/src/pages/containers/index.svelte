<script>
    import {url} from '@roxi/routify'
    import {composeInfoStore, containerListStore} from "./_store"
    import {
        requestDockerCompose,
        listContainers,
        tryActivateContainer,
        tryDeactivateContainer,
    } from "../../utils/api";
    import {onInterval} from "../../utils/onInterval";
    import {ContainerState, getContainerState} from "../../utils/container";

    // TODO introduce in_progress member, where icon is changed to spinner while working

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

    .entity-btn {
        margin-left: 20px;
    }
</style>

<div class="page flex-container-vertical">
    <h4>Containers</h4>
    {#if composeInfo && containerList}
    {#each Object.entries(composeInfo["services"]) as [containerName, composeServiceObject]}
        <div class="container-entry flex-container-horizontal">
            <h5 class="entry-name">{containerName} ({getContainerState(containerName, composeInfo, containerList)})</h5>

            {#if isRunning(containerName)}
                <button class="entity-btn btn-large blue-grey"
                        on:click={tryDeactivateContainer(containerName)}>
                    <i class="material-icons left">{"remove_circle_outline"}</i>
                    Stop
                </button>
            {:else}
                <button class="entity-btn btn-large green darken-1"
                        on:click={tryActivateContainer(containerName)}>
                    <i class="material-icons left">{"add_circle_outline"}</i>
                    Start
                </button>
            {/if}


            <a href={$url("./:containerName", { containerName })}
               disabled={!isRunning(containerName) || null}
               class="entity-btn btn-large blue-grey">
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
