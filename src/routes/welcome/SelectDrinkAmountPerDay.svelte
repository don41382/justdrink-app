<script lang="ts">
    import {getLiquidSystem, LiquidSystem} from "./LquidSystem";
    import type {MeasureSystem} from "./MeasureSystem";
    import {limitNumber} from "./LimitNumber";

    interface Calculated {
        min: number,
        max: number,
        selected: number
    }


    let {config = $bindable({ min: 100, max: 3000, selected: 2000 }), measureSystem = $bindable()}: { config: Calculated, measureSystem: MeasureSystem } = $props()
    let selectedLiquidSystem = $state(getLiquidSystem(measureSystem))

</script>

<div class="flex flex-col w-full h-full">
    <h1 class="flex-none text-4xl text-primary text-left mb-2">How much you should drink</h1>
    <p class="text-secondary/80">
        Based on your age, you should drink between {config.min.toLocaleString()} - {config.max.toLocaleString()} {selectedLiquidSystem.toLowerCase()}  per day.
    </p>
    <div class="flex flex-1 justify-center items-center">
        <div class="flex w-fit h-16 text-2xl items-center rounded-xl bg-secondary/20 pl-3 outline-1 -outline-offset-1 outline-gray-300">
            <input
                    bind:value={() =>  (config.selected === 0) ? "" : config.selected, (v) => {config.selected = limitNumber(v, 9999)}}
                    class="block w-16 grow bg-transparent pl-1 text-primary placeholder:text-gray-400 focus:outline-none no-spin"
                    id="ml"
                    name="ml">
            <div class="grid shrink-0 grid-cols-1 focus-within:relative">
                <select aria-label="Weight" bind:value={selectedLiquidSystem}
                        class="col-start-1 row-start-1 w-full appearance-none rounded-xl py-1.5 pr-7 pl-3 text-primary/50 bg-transparent"
                        id="weightSystem" name="weightSystem">
                    {#each Object.values(LiquidSystem) as liquid }
                        <option value={liquid}>{liquid}</option>
                    {/each}
                </select>
                <svg aria-hidden="true"
                     class="pointer-events-none col-start-1 row-start-1 mr-2 size-5 self-center justify-self-end text-black/50"
                     data-slot="icon" fill="currentColor" viewBox="0 0 16 16">
                    <path clip-rule="evenodd"
                          d="M4.22 6.22a.75.75 0 0 1 1.06 0L8 8.94l2.72-2.72a.75.75 0 1 1 1.06 1.06l-3.25 3.25a.75.75 0 0 1-1.06 0L4.22 7.28a.75.75 0 0 1 0-1.06Z"
                          fill-rule="evenodd"/>
                </svg>
            </div>
        </div>
    </div>
</div>
