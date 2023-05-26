import Input from "../components/input";
import LOCATION_SVG from "../assets/location.svg";
import React from "react";
import "../style/home.css";
import { Api } from "../api";
import { handleApiError } from "../utils/helper";
import { SearchResult } from "../api/generated";

type HomeProps = {};
export default function Home(props: HomeProps) {
    const searchBar = React.useRef<HTMLInputElement>();
    const [postalCode, setPostalCode] = React.useState("");
    return (
        <div className="home">
            <h1>Kauf überall, liefer einmal</h1>
            <SearchBar ref={searchBar} />
            <PostalInput value={postalCode} onChange={setPostalCode} />
            <p>"CyberProject" verbindet alle deine lokalen Läden mit einem Zentralen Lieferdienst.</p>
            <p>
                Somit können Bestellungen mehrerer Kunden zusammengefasst werden, um Emissionen durch transport zu
                reduzieren und den Stadtverkehr zu entlasten
            </p>
            <p>
                <big onClick={() => searchBar.current?.focus()}>Probier es doch mal</big>
            </p>
        </div>
    );
}

type SearchBarProps = {};
const SearchBar = React.forwardRef(function SearchBar(
    props: SearchBarProps,
    ref: React.ForwardedRef<HTMLInputElement>
) {
    const [search, setSearch] = React.useState("");
    const [results, setResults] = React.useState<null | Array<SearchResult>>(null);

    return (
        <div className="search-bar">
            <Input
                ref={ref}
                placeholder="Was möchtest du bestellen?"
                value={search}
                onChange={(newValue) => {
                    setSearch(newValue);
                    Api.search(newValue).then(handleApiError(({ results }) => setResults(results)));
                }}
            />
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
});

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
