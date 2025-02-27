<script lang="ts">
    import {info} from '@tauri-apps/plugin-log';
    import {
        commands, type DrinkCharacter, events, type SipSize,
    } from '../../bindings';
    import {onDestroy, onMount} from 'svelte';
    import {type UnlistenFn} from "@tauri-apps/api/event";
    import CharacterDrinkPlayer from "./CharacterDrinkPlayer.svelte";
    import VideoPlayer from "./VideoPlayer.svelte";
    import {getAllWindows, getCurrentWindow} from "@tauri-apps/api/window";

    info("Initialized Session Window")

    let {data} = $props()
    let countdownInterval: number | undefined = $state(undefined);
    let selectedDrinkCharacter: DrinkCharacter | undefined = $state(undefined)
    let demoMode: boolean = $state(false)
    let sipSize: SipSize | undefined = $state(undefined)
    let sessionListener: UnlistenFn | undefined = $state(undefined);
    let visible: boolean = $state(false)

    let endListenerTimer: number;

    let drinkPlayer: CharacterDrinkPlayer | undefined = $state(undefined);
    let videoPlayer: VideoPlayer | undefined = $state(undefined);

    async function welcomeToFront() {
        let welcome = (await getAllWindows()).find((window) => window.label === "welcome")
        if (welcome) {
            await welcome.setFocus()
        }
    }

    onMount(async () => {
        sessionListener = await events.sessionStartEvent.listen(async ({payload}) => {
            visible = false
            await info(`start session - character: ${payload.selected_drink_character} - sip_size: ${payload.sip_size}`)
            selectedDrinkCharacter = payload.selected_drink_character
            sipSize = payload.sip_size
            demoMode = payload.demo_mode
            cleanup()
            await getCurrentWindow().show()
            await welcomeToFront()
            visible = true
            await drinkPlayer?.play();
            await videoPlayer?.play()
        })
    })

    function cleanup() {
        if (countdownInterval) {
            clearInterval(countdownInterval);
        }
        clearTimeout(endListenerTimer)
    }

    function lastPlay() {
        visible = false
        endListenerTimer = setTimeout(() => {
            commands.endSession(demoMode)
        }, 5000)
    }

    onDestroy(async () => {
        visible = false
        await commands.endSession(demoMode)
        cleanup()
        clearTimeout(endListenerTimer);
        if (sessionListener) {
            sessionListener()
        }
    });

</script>


<div aria-pressed="true"
     class="{visible ? 'fade-in' : 'not-ready'} bg-accent/20 opacity-80 h-screen w-screen flex flex-col justify-between items-center overflow-hidden cursor-default">

    <img alt="Background" class="absolute opacity-10 top-0 left-0 w-full h-full object-cover -z-10"
         src="{data.backgroundImage}"/>

    {#if selectedDrinkCharacter}
        <CharacterDrinkPlayer bind:this={drinkPlayer} drinkAudio={data.drinkAudio} lastPlay={lastPlay}
                              selectedDrinkCharacter={selectedDrinkCharacter}/>
    {/if}
    <div class="absolute right-20 bottom-20">
        <VideoPlayer bind:this={videoPlayer} video={data.video}/>
    </div>
</div>

<style>
    .fade-in {
        opacity: 0.8;
        transition: opacity 3500ms;
    }

    .not-ready {
        opacity: 0;
        transition: opacity 2s;
    }
</style>