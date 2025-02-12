<script lang="ts">
    import type {SipImages} from "./+page";
    import {toVolumeName, VolumeConverter} from "./VolumeConverter";
    import type {MeasureSystem} from "./MeasureSystem";
    import {Sip} from "./SipSize";
    import type {SipSize} from "../../bindings";

    let {sipImages, selectedSipSize = $bindable(), drinkBreakMin = $bindable(), measureSystem}: {
        sipImages: SipImages,
        selectedSipSize: SipSize,
        drinkBreakMin: number,
        measureSystem: MeasureSystem
    } = $props();

    function selectSize(sipSize: SipSize) {
        selectedSipSize = sipSize
    }

    function imagePath(size: SipSize) {
        switch (size) {
            case "FullCup":
                return sipImages.full
            case "HalfCup":
                return sipImages.half
            case "BigSip":
                return sipImages.sip3
        }
    }


</script>

<div class="flex flex-col w-full h-full">
    <div class="flex-none flex justify-between">
        <div class="flex-col">
            <h1 class="flex-none text-4xl text-primary text-left mb-2">Your drink breaks</h1>
            <p class="flex-none text-secondary/80">
                Choose how much you like to drink on each break
            </p>
        </div>
        <div class="flex flex-col items-center justify-center">
            <p class="text-primary/40">{Sip.getTextForSize(selectedSipSize) ?? "sip"} every</p>
            <p class="text-primary text-4xl">{drinkBreakMin.toFixed(0)} min</p>
        </div>
    </div>
    <div class="flex grow items-center justify-center">
        <div class="flex grow gap-x-2 justify-center">
            {#each Object.values(Sip.sizes) as size}
                <button
                        onclick={() => selectSize(size.type)}
                        class="flex p-4 group flex-col cursor-pointer shadow-sm rounded-xl items-center w-32 {(size.type === selectedSipSize) ? 'bg-primary' : 'bg-primary/10 hover:bg-primary/50'}">
                    <img fetchpriority="high" class="w-12" alt="{size.text}" src="{imagePath(size.type)}"/>
                    <p class="text-lg/6 mt-2 {(size.type === selectedSipSize) ? 'text-accent' : 'text-primary'}">{size.text}</p>
                    <p class="text-sm {(size.type === selectedSipSize) ? 'text-accent/70' : 'text-secondary/40'}">{VolumeConverter.from(size.ml, measureSystem)} {toVolumeName(measureSystem)}</p>
                </button>
            {/each}
        </div>
    </div>
</div>