<script lang="ts">

    let {welcomePath} = $props();

</script>

<div class="flex h-64 items-center">
    <div class="flex items-center gap-8">
        <div class="flex justify-center">
            <img class="w-[300px]" src={welcomePath}/>
        </div>
        <div>
            <h2 class="text-2xl">Welcome to</h2>
            <h1 class="text-4xl text-secondary text-left mb-2">Motion Minute</h1>
            <p class="text-neutral-600">
                Let's quickly setup your motion sessions in the next steps.
            </p>
        </div>
    </div>
</div>