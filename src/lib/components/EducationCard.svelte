<div on:mouseenter={() => {showScope = true}} on:mouseleave={() => {showScope = false}} class="border p-2 card rounded-lg cursor-pointer relative">
    <h2>Education</h2>
    {#if showScope}
        <div class="border absolute rounded-md scope" use:beginDraw>
            <svg xmlns="http://www.w3.org/2000/svg" class="absolute" width="1" height="50">
                <rect width="1" height="50" fill="green" />
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-2" width="1" height="50">
                <rect width="1" height="50" fill="green" />
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-4" width="1" height="50">
                <rect width="1" height="50" fill="green" />
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-6" width="1" height="50">
                <rect width="1" height="50" fill="green" />
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-8" width="1" height="50">
                <rect width="1" height="50" fill="green" />
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-10" width="1" height="50">
                <rect width="1" height="50" fill="green" />
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-12" width="1" height="50">
                <rect width="1" height="50" fill="green" />
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" class="absolute top-6" width="50" height="1">
                <rect width="50" height="1" fill="green" />
            </svg>
            <canvas class="absolute z-0 top-3" id="sine" width="50" height="25"></canvas>
            <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50">
                <rect width="50" height="50" fill="#111" />
            </svg>
        </div>
    {/if}
</div>

<script lang="ts">
    let showScope: boolean = false;
    let step: number = 0;
    let animationFrame = -1;

    function beginDraw(node: Node) {
		draw();
		
		return {
			destroy() {
                window.cancelAnimationFrame(animationFrame);
			}
		}
	}

    function plotSinePhase1(context: CanvasRenderingContext2D, frequency: number, xOffset: number) {
        const width = context.canvas.width;
        const height = context.canvas.height;

        context.beginPath();
        context.lineWidth = 2;
        context.strokeStyle = "#00FF00";

        let x = 4;
        let y = 0;
        let amplitude = 8;
        context.moveTo(x, 50);
        while (x < width) {
            y = height/2 + amplitude * Math.sin((x+xOffset)/frequency);
            context.lineTo(x, y);
            x++;
        }
        context.stroke();
        context.save();

        context.stroke();
        context.restore();
    }

    function plotSinePhase2(context: CanvasRenderingContext2D, frequency: number, xOffset: number) {
        const width = context.canvas.width;
        const height = context.canvas.height;

        context.beginPath();
        context.lineWidth = 2;
        context.strokeStyle = "#FFF";

        let x = 4;
        let y = 0;
        let amplitude = 8;
        context.moveTo(x, 50);
        while (x < width) {
            y = height/2 + amplitude * Math.sin((x+xOffset)/frequency);
            context.lineTo(x, y);
            x++;
        }
        context.stroke();
        context.save();

        context.stroke();
        context.restore();
    }

    function plotSinePhase3(context: CanvasRenderingContext2D, frequency: number, xOffset: number) {
        const width = context.canvas.width;
        const height = context.canvas.height;

        context.beginPath();
        context.lineWidth = 2;
        context.strokeStyle = "rgb(66,44,255)";

        let x = 4;
        let y = 0;
        let amplitude = 8;
        context.moveTo(x, 50);
        while (x < width) {
            y = height/2 + amplitude * Math.sin((x+xOffset)/frequency);
            context.lineTo(x, y);
            x++;
        }
        context.stroke();
        context.save();

        context.stroke();
        context.restore();
    }

    function draw() {
        const canvas = document.getElementById("sine") as HTMLCanvasElement;
        const context = canvas.getContext("2d");
        const frequency = 10;

        context.clearRect(0, 0, 500, 500);
        context.save();            
        
        plotSinePhase1(context, frequency, step);
        plotSinePhase2(context, frequency, step + 2.0944 * 10);
        plotSinePhase3(context, frequency, step + 4.18879 * 10);
        context.restore();
        
        step += .2;
        animationFrame = window.requestAnimationFrame(draw);
    }
</script>