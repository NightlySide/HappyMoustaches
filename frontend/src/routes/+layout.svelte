<script>
	import '../app.css';
	import { autoModeWatcher, Toast } from '@skeletonlabs/skeleton';

	import { initializeStores } from '@skeletonlabs/skeleton';
	initializeStores();
</script>

<svelte:head>{@html '<script>(' + autoModeWatcher.toString() + ')();</script>'}</svelte:head>

<Toast position="t" />
<slot />
