<script lang="ts">
    let {
        backVisible = true,
        nextName = "Next",
        nextBackground = "bg-primary",
        back,
        next,
        nextVisible = true,
        nextDisabled = false
    }: {
        backVisible: boolean,
        nextName: string,
        nextBackground: string,
        back: () => void,
        next: () => void,
        nextVisible: boolean,
        nextDisabled: boolean
    } = $props();
</script>

<div class="flex w-full pb-10">
    {#if backVisible}
        <button class="text-secondary/30 py-2 rounded-md" onclick={back}>
            Back
        </button>
    {/if}
    {#if nextVisible}
        <button class="{nextBackground} hover:{nextBackground}/50 text-black py-2 rounded-md px-8 ml-auto disabled:opacity-50 disabled:hover:opacity-50"
                disabled={nextDisabled} onclick={next}>
            {nextName}
        </button>
    {/if}
</div>