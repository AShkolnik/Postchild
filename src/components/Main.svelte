<script lang="ts">
	import { AuthType } from '$lib/network/auth_type';
	import type { HttpResponse, HttpRequest } from '$lib/network/http';
	import { TableEntry } from '$lib/utils/table_entry';
	import { http_response } from '$lib/state/store';

	interface ResponseTabs {
		name: string;
		active: boolean;
		content: string | Record<string, string>;
	}

	interface RequestTabs {
		name: string;
		active: boolean;
		content: string | Record<string, string>;
	}

	let response_tabs = [] as ResponseTabs[];
	let request_tabs = [] as RequestTabs[];

	let responseJson = '';
	let active = 'tab_params';

	// $: if ($http_request) {
	// 	request_tabs = [
	// 		{ name: 'Params', active: true, content: $http_request.params },
	// 		{ name: 'Authorization', active: false, content: $http_request.authorization },
	// 		{ name: 'Headers', active: false, content: $http_request.headers }
	// 	];
	// } else {
	// 	request_tabs = [
	// 		{ name: 'Params', active: true, content: { key: '', value: '' } },
	// 		{ name: 'Authorization', active: false, content: AuthType.None },
	// 		{ name: 'Headers', active: false, content: [] as TableEntry[] }
	// 	];
	// }

	$: if ($http_response) {
		response_tabs = [
			{ name: 'Body', active: true, content: $http_response.body },
			{ name: 'Headers', active: false, content: $http_response.headers }
		];
	} else {
		response_tabs = [
			{ name: 'Body', active: true, content: '' },
			{ name: 'Headers', active: false, content: {} }
		];
	}

	function activateTab(name: string) {
		response_tabs = response_tabs.map((tab) => ({
			...tab,
			active: tab.name === name
		}));
	}
</script>

<div class="flex space-x-4 border-b">
	{#each response_tabs as tab}
		<button
			class="-mb-px border-b-2 px-4 py-2 font-medium"
			class:border-blue-500={tab.active}
			class:text-blue-600={tab.active}
			class:border-transparent={!tab.active}
			on:click={() => {
				console.log(tab.name, tab.active, tab.content);
				activateTab(tab.name);
			}}
		>
			{tab.name}
		</button>
	{/each}
</div>
<div class="flex items-center gap-2 bg-gray-200 px-4 py-3">
	{#if response_tabs[0].active}
		{response_tabs[0].content}
	{:else if response_tabs[1].active}
		<table class="min-w-full rounded border bg-white text-sm">
			<thead>
				<tr class="bg-gray-100">
					<th class="p-2 text-left">Key</th>
					<th class="p-2 text-left">Value</th>
				</tr>
			</thead>
			<tbody>
				{#each Object.values(response_tabs[1].content) as value}
					<tr>
						<td class="p-2 text-left">{value[0]}</td>
						<td class="p-2 text-left">{value[1]}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</div>
