<script lang="ts">

    import type {GenderImages} from "./+page";
    import {WeightConverter} from "./WeightConverter";
    import type {GenderType} from "../../bindings";
    import {allGender, imagePath} from "./Gender";

    let { selectedGender = $bindable(), weightInKg = $bindable(), genderImages } : { selectedGender: GenderType | undefined, weightInKg: number | undefined, genderImages: GenderImages} = $props();

    function select(gender: GenderType) {
        weightInKg = WeightConverter.defaultWeightByGender(gender)
        selectedGender = gender
    }

    if (!selectedGender) {
        selectedGender = "Female"
    }

</script>

<div class="flex-col w-full">
    <div class="flex-none flex-col">
        <h1 class="text-4xl text-primary text-left mb-2">Your Gender</h1>
        <p class="text-secondary/80">
            Choose the gender that best represents you.
        </p>
        <div class="flex space-x-2 justify-center items-stretch mt-8">
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