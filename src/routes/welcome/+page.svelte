<script lang="ts">
    import AutoSize from "../AutoSize.svelte";
    import SelectEnd from "./SelectEnd.svelte";
    import SelectStart from "./SelectStart.svelte";
    import {enable} from "@tauri-apps/plugin-autostart";
    import {commands} from "../../bindings";
    import {type Time, times} from "./Time";
    import {info} from "@tauri-apps/plugin-log";
    import type {Pain} from "./Pain";
    import SelectGender from "./SelectGender.svelte";
    import {GenderType} from "./Gender";
    import SelectWeight from "./SelectWeight.svelte";
    import SelectDrinkAmountPerDay from "./SelectDrinkAmountPerDay.svelte";
    import {getMeasureSystem, MeasureSystem} from "./MeasureSystem";

    let {data} = $props();

    type WelcomeStep = "Start" | "GenderType" | "Weight" | "DrinkAmount" | "Finish"
    let steps: WelcomeStep[] = ["Start", "GenderType", "Weight", "DrinkAmount", "Finish"];
    let currentStep: WelcomeStep = $state("Start")

    let email: string | null = $state(null);
    let consent: boolean = $state(true);
    let selectedDuration: Time = $state(times[1]);
    let selectedPains: Pain[] = $state([]);

    let measureSystem = $state(getMeasureSystem());
    let weightInKg = $state(70)
    let gender: GenderType = $state(GenderType.Female)

    function next() {
        const currentIndex = steps.indexOf(currentStep);
        if (currentIndex < steps.length - 1) {
            currentStep = steps[currentIndex + 1];
        } else if (steps[currentIndex] ==  "Finish") {
            info(`start first session, email: ${email}, consent: ${consent}, selected-pains: ${selectedPains.map(p => p.id)}`)
            enable()
            commands.startFirstSession(
                selectedDuration.minutes,
                email,
                consent,
                true
            )
        }
    }

    function back() {
        const currentIndex = steps.indexOf(currentStep);
        if (currentIndex > 0) {
            currentStep = steps[currentIndex - 1];
        }
    }

    const getProgress = () => (steps.indexOf(currentStep) / (steps.length - 1)) * 100;
</script>


<AutoSize
        class="flex flex-col bg-accent w-[650px] h-[450px] px-12 justify-center cursor-default rounded-2xl"
        ready={true}>

        <!-- Progress Bar -->
        <div class="absolute top-0 left-0 w-full h-4 rounded-t-2xl overflow-hidden">
            <div
                    class="h-2 bg-primary/50 transition-all duration-1000"
                    style="width: {getProgress()}%;"
            ></div>
        </div>

        <div class="pt-8 mb-4">
            <div class="flex items-center">
                <img alt="Drink Now!" class="size-12" src="{data.iconPath}">
                <p class="text-xl ml-2 text-primary">Drink Now!</p>
            </div>
        </div>


        <div class="flex-1">
            {#if currentStep === "Start"}
                <SelectStart welcomePath={data.welcomePath}/>
            {:else if currentStep === "GenderType"}
                <SelectGender bind:selectedGender={gender} genderImages={data.genderImages} />
            {:else if currentStep === "Weight"}
                <SelectWeight bind:measureSystem={measureSystem} weightInKg={weightInKg}/>
            {:else if currentStep === "DrinkAmount"}
                <SelectDrinkAmountPerDay bind:measureSystem={measureSystem} liquidInMl={2000}/>
            {:else if currentStep === "Finish"}
                <SelectEnd bind:email={email} bind:consent={consent} />
            {/if}
        </div>

        <div class="flex w-full pb-10">
            {#if currentStep !== "Start"}
                <button class="text-secondary/30 py-2 rounded-md" onclick={back}>
                    Back
                </button>
            {/if}
            <button class="bg-primary hover:bg-primary/50 text-black py-2 rounded-md px-8 ml-auto" onclick={next}>
                {#if currentStep === "Finish"}
                    Start your first session
                {:else}
                    Next
                {/if}
            </button>
        </div>
</AutoSize>