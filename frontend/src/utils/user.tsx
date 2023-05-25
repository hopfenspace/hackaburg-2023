import React from "react";

export type User = {
    username: string;
};
type UserContext = {
    setUser(user: User | null): void;
    user: User | null;
};
export const USER_CONTEXT = React.createContext<UserContext>({ setUser: console.error, user: null });

type UserProviderProps = {
    children: React.ReactNode;
};
export function UserProvider(props: UserProviderProps) {
    const [user, setUser] = React.useState<User | null>(null);
    return <USER_CONTEXT.Provider value={{ user, setUser }}>{props.children}</USER_CONTEXT.Provider>;
}
