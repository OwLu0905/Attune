export const OAUTH_EVENT = {
    start_server: "start_oauth_server",
    stop_server: "stop_oauth_server",
    verify: "oauth_code_state",
};

export const OAUTH_STATUS_EVENT = {
    verify: "Verify",
    error: "Error",
} as const;

export type OAuthStatus =
    | { type: typeof OAUTH_STATUS_EVENT.verify; code: string }
    | { type: typeof OAUTH_STATUS_EVENT.error; message: string };

export type GoogleOAuthResponse = {
    access_token: string;
    expires_in: string;
    id_token: string;
    refresh_token: string;
    refresh_token_expires_in: string;
    scope: string;
    token_type: "Bearer";
};

export type GoogleOAuthUserResponse = {
    sub: string;
    name: string;
    given_name: string;
    family_name: string;
    picture: string;
    email: string;
    email_verified: boolean;
};
