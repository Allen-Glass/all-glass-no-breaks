<div class="mt-8 w-full h-96 rounded-lg border border-secondary bg-secondary p-2 hero-card">
		<div class="w-full h-full grid grid-cols-6 grid-rows-6 gap-2">
			<WorkExperience />
			<div class="col-span-2 row-span-6 border h-full card-allen rounded-lg px-2 py-1">
				<div class="flex flex-col h-full">
					<div class="mx-auto">
						<img width="200" class="rounded-full" src="$lib/images/Allen.png" />
					</div>
					<div class="px-2 mt-2 border h-full rounded-lg mb-1">
						When not working, I enjoy spending time with my family
					</div>
				</div>
			</div>
			<div class="border-gradient row-span-2 rounded-lg">
				<div class="border p-2 card rounded-lg border-opacity-0 cursor-pointer">
					<h2>Skills</h2>
				</div>
			</div>
			<div class="border-gradient row-span-2 rounded-lg">
				<EducationCard />
			</div>
			<div class="border-gradient row-span-2 rounded-lg">
				<div class="col-span-2 border p-2 card rounded-lg cursor-pointer">
					<h2>Contact</h2>
				</div>
			</div>
			<div class="border-gradient row-span-2 rounded-lg">
				<div class="border bg-card align-middle card rounded-lg flex cursor-pointer">
					<div class="mx-auto">
						<img src="$lib/images/github.svg" alt="github" width="110" />
					</div>
				</div>
			</div>
		</div>
	</div>

<script lang="ts">
	import WorkExperience from '$lib/components/WorkExperience.svelte';
	import EducationCard from '$lib/components/EducationCard.svelte';
    import { onMount } from 'svelte';

	let drawHeight: number = 0;
	let drawWidth: number = 0;
	
	onMount(() => {
		onHashChange();
		window.addEventListener("hashchange", onHashChange);
	})

	function onHashChange() {
		const hash = location.hash;

		if (!hash)
			return;

		const elementId = hash.replace("#", "");
		const element = document.getElementById(elementId);

		if (!element)
			return;

		element.focus();
	}
</script>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}
</style>

