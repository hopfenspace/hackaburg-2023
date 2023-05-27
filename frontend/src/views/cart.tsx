import { useEffect, useReducer, useState } from "react";
import "../style/cart.css";

export function renderEuro(amount: number)
{
    return amount.toFixed(2).replace(".", ",") + " €";
}

type CartProps = {};
export default function Cart(props: CartProps) {
    // https://legacy.reactjs.org/docs/hooks-faq.html#is-there-something-like-forceupdate
    const [, forceUpdate] = useReducer(x => x + 1, 0);

    function renderItem(img: string, name: string, quantity: string, price: number, amount: number,
            setAmount: (amount: number) => any) {
        return (<div className="item">
            <img src={img}/>
            <div className="name">
                {name}
            </div>
            <div className="quantity">
                {quantity}
            </div>
            <div className="price">
                {renderEuro(price)}
            </div>
            <div className="amount">
                Anzahl: <input type="number" step={1} min={1} max={999} value={amount} onChange={e => setAmount(parseInt(e.target.value))}/>
            </div>
            <div className="total">
                Position: {renderEuro(2.99 * amount)}
            </div>
        </div>);
    }
    const [items, setItems] = useState<{ img: string, name: string, quantity: string, price: number, amount: number }[]>(
        JSON.parse(localStorage.getItem("shoppingcart") || "[]"));
    useEffect(() => {
        localStorage.setItem("shoppingcart", JSON.stringify(items));
      }, [items]);
    function amountCallback(index: number) {
        return (value: number) => {
            console.log("Set value to ", value);
            if (items[index].amount != value)
            {
                items[index].amount = value;
                forceUpdate();
            }
        };
    }
    return (
        <div className="cart">
            {items.map((item, index) => renderItem(item.img, item.name, item.quantity, item.price, item.amount, amountCallback(index)))}
            <div className="submit">
                <div className="total">
                    Summe: {renderEuro(items.map(item => item.price * item.amount).reduce((a, b) => a + b, 0))}
                </div>
                <button>Bestellen</button>
            </div>
        </div>
    );
}
