import Input from "../components/input";
import React from "react";
import "../style/login.css";
import { USER_CONTEXT } from "../utils/user";
import { ROUTES } from "../routes";

type LoginProps = {};
export default function Login(props: LoginProps) {
    const { user, setUser } = React.useContext(USER_CONTEXT);
    const onSubmit = (username: string) => {
        setUser({ username });
        ROUTES.HOME.visit({});
    };
    return (
        <div className="login-container">
            <LoginForm onSubmit={onSubmit} />
            <RegisterForm onSubmit={onSubmit} />
        </div>
    );
}

type LoginFormProps = {
    onSubmit(username: string, password: string): void;
};
function LoginForm(props: LoginFormProps) {
    const [username, setUsername] = React.useState("");
    const [password, setPassword] = React.useState("");
    return (
        <form
            className="login-form"
            onSubmit={(event) => {
                event.preventDefault();
                props.onSubmit(username, password);
            }}
        >
            <h1>Login</h1>
            <div>
                <label>
                    Username: <Input value={username} onChange={setUsername} />
                </label>
                <label>
                    Password: <Input type="password" value={password} onChange={setPassword} />
                </label>
            </div>
            <button type="submit" className="button">
                Login
            </button>
        </form>
    );
}

type RegisterFormProps = {
    onSubmit(username: string, email: string, password: string): void;
};
function RegisterForm(props: RegisterFormProps) {
    const [username, setUsername] = React.useState("");
    const [email, setEmail] = React.useState("");
    const [password, setPassword] = React.useState("");
    return (
        <form
            className="login-form"
            onSubmit={(event) => {
                event.preventDefault();
                props.onSubmit(username, email, password);
            }}
        >
            <h1>Registrierung</h1>
            <div>
                <label>
                    Username: <Input value={username} onChange={setUsername} />
                </label>
                <label>
                    Email: <Input value={email} onChange={setEmail} />
                </label>
                <label>
                    Password: <Input type="password" value={password} onChange={setPassword} />
                </label>
            </div>
            <button type="submit" className="button">
                Registrieren
            </button>
        </form>
    );
}
