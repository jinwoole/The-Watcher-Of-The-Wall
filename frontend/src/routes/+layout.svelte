<script>
	import '../app.css';
	import { writable } from 'svelte/store';

	const currentSession = writable(6); // Initialize with 0

	function selectSession(index) {
		currentSession.set(index);
	}
</script>

<div class="flex">
	<aside class="flex flex-col w-48 h-screen px-4 py-6 overflow-y-auto bg-white border-r rtl:border-r-0 rtl:border-l dark:bg-gray-900 dark:border-gray-700">
		<div class="text-2xl font-extrabold text-gray-800 dark:text-gray-200">DailyNews</div>
		for TmaxCloud

		<div class="flex flex-col justify-between flex-1 mt-6">
			<nav class="-mx-3 space-y-6 ">
				<div class="space-y-3 ">
					<label class="px-3 text-xs text-gray-500 uppercase dark:text-gray-400">Service</label>

						<a
							class="flex items-center px-3 py-2 hover:bg-gray-200 dark:hover:bg-gray-700 ..."
							href="/news"
							on:click="{() => selectSession(0)}"
							class:active={$currentSession === 0}
						>
							<span class="mr-2">📰</span>
							<span class="mx-2 text-sm font-medium">News</span>
						</a>

						<a
							class="flex items-center px-3 py-2 hover:bg-gray-200 dark:hover:bg-gray-700 ..."
							href="/report"
							on:click="{() => selectSession(1)}"
							class:active={$currentSession === 1}
						>
							<span class="mr-2">📊</span> <!-- Updated icon for Report -->
							<span class="mx-2 text-sm font-medium">Report</span>
						</a>

						<a
							class="flex items-center px-3 py-2 hover:bg-gray-200 dark:hover:bg-gray-700 ..."
							href="/press"
							on:click="{() => selectSession(2)}"
							class:active={$currentSession === 2}
						>
							<span class="mr-2">🗞️</span> <!-- Updated icon for Press -->
							<span class="mx-2 text-sm font-medium">Press</span>
						</a>

							<a
							class="flex items-center px-3 py-2 hover:bg-gray-200 dark:hover:bg-gray-700 ..."
							href="/ai-pick"
							on:click="{() => selectSession(3)}"
							class:active={$currentSession === 3}
						>
							<span class="mr-2">🤖</span>
							<span class="mx-2 text-sm font-medium">AI Pick</span>
						</a>
				</div>

				<div class="space-y-3 ">
					<label class="px-3 text-xs text-gray-500 uppercase dark:text-gray-400">content</label>

						<a
							class="flex items-center px-3 py-2 hover:bg-gray-200 dark:hover:bg-gray-700 ..."
							href="/guide"
							on:click="{() => selectSession(4)}"
							class:active={$currentSession === 4}
						>
							<span class="mr-2">📘</span>
							<span class="mx-2 text-sm font-medium">Guide</span>
						</a>
						<a
						class="flex items-center px-3 py-2 hover:bg-gray-200 dark:hover:bg-gray-700 ..."
						href="/setting"
						on:click="{() => selectSession(5)}"
						class:active={$currentSession === 5}
					>
						<span class="mr-2">⚙️</span>
						<span class="mx-2 text-sm font-medium">Setting</span>
					</a>
				</div>
			</nav>
		</div>
	</aside>

	<main class="flex-1 p-8">
		<slot />
	</main>
</div>

<style>
	.active {
		background-color: #f0f0f0; /* Example highlight color */
	}
</style>