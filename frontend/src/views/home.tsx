import Input, { InputProps } from "../components/input";
import LOCATION_SVG from "../assets/location.svg";
import React from "react";
import "../style/home.css";
import { Api } from "../api";
import { handleApiError } from "../utils/helper";
import { SearchResult } from "../api/generated";

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
    const [results, setResults] = React.useState<null | Array<SearchResult>>(null);

    const { onChange } = inputProps;
    inputProps.onChange = (newValue: string) => {
        onChange(newValue);
        Api.search(newValue).then(handleApiError(({ results }) => setResults(results)));
    };

    return (
        <div className="search-bar">
            <Input {...inputProps} />
            {results && (
                <div>
                    {results.map(({ name, quantity, description, image, mainCategory }) => (
                        <div>
                            {name} <small>{description}</small>
                        </div>
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
