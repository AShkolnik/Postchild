<script lang="ts">
	import { AuthType } from '$lib/network/auth_type';
	import type { HttpResponse, HttpRequest } from '$lib/network/http';
	import { TableEntry } from '$lib/utils/table_entry';
	import { http_response, http_request } from '$lib/state/store';

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

	let isCollapsed = false;

	$: body = $http_request?.body ?? '';

	function updateBody(value: string) {
		http_request.set({ body: value } as HttpRequest);
	}
</script>

<div class="space-y-2 rounded-2xl border p-4 shadow">
	<button class="w-full text-left font-semibold" on:click={() => (isCollapsed = !isCollapsed)}>
		{isCollapsed ? '▶ Body' : '▼ Body'}
	</button>

	{#if !isCollapsed}
		<textarea
			class="h-40 w-full resize-none rounded border p-2"
			value={body as string}
			on:input={(e) => updateBody(e.currentTarget.value)}
			placeholder="Request body..."
		></textarea>
	{/if}
</div>

{#if $http_response != null}
	<p>Status: {$http_response?.status}</p>
{/if}
<div class="flex-1 overflow-auto bg-gray-200 px-4 py-3">
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
	{#if response_tabs[0].active}
		<pre class="mt-2 max-w-full overflow-auto break-words break-all whitespace-pre-wrap">
			{response_tabs[0].content as string}
		</pre>
	{:else if response_tabs[1].active}
		<table class="min-w-full rounded border bg-white text-sm">
			<thead>
				<tr class="bg-gray-100">
					<th class="p-2 text-left">Key</th>
					<th class="p-2 text-left">Value</th>
				</tr>
			</thead>
			<tbody>
				{#each Object.entries(response_tabs[1].content) as [key, value]}
					<tr>
						<td class="p-2 text-left">{key}</td>
						<td class="p-2 text-left">{value}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</div>
