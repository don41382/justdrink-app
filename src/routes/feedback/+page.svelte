<script lang="ts">
    import AutoSize from "../AutoSize.svelte";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {commands, type FeedbackRate} from "../../bindings";
    import Xmark from "../../icons/Xmark.svelte";

    let {data} = $props();

    const feedbacks: { emoji: string; value: FeedbackRate }[] = [
        {emoji: '😔', value: 'BAD'},
        {emoji: '😐', value: 'OK'},
        {emoji: '🤩', value: 'AWESOME'}
    ];
    let selectedFeedback: FeedbackRate = $state('UNKNOWN');
    let feedback: string = $state("");

    let error: string | undefined = $state(undefined);

    async function send() {
        if (selectedFeedback === 'UNKNOWN') {
            error = "Please select your rating"
        } else {
            await commands.feedbackWindowSendFeedback(
                feedback,
                selectedFeedback
            )
            await getCurrentWindow().close();
        }
    }

    async function openAppStoreFeedback() {
        await commands.openAppStoreFeedback()
        await getCurrentWindow().close()
    }

    async function close() {
        await getCurrentWindow().close();
    }

</script>

<AutoSize class="flex-col rounded-lg bg-accent w-[400px] p-8" ready={true}>
    <div class="flex justify-between items-center space-x-3 mb-4">
        <div class="flex items-center space-x-2 select-none">
            <img alt="mm" class="w-8 h-8" data-tauri-drag-region src="{data.iconPath}">
            <p class="text-xl font-light text-primary text-left whitespace-nowrap" data-tauri-drag-region>
                Just Drink!
            </p>
        </div>
    </div>
    {#if data.fullVersionAndMac}
        <h2 class="mb-2 text-lg font-medium text-secondary">Please Rate our App!</h2>
        <p class="text-gray-400">
            If you've been enjoying our app, would you consider sharing your thoughts by leaving a review on the App
            Store?
        </p>
        <p class="mt-2 text-gray-400">
            Thank you for your support!
        </p>
        <div class="flex h-fit justify-between items-center mt-8">
            <button class="text-secondary/40 hover:text-gray-400 py-2" onclick={close}>
                Maybe later
            </button>
            <button class="bg-primary hover:bg-primary/50 text-black py-2 px-8 rounded-md"
                    onclick={openAppStoreFeedback}>Write a review
            </button>
        </div>
    {:else}
        <h2 class="mb-2 font-medium text-secondary">How is your experience so far?</h2>
        <!-- svelte-ignore a11y_autofocus -->
        <textarea autofocus bind:value={feedback}
                  class="w-full h-24 p-2 mb-1 bg-white/20 text-white placeholder-secondary"
                  placeholder="Please give me feedback ..."></textarea>
        <div class="flex items-center space-x-4">
            {#each feedbacks as feedback}
                <button
                        onclick={() => selectedFeedback = feedback.value}
                        class="size-8 inline-flex justify-center items-center text-2xl rounded-full {selectedFeedback === feedback.value ? 'bg-primary' : ''}  hover:bg-secondary focus:outline-none focus:bg-gray-200 disabled:pointer-events-none"
                        type="button">
                    {feedback.emoji}
                </button>
            {/each}
        </div>
        {#if error}
            <p class="text-red-800 text-xs mt-2">({error})</p>
        {/if}
        <div class="mt-6 flex justify-end">
            <button class="bg-primary hover:bg-primary/50 text-black py-2 px-8 rounded-md"
                    onclick={send}>Submit
            </button>
        </div>
    {/if}
</AutoSize>