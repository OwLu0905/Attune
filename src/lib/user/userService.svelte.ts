import { getContext, setContext } from "svelte";

export const userKey = Symbol("user");

export type UserInfo = {
    token: string | null;
    name: string | null;
    email: string | null;
    picture: string | null;
};

export type UserContext = {
    getUser: () => UserInfo;
    setUser: (userData: UserInfo) => void;
};

export function setUserContext(user: UserContext) {
    setContext(userKey, user);
}
export function getUserContext() {
    return getContext(userKey) as UserContext;
}
