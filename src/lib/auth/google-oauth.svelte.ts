import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { openUrl } from "@tauri-apps/plugin-opener";
import { PUBLIC_GOOGLE_CLIENT_ID } from "$env/static/public";

import type { UnlistenFn } from "@tauri-apps/api/event";

import {
    OAUTH_EVENT,
    OAUTH_STATUS_EVENT,
    type GoogleOAuthResponse,
    type GoogleOAuthUserResponse,
    type OAuthStatus,
} from "./oauth";
import type { UserInfo } from "@/user/userService.svelte";

export class GoogleOAuth {
    tokenEndpint = "https://oauth2.googleapis.com/token";
    authEndpoint = "https://accounts.google.com/o/oauth2/v2/auth";
    clientId = PUBLIC_GOOGLE_CLIENT_ID;
    scope = "profile email"; // Adjust scopes as needed

    redirectUri = $state("");
    state = $state("");

    code_verifier = $state("");
    code_challenge = $state("");
    port: number | null = $state(null);
    unlisten = $state<UnlistenFn | undefined>(undefined);

    isLoading = $state(false);
    error: unknown = $state(null);

    authUrl = $derived.by(() => {
        if (!this.port) return null;

        const url = new URL(this.authEndpoint);
        url.searchParams.append("client_id", this.clientId);
        url.searchParams.append("redirect_uri", this.redirectUri);
        url.searchParams.append("response_type", "code");
        url.searchParams.append("scope", this.scope);
        url.searchParams.append("code_challenge", this.code_challenge);
        url.searchParams.append("code_challenge_method", "S256");
        url.searchParams.append("state", this.state);

        return url.toString();
    });

    async initialize(fn: (userInfo: UserInfo) => void) {
        try {
            this.unlisten = await listen<OAuthStatus>(
                OAUTH_EVENT.verify,
                async (event) => {
                    const status = event.payload;

                    const data = await this.handleVerifyFlow(status);
                    if (data) {
                        fn(data);
                    }
                },
            );
        } catch (error) {
            this.error = error;
            console.error("Failed to initialize OAuth listener:", error);
        }
    }

    async handleVerifyFlow(status: OAuthStatus) {
        try {
            switch (status.type) {
                case OAUTH_STATUS_EVENT.verify:
                    const code = status.code;
                    const data = await this.getAuthInfo(code);
                    const userInfo = await this.getUserInfo(data.access_token);

                    // TODO: store in db?

                    // return data;
                    return {
                        name: userInfo.name,
                        token: data.access_token,
                        email: userInfo.email,
                        picture: userInfo.picture,
                    };

                case OAUTH_STATUS_EVENT.error:
                    this.error = status.message;
                    break;
            }
        } catch (error) {
            this.error = error;
            console.error("Verification flow error:", error);
        }
    }

    async generateOAuthState() {
        // Generate 32 bytes of random values
        const randomBuffer = new Uint8Array(32);
        window.crypto.getRandomValues(randomBuffer);

        // Convert to hex string
        const state = Array.from(randomBuffer)
            .map((b) => b.toString(16).padStart(2, "0"))
            .join("");

        this.state = state;

        return state;
    }

    base64UrlEncode(arrayBuffer: ArrayBuffer) {
        const bytes = new Uint8Array(arrayBuffer);
        const binString = Array.from(bytes)
            .map((byte) => String.fromCharCode(byte))
            .join("");

        let base64 = btoa(binString);

        return base64
            .replace(/\+/g, "-")
            .replace(/\//g, "_")
            .replace(/=+$/, "");
    }
    async generateCodeVerifier() {
        const possible =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
        const length = 64;

        const randomValues = new Uint8Array(length);
        window.crypto.getRandomValues(randomValues);

        return Array.from(randomValues)
            .map((val) => possible.charAt(val % possible.length))
            .join("");
    }

    async generateCodeChallenge(codeVerifier: string) {
        const encoder = new TextEncoder();
        const data = encoder.encode(codeVerifier);
        const digest = await window.crypto.subtle.digest("SHA-256", data);

        return this.base64UrlEncode(digest);
    }

    async generatePKCEPair() {
        try {
            const codeVerifier = await this.generateCodeVerifier();
            const codeChallenge =
                await this.generateCodeChallenge(codeVerifier);

            this.code_verifier = codeVerifier;
            this.code_challenge = codeChallenge;

            return {
                codeVerifier,
                codeChallenge,
            };
        } catch (error) {
            this.error = error;
            console.error("Error generating PKCE pair:", error);
            throw error;
        }
    }

    async handleOAuthLogin() {
        this.isLoading = true;
        this.error = null;
        try {
            await this.handleStopOAuthServer();

            const oauthState = await this.generateOAuthState();

            this.port = await invoke(OAUTH_EVENT.start_server, {
                state: oauthState,
            });

            if (!this.port) {
                throw new Error("Invalid port received from server");
            }

            this.redirectUri = `http://localhost:${this.port}`;
            await this.generatePKCEPair();

            if (!this.authUrl) {
                throw new Error("Failed to generate authorization URL");
            }

            await openUrl(this.authUrl);
        } catch (error) {
            this.error = error;
            console.error("OAuth login error:", error);
        } finally {
            this.isLoading = false;
        }
    }
    async handleStopOAuthServer() {
        if (!this.port) return;

        try {
            const message = await invoke(OAUTH_EVENT.stop_server, {
                port: this.port,
            });
            return message;
        } catch (error) {
            this.error = error;
            console.error("Error stopping OAuth server:", error);
        } finally {
            this.port = null;
        }
    }

    async getAuthInfo(code: string) {
        // NOTE: client secret code issue for PKCE
        // https://stackoverflow.com/questions/76528208/google-oauth-2-0-authorization-code-with-pkce-requires-a-client-secret
        try {
            const response = await fetch(this.tokenEndpint, {
                method: "POST",
                headers: {
                    "Content-Type": "application/x-www-form-urlencoded",
                },
                body: new URLSearchParams({
                    client_id: this.clientId,
                    code: code,
                    code_verifier: this.code_verifier,
                    grant_type: "authorization_code",
                    redirect_uri: this.redirectUri,
                    code_challenge_method: "S256",
                }),
            });

            if (!response.ok) {
                const errorData = await response.text();
                throw new Error(
                    `Failed to exchange code for tokens: ${response.status} ${errorData}`,
                );
            }

            const data: GoogleOAuthResponse = await response.json();

            return data;
        } catch (error) {
            this.error = error;
            console.error("Error getting auth info:", error);
            throw error;
        }
    }
    async getUserInfo(token: string) {
        try {
            const response = await fetch(
                "https://www.googleapis.com/oauth2/v3/userinfo",
                {
                    headers: {
                        Authorization: `Bearer ${token}`,
                    },
                },
            );
            if (!response.ok) {
                const errorData = await response.text();
                throw new Error(
                    `Failed to get user info: ${response.status} ${errorData}`,
                );
            }
            const user: GoogleOAuthUserResponse = await response.json();

            return user;
        } catch (error) {
            this.error = error;
            console.error("Error getting user info:", error);
            throw error;
        }
    }

    cleanup() {
        if (this.unlisten) {
            this.unlisten();
            this.unlisten = undefined;
        }
        this.handleStopOAuthServer();
    }
}
