import Input from "../components/input";
import LOCATION_SVG from "../assets/location.svg";
import React from "react";

type HomeProps = {};
export default function Home(props: HomeProps) {
    const [search, setSearch] = React.useState("");
    const [postalCode, setPostalCode] = React.useState("");
    return (
        <div>
            <Input placeholder="Was möchtest du bestellen?" value={search} onChange={setSearch} />
            <PostalInput value={postalCode} onChange={setPostalCode} />
        </div>
    );
}

type PostalInputProps = {
    value: string;
    onChange(newValue: string): void;
};
function PostalInput(props: PostalInputProps) {
    return (
        <div>
            <Input value={props.value} onChange={props.onChange} placeholder="Postleitzahl" />
            <button
                type="button"
                onClick={() => {
                    window.navigator.geolocation.getCurrentPosition(() => props.onChange("93053"));
                }}
            >
                <img className="icon" src={LOCATION_SVG} alt="Get location" />
            </button>
        </div>
    );
}
