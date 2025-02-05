<script lang="ts">

    import {GenderType} from "./Gender";
    import type {GenderImages} from "./+page";
    import {WeightConverter} from "./WeightConverter";

    let { selectedGender = $bindable(), weightInKg = $bindable(), genderImages } : { selectedGender: GenderType | undefined, weightInKg: number | undefined, genderImages: GenderImages} = $props();

    function select(gender: GenderType) {
        weightInKg = WeightConverter.defaultWeightByGender(gender)
        selectedGender = gender
    }

    function imagePath(gender: GenderType) {
        switch (gender) {
            case GenderType.Male: return genderImages.male
            case GenderType.Female: return genderImages.female
            case GenderType.Other: return genderImages.other
        }
    }

    if (!selectedGender) {
        selectedGender = GenderType.Female
    }

</script>

<div class="flex-col w-full">
    <div class="flex-none flex-col">
        <h1 class="text-4xl text-primary text-left mb-2">Your Gender</h1>
        <p class="text-secondary/80">
            Choose the gender that best represents you.
        </p>
        <div class="flex space-x-2 justify-center mt-8">
            {#each Object.values(GenderType) as gender}
                <button
                        onclick={() => select(gender)}
                        class="group flex-col cursor-pointer shadow-sm rounded-xl items-center w-36
                               {(selectedGender === gender) ? 'bg-primary' : 'bg-primary/10 hover:bg-primary/50'}">
                    <img class="rounded-xl" alt="{gender}" src="{imagePath(gender)}"/>
                </button>
            {/each}
        </div>
    </div>
</div>