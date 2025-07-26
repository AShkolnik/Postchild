<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	async function request(http_method: String, url: String) {
		const result = await invoke('request', { http_method, url });
		console.log(result);
	}

	let input_url = '';
	let input_http_method = 'GET';
</script>

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
