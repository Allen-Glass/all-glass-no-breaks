<svelte:head>
	<title>All Glass No Breaks</title>
	<meta name="description" content="Just Allen" />
	<meta name="title" content="All Glass No Breaks" />
</svelte:head>

<section class="w-full relative">
	<div class="flex w-full">
		<div class="mr-auto">
			<div class="text-6xl mr-auto mt-6 ">
				Software Engineer & <br /> Engineering Manager
			</div>
			<div class="text-xl mr-auto mt-2">
				Building modern, scalable SaaS platforms
			</div>
		</div>
		<div class="mx-auto my-auto">
			<img width="150" alt="allen" class="rounded-md" src="$lib/images/Allen.png" />
		</div>
	</div>
	<div class="mt-4 w-full pb-2 pr-2 rounded-lg">
		<h1>Work Experience</h1>
		<div class="border p-2 rounded-lg bg-card">
			<h2>Momentus Technologies (formerly Ungerboeck and EventBooking), 2020 Sep - Present</h2>
			<hr class="my-2" />
			<div class="grid grid-row-9">
				<div class="row-span-4">
					<h3 class="font-bold inline">Director of Engineering | Momentus Elite,</h3>
					<h4 class="font-semibold inline">2023 Feb - Present</h4>
				</div>
				<div class="row-span-5">
					<ol class="list-disc list-outside ml-6">
						<li>Leading multiple development teams consisting of Dev + QA to deliver a modern, scalable SaaS platform used by the Event and Venue Management Industry</li>
						<li>Orchestrating shared deliverables between products with partner teams across Asia Pacific, India, and Europe to further empower our customers</li>
						<li>Building a strong, communicative remote culture by orchestrating townhalls and social gatherings</li>
						<li>Worked alongside other Engineering Directors to align best engineering practices including the following:
							<ol class="list-disc list-outside ml-6">
								<li>Cut over to Jira from Github Projects</li>
								<li>Migrating engineering teams from 3 month development cycles to 2 week sprints</li>
								<li>Utilizing SonarQube and Dependabot to find then address vulnerabilities in a set window</li>
							</ol>
						</li>
						<li>Decreased time to deploy by over 60%</li>
						<li>Established E2E testing team using Playwright to improve product quality while reducing overall regression testing duration per release cycle</li>
					</ol>
				</div>
			</div>
			<hr class="border-dashed my-4 mx-2" />
			<div class="grid grid-row-9">
				<div class="row-span-4">
					<h3 class="font-bold inline">Team Lead,</h3>
					<h4 class="font-semibold inline">2021 Oct - 2023 Feb</h4>
				</div>
				<div class="row-span-5">
					<ol class="list-disc list-outside ml-6">
						<li>Facilitate group coding standards and ensure utilization of design patterns across a team of full-stack developers</li>
						<li>Led design reviews and architected new .NET Core & Vue functionality to be used by internal and external clients and stakeholders</li>
						<li>Guided engineering team through private equity merger</li>
					</ol>
				</div>
			</div>
			<hr class="border-dashed my-4 mx-2" />
			<div class="grid grid-row-9 mb-2">
				<div class="row-span-4">
					<h3 class="font-bold inline">Software Engineer II,</h3>
					<h4 class="font-semibold inline">2020 Sep - 2021 Sep</h4>
				</div>
				<div class="row-span-5">
					<ol class="list-disc list-outside ml-6">
						<li>As a full-stack developer, utilize AWS, Docker, .Net Core, Vue 2 with Typescript, RavenDb, MySQL, and Docker to design and develop Momentus Elite</li>
					</ol>
				</div>
			</div>
		</div>
		<div class="border p-2 rounded-lg mt-2 bg-card">
			<div class="flex">
				<h2>Microsoft, 2016 Jul - 2020 Sep</h2> 
			</div>
			<hr class="my-2" />
			<div class="grid grid-row-9">
				<div class="row-span-4">
					<h3 class="font-bold inline">Software Engineer II,</h3>
					<h4 class="font-semibold inline">2018 Aug - 2020 Sep</h4>
				</div>
				<div class="row-span-5">
					<ol class="list-disc list-outside ml-6">
						<li>Developed and architected PoC in .Net Core, React.js, and React Native to help handle employee onboarding in a passwordless environment</li>
						<li>Managing 400+ Azure resources security and compliance</li>
					</ol>
				</div>
			</div>
			<hr class="border-dashed my-4 mx-2" />
			<div class="grid grid-row-9">
				<div class="row-span-4">
					<h3 class="font-bold inline">Service Engineer,</h3>
					<h4 class="font-semibold inline">2016 Jul - 2018 Aug</h4>
				</div>
				<div class="row-span-5">
					<ol class="list-disc list-outside ml-6">
						<li>Improved speed and service reliability of several Monolithic IT services by refactoring code base, improved monitoring, and improved automation of operation tasks</li>
						<li>Reported data analytics with the assistance of SQL, Kusto, AppInsights, Microsoft OMS, and Geneva to Power BI to help guide business decisions</li>
					</ol>
				</div>
			</div>
		</div>
		<div class="border p-2 rounded-lg mt-2 bg-card">
			<h2>Yaskawa America, 2015 May - 2015 Aug</h2>
			<hr class="my-2" />
			<div class="grid grid-row-9">
				<div class="row-span-4">
					<h3 class="font-bold inline">Electrical Engineering Intern,</h3>
					<h4 class="font-semibold inline">2015 May - 2015 Aug</h4>
				</div>
				<div class="row-span-5">
					<ol class="list-disc list-outside ml-6">
						<li>Designed and tested circuitry to detect resistor failures on 3-phase load reactors</li>
						<li>Simulated circuit conditions to ensure new circuit designs were viable for testing</li>
					</ol>
				</div>
			</div>
		</div>
	</div>
	<div class="mt-2 w-full pb-2 pr-2 rounded-lg">
		<div class="flex flex-row flex-nowrap">
			<h1 class="float-left mr-2">Skills</h1>
			<div class="flex flex-row my-auto">
				<Pill text="Language" category="Language" />
				<Pill text="Backend" category="Backend" />
				<Pill text="Frontend" category="Frontend" />
				<Pill text="Databases" category="Databases" />
				<Pill text="AWS" category="AWS" />
				<Pill text="Azure" category="Azure" />
				<Pill text="Tooling" category="Tooling" />
			</div>
		</div>
		<div class="border p-2 rounded-lg bg-card">
			<div class="flex flex-row flex-wrap">
				<Pill text="C#" category="Language" />
				<Pill text="Javascript/Typescript" category="Language" />
				<Pill text=".NET Core" category="Backend" />
				<Pill text=".NET Framework" category="Backend" />
				<Pill text="Express.js" category="Backend" />
				<Pill text="Svelte" category="Frontend" />
				<Pill text="Sveltekit" category="Frontend" />
				<Pill text="React" category="Frontend" />
				<Pill text="Vue" category="Frontend" />
				<Pill text="jQuery" category="Frontend" />
				<Pill text="AngularJs" category="Frontend" />
				<Pill text="ASP.NET" category="Frontend" />
				<Pill text="Blazor" category="Frontend" />
				<Pill text="SQL Server" category="Databases" />
				<Pill text="MySQL" category="Databases" />
				<Pill text="Redis" category="Databases" />
				<Pill text="RavenDb" category="Databases" />
				<Pill text="MongoDb" category="Databases" />
				<Pill text="S3" category="AWS" />
				<Pill text="ECS/EC2" category="AWS" />
				<Pill text="ECR" category="AWS" />
				<Pill text="Systems Manager" category="AWS" />
				<Pill text="Elasticache" category="AWS" />
				<Pill text="RDS" category="AWS" />
				<Pill text="Lambda" category="AWS" />
				<Pill text="Secrets Manager" category="AWS" />
				<Pill text="AWS Cognito" category="AWS" />
				<Pill text="App Services" category="Azure" />
				<Pill text="Key Vault" category="Azure" />
				<Pill text="Blob Storage" category="Azure" />
				<Pill text="Azure VM" category="Azure" />
				<Pill text="Logic Apps" category="Azure" />
				<Pill text="Functions" category="Azure" />
				<Pill text="Lambda" category="Azure" />
				<Pill text="Git" category="Tooling" />
				<Pill text="Docker" category="Tooling" />
				<Pill text="Github Actions" category="Tooling" />
				<Pill text="Azure Pipelines" category="Tooling" />
				<Pill text="Jira" category="Tooling" />
				<Pill text="Azure DevOps" category="Tooling" />
			</div>
		</div>
	</div>
	<div class="mt-2 w-full pb-2 pr-2 rounded-lg">
		<h1>Education</h1>
		<div class="border p-2 rounded-lg bg-card flex flex-row">
			<div class="mr-auto">
				<h3>Bachelor of Science in Electrical Engineering</h3> 
				<span>
					<i>Purdue University, West Lafayette</i>
				</span>
			</div>
			<div class="ml-auto mr-2">
				2012 Aug - 2016 May
			</div>
		</div>
	</div>
</section>

<script lang="ts">
    import { onMount } from 'svelte';
	import Pill from '$lib/components/Pill.svelte';

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