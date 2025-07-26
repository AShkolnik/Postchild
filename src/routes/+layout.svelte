<script lang="ts">
	import '../app.css';
	import Sidebar from '../components/Sidebar.svelte';
	import Header from '../components/Header.svelte';

	let { children } = $props();
</script>

<div class="flex h-screen">
	<Sidebar />
	<div class="flex flex-1 flex-col">
		<main class="flex flex-1 flex-col">
			<Header />
			{@render children()}
		</main>
	</div>
</div>
