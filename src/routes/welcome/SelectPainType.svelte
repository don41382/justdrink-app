<script lang="ts">
    import type { PainTypeImages } from "./+page";
    import { type Pain, pains } from "./Pain";

    let { selectedPains = $bindable(), images }: { selectedPains: Pain[], images: PainTypeImages } = $props();

    function select(pain: Pain) {
        const index = selectedPains.findIndex(p => p.title === pain.title);
        if (index !== -1) {
            // Remove the pain from the array by mutating it
            selectedPains.splice(index, 1); // Remove 1 item from the position `index`
        } else {
            // Add the pain to the array by mutating it
            selectedPains.push(pain); // Add the pain to the array
        }
    }

    function getImageSrc(pain: Pain): string {
        switch (pain.id) {
            case 'neck-upper-back':
                return images.upperBackImage;
            case 'lower-back':
                return images.lowerBackImage;
            case 'wrist-hand':
                return images.wristImage;
            case 'stress':
                return images.stress;
        }
    }
</script>

<div class="flex-col w-full">
    <div class="flex-none flex-col">
        <h1 class="text-4xl text-secondary text-left mb-2">What are you struggling with?</h1>
        <p class="text-neutral-600">
            Select the area of discomfort so we can help you find the right solution.
        </p>
        <div class="flex space-x-2 justify-center mt-8">
            {#each pains as pain}
                <button
                        onclick={() => select(pain)}
                        class="group flex-col shadow-sm rounded-xl p-4 items-center w-36 border-2 border-white cursor-pointer
                        {selectedPains.some(p => p.title === pain.title) ? 'bg-gray-800 text-white' : 'bg-gray-200 hover:bg-gray-300'}">
                    <div class="h-24 mb-4">
                        <img src={getImageSrc(pain)} alt={pain.title} class="mb-4"/>
                    </div>
                    <div class="w-full mb-1">
                        {pain.title}
                    </div>
                </button>
            {/each}
        </div>
    </div>
</div>