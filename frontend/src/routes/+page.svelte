<script>
	import { AppBar } from '@skeletonlabs/skeleton';
</script>

<AppBar>
	<!-- <svelte:fragment slot="lead"
		><img src="/logo.png" alt="Happy moustaches logo" width="100" /></svelte:fragment
	> -->
	<svelte:fragment slot="lead"><a href="/"><h1 class="h3">Happy moustaches</h1></a></svelte:fragment
	>
	<svelte:fragment slot="trail">
		<a href="/login" class="btn variant-filled">Connexion</a>
	</svelte:fragment>
</AppBar>

<div class="px-4 py-4">
	<h1>Title</h1>
</div>
