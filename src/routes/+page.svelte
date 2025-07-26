<script lang="ts">
	import { AuthType } from '$lib/network/auth_type';
	import type { Cookie, Header, HttpResponse } from '$lib/network/http';
	import { TableEntry } from '$lib/utils/table_entry';

	export let response: HttpResponse | null = null;

	let title = '';
	let json = {};
	let params: TableEntry[] = [];

	let responseJson = '';
	let active = 'tab_params';

	let request_tabs = [
		{ name: 'Params', active: true, content: [] as TableEntry[] },
		{ name: 'Authorization', active: false, content: AuthType.None },
		{ name: 'Headers', active: false, content: [] as TableEntry[] }
	];

	let response_tabs = [
		{ name: 'Body', active: true, content: response?.body ?? '' },
		{ name: 'Cookies', active: false, content: response?.cookies ?? [] },
		{ name: 'Headers', active: false, content: response?.headers ?? [] }
	];

	function activateTab(name: string) {
		response_tabs = response_tabs.map((tab) => ({
			...tab,
			active: tab.name === name
		}));
	}
</script>

<section class="flex-1 bg-gray-100 p-6">
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
						{#each Object.keys(response_tabs[1].content as Cookie) as key}
							<th class="p-2 text-left">{key}</th>
						{/each}
					</tr>
				</thead>
				<tbody>
					<tr>
						{#each Object.values(response_tabs[1].content as Cookie) as value}
							<td class="p-2 text-left">{value}</td>
						{/each}
					</tr>
				</tbody>
			</table>
		{:else if response_tabs[2].active}
			<table class="min-w-full rounded border bg-white text-sm">
				<thead>
					<tr class="bg-gray-100">
						{#each Object.keys(response_tabs[2].content as Header) as key}
							<th class="p-2 text-left">{key}</th>
						{/each}
					</tr>
				</thead>
				<tbody>
					<tr>
						{#each Object.values(response_tabs[1].content as Header) as value}
							<td class="p-2 text-left">{value}</td>
						{/each}
					</tr>
				</tbody>
			</table>
		{/if}
	</div>
</section>
