<script lang="ts">
	import './styles.css';
	import "../app.css";
</script>

<div class="app bg-background relative">
	<div class="absolute shell">
		<div class="absolute w-full h-full">
			<svg class="draw w-full h-full" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="20em" height="50em" viewBox="0 0 100 100">
				<polygon fill="#1a1625" fill-opacity="0.0" stroke="#d4af37" stroke-width=".25" points="0 50, 50 100, 100 50, 50 0"/>
			</svg>
		</div>
		<div class="absolute w-full h-full">
			<svg class="draw w-full h-full" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="20em" height="50em" viewBox="0 0 100 100">
				<polygon fill="#1a1625" fill-opacity="0.0" stroke="#382bf0" stroke-width=".1" points="0 50, 50 100, 100 50, 50 0"/>
			</svg>
		</div>
	</div>
	<main>
		<div class="relative ml-1">
			<div class="absolute w-full h-full pr-4">
				<svg class="draw w-full h-full" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="20em" height="50em" viewBox="0 0 100 100">
					<polygon fill="#1a1625" fill-opacity="0.0" stroke="#d4af37" stroke-width=".1" points="0 50, 50 100, 100 50, 50 0"/>
				</svg>
			</div>
			<div class="absolute w-full h-full pr-4">
				<svg class="draw w-full h-full" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="20em" height="50em" viewBox="0 0 100 100">
					<polygon fill="#1a1625" fill-opacity="0.0" stroke="#d4af37" stroke-width=".1" points="25 25, 25 75, 75 75, 75 25"/>
				</svg>
			</div>
			<div class="absolute w-full h-full pr-4">
				<svg class="draw w-full h-full" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="20em" height="50em" viewBox="0 0 100 100">
					<polygon fill="#1a1625" fill-opacity="0.0" stroke="#d4af37" stroke-width=".1" points="25 50, 50 75, 75 50, 50 25"/>
				</svg>
			</div>
			<div class="absolute w-full h-full pr-4">
				<svg class="draw w-full h-full" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="20em" height="50em" viewBox="0 0 100 100">
					<polygon fill="#1a1625" fill-opacity="0.0" stroke="#d4af37" stroke-width=".1" points="38 38, 38 62, 62 62, 62 38"/>
				</svg>
			</div>
			<div class="absolute w-full h-full pr-4">
				<svg class="draw w-full h-full" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="20em" height="50em" viewBox="0 0 100 100">
					<polygon fill="#1a1625" fill-opacity="0.0" stroke="#000" stroke-width=".05" points="0 50, 50 100, 100 50, 50 0"/>
				</svg>
			</div>
			<div class="p-4">
				<slot />
			</div>
		</div>
	</main>

	<footer>
		<p>This site was made by a non Mitch.</p>
	</footer>
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	.shell {
		width: 100%;
		max-height: 100vh;
	}

	.polygon1 {
		clip-path: polygon(100% 0%, 75% 50%, 100% 100%, 25% 100%, 0% 50%, 25% 0%);
	}

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		width: 100%;
		max-width: 68rem;
		margin: 0 auto;
		box-sizing: border-box;
	}

	footer {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 12px;
	}

	@media (min-width: 480px) {
		footer {
			padding: 12px 0;
		}
	}
</style>
