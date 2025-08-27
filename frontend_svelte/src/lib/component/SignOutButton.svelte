<script lang="ts">
	import { page } from '$app/state';
	import { userManager } from '$lib/service/auth';
	import { authReturnPage } from '$lib/store/auth.svelte';

	const onclick = async () => {
		authReturnPage.value = page.url.href;
		await userManager.signoutRedirect();
	};
</script>

<button class="btn bg-primary" {onclick}>Sign Out</button>
