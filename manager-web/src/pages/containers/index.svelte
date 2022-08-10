<script>
    import {url} from '@roxi/routify'
    import {composeInfoStore} from "./_store"
    import {
        requestDockerCompose,
    } from "../../utils/api";
    import {onMount} from "svelte";

    let containers = {
        "arg-poc": {
            name: "arg-poc",
            description: "description1",
            status: "Active",
        },
        "network-poc": {
            name: "network-poc",
            description: "description2",
            status: "Inactive",
        }
    }

    let images = {
        "first-image": {
            name: "image 1",
        },
        "second-image": {
            name: "image 2",
        }
    }

    let selectedImage = Object.values(images).length > 0 ? images[0] : null;
    let answerImage;
    let newContainerName = "";

    let composeInfo = null;
    composeInfoStore.subscribe(value => {
        if (value !== null) {
            composeInfo = value;
        }
    });

    onMount(async () => {
        let composeInfo = await requestDockerCompose();
        composeInfoStore.set(composeInfo);
    });

    // TODO: display all those containers, and active if they do exist
    // TODO: this way there will be no create container functionality, it will simply be to activate one of them, potentially with different args.
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

    .image-entry {
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

    .image-select {
        display: flex;
        width: 200px;
        height: 50px;
        margin: 20px 0 20px 0;
    }

    .image-namefield {
        display: flex;
        width: 200px;
        height: 50px;
        margin: 0 20px 0 20px;
    }
</style>

<div class="page flex-container-vertical">
    <h4>Containers:</h4>
    {#if composeInfo}
    {#each Object.entries(composeInfo["services"]) as [containerName, composeServiceObject]}
        <div class="container-entry flex-container-horizontal">
            <h5 class="entry-name">{containerName}</h5>
            <a href={$url("./:containerName", { containerName })}
               class={`entity-btn btn-large ${containers[containerName]?.status === "Active" ? "blue-grey" : "yellow darken-4"}`}>
                <i class="material-icons left">{containers[containerName]?.status === "Active" ? "info" : "warning"}</i>
                View
            </a>
            <br/>
        </div>
    {/each}
    {:else}
        <p>Fetching service list..</p>
    {/if}
</div>
