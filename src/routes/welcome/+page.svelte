<script lang="ts">
    import AutoSize from "../AutoSize.svelte";
    import SelectEnd from "./SelectEnd.svelte";
    import SelectStart from "./SelectStart.svelte";
    import {enable} from "@tauri-apps/plugin-autostart";
    import {commands, type DrinkCharacter, type SipSize} from "../../bindings";
    import {info} from "@tauri-apps/plugin-log";
    import SelectGender from "./SelectGender.svelte";
    import {GenderType} from "./Gender";
    import SelectWeight from "./SelectWeight.svelte";
    import SelectDrinkAmountPerDay from "./SelectDrinkAmountPerDay.svelte";
    import {MeasureSystem} from "./MeasureSystem";
    import {CalculatedDrinkAmount} from "./CalculatedDrinkAmount";
    import SelectSipSize from "./SelectSipSize.svelte";
    import {WeightConverter} from "./WeightConverter";
    import SelectReminder from "./SelectReminder.svelte";
    import {Sip} from "./SipSize";

    let {data} = $props();

    type WelcomeStep = "Start" | "GenderType" | "Weight" | "DrinkAmount" | "SipSize" | "Reminder" | "Finish"
    let steps: WelcomeStep[] = ["Start", "GenderType", "Weight", "DrinkAmount", "SipSize", "Reminder", "Finish"];
    let currentStep: WelcomeStep = $state("Start")

    let defaultGender = GenderType.Female

    let email: string | null = $state(null);
    let consent: boolean = $state(true);

    let measureSystem = $state(MeasureSystem.getMeasureSystem());
    let gender: GenderType | undefined = $state()
    let weightInKg: number = $state(WeightConverter.defaultWeightByGender(defaultGender))
    let drinkAmount: number = $state(0)
    let drinkAmountBasedOnGender: number = $state(0)
    let selectedSipSize: SipSize = $state("BigSip")
    let selectedDrinkCharacter: DrinkCharacter | undefined = $state(undefined)
    let drinkBreakMin = $derived(roundToNearestFive((12*60)/(drinkAmount/Sip.getMlForSize(selectedSipSize))))

    function roundToNearestFive(num: number): number {
        return Math.round(num / 5) * 5;
    }

    $effect(() => {
        drinkAmount = CalculatedDrinkAmount.calc(gender ?? defaultGender, weightInKg)
        drinkAmountBasedOnGender = CalculatedDrinkAmount.calc(gender ?? defaultGender, weightInKg)
    })

    function next() {
        const currentIndex = steps.indexOf(currentStep);
        if (currentIndex < steps.length - 1) {
            currentStep = steps[currentIndex + 1];
        } else if (steps[currentIndex] == "Finish") {
            info(`start first session, email: ${email}, consent: ${consent}`)
            enable()
            commands.startFirstSession(
                drinkBreakMin,
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

    function canStepNext(): boolean {
        if (currentStep === "Reminder" && !selectedDrinkCharacter) {
            return false
        } else {
            return true
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
            <SelectGender bind:selectedGender={gender} bind:weightInKg={weightInKg} genderImages={data.genderImages}/>
        {:else if currentStep === "Weight"}
            <SelectWeight bind:measureSystem={measureSystem} bind:weightInKg={weightInKg}/>
        {:else if currentStep === "DrinkAmount"}
            <SelectDrinkAmountPerDay bind:drinkAmount={drinkAmount} measureSystem={measureSystem}
                                     min={drinkAmountBasedOnGender - 500} max={drinkAmountBasedOnGender + 500}/>
        {:else if currentStep === "SipSize"}
            <SelectSipSize sipImages={data.sipImages} bind:selectedSipSize={selectedSipSize}
                           drinkBreakMin={drinkBreakMin}
                           measureSystem={measureSystem}/>
        {:else if currentStep === "Reminder"}
            <SelectReminder bind:selectedDrinkCharacter={selectedDrinkCharacter} sipSize={selectedSipSize}
                            reminderImages={data.reminderImages}/>
        {:else if currentStep === "Finish"}
            <SelectEnd bind:email={email} bind:consent={consent}/>
        {/if}
    </div>

    <div class="flex w-full pb-10">
        {#if currentStep !== "Start"}
            <button class="text-secondary/30 py-2 rounded-md" onclick={back}>
                Back
            </button>
        {/if}
        <button class="bg-primary hover:bg-primary/50 text-black py-2 rounded-md px-8 ml-auto disabled:bg-primary/50" disabled={!canStepNext()} onclick={next}>
            {#if currentStep === "Finish"}
                Finish
            {:else}
                Next
            {/if}
        </button>
    </div>
</AutoSize>