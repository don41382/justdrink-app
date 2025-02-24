<script lang="ts">

    import type {GenderImages} from "./+page";
    import {WeightConverter} from "./WeightConverter";
    import type {GenderType} from "../../bindings";
    import {allGender, imagePath} from "./Gender";
    import Navigation from "./Navigation.svelte";

    let {selectedGender = $bindable(), weightInKg = $bindable(), genderImages, back, next}: {
        selectedGender: GenderType | undefined,
        weightInKg: number | undefined,
        genderImages: GenderImages,
        back: () => void,
        next: () => void
    } = $props();

    function select(gender: GenderType) {
        weightInKg = WeightConverter.defaultWeightByGender(gender)
        selectedGender = gender
    }

    if (!selectedGender) {
        selectedGender = "Female"
    }

</script>

<div class="flex-1">
    <div class="flex flex-col w-full h-full">
        <h1 class="text-4xl text-primary text-left mb-2">Your Gender</h1>
        <p class="text-secondary/80 font-light">
            Choose the gender that best represents you.
        </p>
        <div class="flex flex-col grow items-center justify-center mt-7">
            <div class="flex space-x-2 justify-center items-stretch">
                {#each allGender as gender}
                    <button
                            onclick={() => select(gender)}
                            class="group flex cursor-pointer shadow-sm rounded-xl items-center justify-center w-36
                               {(selectedGender === gender) ? 'bg-primary' : 'bg-primary/10 hover:bg-primary/50'}">
                        {#if gender === "Other"}
                            <p class="flex font-medium {(gender === selectedGender) ? 'text-accent' : 'text-secondary'}">
                                Other
                            </p>
                        {:else}
                            <img class="rounded-xl" alt="{gender}" src="{imagePath(gender, genderImages)}"/>
                        {/if}
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
            nextName="Next"/>