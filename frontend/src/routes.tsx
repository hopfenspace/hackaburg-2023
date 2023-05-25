import Home from "./views/home";
import { Router } from "./utils/router";
import Login from "./views/login";

export const ROUTER = new Router();

export const ROUTES = {
    HOME: ROUTER.add({ url: "", parser: {}, render: () => <Home /> }),
    LOGIN: ROUTER.add({ url: "login", parser: {}, render: () => <Login /> }),
};

ROUTER.finish();
