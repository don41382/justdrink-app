<script lang="ts">
    import SelectStart from "./SelectStart.svelte";
    import {
        commands,
        type DrinkCharacter,
        type GenderType, type LicenseData, type LicensePaymentStatus, type SettingsUserDetails,
        type SipSize, type WelcomeLoadSettings, type WelcomeWizardMode
    } from "../../bindings";
    import {info, warn} from "@tauri-apps/plugin-log";
    import SelectGender from "./SelectGender.svelte";
    import SelectWeight from "./SelectWeight.svelte";
    import SelectDrinkAmountPerDay from "./SelectDrinkAmountPerDay.svelte";
    import {MeasureSystem} from "./MeasureSystem";
    import {CalculatedDrinkAmount} from "./CalculatedDrinkAmount";
    import SelectSipSize from "./SelectSipSize.svelte";
    import {WeightConverter} from "./WeightConverter";
    import SelectReminder from "./SelectReminder.svelte";
    import SelectPayment from "./SelectPayment.svelte";
    import {onMount} from "svelte";
    import SelectSubscribe from "./SelectSubscribe.svelte";
    import SelectProduct from "./SelectProduct.svelte";
    import ThankYou from "./ThankYou.svelte";
    import type {WelcomeImages} from "./+page";
    import type {WelcomeStep} from "./WelcomeStep";
    import {DrinkTimeCalculator} from "./DrinkTimeCalculator";
    import LoadingSpinner from "./LoadingSpinner.svelte";

    let {images, welcomeMode, licenseDataInitial, settings, currentStep = $bindable()}: {
        images: WelcomeImages,
        welcomeMode: WelcomeWizardMode,
        licenseDataInitial: LicenseData,
        settings: WelcomeLoadSettings,
        currentStep: WelcomeStep | undefined,
    } = $props();

    let licenseData: LicenseData = $state(licenseDataInitial)


    function getPaymentSteps(paymentStatus: LicensePaymentStatus): WelcomeStep[] {
        if (welcomeMode !== "OnlySipSettings") {
            switch (paymentStatus) {
                case "Error":
                    return []
                case "Paid":
                    return ["ThankYou"]
                case "ReadyToCapture":
                    return ["ThankYou"]
                case "GoToCheckout":
                    return ["Product", "Purchase", "ThankYou"]
                case "Canceled":
                    return ["Product", "Purchase", "ThankYou"]
            }
        } else {
            return []
        }
    }

    function getSteps(): WelcomeStep[] {
        switch (welcomeMode) {
            case "Complete":
                return ["Start", "GenderType", "Weight", "DrinkAmount", "SipSize", "Reminder", "Subscribe"]
            case "OnlySipSettings":
                return ["GenderType", "Weight", "DrinkAmount", "SipSize", "Reminder"]
            case "CancelPayment":
                return [];
            case "OnlyPayment":
                return [];
        }
    }

    let loading = $state(false);

    let steps: WelcomeStep[] = $state(getSteps().concat(getPaymentSteps(licenseDataInitial.payment.payment_status)))
    let lastStep: boolean = $derived(steps.indexOf(getCurrentStep()) === steps.length - 1)

    let initialGender: GenderType = settings.user?.gender_type ?? "Female"
    let initialDrinkCharacter: DrinkCharacter = settings.user?.character ?? "YoungMan"

    let email: string | null = $state(null);
    let consent: boolean = $state(true);

    let measureSystem = $state(MeasureSystem.getMeasureSystem());
    let gender: GenderType | undefined = $state()
    let weightInKg: number = $state(WeightConverter.defaultWeightByGender(initialGender))
    let drinkAmount: number = $state(0)
    let drinkAmountBasedOnGender: number = $state(0)
    let selectedSipSize: SipSize = $state("BigSip")
    let selectedDrinkCharacter: DrinkCharacter | undefined = $state(undefined)
    let drinkBreakMin = $derived(DrinkTimeCalculator.calc(drinkAmount, selectedSipSize))

    $effect(() => {
        drinkAmount = CalculatedDrinkAmount.calc(gender ?? initialGender, weightInKg)
        drinkAmountBasedOnGender = CalculatedDrinkAmount.calc(gender ?? initialGender, weightInKg)
    })

    onMount(async () => {
        await info(`mount welcome, mode: ${welcomeMode}, paymentInfo: ${licenseData.payment.payment_status}`)
    })

    function getCurrentStep(): WelcomeStep {
        return currentStep ? currentStep : (steps.at(0) ?? "Start")
    }

    async function nextFinishWelcomeUserSettings() {
        await info(`finish reset`)
        loading = true;
        await commands.welcomeSave(
            null,
            null,
            {
                next_break_duration_minutes: drinkBreakMin,
                drink_amount_ml: drinkAmount,
                sip_size: selectedSipSize,
                character: selectedDrinkCharacter ?? initialDrinkCharacter,
                gender_type: gender ?? initialGender,
            }
        ).catch((err) => {
            loading = false;
            warn(`failed to save welcome: ${err}`)
        }).then(() => {
            loading = false;
        })
        if (lastStep) {
            await commands.welcomeClose(getCurrentStep())
        } else {
            next()
        }
    }

    function next() {
        const currentIndex = steps.indexOf(getCurrentStep());
        if (currentIndex < steps.length - 1) {
            currentStep = steps[currentIndex + 1];
        }
    }

    function back() {
        const currentIndex = steps.indexOf(getCurrentStep());
        if (currentIndex > 0) {
            currentStep = steps[currentIndex - 1];
        }
    }

    function firstStep(): boolean {
        return steps.indexOf(getCurrentStep()) == 0
    }

    const getProgress = () => (steps.indexOf(getCurrentStep()) / (steps.length - 1)) * 100;
