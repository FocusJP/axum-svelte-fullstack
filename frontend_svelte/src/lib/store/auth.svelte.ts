import type { User } from 'oidc-client-ts';

const authOriginalUriKey = 'authOriginalUri';

/** The original page to which redirect signin should ultimately return */
let authReturnPageValue: string | undefined | null = $state(
	sessionStorage.getItem(authOriginalUriKey)
);

export const authReturnPage = {
	get value() {
		return authReturnPageValue;
	},
	set value(newValue) {
		if (!newValue) {
			sessionStorage.removeItem(authOriginalUriKey);
		} else {
			sessionStorage.setItem(authOriginalUriKey, newValue);
		}
		authReturnPageValue = newValue;
	}
};

/** Authenticated user state */
export const userStore: { value: User | undefined | null } = $state({ value: undefined });
