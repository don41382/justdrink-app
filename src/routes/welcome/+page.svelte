<script lang="ts">
    import AutoSize from "../AutoSize.svelte";
    import SelectStart from "./SelectStart.svelte";
    import {enable} from "@tauri-apps/plugin-autostart";
    import {
        commands,
        type DrinkCharacter,
        type GenderType, type LicensePaymentStatus,
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
    import SelectSubscribe from "./SelectSubscribe.svelte";
    import SelectProduct from "./SelectProduct.svelte";
    import Icon from "@iconify/svelte";
    import ThankYou from "./ThankYou.svelte";

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
        | "ThankYou"

    function getPaymentSteps(paymentStatus: LicensePaymentStatus): WelcomeStep[] {
        if (data.welcomeMode !== "OnlySipSettings") {
            switch (paymentStatus) {
                case "Error":
                    return []
                case "Paid":
                    return ["Purchase"]
                case "ReadyToCapture":
                    return ["ThankYou"]
                case "Start":
                case "Canceled":
                    return ["Product", "Purchase", "ThankYou"]
                case "RequireInfo":
                    return ["Purchase", "ThankYou"]
            }
        } else {
            return []
        }
    }

    function getSteps(): WelcomeStep[] {
        switch (data.welcomeMode) {
            case "Complete":
                return ["Start", "GenderType", "Weight", "DrinkAmount", "SipSize", "Reminder", "Subscribe"]
            case "OnlySipSettings":
                return ["GenderType", "Weight", "DrinkAmount", "SipSize", "Reminder"]
            case "OnlyPayment":
                return [];
        }
    }

    let steps: WelcomeStep[] = $state(getSteps().concat(getPaymentSteps(data.licenseData.payment.payment_status)))
    let currentStep: WelcomeStep = $state(steps.at(0) ?? "Start")
    let lastStep: boolean = $derived(steps.indexOf(currentStep) === steps.length - 1)

    let initialGender: GenderType = data.settings.user?.gender_type ?? "Female"
    let initialDrinkCharacter: DrinkCharacter = data.settings.user?.character ?? "YoungMan"

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
        await info(`mount welcome, mode: ${data.welcomeMode}, paymentInfo: ${data.licenseData.payment.payment_status}`)
    })

    function nextFinishWelcomeUserSettings() {
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
        if (lastStep) {
            commands.welcomeClose(currentStep)
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

    function firstStep(): boolean {
        return steps.indexOf(currentStep) == 0
    }

    async function close() {
        await commands.welcomeClose(currentStep)
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

    <div class="flex pt-8 mb-4 justify-between items-center">
        <div class="flex items-center">
            <img alt="Drink Now!" class="size-12" src="{data.iconPath}">
            <p class="text-xl ml-2 text-primary">Drink Now!</p>
        </div>
        <button class="flex flex-col items-center justify-center cursor-pointer rounded-full hover:bg-gray-600 text-secondary/20 hover:text-white p-1"
                onclick={async () => { await close() }}>
            <Icon class="size-6" icon="iconoir:xmark"/>
        </button>
    </div>

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
                        next={nextFinishWelcomeUserSettings} lastStep={lastStep}/>
    {:else if currentStep === "Subscribe"}
        <SelectSubscribe bind:email={email} bind:consent={consent} back={back} next={next}/>
    {:else if currentStep === "Product"}
        <SelectProduct backVisible={!firstStep()} licenseData={data.licenseData} back={back} next={next}/>
    {:else if currentStep === "Purchase"}
        <SelectPayment backendUrl={data.settings.backend_url} licenseData={data.licenseData} email={email} deviceId={data.deviceId}
                       welcomeWizardMode={data.welcomeMode} back={back}/>
    {:else if currentStep === "ThankYou"}
        <ThankYou licenseData={data.licenseData} backVisible={!lastStep} back={back}/>
    {/if}
</AutoSize>