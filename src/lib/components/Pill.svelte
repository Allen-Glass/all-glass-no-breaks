<div style="background: {backgroundColor};" class="px-2 py-1 my-2 mx-1 text-xs border rounded-full flex max-h-7">
    <div class="my-auto mx-auto">{text}</div>
</div>

<script lang="ts">
  import { onMount } from "svelte";

    export let category: "Language" | "AWS" | "Azure" | "Databases" | "Tooling" | "Frontend" | "Backend" = "Language";
    export let text: string = "";
    let backgroundColor: string = "";

    onMount(() => {
        setBackgroundColor();
    });

    function setBackgroundColor(): void {
        switch(category) {
            case "Language":
                backgroundColor = "rgb(238, 238, 238, .4)";
                break;
            case "Azure":
                backgroundColor = "rgb(56, 43, 240, .8)";
                break;
            case "AWS":
                backgroundColor = "rgb(52, 65, 123, .8)";
                break;
            case "Databases":
                backgroundColor = "rgb(97, 43, 58, .8)";
                break;
            case "Tooling":
                backgroundColor = "rgb(18, 18, 18, .8)";
                break;
            case "Frontend":
                backgroundColor = "rgb(26, 22, 37, .8)";
                break;
            case "Backend":
                backgroundColor = "rgb(70, 66, 79, .8)";
                break;
        }
    }
</script>