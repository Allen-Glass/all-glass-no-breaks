<div class="green absolute left-60 w-96">
    {displayedScript}
</div>

<script lang="ts">
    import { onMount } from "svelte";

    let script: string = `fn hanoi(n: i32, source: char, auxiliary: char, target: char) {
        if n == 1 {
            println!("Move disk 1 from {} to {}", source, target);
            return;
        }

        hanoi(n - 1, source, target, auxiliary);
        println!("Move disk {} from {} to {}", n, source, target);
        hanoi(n - 1, auxiliary, source, target);
    }`
    let displayedScript: string = "";
    let i: number = 0;
    let timer: number = -1;

    onMount(() => {
        typewriter();
    })

    function typewriter() {
        if (displayedScript.length === script.length) {
            window.setTimeout(() => {
                resetAnimation();
            }, 3000);
            return;
        }

        displayedScript = displayedScript + script[i];
        i = i + 1;
        timer = window.setTimeout(() => {
            typewriter()
        }, 50)
    }

    function resetAnimation() {
        displayedScript = "";
        i = 0;
        typewriter();
    }
</script>