<script lang="ts">
    import SelectSessionTime from "./SelectSessionTime.svelte";
    import AutoSize from "../AutoSize.svelte";
    import SelectEnd from "./SelectEnd.svelte";
    import SelectStart from "./SelectStart.svelte";
    import {enable} from "@tauri-apps/plugin-autostart";
    import {commands} from "../../bindings";
    import {type Time, times} from "./Time";
    import {info} from "@tauri-apps/plugin-log";

    let {data} = $props();

    type WelcomeStep = "Start" | "Time" | "Finish"
    let steps: WelcomeStep[] = ["Start", "Time", "Finish"];
    let currentStep: WelcomeStep = $state("Start")

    let email: string | null = $state(null);
    let consent: boolean = $state(true);
    let selectedDuration: Time = $state(times[1]);

    function next() {
        const currentIndex = steps.indexOf(currentStep);
        if (currentIndex < steps.length - 1) {
            currentStep = steps[currentIndex + 1];
        } else if (steps[currentIndex] ==  "Finish") {
            info(`start first session, email: ${email}, consent: ${consent}`)
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
</script>


<AutoSize
        class="flex flex-col bg-gray-100 w-[650px] h-min-[600px] px-12 justify-center cursor-default rounded-2xl"
        ready={true}>
        <div class="flex-none pt-8 mb-8">
            <div class="flex">
                <img alt="Motion Minute" class="size-8" src="{data.iconPath}">
                <p class="text-xl ml-2">Motion Minute</p>
            </div>
        </div>


        <div class="flex-grow mb-12 w-full">
            {#if currentStep === "Start"}
                <SelectStart welcomePath={data.welcomePath}/>
            {:else if currentStep === "Time"}
                <SelectSessionTime bind:selectedDuration={selectedDuration} />
            {:else if currentStep === "Finish"}
                <SelectEnd bind:email={email} bind:consent={consent} />
            {/if}
        </div>

        <div class="flex-none flex w-full pb-12">
            {#if currentStep !== "Start"}
                <button class="text-gray-600 py-2 rounded-md" onclick={back}>
                    Back
                </button>
            {/if}
            <button class="bg-primary hover:bg-primary/50 text-white py-2 rounded-md px-8 ml-auto" onclick={next}>
                {#if currentStep === "Finish"}
                    Start Session
                {:else}
                    Next
                {/if}
            </button>
        </div>
</AutoSize>