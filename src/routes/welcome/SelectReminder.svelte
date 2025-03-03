<script lang="ts">

    import type {ReminderImages} from "./+page";
    import {commands, type DrinkCharacter, type SipSize} from "../../bindings";
    import {DrinkCharacters} from "../DrinkCharacters";
    import Navigation from "./Navigation.svelte";
    import {Sip} from "./SipSize";
    import Speaker from "../../icons/Speaker.svelte";

    let {selectedDrinkCharacter = $bindable(), sipSize, reminderImages, back, next, lastStep}: {
        selectedDrinkCharacter: undefined | DrinkCharacter,
        sipSize: SipSize,
        reminderImages: ReminderImages,
        back: () => void,
        next: () => void,
        lastStep: boolean
    } = $props()

    function select(character: DrinkCharacter) {
        selectedDrinkCharacter = character
        commands.startSession({
            sip_size: sipSize,
            selected_drink_character: character,
            demo_mode: true
        })
    }

    function imagePath(character: DrinkCharacter): string {
        switch (character) {
            case "YoungWoman":
                return reminderImages.woman
            case "YoungMan":
                return reminderImages.man
        }
    }

</script>

<div class="flex-1">
    <div class="flex flex-col w-full h-full">
        <div class="flex-none flex justify-between">
            <div class="flex-col">
                <h1 class="text-4xl text-primary text-left mb-2">Who should remind you?</h1>
                <p class="text-secondary/80 font-light">
                    Choose your immersive character as your drink reminder.
                </p>
            </div>
            <div class="relative group inline-block">
                <div class="flex flex-col items-center justify-center text-secondary/50">
                    <div class="size-12">
                        <Speaker/>
                    </div>
                    <p class="text-xs mt-2">unmute speaker</p>
                </div>
                <div class="absolute bottom-full mb-2 hidden group-hover:block">
                    <div class="px-2 py-1 w-28 text-xs text-white bg-gray-950/50 text-center rounded shadow-lg">
                        Please unmute your speakers
                    </div>
                </div>
            </div>
        </div>
        <div class="flex flex-col grow items-center justify-center mt-12">
            <div class="flex space-x-2 justify-center items-stretch">
                {#each DrinkCharacters.all as character}
                    <button
                            onclick={() => select(character)}
                            class="group flex cursor-pointer shadow-sm rounded-xl items-center justify-center w-36
                               {(selectedDrinkCharacter === character) ? 'bg-primary' : 'bg-primary/10 hover:bg-primary/50'}">
                        <img class="rounded-xl" alt="{character}" src="{imagePath(character)}"/>
                    </button>
                {/each}
            </div>
        </div>
    </div>
</div>
<Navigation back={back}
            backVisible={true}
            next={selectedDrinkCharacter === undefined ? (() => {}) : next}
            nextBackground="bg-primary"
            nextDisabled={selectedDrinkCharacter === undefined}
            nextName={lastStep ? 'Finish' : 'Next'}/>