<script lang="ts">
	import { returnToOriginalPage, userManager } from '$lib/service/auth';
	import type { User } from 'oidc-client-ts';
	import { onMount } from 'svelte';

	let promise: Promise<User | undefined> = $state(Promise.resolve(undefined));

	onMount(async () => {
		promise = userManager.signinCallback();
		try {
			let user = await promise;
			if (undefined === user) {
				console.error('Sign in returned empty user');
			}
		} catch (error) {
			console.error('Sign in failed with:', error);
		} finally {
			setTimeout(returnToOriginalPage, 1000);
		}
	});
</script>

<svelte:head>
	<title>App Login Redirect</title>
</svelte:head>

{#await promise}
	<div>Handling signin callback</div>
{:then value}
	<div>Welcome {value?.profile.name}! Returning you to original page...</div>
{:catch error}
	<div>Error from signin callback: {error}</div>
{/await}
