import {onDestroy, onMount} from 'svelte';

export function onInterval(callback, milliseconds) {
    let interval = null;
    onMount(() => {
        callback();
        interval = setInterval(callback, milliseconds)
    });
    onDestroy(() => {
        if (interval !== null) {
            clearInterval(interval);
        }
    });
}
