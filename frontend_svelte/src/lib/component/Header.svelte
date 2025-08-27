<script lang="ts">
	import { userStore } from '$lib/store/auth.svelte';
	import SignInButton from './SignInButton.svelte';
	import SignOutButton from './SignOutButton.svelte';
</script>

<div class="navbar bg-base-200 shadow-sm">
	<div class="flex-1">
		<div class="text-xl">Example App</div>
	</div>
	<div class="flex-none">
		<a href="/ssa-names" class="btn">SSA Names</a>
		{#if !userStore.value}
			<SignInButton />
		{:else}
			<SignOutButton />
		{/if}
	</div>
</div>
