<script lang="ts">
	import { userStore } from '$lib/store/auth.svelte';
	let { children } = $props();
</script>

{#if !userStore.value}
	<div>This is a protected page, sign in to access</div>
{:else}
	{@render children?.()}
{/if}
