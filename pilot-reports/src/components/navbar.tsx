import React from 'react';
import './navbar.css';
import {BrowserRouter, Link, Route, Routes} from 'react-router-dom';
import Home from '../views/home';

export interface Regions {
	count: number,
	results: Region[]
}

export interface Region {
	last_updated: string,
	program_info: ProgramInfo
}

export interface ProgramInfo {
	name: string,
	latitude: number,
	longitude: number
}

export class Navbar extends React.Component<any, any> {
	constructor(props?: any) {
		super(props);
		this.state = {
			count: 0,
			regions: {},
			regionsVisible: false
		}
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
					regions: res.results
				})
			)
			.catch(e => this.setState({
				count: 0,
				regions: {}
			}));
	}

	loadRegions(regions: Regions) {
		return regions.results.map((region: Region) => {
			return (
				<Link className="navitem" to={region.program_info.name}>
					<span>{region.program_info.name}</span>
				</Link>
			)
		});
	}

	loadRegionRoutes(regions: Regions){
		return regions.results.map((region: Region) => {
			return (
				<Route key={region.program_info.name}
					   path={region.program_info.name} />
			)
		});
	}

	render() {
		if(this.state.count == 0){
			return (
				<div>Loading...</div>
			)
		} else {
			const regions: Regions = {count: this.state.count,
				results: this.state.regions};
			return (
				<BrowserRouter>
					<nav className="navbar">
						<ul className="navlist">
							<Link className="navButton" to="home">
								<span>Home</span>
							</Link>
							<button className="navButton"
									onClick={(e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
										this.state.regionsVisible ?
											this.setState({ regionsVisible: false}) :
											this.setState({regionsVisible: true})}
									}>Regions</button>
							{this.state.regionsVisible ? this.loadRegions(regions) : null}
						</ul>
					</nav>
					<Routes>
						<Route path="/home" element={<Home/>} />
						{this.loadRegionRoutes(regions)}
					</Routes>
				</BrowserRouter>
			);
		}
	}
}