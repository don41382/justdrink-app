<script lang="ts">

    let {welcomePath} = $props();

</script>

<div class="flex flex-col w-full h-full">
    <div class="flex flex-col grow items-center justify-center">
        <div class="flex space-x-2 justify-center items-stretch">


            <div class="flex items-center gap-8">

                <div class="flex justify-center">
                    <img alt="welcome" class="w-96" src={welcomePath}/>
                </div>
                <div>
                    <h2 class="text-2xl text-secondary">Welcome to</h2>
                    <h1 class="text-4xl text-primary text-left mb-2">Drink Now!</h1>
                    <p class="text-white/80">
                        Discover the ideal daily water intake tailored just for you.
                    </p>
                </div>
            </div>
        </div>
    </div>
</div>