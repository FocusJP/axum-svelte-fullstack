<script lang="ts">
	import { returnToOriginalPage, userManager } from '$lib/service/auth';
	import type { SignoutResponse } from 'oidc-client-ts';
	import { onMount } from 'svelte';

	let promise: Promise<SignoutResponse | undefined> = $state(Promise.resolve(undefined));

	onMount(async () => {
		console.log('Logout page onMount');
		promise = userManager.signoutCallback();
		try {
			let signoutResponse = await promise;
			console.log(signoutResponse);
		} catch (error) {
			console.error('Sign out failed with:', error);
		} finally {
			setTimeout(returnToOriginalPage, 1000);
		}
	});
</script>

<svelte:head>
	<title>App Logout Redirect</title>
</svelte:head>

{#await promise}
	<div>Handling signout callback</div>
{:then value}
	<div>Returning you to original page...</div>
{:catch error}
	<div>Error from signout callback: {error}</div>
{/await}
