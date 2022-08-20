<script lang="ts">
    import {
        requestContainerInfo,
        requestContainerLogsLast,
        requestContainerLogsSpan,
        requestDefaultContainerOptions,
        tryStartContainer,
        tryStopContainer,
        tryRestartContainer,
    } from "../../utils/api.ts";
    import {afterUpdate, onMount} from "svelte";
    import {onInterval} from "../../utils/onInterval.js";
    import {isRunning} from "../../utils/container.js"
    import {composeInfoStore} from "./_store.js"
    import {formatDuration} from "../../utils/dateTimeUtils"
    import StatusTag from "../_components/StatusTag.svelte";
    import {Circle} from "svelte-loading-spinners";

    // TODO horrible semantics, need to be improved
    export let containerName = null;

    const UPDATE_INTERVAL_MS = 2000;

    let log = {
        messages: "",
        lastUpdatedTimestamp: Date.now(),
        autoScroll: true,
        updateError: false,
    };
    let containerCreated = true;
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

    onMount(() => {
        // TODO make update and requestDefaultContainerOptions parallel
        updateInfo()
            .then(() => {return requestDefaultContainerOptions(containerName)})
            .then((defaultContainerOptions) => {
                if (containerInfo == null) {
                    commandLineArgsInput = defaultContainerOptions["entrypoint"];
                } else {
                    commandLineArgsInput = containerInfo["args"];
                }
            });
    });

    async function updateInfo() {
        if (containerCreated) {
            let response = await requestContainerInfo(containerName);
            if (response == null) {
                containerCreated = false;
                return;
            } else {
                containerInfo = response;
            }
            log.messages += await requestContainerLogsLast(containerInfo.id, 100);
        }
    }

    async function onStartContainerClick(containerInfo) {
        requestInProgress = true;
        await tryStartContainer(containerInfo.id);
        requestInProgress = false;
        await updateInfo();
    }

    async function onStopContainerClick(containerInfo) {
        requestInProgress = true;
        await tryStopContainer(containerInfo.id);
        requestInProgress = false;
        await updateInfo();
    }

    async function onRestartContainerClick(containerInfo) {
        requestInProgress = true;
        await tryRestartContainer(containerInfo.id);
        requestInProgress = false;
        await updateInfo();
    }

    async function modifyContainerCommandLineArgs(containerName, commandLineArgsInput) {
        // TODO first, remove the existing container, if it exists
        // TODO second, create new container, but with entrypoint overridden
    }

    onInterval(async () => await fetchLogs(), UPDATE_INTERVAL_MS);

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

    .entity-btn {
        margin-left: 20px;
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
           on:click={modifyContainerCommandLineArgs(containerName, commandLineArgsInput)}>Save and
            recreate</a>
    </div>

    <div class="button-row flex-container-horizontal">
        <div class="flex-item-grow"></div>
        {#if isRunning(containerName, containerInfo)}
            <button class={`entity-btn btn-large blue-grey ${requestInProgress ? "disabled" : ""}`}
                    on:click={onStopContainerClick(containerInfo)}>
                <i class="material-icons left">{"remove_circle_outline"}</i>
                Stop
            </button>
        {:else}
            <button class={`entity-btn btn-large green darken-1 ${requestInProgress ? "disabled" : ""}`}
                    on:click={onStartContainerClick(containerInfo)}>
                <i class="material-icons left">{"add_circle_outline"}</i>
                Start
            </button>
        {/if}

        <button class={`entity-btn btn-large blue-grey ${requestInProgress ? "disabled" : ""}`}
                on:click={onRestartContainerClick(containerInfo)}>
            Restart
        </button>
    </div>
</div>
