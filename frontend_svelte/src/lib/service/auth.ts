import { goto } from '$app/navigation';
import { userManagerSettings } from '$lib/config/auth';
import { authReturnPage, userStore } from '$lib/store/auth.svelte';
import { UserManager, Log, WebStorageStateStore } from 'oidc-client-ts';

Log.setLogger(console);

// Navigate user back to original page after sign in / out redirect
export const returnToOriginalPage = () => goto(authReturnPage.value || '/');

// Choose how to persist (or not) the user auth state
userManagerSettings.userStore = new WebStorageStateStore({ store: window.localStorage });

// Main API for auth flow
export const userManager = new UserManager(userManagerSettings);

// Ensure we capture the initial state if user already authenticated in browser.
userStore.value = await userManager.getUser();

// Fires after Sign In redirect flow
userManager.events.addUserLoaded((user) => {
	userStore.value = user;
});

// Fires after Sign Out redirect flow
userManager.events.addUserUnloaded(() => {
	userStore.value = undefined;
});
