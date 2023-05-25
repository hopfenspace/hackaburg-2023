import { ROUTES } from "../routes";
import { USER_CONTEXT } from "../utils/user";
import React from "react";
import "../style/header.css";
import USER_SVG from "../assets/user.svg";
import CART_SVG from "../assets/cart.svg";

type HeaderProps = {};
export default function Header(props: HeaderProps) {
    const { user } = React.useContext(USER_CONTEXT);
    return (
        <div className={"header"}>
            <div {...ROUTES.HOME.clickHandler({})}>Hackaburg 2023</div>
            {user === null ? (
                <div {...ROUTES.LOGIN.clickHandler({})}>
                    <img className="icon" src={USER_SVG} alt="Login" />
                </div>
            ) : (
                <div className={"flex-row"}>
                    <img className="icon" src={USER_SVG} alt="Login" /> {user.username}
                </div>
            )}
            <div {...ROUTES.CART.clickHandler({})}>
                <img className="icon" src={CART_SVG} alt="Cart" />
            </div>
            <div>
                <LangSelect />
            </div>
        </div>
    );
}

function LangSelect() {
    return (
        <div className="lang-select">
            <div />
            <div />
            <div />
        </div>
    );
}
