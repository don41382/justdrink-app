<script lang="ts">
    import {info} from '@tauri-apps/plugin-log';
    import {
        commands, events, type SipSize,
    } from '../../bindings';
    import {onDestroy, onMount} from 'svelte';
    import type {UnlistenFn} from "@tauri-apps/api/event";
    import CharacterDrinkPlayer from "./CharacterDrinkPlayer.svelte";
    import VideoPlayer from "./VideoPlayer.svelte";
    import {getCurrentWindow} from "@tauri-apps/api/window";

    info("Initialized Session Window")

    let {data} = $props()
    let countdownInterval: number | undefined = $state(undefined);
    let selectedDrinkCharacter = $state(data.selectedDrinkCharacter)
    let sipSize: SipSize = $state(data.sipSize)
    let sessionListener: UnlistenFn | undefined = $state(undefined);
    let endListener: number | undefined = $state(undefined);
    let visible: boolean = $state(false)

    let audioPlayer: CharacterDrinkPlayer;
    let videoPlayer: VideoPlayer;


    onMount(async () => {
        await initScene()
        await getCurrentWindow().show()
        clearTimeout(endListener)
        sessionListener = await events.sessionStartEvent.listen(async (event) => {
            await info("start session")
            await getCurrentWindow().show()
            selectedDrinkCharacter = event.payload.selected_drink_character
            sipSize = event.payload.sip_size
            await initScene()
        })
    })

    function cleanup() {
        if (countdownInterval) {
            clearInterval(countdownInterval);
        }
    }

    function lastPlay() {
        visible = false
        endListener = setTimeout(() => {
            commands.endSession()
        }, 3000)
    }

    async function initScene() {
        await info("init scene")
        cleanup()
        await audioPlayer.play()
        await videoPlayer.play()
        visible = true
    }

    onDestroy(() => {
        cleanup()
        if (sessionListener) {
            sessionListener()
        }
    });

</script>


<div aria-pressed="true"
     class="{visible ? 'fade-in' : 'not-ready'} bg-accent/20 h-screen flex flex-col justify-between items-center overflow-hidden cursor-default">
    <CharacterDrinkPlayer bind:this={audioPlayer} drinkAudio={data.drinkAudio} lastPlay={lastPlay}
                          selectedDrinkCharacter={selectedDrinkCharacter} sipSize={sipSize}/>
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
