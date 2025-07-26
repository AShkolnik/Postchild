<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	async function request(http_method: String, url: String) {
		const result = await invoke('request', { http_method, url });
		console.log(result);
	}

	let input_url = '';
	let input_http_method = 'GET';
</script>

<div class="flex h-screen">
	<aside class="flex w-16 flex-col items-center space-y-6 bg-gray-900 py-4 text-gray-100">
		<button class="rounded p-2 hover:bg-gray-800" title="Workspaces" aria-label="Workspaces">
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="h-6 w-6"
			>
				<rect x="3" y="3" width="7" height="7" rx="1.5" />
				<rect x="14" y="3" width="7" height="7" rx="1.5" />
				<rect x="3" y="14" width="7" height="7" rx="1.5" />
				<rect x="14" y="14" width="7" height="7" rx="1.5" />
			</svg>
		</button>
		<button class="rounded p-2 hover:bg-gray-800" title="History" aria-label="History">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="h-6 w-6"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
			>
				<path d="M12 8v4l3 3" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" />
				<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" fill="none" />
			</svg>
		</button>
		<button class="rounded p-2 hover:bg-gray-800" title="Collections" aria-label="Collections">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="h-6 w-6"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
			>
				<path d="M4 6h16M4 10h16M4 14h16M4 18h16" />
			</svg>
		</button>
	</aside>

	<main class="flex flex-1 flex-col">
		<header class="flex items-center gap-2 bg-gray-800 px-4 py-3">
			<select
				bind:value={input_http_method}
				class="ml-2 rounded bg-blue-500 px-4 py-1 text-white transition hover:bg-blue-600"
			>
				<option>GET</option>
				<option>POST</option>
				<option>PUT</option>
				<option>DELETE</option>
			</select>
			<input
				bind:value={input_url}
				placeholder="Enter request URL"
				class="flex-1 rounded bg-gray-700 px-3 py-1 text-white focus:outline-none"
				type="text"
			/>
			<button
				class="ml-2 rounded bg-blue-600 px-4 py-1 text-white transition hover:bg-blue-700"
				on:click={() => request(input_http_method, input_url)}>Send</button
			>
		</header>
		<section class="flex-1 bg-gray-100 p-6">
			<div class="mt-12 text-center text-gray-500">Response output (implement as needed)</div>
		</section>
	</main>
</div>
