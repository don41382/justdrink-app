<script lang="ts">
    import {MeasureSystem} from "./MeasureSystem";
    import {limitNumber} from "./LimitNumber";
    import {WeightConverter} from "./WeightConverter.js";
    import Navigation from "./Navigation.svelte";

    let {weightInKg = $bindable(), measureSystem = $bindable(), back, next}: {
        weightInKg: number,
        measureSystem: MeasureSystem,
        back: () => void,
        next: () => void
    } = $props()

    let weightUnit = $derived(WeightConverter.toWeightName(measureSystem))

    interface WeightRange {
        metric: number,
        imperial: number,
        clazz: string,
    }

    const ranges: WeightRange[] = [{
        metric: 40,
        imperial: 100,
        clazz: "start-0"
    }, {
        metric: 65,
        imperial: 135,
        clazz: "start-1/3 -translate-x-1/2"
    }, {
        metric: 90,
        imperial: 180,
        clazz: "start-2/3 -translate-x-1/2"
    }, {
        metric: 115,
        imperial: 220,
        clazz: "end-0"
    }]

    function getMetric(idx: number): number {
        switch (measureSystem) {
            case MeasureSystem.Metric:
                return ranges[idx].metric
            case MeasureSystem.Imperial:
                return ranges[idx].imperial
        }
    }

    let min = $derived(WeightConverter.from(getMetric(0), measureSystem))
    let max = $derived(WeightConverter.from(getMetric(ranges.length - 1), measureSystem))

</script>

<div class="flex-1">
    <div class="flex flex-col w-full h-full">
        <h1 class="flex-none text-4xl text-primary text-left mb-2">What's Your Weight?</h1>
        <p class="text-secondary/80 font-light">
            Knowing your weight helps us calculate your optimal hydration needs.
        </p>
        <div class="flex flex-col flex-1 w-full justify-center items-center mt-14">
            <div class="flex h-16 text-2xl items-center rounded-xl bg-secondary/20 pl-3 outline-1 -outline-offset-1 outline-gray-300">
                <input
                        bind:value={() =>  (weightInKg === 0) ? "" : WeightConverter.to(weightInKg, measureSystem), (v) => {weightInKg = WeightConverter.to(limitNumber(String(v), 4), measureSystem)}}
                        class="block grow {(WeightConverter.to(weightInKg, measureSystem) > 99) ? 'w-12' : 'w-8'} bg-transparent pl-1 text-primary placeholder:text-gray-400 focus:outline-none"
                        id="weight" name="weight">
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
            <div class="relative mt-4 w-1/2">
                <label class="sr-only" for="labels-range-input">Labels range</label>
                <div class="relative z-50">
                    <input
                            bind:value={weightInKg}
                            class="w-full range-lg h-2 bg-secondary brightness-50 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
                            id="labels-range-input" max="{max}" min="{min}" step="1" type="range">
                </div>
                {#each ranges as range}
                    <div class="flex flex-col items-center text-sm text-secondary/80 dark:text-gray-400 absolute {range.clazz} -bottom-6">
                    <span>
                        {measureSystem === MeasureSystem.Metric ? range.metric : range.imperial} {weightUnit}
                    </span>
                        <span class="absolute bottom-5 z-10 text-xs">|</span>
                    </div>

                {/each}
            </div>
        </div>
    </div>
</div>
<Navigation back={back}
            backVisible={true}
            next={next}
            nextBackground="bg-primary"
            nextDisabled={false}
            nextName="Next"/>
