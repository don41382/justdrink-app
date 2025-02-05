<script lang="ts">
    import {MeasureSystem} from "./MeasureSystem";
    import {limitNumber} from "./LimitNumber";
    import {WeightConverter} from "./WeightConverter.js";

    let {weightInKg = $bindable(), measureSystem = $bindable()}: { weightInKg: number, measureSystem: MeasureSystem } = $props()

    let weightUnit = WeightConverter.toWeightName(measureSystem)

</script>

<div class="flex flex-col w-full h-full">
    <h1 class="flex-none text-4xl text-primary text-left mb-2">What's Your Weight?</h1>
    <p class="text-secondary/80">
        Knowing your weight helps us calculate your optimal hydration needs.
    </p>
    <div class="flex flex-col flex-1 w-full justify-center items-center">
        <div class="flex h-16 text-2xl items-center rounded-xl bg-secondary/20 pl-3 outline-1 -outline-offset-1 outline-gray-300">
            <input
                    bind:value={() =>  (weightInKg === 0) ? "" : WeightConverter.from(weightInKg, measureSystem), (v) => {weightInKg = WeightConverter.to(limitNumber(v, 4), measureSystem)}}
                    class="block grow {(WeightConverter.from(weightInKg, measureSystem) > 99) ? 'w-12' : 'w-8'} bg-transparent pl-1 text-primary placeholder:text-gray-400 focus:outline-none"
                    id="weight"
                    max="999" min="20" name="weight">
            <div class="grid shrink-0 grid-cols-1 focus-within:relative">
                <select aria-label="Weight" bind:value={measureSystem}
                        class="col-start-1 row-start-1 w-full appearance-none rounded-xl py-1.5 pr-7 pl-3 text-primary/50 bg-transparent focus:outline-none"
                        id="weightSystem" name="weightSystem">
                    {#each [MeasureSystem.Metric, MeasureSystem.Imperial] as weight }
                        <option value={weight}>{WeightConverter.toWeightName(weight)}</option>
                    {/each}
                </select>
                <svg aria-hidden="true"
                     class="pointer-events-none col-start-1 row-start-1 mr-2 size-5 self-center justify-self-end text-primary/50"
                     data-slot="icon" fill="currentColor" viewBox="0 0 16 16">
                    <path clip-rule="evenodd"
                          d="M4.22 6.22a.75.75 0 0 1 1.06 0L8 8.94l2.72-2.72a.75.75 0 1 1 1.06 1.06l-3.25 3.25a.75.75 0 0 1-1.06 0L4.22 7.28a.75.75 0 0 1 0-1.06Z"
                          fill-rule="evenodd"/>
                </svg>
            </div>
        </div>
        <div class="relative mt-2 w-1/2">
            <label class="sr-only" for="labels-range-input">Labels range</label>
            <div class="relative z-50">
                <input bind:value={weightInKg} class="w-full range-lg h-2 bg-secondary brightness-50 rounded-lg appearance-none cursor-pointer dark:bg-gray-700" id="labels-range-input" max="120" min="40" step="1" type="range">
            </div>
            <span class="text-sm text-secondary/80 dark:text-gray-400 absolute start-0 -bottom-6">40 {weightUnit}</span>
            <span class="text-sm text-secondary/80 dark:text-gray-400 absolute start-1/3 -translate-x-1/2 -bottom-6">65 {weightUnit}</span>
            <span class="text-sm text-secondary/80 dark:text-gray-400 absolute start-2/3 -translate-x-1/2 -bottom-6">95 {weightUnit}</span>
            <span class="text-sm text-secondary/80 dark:text-gray-400 absolute end-0 -bottom-6">120 {weightUnit}</span>
            {#each [
                "start-4",
                "start-1/3 -translate-x-1/2",
                "start-2/3 -translate-x-1/2",
                "end-4"] as clazz}
                <span class="text-sm text-secondary brightness-50 dark:text-gray-400 absolute {clazz} -bottom-0.5 z-10">|</span>
            {/each}
        </div>
    </div>
</div>
