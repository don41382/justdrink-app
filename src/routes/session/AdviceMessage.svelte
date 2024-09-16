<script lang="ts">
    import { fade } from 'svelte/transition';
    import { onDestroy } from 'svelte';
    import { createEventDispatcher } from 'svelte';

    export let classes: string = '';
    export let advices: string[];

    let currentMessageIndex = 0;
    let currentMessage = advices[currentMessageIndex];
    let visible = true;

    const dispatch = createEventDispatcher();

    function nextMessage() {
        visible = false; // Start fade-out
    }

    function handleOutroEnd() {
        // Update the message after fade-out completes
        currentMessageIndex = (currentMessageIndex + 1) % advices.length;
        currentMessage = advices[currentMessageIndex];
        visible = true; // Start fade-in
        dispatch('messageChange', { message: currentMessage });
    }

    const interval = setInterval(nextMessage, 6000);

    onDestroy(() => {
        clearInterval(interval);
    });
</script>

<div class="text-4xl font-extralight italic leading-normal text-center w-full items-center {classes}">
    {#if advices.length > 0}
        {#if visible}
            <div class="text-center"
                    transition:fade={{ duration: 2000 }}
                    on:outroend={handleOutroEnd}
            >
                {currentMessage}
            </div>
        {/if}
    {/if}
</div>