</script>


<!-- Progress Bar -->
<div class="absolute top-0 left-0 w-full h-4 rounded-t-2xl overflow-hidden">
    <div
            class="h-2 bg-primary/50 transition-all duration-1000"
            style="width: {getProgress()}%;"
    ></div>
</div>

{#if loading}
    <LoadingSpinner fullScreen={true}/>
{/if}

{#if getCurrentStep() === "Start"}
    <SelectStart welcomePath={images.welcomePath} next={next}/>
{:else if getCurrentStep() === "GenderType"}
    <SelectGender bind:selectedGender={gender} bind:weightInKg={weightInKg} genderImages={images.gender}
                  backVisible={!firstStep()} back={back} next={next}/>
{:else if getCurrentStep() === "Weight"}
    <SelectWeight bind:measureSystem={measureSystem} bind:weightInKg={weightInKg} back={back} next={next}/>
{:else if getCurrentStep() === "DrinkAmount"}
    <SelectDrinkAmountPerDay bind:drinkAmount={drinkAmount} measureSystem={measureSystem}
                             min={drinkAmountBasedOnGender - 500} max={drinkAmountBasedOnGender + 500}
                             back={back} next={next}/>
{:else if getCurrentStep() === "SipSize"}
    <SelectSipSize sipImages={images.sip}
                   bind:selectedSipSize={selectedSipSize}
                   drinkAmountMl={drinkAmount}
                   drinkBreakMin={drinkBreakMin}
                   measureSystem={measureSystem} back={back} next={next}/>
{:else if getCurrentStep() === "Reminder"}
    <SelectReminder bind:selectedDrinkCharacter={selectedDrinkCharacter} sipSize={selectedSipSize}
                    reminderImages={images.reminder} back={back}
                    next={nextFinishWelcomeUserSettings} lastStep={lastStep}/>
{:else if getCurrentStep() === "Subscribe"}
    <SelectSubscribe bind:email={email} bind:consent={consent} back={back} next={next}/>
{:else if getCurrentStep() === "Product"}
    <SelectProduct backVisible={!firstStep()} licenseData={licenseData} welcomeWizardMode={welcomeMode} back={back} next={next}/>
{:else if getCurrentStep() === "Purchase"}
    <SelectPayment backendUrl={settings.backend_url} bind:licenseData={licenseData} email={email}
                   deviceId={settings.device_id}
                   welcomeWizardMode={welcomeMode} back={back} next={next}/>
{:else if getCurrentStep() === "ThankYou"}
    <ThankYou licenseData={licenseData} backVisible={!lastStep} back={back}/>
{/if}
