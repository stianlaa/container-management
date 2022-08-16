<script lang="ts">
    import {
        requestContainerInfo,
        requestContainerLogsLast,
        requestContainerLogsSpan,
        tryActivateContainer,
        tryDeactivateContainer
    } from "../../utils/api.ts";
    import {afterUpdate, onMount} from "svelte";
    import {onInterval} from "../../utils/onInterval.js";
    import {isRunning} from "../../utils/container.js"
    import {composeInfoStore} from "./_store.js"
    import {formatDuration} from "../../utils/dateTimeUtils"
    import StatusTag from "../_components/StatusTag.svelte";
    import {Circle} from "svelte-loading-spinners";

    export let containerName = null;

    const UPDATE_INTERVAL_MS = 2000;

    let log = {
        messages: "",
        lastUpdatedTimestamp: Date.now(),
        autoScroll: true,
        updateError: false,
    };
    let containerInfo = null;
    let commandLineArgsInput = null;
    let commandLineArgsButtonDisabled = true;
    let requestInProgress = false;

    let dockerComposeInfo = null;
    composeInfoStore.subscribe(value => {
        if (value !== null) {
            dockerComposeInfo = value;
        }
    });

    onMount(updateInfo);

    onInterval(async () => await fetchLogs(), UPDATE_INTERVAL_MS);

    async function updateInfo() {
        containerInfo = await requestContainerInfo(containerName);
        log.messages += await requestContainerLogsLast(containerInfo.id, 100);
    }

    async function onActivateContainerClick(containerInfo) {
        requestInProgress = true;
        await tryActivateContainer(containerInfo.id);
        requestInProgress = false;
        await updateInfo();
    }

    async function onDeactivateContainerClick(containerInfo) {
        requestInProgress = true;
        await tryDeactivateContainer(containerInfo.id);
        requestInProgress = false;
        await updateInfo();
    }

    async function fetchLogs() {
        if (containerInfo !== null) {
            let now = Date.now();
            let logResponse = await requestContainerLogsSpan(containerInfo.id, log.lastUpdatedTimestamp, now);
            if (logResponse === null) {
                log.updateError = true;
            } else {
                log.updateError = false;
                log.messages += logResponse;
                log.lastUpdatedTimestamp = now;
            }
        }
    }

    let logContent;
    afterUpdate(() => {
        if (log.autoScroll) {
            logContent.scrollTop = logContent.scrollHeight;
        }
    })
</script>

<style>
    .container-log-textfield {
        overflow: auto;
        font-family: monospace;
        white-space: pre;
        resize: none;
        height: 600px;
    }

    .button-row > .btn {
        margin-left: 5px;
    }

    .log-info {
        font-family: monospace, monospace;
    }

    .log-info [type="checkbox"] {
        position: unset;
        opacity: 1;
    }

    .log-outdated-text {
        color: red;
    }

    .page {
        display: flex;
        justify-content: center;
        text-align: center;
        margin: 20px
    }
</style>

<div class="page flex-container-vertical">
    <h4>{containerName}</h4>
    <textarea bind:this={logContent} class="flex-item-grow container-log-textfield" readonly>{log.messages}</textarea>

    <div class="log-info flex-item-shrink flex-container-horizontal">
        <p class="flex-item-grow">={new Date(log.lastUpdatedTimestamp).toLocaleString()}=</p>
        {#if log.updateError}
            <strong class="flex-item-grow log-outdated-text">⚠️ Outdated log update
                ⚠️ {formatDuration(Date.now() - log.lastUpdatedTimestamp)} old</strong>
        {/if}
        <label class="flex-item-shrink flex-container-horizontal valign-wrapper">
            <span class="flex-item-grow">Autoscroll:</span>
            <input class="flex-item-grow" type="checkbox" bind:checked={log.autoScroll}/>
        </label>
    </div>

    <div class="container-info">
                {#if requestInProgress}
            <Circle size="50" color="#607d8b"/>
        {:else}
            <StatusTag containerName={containerName} containerInfo={containerInfo}/>
        {/if}
        <p>
            <b>Image: </b>{dockerComposeInfo?.services[containerName] === null ? "unknown" : dockerComposeInfo.services[containerName]?.image}
        </p>
        <p>
            <b>Dependency: </b>{dockerComposeInfo?.services[containerName] === null ? "none" : dockerComposeInfo.services[containerName]?.depends_on}
        </p>
        <p><b>Command-line:</b></p>
        <input bind:value={commandLineArgsInput} on:input={() => commandLineArgsButtonDisabled = false}/>
        <a class={`btn blue-grey ${commandLineArgsButtonDisabled ? "disabled" : ""}`}
           on:click={console.log("modifyContainerCommandLineArgs(containerName, commandLineArgsInput)")}>Save and
            restart</a>
    </div>

    <div class="button-row flex-container-horizontal">
        <div class="flex-item-grow"></div>

        {#if isRunning(containerName, containerInfo)}
            <button class="entity-btn btn-large blue-grey" on:click={onDeactivateContainerClick(containerInfo)}>
                <i class="material-icons left">{"remove_circle_outline"}</i>
                Deactivate
            </button>
        {:else}
            <button class="entity-btn btn-large green darken-1" on:click={onActivateContainerClick(containerInfo)}>
                <i class="material-icons left">{"add_circle_outline"}</i>
                Activate
            </button>
        {/if}

        <button class="flex-item-shrink btn blue-grey"
                on:click={console.log("TODO implement tryRestartContainer(containerName)")}>
            Restart
        </button>
    </div>
</div>
