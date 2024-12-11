<script lang="ts">
    import {onMount, onDestroy} from 'svelte';
    import {getCurrentWindow, PhysicalSize} from '@tauri-apps/api/window';
    import type {UnlistenFn} from "@tauri-apps/api/event";
    import {type} from "@tauri-apps/plugin-os"

    let {ready = true, ...rest} = $props();

    let container: HTMLDivElement;
    let resize: UnlistenFn;

    async function resizeWindow() {
        const currentWindow = getCurrentWindow();

        if (container && ready) {
            let rect = container.getBoundingClientRect()
            const factor = window.devicePixelRatio;
            const width: number = Math.ceil(rect.width * factor);
            const height: number = Math.ceil(rect.height * factor);

            let topPadding = await currentWindow.isDecorated() && type() === 'macos' ? 55 : 0
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
        resize?.();
    });
</script>

<div {...rest} bind:this={container}>
    <slot></slot>
</div>