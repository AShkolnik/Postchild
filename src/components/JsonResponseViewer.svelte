<script>
	export let json = {};
</script>

<div class="rounded bg-gray-100 p-2">
	<pre class="overflow-auto rounded bg-gray-900 p-3 text-xs text-green-300">{JSON.stringify(
			json,
			null,
			2
		)}</pre>
</div>
