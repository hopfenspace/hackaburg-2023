import Home from "./views/home";
import { Router } from "./utils/router";
import Login from "./views/login";
import Cart from "./views/cart";

export const ROUTER = new Router();

export const ROUTES = {
    HOME: ROUTER.add({ url: "", parser: {}, render: () => <Home /> }),
    LOGIN: ROUTER.add({ url: "login", parser: {}, render: () => <Login /> }),
    CART: ROUTER.add({ url: "cart", parser: {}, render: () => <Cart /> }),
};

ROUTER.finish();
