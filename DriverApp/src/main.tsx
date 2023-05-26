import React from "react";
import ReactDOM from "react-dom/client";

import { ToastContainer } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import "./index.css";
import {MapContainer, Marker, Polyline, Popup, TileLayer} from "react-leaflet";
import "leaflet/dist/leaflet.css";

async function getRoute(start: [number, number], end: [number, number]): Promise<Array<[number, number]>> {
    let result = await fetch(`https://bayern.ors.birby.de/ors/v2/directions/driving-car?start=${start[1]},${start[0]}&end=${end[1]},${end[0]}`);
    let content = await result.json();
    if (content.hasOwnProperty("error")) {
        console.error("Routing failed");
        return [];
    }
    return content.features[0].geometry.coordinates.map((i) => [i[1], i[0]]);
}

async function getWaypoints(): Promise<Array<[number, number]>> {
    // TODO: Make call to the backend to get the waypoints
    return [
        [49.01284, 12.07622],
        [49.00581, 12.08128],
        [49.00828, 12.07421],
        [49.01025, 12.11706]
    ];
}

type Order = {
    id: number,
    name: string,
    pickup: boolean,
    time: number,
    estimated: boolean,
    address: string,
    waypoint: [number, number],
}

async function getOrders(): Promise<Array<Order>> {
    // TODO: Make call to the backend to get the waypoints
    return [
        {
            id: 13861,
            name: "dm",
            pickup: true,
            time: 1685113972,
            estimated: false,
            address: "Regensburg Arcaden, Friedenstraße 23, 93053 Regensburg",
            waypoint: [49.01055, 12.09767]
        },
        {
            id: 13890,
            name: "ALDI Süd",
            pickup: true,
            time: 1685114132,
            estimated: false,
            address: "ALDI Süd, Balwinusstraße 2, 93051 Regensburg",
            waypoint: [49.00977, 12.08368]
        },
        {
            id: 13937,
            name: "ALDI Süd",
            pickup: true,
            time: 1685114159,
            estimated: false,
            address: "ALDI Süd, Balwinusstraße 2, 93051 Regensburg",
            waypoint: [49.00977, 12.08368]
        },
        {
            id: 14068,
            name: "Andrea Maier",
            pickup: false,
            time: 1685114581,
            estimated: true,
            address: "Sarmanna-Straße 21, 93049 Regensburg",
            waypoint: [49.01265, 12.0854]
        },
        {
            id: 14006,
            name: "Blumen Böswirth",
            pickup: true,
            time: 1685114951,
            estimated: true,
            address: "Kumpfmühler Straße 43, 93051 Regensburg",
            waypoint: [49.00805, 12.08743]
        },
        {
            id: 14068,
            name: "Hubert Zentgraf-Schwarz",
            pickup: false,
            time: 1685115381,
            estimated: true,
            address: "Universitätsstraße 76, 93053 Regensburg",
            waypoint: [49.00304, 12.09291]
        }
    ];
}

type MainState = {
    startPosition: [number, number],
    target: [number, number],
    path: Array<[number, number]>,
    markers: Array<[number, number]>,
    orders: Array<Order>,
};

class Main extends React.Component<{}, MainState> {
    constructor(props: {}) {
        super(props);
        this.state = {
            startPosition: [49.00225, 12.10006],
            target: [48.64154, 10.42841],
            path: [],
            markers: [],
            orders: [],
        };
        getOrders().then((orders) => {
            console.log(`Got ${orders.length} orders: ${orders}`);
            this.setState({orders: orders});
            this.setState({markers: orders.map(o => o.waypoint)});
            let previousWaypoint = this.state.startPosition;
            const segmentPromises = [];
            for (const waypoint of orders.map(o => o.waypoint)) {
                segmentPromises.push(getRoute(previousWaypoint, waypoint));
                previousWaypoint = waypoint;
            }
            Promise.all(segmentPromises).then((segmentList: Awaited<Array<Array<[number, number]>>>) => {
                console.log("segment list:", segmentList);
                this.setState({path: segmentList.flat(1)});
            });
        });
    }

    render() {
        return <div id={"main"}>
            <MapContainer center={this.state.startPosition} zoom={13} scrollWheelZoom={true} className={"upper"}>
                <TileLayer
                    attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                    url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                />
                <div id={"startMarker"}>
                    <Marker title={"Start"} position={this.state.startPosition}>
                        <Popup>You are here.</Popup>
                    </Marker>
                </div>
                <div>
                    <Polyline color={'red'} positions={this.state.path} />
                </div>
                <div id={"map-marker"}>
                    {
                        this.state.orders.map(o => o.waypoint).map((position) =>
                            <Marker position={position}>
                            </Marker>)
                    }
                </div>
            </MapContainer>
            <div className={"lower"}>
                {
                    this.state.orders.map((o) =>
                        <div className={"item"}>
                            <div className={"item-name"}>
                                {o.name}
                            </div>
                            <div className={"item-count"}>
                                #{o.id}
                            </div>
                            <div className={"item-time"}>
                                {o.estimated ? "~": ""}{new Intl.DateTimeFormat(undefined, {hour: "numeric", minute: "numeric"}).format(new Date(o.time * 1000))}
                            </div>
                            <div className={"item-address"}>
                                {o.address}
                            </div>
                        </div>)
                }
            </div>
        </div>;
    }
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <>
        <Main />
        <ToastContainer
            autoClose={3500}
            theme="dark"
            toastClassName="toast-pane"
            progressClassName="toast-neon toast-progress"
        />
    </>
);
