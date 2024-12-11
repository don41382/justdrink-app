<script lang="ts">

    import {type Time, times} from "./Time";
    let { selectedDuration = $bindable() } : { selectedDuration: Time} = $props();

    function select(time: Time) {
        selectedDuration = time
    }

</script>

<div class="flex-col w-full">
    <div class="flex-none flex-col">
        <h1 class="text-4xl text-secondary text-left mb-2">Your Session Interval</h1>
        <p class="text-neutral-600">
            How often do you like to stretch & pause?
        </p>
        <div class="flex space-x-2 justify-center mt-8">
            {#each times as time}
                <button
                        onclick={() => select(time)}
                        class="group flex-col cursor-pointer shadow-sm rounded-xl p-4 items-center w-36 {(selectedDuration.minutes === time.minutes) ? 'bg-primary text-white' : 'bg-gray-200 hover:border-gray-600 hover:bg-gray-300'}">
                    <span class="text-xs font-light">every</span>
                    <div class="text-lg w-full mb-1">
                        {time.minutes} min
                    </div>
                    <span class="h-[1px] mb-1 w-24 {selectedDuration.minutes === time.minutes ? 'bg-gray-400' : 'bg-gray-300 group-hover:bg-gray-400'} block"></span>
                    <span class="text-center font-light {selectedDuration.minutes === time.minutes ? 'text-white' : 'text-gray-600'}">{time.text}</span>
                </button>
            {/each}
        </div>
    </div>
</div>