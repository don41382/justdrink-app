<script lang="ts">

    let {welcomePath} = $props();

</script>

<div class="flex flex-1 h-full items-center">
    <div class="flex items-center gap-8">
        <div class="flex justify-center">
            <img alt="welcome" class="w-96" src={welcomePath}/>
        </div>
        <div>
            <h2 class="text-2xl text-secondary">Welcome to</h2>
            <h1 class="text-4xl text-primary text-left mb-2">Drink Now!</h1>
            <p class="text-white/80">
                Let's find out what the perfect amount, which amount you should drink every day.
            </p>
        </div>
    </div>
</div>