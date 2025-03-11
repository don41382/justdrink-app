<script lang="ts">
    import type {SipImages} from "./+page";
    import type {MeasureSystem} from "./MeasureSystem";
    import {Sip} from "./SipSize";
    import type {SipSize} from "../../bindings";
    import Navigation from "./Navigation.svelte";
    import {DrinkTimeCalculator} from "./DrinkTimeCalculator";

    let {
        sipImages,
        selectedSipSize = $bindable(),
        drinkAmountMl,
        drinkBreakMin = $bindable(),
        measureSystem,
        back,
        next
    }: {
        sipImages: SipImages,
        selectedSipSize: SipSize,
        drinkAmountMl: number,
        drinkBreakMin: number,
        measureSystem: MeasureSystem,
        back: () => void,
        next: () => void
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

<div class="flex-1">
    <div class="flex flex-col w-full h-full">
        <div class="flex-none flex justify-between">
            <div class="flex-col">
                <h1 class="flex-none text-4xl text-primary text-left mb-2">Select your cup size</h1>
                <p class="flex-none text-secondary/80">
                    Your selection determines how often you'll get water reminders
                </p>
            </div>
            <div class="flex flex-col items-center justify-center">
                <p class="text-primary/40 whitespace-nowrap">{Sip.getTextForSize(selectedSipSize) ?? "sip"} every</p>
                <p class="text-primary text-4xl whitespace-nowrap">{drinkBreakMin.toFixed(0)} min</p>
            </div>
        </div>
        <div class="flex grow items-center justify-center mt-7">
            <div class="flex grow gap-x-2 justify-center">
                {#each Object.values(Sip.sizes) as size}
                    <button
                            onclick={() => selectSize(size.type)}
                            class="flex p-4 group flex-col cursor-pointer shadow-sm rounded-xl items-center w-32 {(size.type === selectedSipSize) ? 'bg-primary' : 'bg-primary/10 hover:bg-primary/50'}">
                        <img fetchpriority="high" class="w-14 h-18" alt="{size.text}" src="{imagePath(size.type)}"/>
                        <p class="text-lg/6 mt-2 {(size.type === selectedSipSize) ? 'text-accent' : 'text-primary'}">{DrinkTimeCalculator.calc(drinkAmountMl, size.type)}
                            min</p>
                        <p class="text-sm {(size.type === selectedSipSize) ? 'text-accent/70' : 'text-secondary/40'}">{size.text}</p>
                    </button>
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
            nextName="Next"
            nextVisible={true}/>