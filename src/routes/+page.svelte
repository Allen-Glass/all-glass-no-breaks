<svelte:head>
	<title>All Glass No Breaks</title>
	<meta name="description" content="Just Allen" />
</svelte:head>

<section class="w-full">
	<div class="text-6xl mr-auto mt-6">
		Software Engineer & <br /> Software Engineering Manager
	</div>
	<div class="text-xl mr-auto mt-2">
		Building modern, scalable SaaS platforms
	</div>
	<div class="mt-8 w-full h-96 rounded-lg border border-secondary bg-secondary p-2 card1">
		<div class="w-full h-full grid grid-cols-6 grid-rows-6 gap-2">
			<WorkExperience />
			<div class="col-span-2 row-span-6 border h-full card rounded-lg px-2 py-1">
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
	<div class="mt-8 w-full border pl-2 pb-2 pr-2 rounded-lg">
		<h1>Work Experience</h1>
		<div class="border p-2 rounded-lg bg-card">
			<h2>Momentus Technologies (formerly Ungerboeck and EventBooking) | 2020 - Present</h2>
			<hr class="my-2" />
			<div class="grid grid-cols-9">
				<div class="col-span-3">
					<h3>Director of Engineering | Momentus Elite</h3>
				</div>
				<div class="col-span-1">
					<h3>2023 - Present</h3>
				</div>
				<div class="col-span-5">
					<ul>
						<li>Did x</li>
						<li>Did y</li>
						<li>Did z</li>
					</ul>
				</div>
			</div>
			<div class="grid grid-cols-9">
				<div class="col-span-3">
					<h3>Team Lead</h3>
				</div>
				<div class="col-span-1">
					<h3>2021 - 2023</h3>
				</div>
				<div class="col-span-5">
					<ul>
						<li>Did x</li>
						<li>Did y</li>
						<li>Did z</li>
					</ul>
				</div>
			</div>
			<div class="grid grid-cols-9">
				<div class="col-span-3">
					<h3>Software Engineer II</h3>
				</div>
				<div class="col-span-1">
					<h3>2020 - 2021</h3>
				</div>
				<div class="col-span-5">
					<ul>
						<li>Did x</li>
						<li>Did y</li>
						<li>Did z</li>
					</ul>
				</div>
			</div>
		</div>
		<div class="border p-2 rounded-lg mt-2 bg-card">
			<div class="flex">
				<img class="inline-block" alt="Microsoft" src="https://upload.wikimedia.org/wikipedia/commons/4/44/Microsoft_logo.svg" width="30" />
				<h2 class="ml-2">Microsoft | 2016 - 2020</h2> 
			</div>
			<hr class="my-2" />
			<div class="grid grid-cols-9">
				<div class="col-span-3">
					<h3>Software Engineer II</h3>
				</div>
				<div class="col-span-1">
					<h3>2019 - 2020</h3>
				</div>
				<div class="col-span-5">
					<ul>
						<li>Did x</li>
						<li>Did y</li>
						<li>Did z</li>
					</ul>
				</div>
			</div>
			<div class="grid grid-cols-9">
				<div class="col-span-3">
					<h3>Software Engineer</h3>
				</div>
				<div class="col-span-1">
					<h3>2018 - 2019</h3>
				</div>
				<div class="col-span-5">
					<ul>
						<li>Did x</li>
						<li>Did y</li>
						<li>Did z</li>
					</ul>
				</div>
			</div>
			<div class="grid grid-cols-9">
				<div class="col-span-3">
					<h3>Service Engineer</h3>
				</div>
				<div class="col-span-1">
					<h3>2016 - 2018</h3>
				</div>
				<div class="col-span-5">
					<ul>
						<li>Did x</li>
						<li>Did y</li>
						<li>Did z</li>
					</ul>
				</div>
			</div>
		</div>
		<div class="border p-2 rounded-lg mt-2 bg-card">
			<h2>Yaskawa America | 2016</h2>
			<hr class="my-2" />
			<div class="grid grid-cols-9">
				<div class="col-span-3">
					<h3>Electrical Engineering Intern</h3>
				</div>
				<div class="col-span-1">
					<h3>2016</h3>
				</div>
				<div class="col-span-5">
					<ul>
						<li>Did x</li>
						<li>Did y</li>
						<li>Did z</li>
					</ul>
				</div>
			</div>
		</div>
	</div>
	<div class="mt-8 w-full border pl-2 pb-2 pr-2 rounded-lg">
		<h1>Skills</h1>
		<div class="border p-2 rounded-lg bg-card">

		</div>
	</div>
	<div class="mt-8 w-full border pl-2 pb-2 pr-2 rounded-lg">
		<h1>Education</h1>
		<div class="border p-2 rounded-lg bg-card">

		</div>
	</div>
</section>

<script lang="ts">
	import WorkExperience from '$lib/components/WorkExperience.svelte';
	import EducationCard from '$lib/components/EducationCard.svelte';
    import { onMount } from 'svelte';
	
	
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