<script lang="ts">

    let {welcomePath} = $props();

</script>

<div class="flex-col w-full">
    <div class="flex-none flex-col">
        <h1 class="text-4xl text-secondary text-left mb-2">Welcome to Motion Minute!</h1>
        <p class="text-neutral-600">
            Thanks for choosing Motion Minute. Let's quickly setup your motion sessions in the next steps.
        </p>
        <div class="flex justify-center">
            <img class="w-[350px]" src={welcomePath}/>
        </div>
    </div>
</div>