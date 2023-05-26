import Input from "../components/input";
import LOCATION_SVG from "../assets/location.svg";
import React from "react";
import "../style/home.css";
import { Api } from "../api";
import { handleApiError } from "../utils/helper";
import { ImageState, SearchResult } from "../api/generated";
import INTRO_SVG from "../assets/intro-shopping-bag.svg";
import INTRO_HEART_SVG from "../assets/intro-heart.svg";
import NOT_FOUND_SVG from "../assets/products-not-found.svg";

type HomeProps = {};
export default function Home(props: HomeProps) {
    const searchBar = React.useRef<HTMLInputElement>();
    const [postalCode, setPostalCode] = React.useState("");
    return (
        <div className="home">
            <h1 className={"heading"}>Kauf überall, liefer einmal</h1>
            <SearchBar ref={searchBar} />
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
    let lastTimer = 0;

    return (
        <>
            <Input
                ref={ref}
                placeholder="Was möchtest du bestellen?"
                value={search}
                onChange={(newValue) => {
                    setSearch(newValue);
                    setResults(null);
                    clearTimeout(lastTimer);
                    lastTimer = setTimeout(() => {
                        Api.search(newValue).then(handleApiError(({ results }) => setResults(results)));
                    }, 300);
                }}
                className={"search-bar"}
            />

            {
                search?.length > 0
                    ? (results && results.length > 0)
                        ? (
                            <div className={"search-results"}>
                                {results.map(({ uuid, name, quantity, description, image, mainCategory }) => (
                                    <div>
                                        {image
                                            ? <img src={image} alt="" />
                                            : <LazyLoadImage uuid={uuid} />
                                        }
                                        <div className={"amount"}>
                                            {quantity}
                                        </div>
                                        <div className={"name"}>
                                            {name}
                                        </div>
                                    </div>
                                ))}
                            </div>
                        ) : results ? (
                            <div className={"teasers"}>
                                <div>
                                    <img src={NOT_FOUND_SVG} alt="Keine Produkte gefunden" />
                                    <p>
                                        Keine Produkte gefunden
                                    </p>
                                </div>
                            </div>
                        ) : (
                            <div className="loading">
                                <div className="bars">
                                    <div className="bar"></div>
                                    <div className="bar"></div>
                                    <div className="bar"></div>
                                </div>
                            </div>
                        )
                    : (
                        <div className={"teasers"}>
                            <div>
                                <img src={INTRO_SVG} alt="Collegit Lieferung" />
                                <p>
                                    Zeit sparen, Umgebung unterstützen, Umwelt schonen.
                                </p>
                            </div>
                            <div>
                                <img src={INTRO_HEART_SVG} alt="Collegit Lieferung" />
                                <p>
                                    Mit Collegit können lokale Betriebe und Unternehmen
                                    unterstützt werden und gleichzeitig nachhaltiger
                                    eingekauft werden.
                                </p>
                            </div>
                        </div>
                    )
            }
        </>
    );
});

type PostalInputProps = {
    value: string;
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

interface LazyLoadImageProps { uuid?: string; }
interface LazyLoadImageState { imgSrc?: string; }

class LazyLoadImage extends React.Component<LazyLoadImageProps, LazyLoadImageState> {
    constructor(props: LazyLoadImageProps) {
        super(props);
        this.state = {
            imgSrc: undefined
        };
    }

    componentDidMount() {
        if (this.props.uuid)
            Api.image(this.props.uuid).then(r =>
                r.is_ok() && r.unwrap().imageState == ImageState.Found
                    ? this.setState({
                        imgSrc: r.unwrap().image!
                    }) : undefined);
    }

    render() {
        return (
            <img src={this.state.imgSrc} />
        );
    }
}
