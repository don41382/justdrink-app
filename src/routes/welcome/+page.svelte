<script lang="ts">
    import SelectSessionTime from "./SelectSessionTime.svelte";
    import AutoSize from "../AutoSize.svelte";
    import SelectEnd from "./SelectEnd.svelte";
    import SelectStart from "./SelectStart.svelte";
    import {enable} from "@tauri-apps/plugin-autostart";
    import {commands} from "../../bindings";
    import {type Time, times} from "./Time";
    import {info} from "@tauri-apps/plugin-log";
    import SelectPainType from "./SelectPainType.svelte";
    import SelectDisclaimer from "./SelectDisclaimer.svelte";
    import type {Pain} from "./Pain";

    let {data} = $props();

    type WelcomeStep = "Start" | "PainType" | "SessionTime" | "Disclaimer" | "Finish"
    let steps: WelcomeStep[] = ["Start", "SessionTime", "Disclaimer", "Finish"];
    let currentStep: WelcomeStep = $state("Start")

    let email: string | null = $state(null);
    let consent: boolean = $state(true);
    let selectedDuration: Time = $state(times[1]);
    let selectedPains: Pain[] = $state([]);

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
                selectedPains.map(pain => pain.id),
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
        class="flex flex-col bg-gray-100 w-[650px] min-h-[550px] px-12 justify-center cursor-default rounded-2xl"
        ready={true}>

        <!-- Progress Bar -->
        <div class="absolute top-0 left-0 w-full h-4 rounded-t-2xl overflow-hidden">
            <div
                    class="h-2 bg-primary/50 transition-all duration-1000"
                    style="width: {getProgress()}%;"
            ></div>
        </div>

        <div class="flex-none pt-8 mb-4">
            <div class="flex">
                <img alt="Motion Minute" class="size-8" src="{data.iconPath}">
                <p class="text-xl ml-2">Motion Minute</p>
            </div>
        </div>


        <div class="flex-grow mb-12 w-full">
            {#if currentStep === "Start"}
                <SelectStart welcomePath={data.welcomePath}/>
            {:else if currentStep === "PainType"}
                <SelectPainType images={data.painTypeImages} selectedPains={selectedPains}/>
            {:else if currentStep === "SessionTime"}
                <SelectSessionTime bind:selectedDuration={selectedDuration} />
            {:else if currentStep === "Disclaimer"}
                <SelectDisclaimer />
            {:else if currentStep === "Finish"}
                <SelectEnd bind:email={email} bind:consent={consent} />
            {/if}
        </div>

        <div class="flex-none flex w-full pb-10">
            {#if currentStep !== "Start"}
                <button class="text-gray-600 py-2 rounded-md" onclick={back}>
                    Back
                </button>
            {/if}
            <button class="bg-primary hover:bg-primary/50 text-white py-2 rounded-md px-8 ml-auto" onclick={next}>
                {#if currentStep === "Finish"}
                    Start your first session
                {:else}
                    Next
                {/if}
            </button>
        </div>
</AutoSize>