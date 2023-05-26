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

type MainState = {
    startPosition: [number, number],
    target: [number, number],
    path: Array<[number, number]>,
    markers: Array<[number, number]>,
};

class Main extends React.Component<{}, MainState> {
    constructor(props: {}) {
        super(props);
        this.state = {
            startPosition: [49.00225, 12.10006],
            target: [48.64154, 10.42841],
            path: [],
            markers: [],
        };
        getWaypoints().then((waypoints) => {
            console.log(`Got ${waypoints.length} waypoints: ${waypoints}`);
            this.setState({markers: waypoints});
            let previousWaypoint = this.state.startPosition;
            const segmentPromises = [];
            for (const waypoint of waypoints) {
                segmentPromises.push(getRoute(previousWaypoint, waypoint));
                previousWaypoint = waypoint;
            }
            Promise.all(segmentPromises).then((segmentList: Awaited<Array<Array<[number, number]>>>) => {
                console.log("segment list:", segmentList);
                this.setState({path: segmentList.flat(1)})
            });
        });
    }

    render() {
        return <div>
            <MapContainer center={this.state.startPosition} zoom={13} scrollWheelZoom={true} className={"fullscreen"}>
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
                        this.state.markers.map((position) =>
                            <Marker position={position}>
                            </Marker>)
                    }
                </div>
            </MapContainer>
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
