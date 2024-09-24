<script lang="ts">
    import Icon from "@iconify/svelte";
</script>

<div class="p-4 mb-4 text-blue-800 border border-blue-300 rounded-lg bg-blue-50 dark:bg-gray-800 dark:text-blue-400 dark:border-blue-800"
     id="alert" role="alert">
    <div class="flex items-center">
        <Icon height="24" icon="mdi-light:alert"/>
        <h3 class="text-lg font-medium">This is a info alert</h3>
    </div>
    <div class="mt-2 mb-4 text-sm">
        More info about this info alert goes here. This example text is going to run a bit longer so that you can see
        how spacing within an alert works with this kind of content.
    </div>
    <div class="flex">
        <button class="text-white bg-blue-800 hover:bg-blue-900 focus:ring-4 focus:outline-none focus:ring-blue-200 font-medium rounded-lg text-xs px-3 py-1.5 me-2 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                type="button">
            <Icon height="24" icon="mdi-light:send-rounded"/>
            Send Error
        </button>
        <button aria-label="Close"
                class="text-blue-800 bg-transparent border border-blue-800 hover:bg-blue-900 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-200 font-medium rounded-lg text-xs px-3 py-1.5 text-center dark:hover:bg-blue-600 dark:border-blue-600 dark:text-blue-400 dark:hover:text-white dark:focus:ring-blue-800"
                data-dismiss-target="#alert-additional-content-1" type="button">
            Dismiss
        </button>
    </div>
</div>