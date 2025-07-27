<script lang="ts">
	import '../app.css';
	import Sidebar from '../components/Sidebar.svelte';
	import Header from '../components/Header.svelte';

	let { children } = $props();
</script>

<div class="flex h-screen overflow-hidden">
	<Sidebar />
	<div class="flex flex-1 flex-col">
		<main class="flex-1 overflow-y-auto p-4">
			<Header />
			{@render children()}
		</main>
	</div>
</div>
