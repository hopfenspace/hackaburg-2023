import Input, { InputProps } from "../components/input";
import LOCATION_SVG from "../assets/location.svg";
import React from "react";
import "../style/home.css";

type HomeProps = {};
export default function Home(props: HomeProps) {
    const [search, setSearch] = React.useState("");
    const [postalCode, setPostalCode] = React.useState("");
    return (
        <div className="home">
            <SearchBar
                placeholder="Was möchtest du bestellen?"
                value={search}
                onChange={setSearch}
                suggestions={["foo", "bar", "baz"]}
            />
            <PostalInput value={postalCode} onChange={setPostalCode} />
        </div>
    );
}

type SearchBarProps = InputProps & {
    suggestions?: Array<string>;
};
function SearchBar(props: SearchBarProps) {
    const { suggestions, ...inputProps } = props;
    return (
        <div className="search-bar">
            <Input {...inputProps} />
            {suggestions && suggestions.length > 0 && (
                <div>
                    {suggestions.map((suggestion) => (
                        <div>{suggestion}</div>
                    ))}
                </div>
            )}
        </div>
    );
}

type PostalInputProps = {
    value: string;
    onChange(newValue: string): void;
};
function PostalInput(props: PostalInputProps) {
    return (
        <div className="postal-input">
            <Input value={props.value} onChange={props.onChange} placeholder="Postleitzahl" />
            <button
                type="button"
                className="button"
                onClick={() => {
                    window.navigator.geolocation.getCurrentPosition(() => props.onChange("93053"));
                }}
            >
                <img className="icon" src={LOCATION_SVG} alt="Get location" />
            </button>
        </div>
    );
}
