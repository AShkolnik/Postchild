<script>
	export let title = '';
	export let params = [{ key: '', value: '', description: '' }];
</script>

<div class="mb-4">
	<h3 class="mb-2 font-bold">{title}</h3>
	<table class="min-w-full rounded border bg-white text-sm">
		<thead>
			<tr class="bg-gray-100">
				<th class="p-2 text-left">KEY</th>
				<th class="p-2 text-left">VALUE</th>
				<th class="p-2 text-left">DESCRIPTION</th>
			</tr>
		</thead>
		<tbody>
			{#each params as param}
				<tr>
					<td class="p-2">{param.key}</td>
					<td class="p-2">{param.value}</td>
					<td class="p-2">{param.description}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
