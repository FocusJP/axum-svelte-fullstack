import { apiUrl } from '$lib/config/api';
import { userStore } from '$lib/store/auth.svelte';
import axios from 'axios';

axios.interceptors.request.use((config) => {
	if (config.url?.startsWith(apiUrl)) {
		const token = userStore.value?.access_token;
		config.headers.Authorization = `Bearer ${token}`;
	}
	return config;
});
