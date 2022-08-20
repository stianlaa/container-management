<script lang="ts">
    import {
        requestContainerLogsLast,
        requestContainerLogsSpan,
    } from "../../utils/api.ts";
    import {afterUpdate, onMount} from "svelte";
    import {onInterval} from "../../utils/onInterval.js";
    import {formatDuration} from "../../utils/dateTimeUtils"
    import {ContainerState} from "../../utils/container.js"

    const UPDATE_INTERVAL_MS = 2000;

    export let containerId = null;
    export let containerState = null;

    let log = {
        messages: "",
        lastUpdatedTimestamp: Date.now(),
        autoScroll: true,
        updateError: false,
    };

    onMount(initialLogFetch);

    onInterval(async () => await fetchLogs(), UPDATE_INTERVAL_MS);

    async function initialLogFetch() {
        if (containerState == ContainerState.Running) {
            log.messages += await requestContainerLogsLast(containerId, 100);
        }
    }

    async function fetchLogs() {
        if (containerId !== null) {
            let now = Date.now();
            let logResponse = await requestContainerLogsSpan(containerId, log.lastUpdatedTimestamp, now);
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
</style>

<div class="flex-container-vertical">
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
</div>
