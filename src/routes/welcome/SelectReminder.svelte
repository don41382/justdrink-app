<script lang="ts">

    import {GenderType} from "./Gender";
    import type {ReminderImages} from "./+page";
    import {commands} from "../../bindings";

    let {selectedGender, reminderImages}: { selectedGender: GenderType, reminderImages: ReminderImages } = $props()

    function select(gender: GenderType) {
        commands.startSession()
        selectedGender = gender
    }

    function imagePath(gender: GenderType): string {
        switch (gender) {
            case GenderType.Female:
                return reminderImages.woman
            case GenderType.Male:
                return reminderImages.man
            default:
                return reminderImages.man
        }
    }

</script>

<div class="flex flex-col w-full h-full">
    <h1 class="flex-none text-4xl text-primary text-left mb-2">Personalize your reminder</h1>
    <p class="text-secondary/80">
        Try your personal reminder.
    </p>
    <div class="flex flex-col grow items-center justify-center">
        <div class="flex space-x-2 justify-center items-stretch">
            {#each [GenderType.Male, GenderType.Female] as gender}
                <button
                        onclick={() => select(gender)}
                        class="group flex cursor-pointer shadow-sm rounded-xl items-center justify-center w-36
                               {(selectedGender === gender) ? 'bg-primary' : 'bg-primary/10 hover:bg-primary/50'}">
                    {#if gender === GenderType.Other}
                        <p class="flex font-medium {(gender === selectedGender) ? 'text-accent' : 'text-secondary'}">
                            Other
                        </p>
                    {:else}
                        <img class="rounded-xl" alt="{gender}" src="{imagePath(gender)}"/>
                    {/if}
                </button>
            {/each}
        </div>
    </div>
</div>