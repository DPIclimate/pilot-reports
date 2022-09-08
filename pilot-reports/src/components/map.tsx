import React from 'react';
import mapboxgl from 'mapbox-gl';
import 'mapbox-gl/dist/mapbox-gl.css';
import './map.css';
import {Region, Regions} from "./navbar";

(mapboxgl as any).accessToken = 'pk.eyJ1IjoiaGFydmV5YmF0ZXMiLCJhIjoiY2w3c2k4N3kzMDdvejN4bnh4YnA4bWtuYSJ9.hSGj3OSZE89Ub1e7LkH48Q';

export class Map extends React.Component<any, any> {
    private readonly mapContainer: React.RefObject<HTMLDivElement>;
    constructor(props?: any) {
        super(props);
        this.state = {
            count: 0,
            regions: {},
            zoom: 5.5,
        }
        this.mapContainer = React.createRef();
    }

    componentDidMount(){
        fetch("http://localhost:8080/oyster_regions/list", {
            headers: {"Accept": "*/*", "User-Agent": "NSW DPI"},
            method: "GET"
        })
            .then(res => res.json())
            .then(res =>
                this.setState({
                    count: res.count,
                    regions: res
                })
            )
            .catch(e => this.setState({
                count: 0,
                regions: {}
            }));
    }

    render() {
        if(this.state.count == 0){
            return (
                <div ref={this.mapContainer} className="map-container">Map not available</div>
            )
        } else {
            const regionCoords = this.state.regions.results.map((region: Region, i: number) => {
                return (
                    {
                        "type": "Feature",
                        "properties": {
                            "title": region.program_info.name
                        },
                        "geometry": {
                            "type": "Point",
                            "coordinates": [region.program_info.longitude,
                                region.program_info.latitude]
                        },
                        "id": i
                    }
                )
            });

            //// Build map
            const map =  new mapboxgl.Map({
                container: this.mapContainer.current!,
                style: 'mapbox://styles/mapbox/outdoors-v11',
                center: [151.701323, -32.845871],
                zoom: this.state.zoom
            });

            // Add region coordinates
            map.on('load', () => {
                map.addSource('region-location', {
                    type: 'geojson',
                    data: {
                        "type": "FeatureCollection",
                        "features": regionCoords
                    }
                });
                map.addLayer({
                    "id": "region-locations",
                    "type": "circle",
                    "source": "region-location",
                    'paint': {
                        'circle-color': '#000'
                    }
                });
            });

            const infoPopup = new mapboxgl.Popup({
                closeButton: false,
                closeOnClick: false
            });

            map.on('mouseenter', 'region-locations', (e) => {
                map.getCanvas().style.cursor = 'pointer';
                if(e.features![0].geometry.type === "Point"){
                    const coordinates = e.features![0].geometry!.coordinates.slice()
                    const location = e.features![0].properties!.title;
                    while (Math.abs(e.lngLat.lng - coordinates[0]) > 180) {
                        coordinates[0] += e.lngLat.lng > coordinates[0] ? 360 : -360;
                    }
                    // @ts-ignore
                    infoPopup.setLngLat(coordinates).setHTML(location).addTo(map);
                }
            });

            map.on('mouseleave', 'region-locations', () => {
                map.getCanvas().style.cursor = '';
                infoPopup.remove();
            });
        }
        return (
            <div ref={this.mapContainer} className="map-container"></div>
        )
    }

}