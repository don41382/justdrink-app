<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window';
    import type {UnlistenFn} from "@tauri-apps/api/event";
    import {debug} from "@tauri-apps/plugin-log";

    export let ready: boolean = true;

    let container: HTMLDivElement;
    let resize: UnlistenFn;

    async function resizeWindow() {
        const currentWindow = getCurrentWindow();

        if (container && ready) {
            await debug("adjust window size")
            let rect = container.getBoundingClientRect()
            const factor = window.devicePixelRatio;
            const width: number = Math.ceil(rect.width * factor);
            const height: number = Math.ceil(rect.height * factor);

            let topPadding = await currentWindow.isDecorated() ? 55 : 0
            let size = new PhysicalSize(width, height + topPadding);

            await currentWindow.setSize(size);
            await getCurrentWindow().center();
            await getCurrentWindow().show();
            await getCurrentWindow().setFocus();
        }
    }

    let observer: ResizeObserver;

    onMount(async () => {
        observer = new ResizeObserver(async () => {
            await resizeWindow();
        });
        if (container) observer.observe(container);
    });

    onDestroy(() => {
        if (observer) observer.disconnect();
        resize();
    });
</script>

<div {...$$restProps} bind:this={container}>
    <slot></slot>
</div>