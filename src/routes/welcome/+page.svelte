<script lang="ts">
    import AutoSize from "../AutoSize.svelte";
    import SelectStart from "./SelectStart.svelte";
    import {enable} from "@tauri-apps/plugin-autostart";
    import {
        commands,
        type DrinkCharacter,
        type GenderType,
        type SipSize
    } from "../../bindings";
    import {info} from "@tauri-apps/plugin-log";
    import SelectGender from "./SelectGender.svelte";
    import SelectWeight from "./SelectWeight.svelte";
    import SelectDrinkAmountPerDay from "./SelectDrinkAmountPerDay.svelte";
    import {MeasureSystem} from "./MeasureSystem";
    import {CalculatedDrinkAmount} from "./CalculatedDrinkAmount";
    import SelectSipSize from "./SelectSipSize.svelte";
    import {WeightConverter} from "./WeightConverter";
    import SelectReminder from "./SelectReminder.svelte";
    import {Sip} from "./SipSize";
    import {sessionTimes} from "../session-times";
    import SelectPayment from "./SelectPayment.svelte";
    import {onMount} from "svelte";
    import {fetchAndInitStripe, type StripeSetup} from "./StripePayment";
    import SelectSubscribe from "./SelectSubscribe.svelte";
    import SelectProduct from "./SelectProduct.svelte";
    import {StripePaymentInfo} from "./StripePaymentInfo.js";

    let {data} = $props();

    type WelcomeStep =
        "Start"
        | "GenderType"
        | "Weight"
        | "DrinkAmount"
        | "SipSize"
        | "Reminder"
        | "Subscribe"
        | "Product"
        | "Purchase"

    function getSteps(): WelcomeStep[] {
        switch (data.welcomeMode) {
            case "Complete":
                return ["Start", "GenderType", "Weight", "DrinkAmount", "SipSize", "Reminder", "Subscribe", "Product", "Purchase"]
            case "OnlySipSettings":
                return ["GenderType", "Weight", "DrinkAmount", "SipSize", "Reminder"]
            case "OnlyPayment":
                return ["Purchase"]
        }
    }

    let steps: WelcomeStep[] = getSteps()
    let currentStep: WelcomeStep = $state(steps.at(0) ?? "Start")

    let initialGender: GenderType = data.settings?.gender_type ?? "Female"
    let initialDrinkCharacter: DrinkCharacter = data.settings?.character ?? "YoungMan"

    let email: string | null = $state(null);
    let consent: boolean = $state(true);

    let measureSystem = $state(MeasureSystem.getMeasureSystem());
    let gender: GenderType | undefined = $state()
    let weightInKg: number = $state(WeightConverter.defaultWeightByGender(initialGender))
    let drinkAmount: number = $state(0)
    let drinkAmountBasedOnGender: number = $state(0)
    let selectedSipSize: SipSize = $state("BigSip")
    let selectedDrinkCharacter: DrinkCharacter | undefined = $state(undefined)
    let drinkBreakMin = $derived(roundToNearestSessionTime((12 * 60) / (drinkAmount / Sip.getMlForSize(selectedSipSize))))

    let paymentInfo: Promise<StripePaymentInfo.Info> = $state(Promise.reject("stripe was not initialized yet"));

    function roundToNearestSessionTime(num: number): number {
        let closest = sessionTimes[0];

        // Iterate through sessionTimes to find the closest number
        for (const time of sessionTimes) {
            if (Math.abs(time - num) < Math.abs(closest - num)) {
                closest = time;
            }
        }

        return closest;
    }

    $effect(() => {
        drinkAmount = CalculatedDrinkAmount.calc(gender ?? initialGender, weightInKg)
        drinkAmountBasedOnGender = CalculatedDrinkAmount.calc(gender ?? initialGender, weightInKg)
    })

    onMount(async () => {
        await info("mount welcome")
        paymentInfo = StripePaymentInfo.fetchPaymentInfo();
    })

    function finishWelcome() {
        info(`start first session, email: ${email}, consent: ${consent}`)
        enable()
        commands.welcomeSave(
            email,
            consent,
            {
                next_break_duration_minutes: drinkBreakMin,
                drink_amount_ml: drinkAmount,
                sip_size: selectedSipSize,
                character: selectedDrinkCharacter ?? initialDrinkCharacter,
                gender_type: gender ?? initialGender,
            }
        )
        commands.welcomeClose()
    }

    function nextFinishWelcomeSettings() {
        info(`finish reset`)
        commands.welcomeSave(
            null,
            null,
            {
                next_break_duration_minutes: drinkBreakMin,
                drink_amount_ml: drinkAmount,
                sip_size: selectedSipSize,
                character: selectedDrinkCharacter ?? initialDrinkCharacter,
                gender_type: gender ?? initialGender,
            }
        )
        if (data.welcomeMode === "OnlySipSettings") {
            commands.welcomeClose()
        } else {
            next()
        }
    }

    function next() {
        const currentIndex = steps.indexOf(currentStep);
        if (currentIndex < steps.length - 1) {
            currentStep = steps[currentIndex + 1];
        }
    }

    function back() {
        const currentIndex = steps.indexOf(currentStep);
        if (currentIndex > 0) {
            currentStep = steps[currentIndex - 1];
        }
    }

    function load() {

    }

    const getProgress = () => (steps.indexOf(currentStep) / (steps.length - 1)) * 100;
