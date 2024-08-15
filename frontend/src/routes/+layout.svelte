<script>
	import "../app.css";
	import { autoModeWatcher, Toast } from "@skeletonlabs/skeleton";

	import { initializeStores } from "@skeletonlabs/skeleton";
	initializeStores();

	import { pwaInfo } from 'virtual:pwa-info';
	$: webManifest = pwaInfo ? pwaInfo.webManifest.linkTag : "";
</script>

<svelte:head>
	{@html "<script>(" + autoModeWatcher.toString() + ")();</script>"}
	{@html webManifest}
</svelte:head>

<Toast position="t" />
<slot />

{#await import('$lib/ReloadPrompt.svelte') then { default: ReloadPrompt}}
  <ReloadPrompt />
{/await}