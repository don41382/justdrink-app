<script lang="ts">
    import {fade} from 'svelte/transition';
    import {onDestroy, onMount} from 'svelte';
    import {createEventDispatcher} from 'svelte';
    import Icon from "@iconify/svelte";

    export let advices: string[];

    let currentMessageIndex = 0;
    let currentMessage = advices[currentMessageIndex];
    let visible = false;

    onMount(() => {
        visible = true;
    })

    function nextMessage() {
        visible = false; // Start fade-out
    }

    function handleOutroEnd() {
        // Update the message after fade-out completes
        currentMessageIndex = (currentMessageIndex + 1) % advices.length;
        currentMessage = advices[currentMessageIndex];
        setTimeout(() => {
            visible = true;
        }, 500);
    }

    const interval = setInterval(nextMessage, 6000);

    onDestroy(() => {
        clearInterval(interval);
    });
</script>

{#if advices.length > 0}
    {#if visible}
        <div class="flex"
             in:fade={{ duration: 1000 }}
             out:fade={{ duration: 1000 }}
             on:outroend={handleOutroEnd}>
            <div class="mr-4">
                <Icon class="size-6" icon="iconoir:info-circle"/>
            </div>
            <div>{currentMessage}</div>
        </div>
    {/if}
{/if}
