<div on:mouseenter={() => showAnimations = true} on:mouseleave={() => showAnimations = false} on:click={focus} class="border-gradient col-span-4 row-span-4 rounded-lg">
    <div id="work" style="height: 98.5%;" class="border card rounded-lg p-4 cursor-pointer">
        <div class="relative">
            <h1>Work Experience</h1>
            {#if showAnimations}
                <div style="top: {imageYPosition}px; left: {imageXPosition}px;" class="absolute" use:beginTick>
                    <img id="msft" class=" opacity-30" alt="Microsoft" src="https://upload.wikimedia.org/wikipedia/commons/4/44/Microsoft_logo.svg" width="30" />
                </div>
                <div class="absolute">
					<Hanoi />
                </div>
            {/if}
        </div>
    </div>
</div>

<script lang="ts">
    import Hanoi from './Hanoi.svelte';
    import { onMount } from 'svelte';
	
	let animationRequest: number = -1;
	let boundaries: DOMRect = null;
	let imageBoundaries: DOMRect = null;
	let showAnimations: boolean = false;

	const movementSpeed = .5;
	let xVelocity = -movementSpeed;
	let yVelocity = movementSpeed;
	let imageXPosition = 250;
	let imageYPosition = 100;

	onMount(() => {
		calculateBoundaries();
		addEventListener("resize", (event) => calculateBoundaries());
	})

	function beginTick(node: Node) {
		imageBoundaries = document.getElementById("msft").getBoundingClientRect();
		tick();
		
		return {
			destroy() {
				window.cancelAnimationFrame(animationRequest);
			}
		}
	}

	function tick() {
		moveImage();
		animationRequest = window.requestAnimationFrame(tick);
	}

	function calculateBoundaries() {
		const work = document.getElementById("work");
		boundaries = work.getBoundingClientRect();
	}

	function moveImage() {
		const imageXPositionOffset = imageXPosition + boundaries.left + imageBoundaries.height;
		const imageYPositionOffset = imageYPosition + boundaries.top + imageBoundaries.width;
		const rightCollision = imageXPositionOffset >= boundaries.right - 35;
		const leftCollision = imageXPositionOffset <= boundaries.left + 15;
		const bottomCollision = imageYPositionOffset >= boundaries.bottom - 15;
		const topCollision = imageYPositionOffset <= boundaries.top + 15;

		if (rightCollision || leftCollision)
			xVelocity = -1 * xVelocity;

		if (topCollision || bottomCollision)
			yVelocity = -1 * yVelocity;

		imageXPosition = xVelocity + imageXPosition;
		imageYPosition = yVelocity + imageYPosition;
	}

	function focus() {
		window.location.hash = "work";
	}

</script>