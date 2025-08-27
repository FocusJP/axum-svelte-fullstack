import { type ExtraSigninRequestArgs, type UserManagerSettings } from 'oidc-client-ts';

export const signInRequestArgs: ExtraSigninRequestArgs = {
	extraQueryParams: {
		audience: import.meta.env.VITE_AUTH_AUDIENCE
	}
};

export const userManagerSettings: UserManagerSettings = {
	authority: import.meta.env.VITE_AUTH_AUTHORITY,
	client_id: import.meta.env.VITE_AUTH_CLIENT_ID,
	redirect_uri: import.meta.env.VITE_AUTH_LOGIN_REDIRECT_URI,
	response_type: 'code',
	scope: 'openid profile email read:ssa_stats',
	post_logout_redirect_uri: import.meta.env.VITE_AUTH_LOGOUT_REDIRECT_URI
};