</script>


<AutoSize
        class="flex flex-col bg-accent w-[650px] min-h-[450px] px-12 justify-center cursor-default rounded-2xl"
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

    {#await paymentInfo}
    {:then info}

        {#if currentStep === "Start"}
            <SelectStart welcomePath={data.welcomePath} next={next}/>
        {:else if currentStep === "GenderType"}
            <SelectGender bind:selectedGender={gender} bind:weightInKg={weightInKg} genderImages={data.genderImages}
                          back={back} next={next}/>
        {:else if currentStep === "Weight"}
            <SelectWeight bind:measureSystem={measureSystem} bind:weightInKg={weightInKg} back={back} next={next}/>
        {:else if currentStep === "DrinkAmount"}
            <SelectDrinkAmountPerDay bind:drinkAmount={drinkAmount} measureSystem={measureSystem}
                                     min={drinkAmountBasedOnGender - 500} max={drinkAmountBasedOnGender + 500}
                                     back={back} next={next}/>
        {:else if currentStep === "SipSize"}
            <SelectSipSize sipImages={data.sipImages} bind:selectedSipSize={selectedSipSize}
                           drinkBreakMin={drinkBreakMin}
                           measureSystem={measureSystem} back={back} next={next}/>
        {:else if currentStep === "Reminder"}
            <SelectReminder bind:selectedDrinkCharacter={selectedDrinkCharacter} sipSize={selectedSipSize}
                            reminderImages={data.reminderImages} back={back}
                            next={nextFinishWelcomeSettings}/>
        {:else if currentStep === "Subscribe"}
            <SelectSubscribe bind:email={email} bind:consent={consent} back={back} next={next}/>
        {:else if currentStep === "Product"}
            <SelectProduct paymentInfo={info} back={back} next={next}/>
        {:else if currentStep === "Purchase"}
            <SelectPayment paymentInfo={info} email={email} deviceId={data.deviceId}
                           welcomeWizardMode={data.welcomeMode} back={back}/>
        {/if}
    {:catch error}
        <div class="flex-1">
            <div class="flex flex-col w-full h-full">
                <h1 class="flex-none text-4xl text-primary text-left mb-2">Please try again</h1>
                <p class="text-secondary/80 font-light">
                    We have problems reaching the Drink Now! server.
                </p>
                <div class="flex flex-col flex-1 w-full justify-center items-center mt-7">

                    <p class="text-highlight mt-4">We are unable to access the server. Please ensure that you have a
                        working internet connection.</p>
                    <p class="text-highlight/50 text-sm">Error reason: "{error}"</p>
                    <button class="bg-primary hover:bg-primary/50 text-black py-2 rounded-md px-8 ml-auto mt-4"
                            onclick={load}>Reload
                    </button>
                </div>
            </div>
        </div>
    {/await}
</AutoSize>