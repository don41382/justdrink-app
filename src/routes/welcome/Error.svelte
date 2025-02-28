<script lang="ts">
    import Navigation from "./Navigation.svelte";

    let {error, reload}: { error: any, reload: () => void } = $props();
</script>

<div class="flex grow">
    <div class="flex flex-col w-full">
        <h1 class="text-4xl text-primary text-left mb-2">We have a problem</h1>
        <span class="text-secondary/80 font-light">I am sorry, we are experiencing turbulence. Please contact us, if you continue to experience this issue: <span class="text-primary">info@rocket-solutions.de</span></span>
        <div class="flex flex-col h-fit w-full items-start mt-7 bg-secondary/20 p-4 rounded-xl">
            <p class="text-gray-400 font-medium">Details about the error</p>
            <p class="text-highlight">{error}</p>
        </div>
    </div>
</div>
<Navigation back={() => {}}
            backVisible={false}
            next={reload}
            nextBackground="bg-primary"
            nextDisabled={false}
            nextName="Reload"/>
