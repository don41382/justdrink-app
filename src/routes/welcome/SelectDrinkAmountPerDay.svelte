<script lang="ts">
    import {MeasureSystem} from "./MeasureSystem";
    import {limitNumber} from "./LimitNumber";
    import {toVolumeName, VolumeConverter} from "./VolumeConverter";
    import Navigation from "./Navigation.svelte";

    let {drinkAmount = $bindable(), measureSystem = $bindable(), min, max, back, next}: {
        drinkAmount: number,
        measureSystem: MeasureSystem,
        min: number,
        max: number,
        back: () => void,
        next: () => void,
    } = $props()

</script>

<div class="flex-1">
    <div class="flex flex-col w-full h-full">
        <h1 class="flex-none text-4xl text-primary text-left mb-2">How much you should drink</h1>
        <p class="text-secondary/80 font-light">
            Based on your age, you should drink
            between {VolumeConverter.to(min, measureSystem).toFixed(0).toLocaleString()}
            - {VolumeConverter.to(max, measureSystem).toFixed(0).toLocaleString()} {toVolumeName(measureSystem)}  per
            day.
            You can adjusted, if you like.
        </p>
        <div class="flex grow justify-center mt-7">
            <div class="flex flex-col justify-center items-center">
                <div class="flex w-fit h-16 text-2xl items-center rounded-xl bg-secondary/20 px-3 outline-1 -outline-offset-1 outline-gray-300">
                    <input
                            bind:value={() =>  (drinkAmount === 0) ? "" : VolumeConverter.to(drinkAmount, measureSystem), (v) => {drinkAmount = VolumeConverter.from(limitNumber(String(v), 4), measureSystem)}}
                            class="block w-16 grow bg-transparent pl-1 text-primary placeholder:text-gray-400 focus:outline-none no-spin"
                            id="ml"
                            name="ml">
                    <div class="flex focus-within:relative">
                        <select aria-label="Weight" bind:value={measureSystem}
                                class="col-end-1 row-end-1 appearance-none rounded-xl {(measureSystem === MeasureSystem.Metric) ? 'w-10' : 'w-14'} text-primary/50 bg-transparent focus:outline-none"
                                id="weightSystem" name="weightSystem">
                            {#each [MeasureSystem.Metric, MeasureSystem.Imperial] as system }
                                <option value={system}>{toVolumeName(system)}</option>
                            {/each}
                        </select>
                    </div>
                </div>
                <p class="text-m whitespace-nowrap text-primary/50 h-fit items-center mt-1">per day</p>
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