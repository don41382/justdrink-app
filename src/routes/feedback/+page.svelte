<script lang="ts">
    import AutoSize from "../AutoSize.svelte";
    import Icon from "@iconify/svelte";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {commands, type FeedbackRate} from "../../bindings";

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

    async function close() {
        await getCurrentWindow().close();
    }

</script>

<AutoSize class="flex-col rounded-lg bg-gray-100 w-[400px] p-8">
    <div class="flex justify-between items-center space-x-3 mb-6">
        <div class="flex items-center space-x-2 mr-16 select-none">
            <img alt="mm" class="w-8 h-8" data-tauri-drag-region src="{data.iconPath}">
            <p class="text-xl font-semibold text-left whitespace-nowrap" data-tauri-drag-region>Drink Now!</p>
        </div>
        <div class="flex space-x-2 justify-end">
            <button class="flex flex-col items-center justify-center cursor-pointer rounded-full hover:bg-mm-green-100 hover:text-white p-1"
                    onclick={async () => { await close() }}>
                <Icon class="size-6" icon="iconoir:xmark"/>
            </button>

        </div>
    </div>
    <h2 class="mb-2 font-medium">How is your experience so far?</h2>
    <textarea autofocus bind:value={feedback} class="w-full h-24 p-2 mb-1"
              placeholder="Please give me feedback ..."></textarea>
    <div class="flex items-center space-x-4">
        {#each feedbacks as feedback}
            <button
                    onclick={() => selectedFeedback = feedback.value}
                    class="size-8 inline-flex justify-center items-center text-2xl rounded-full {selectedFeedback === feedback.value ? 'bg-gray-200' : ''}  hover:bg-gray-200 focus:outline-none focus:bg-gray-200 disabled:pointer-events-none"
                    type="button">
                {feedback.emoji}
            </button>
        {/each}
    </div>
    {#if error}
        <p class="text-red-800 text-xs mt-2">({error})</p>
    {/if}
    <div class="mt-6 flex justify-end">
        <button class="bg-primary hover:bg-primary/50 text-white py-2 px-8 rounded-md disabled:bg-primary/50"
                onclick={send}>Submit
        </button>
    </div>
</AutoSize>