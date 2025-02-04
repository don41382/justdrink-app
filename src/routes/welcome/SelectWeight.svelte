<script lang="ts">
    import {MeasureSystem} from "./MeasureSystem";
    import {limitNumber} from "./LimitNumber";
    import {WeightConverter} from "./WeightConverter.js";

    let {weightInKg = $bindable(70), measureSystem = $bindable()}: { weightInKg: number, measureSystem: MeasureSystem } = $props()

    function toWeightName(measureSystem: MeasureSystem) {
        switch (measureSystem) {
            case MeasureSystem.Metric: return "kg"
            case MeasureSystem.Imperial: return "lbs"
        }
    }

</script>

<div class="flex flex-col w-full h-full">
    <h1 class="flex-none text-4xl text-primary text-left mb-2">What's Your Weight?</h1>
    <p class="text-secondary/80">
        Knowing your weight helps us calculate your optimal hydration needs.
    </p>
    <div class="flex flex-1 justify-center items-center">
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
                        <option value={weight}>{toWeightName(weight)}</option>
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
    </div>
</div>
