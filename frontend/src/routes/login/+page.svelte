<script lang="ts">
	import { getToastStore, type ToastSettings } from "@skeletonlabs/skeleton";
	import { api } from "$lib/api";
	import { clearCookies } from "$lib/utils";
	import { goto } from "$app/navigation";

	let email: string = "";
	let password: string = "";

	const toastStore = getToastStore();

	const show_error = (text: string) => {
		const t: ToastSettings = {
			message: text,
			// Provide any utility or variant background style:
			background: "variant-filled-error",
			timeout: 10000
		};
		toastStore.trigger(t);
	};

	const login = async () => {
		// check that the form is correctly filled
		if (!email || !password) {
			show_error("Les champs doivent être remplis");
			return;
		}

		// clear cookies
		clearCookies();

		// login the user
		if (!(await api.login(email, password))) {
			show_error("Mauvaise combinaison identifiant/mot de passe.");
			return;
		}

		goto("/dashboard");
	};
</script>

<div class="h-screen w-screen flex items-center justify-center">
	<div class="h-fit min-h-1/4 w-fit min-w-1/2 space-y-8">
		<h1 class="h1">Happy Moustaches</h1>
		<form on:submit|preventDefault={login} class="card px-4 py-8 w-full text-token space-y-4">
			<label class="label">
				<span>Email</span>
				<input
					bind:value={email}
					class="input form-input"
					type="email"
					placeholder="tom@moustaches.fr"
				/>
			</label>

			<label class="label">
				<span>Password</span>
				<input
					bind:value={password}
					class="input form-input"
					type="password"
					placeholder="********"
				/>
			</label>

			<div class="w-full items-center flex !mt-6 justify-center">
				<button class="btn variant-filled w-3/4" disabled={$api.loading} type="submit">
					Se connecter
				</button>
			</div>
		</form>
	</div>
</div>
