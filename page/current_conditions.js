/*
 * Gathers weather station data from an AWS linked to Ubidots.
 * Used chart.js as the backend for creating a chart.
 * Sets the inner HTML of some variables to display the most
 * recent values.
 */
async function getTempAndHumid() {

	var dataset = {
		temperature: {
			values: [],
			colors: []
		},
		humidity: {
			values: [],
			colors: []
		},
		wind: {
			speed: [],
			direction: [],
			colors: []
		},
	};
	
	// Temperature 

	const air_temperature_var = "61788e45852f090346add2bd";

	var body = {
	  "variables": [air_temperature_var],
	  "aggregation": "mean",
	  "period": "1H",
	  "join_dataframes": false,
	  "start": new Date() - 604800000 // 7 days
	};

	var options = {
		method: "POST",
		headers: {
			"x-auth-token": "BBAU-5C7fdQtm2qlveEOSDc0gCk85e7a5Sa",
			"Content-Type": "application/json"
		},
		body: JSON.stringify(body)
	};

	var response = await fetch("https://industrial.api.ubidots.com.au/api/v1.6/data/stats/resample/", options)
		.then(res => res.json());

	const ts_options = {
		day: "numeric",
		month: "short"
	};

	response.results.map(function(r, i) {
		if(r.length != 0) {
			r.slice(0).reverse().map(function(v, x) {
				var value = v[1];
				if (value < 50 && value >= -10) {
					var ts = v[0];
					dataset.temperature.values.push({x: ts, y: value.toFixed(1)});
				}
			})
		}
	});

	// Humidity

	const humidity_var = "61788e47dc917002aa2562e0";

	body = {
	  "variables": [humidity_var],
	  "aggregation": "mean",
	  "period": "1H",
	  "join_dataframes": false,
	  "start": new Date() - 604800000 // 7 days
	};

	options = {
		method: "POST",
		headers: {
			"x-auth-token": "BBAU-5C7fdQtm2qlveEOSDc0gCk85e7a5Sa",
			"Content-Type": "application/json"
		},
		body: JSON.stringify(body)
	};

	response = await fetch("https://industrial.api.ubidots.com.au/api/v1.6/data/stats/resample/", options)
		.then(res => res.json());

	response.results.map(function(r, i) {
		if(r.length != 0) {
			r.slice(0).reverse().map(function(v, x) {
				var value = v[1];
				if (value < 100.1 && value >= 0) {
					var ts = v[0];
					dataset.humidity.values.push({x: ts, y: value.toFixed(1)});
				}
			})
		}
	});
	
	// Wind speed

	const wind_speed_var = "61788e49dc917002e7774656";

	body = {
	  "variables": [wind_speed_var],
	  "aggregation": "mean",
	  "period": "1H",
	  "join_dataframes": false,
	  "start": new Date() - 604800000 // 7 days
	};

	options = {
		method: "POST",
		headers: {
			"x-auth-token": "BBAU-5C7fdQtm2qlveEOSDc0gCk85e7a5Sa",
			"Content-Type": "application/json"
		},
		body: JSON.stringify(body)
	};

	response = await fetch("https://industrial.api.ubidots.com.au/api/v1.6/data/stats/resample/", options)
		.then(res => res.json());

	response.results.map(function(r, i) {
		if(r.length != 0) {
			r.slice(0).reverse().map(function(v, x) {
				var value = v[1];
				if (value < 60 && value >= 0) {
					var ts = v[0];
					var wind_knts = value * 1.9438445;
					dataset.wind.speed.push({x: ts, y: wind_knts.toFixed(1)});
				}
			})
		}
	});

	const wind_dir = [
		{ direction: 0, value: "N" },
		{ direction: 22.5, value: "NNE" },
		{ direction: 45, value: "NE" },
		{ direction: 67.5, value: "ENE" },
		{ direction: 90, value: "E" },
		{ direction: 112.5, value: "ESE" },
		{ direction: 135, value: "SE" },
		{ direction: 157.5, value: "SSE" },
		{ direction: 180, value: "S" },
		{ direction: 202.5, value: "SSW" },
		{ direction: 225, value: "SW" },
		{ direction: 247.5, value: "WSW" },
		{ direction: 270, value: "W" },
		{ direction: 292.5, value: "WNW" },
		{ direction: 315, value: "NW" },
		{ direction: 337.5, value: "NNW" },
	];

	const wind_dir_var = "61788e48852f0902cbf8756f";

	body = {
	  "variables": [wind_dir_var],
	  "aggregation": "mean",
	  "period": "1H",
	  "join_dataframes": false,
	  "start": new Date() - 604800000 // 7 days
	};

	options = {
		method: "POST",
		headers: {
			"x-auth-token": "BBAU-5C7fdQtm2qlveEOSDc0gCk85e7a5Sa",
			"Content-Type": "application/json"
		},
		body: JSON.stringify(body)
	};

	response = await fetch("https://industrial.api.ubidots.com.au/api/v1.6/data/stats/resample/", options)
		.then(res => res.json());

	response.results.map(function(r, i) {
		if(r.length != 0) {
			r.slice(0).reverse().map(function(v, x) {
				var min_diff = 0;
				var direction = "N";
				var init_diff = true;
				wind_dir.map(function(d, _) {
					var diff = Math.abs(d.direction - v[1]);
					if(init_diff) {
						min_diff = diff;
						direction = d.value;
						init_diff = false;
					}
					if(diff < min_diff) {
						min_diff = diff;
						direction = d.value;
					}
				});
				dataset.wind.direction.push({x: v[0], y: direction});
			})
		}
	});


	const data = {
		datasets: [
			{
				label: 'Air Temperature',
				backgroundColor: 'rgb(255, 99, 132)',
				borderColor: 'rgb(255, 99, 132)',
				showLine: true,
				pointRadius: 0,
				tension: 0.4,
				data: dataset.temperature.values,
				yAxisID: 'y'
			},
			{
				label: 'Humidity',
				backgroundColor: "#1BA098",
				borderColor: "#1BA098",
				showLine: true,
				pointRadius: 0,
				tension: 0.4,
				data: dataset.humidity.values,
				yAxisID: 'y1'
			},
			{
				label: 'Wind Speed',
				backgroundColor: "#1D1D2C",
				borderColor: "#1D1D2C",
				showLine: true,
				pointRadius: 0,
				tension: 0.4,
				data: dataset.wind.speed,
				yAxisID: 'y'
			},
			{
				label: 'Wind Direction',
				hidden: true,
				backgroundColor: "#1D1D2C",
				borderColor: "#1D1D2C",
				showLine: true,
				pointRadius: 0,
				tension: 0.4,
				data: dataset.wind.direction,
				yAxisID: 'y'
			}
		]
	};

	// Check if its night time
	function isNight(ts) {
		return ts.getHours() < 7 || ts.getHours() > 18;
	}

	// Identifiy and extract points that represent night time 
	// Used to create night boxes (grey shadows) below
	var night_intervals = [];
	var night = false;
	dataset.temperature.values.map(function(v, i) {
		var ts = new Date(v.x);
		if(isNight(ts)) {
			if (night == false) {
				night_intervals.push(i);
				night = true;
			}
		} else {
			if (night == true) {
				night_intervals.push(i-1);
			}
			night = false;
		}
	});

	// Generate boxes that represent night time.
	const night_boxes = [];
	for (var i = 0; i < night_intervals.length - 1; i+=2){
		// Accounts for data currently being night time
		var box = {
			type: 'box',
			xMin: dataset.temperature.values[night_intervals[i]].x,
			xMax: dataset.temperature.values[night_intervals[i+1]].x,
			drawTime: "beforeDraw",
			yMin: 0,
			yMax: 50,
			backgroundColor: 'rgba(221, 221, 221, 0.4)',
			borderWidth: 0
		}
		night_boxes.push(box);
	}

	// Extra box needed if its currently night time
	if(night_intervals.length % 2 != 0) {
		var box = {
			type: 'box',
			xMin: dataset.temperature.values[night_intervals[night_intervals.length - 1]].x,
			xMax: dataset.temperature.values[dataset.temperature.values.length - 1].x,
			drawTime: "beforeDraw",
			yMin: 0,
			yMax: 50,
			backgroundColor: 'rgba(221, 221, 221, 0.4)',
			borderWidth: 0
		}
		night_boxes.push(box);
	}

	const config = {
		type: 'scatter',
		data: data,
		options: {
			responseive: true,
			interaction: {
				intersect: false,
				axis: "x",
				mode: "index"
			},
			showLine: true,
			scales: {
				x: {
					min: dataset.temperature.values[0].x,
					max: dataset.temperature.values[dataset.temperature.values.length - 1].x,
					ticks: {
						callback: function(v, i) {
							var ts = new Date(v);
							ts = ts.toLocaleDateString("en-US", ts_options);
							return ts;
						},
						major: {
							enabled: true,
						},
						maxRotation: 0,
						minRotation: 0,
						maxTicksLimit: 7,
						minTicksLimit: 7,
						font: {
							size: 14
						}
					},
				},
				y: {
					title: {
						display: false,
						text: "Temperature (C)",
						color: "rgb(255, 99, 132)",
						font: {
							size: 14
						}
					},
					position: "right",
					ticks: {
						color: "rgb(255, 99, 132)",
						font: {
							size: 14
						}
					}, 
					min: 0, 
					max: 50
				},
				y1: {
					title: {
						display: false,
						text: "Humidity (%)",
						color: "#1BA098",
						font: {
							size: 14
						}
					},
					position: "left",
					ticks: {
						color: "#1BA098",
						font: {
							size: 14
						}
					},
					min: 0,
					max: 100
				}
			},
			plugins: {
				legend: {
					labels: {
						filter: function(v, _) {
							return !v.text.includes("Wind Direction");
						},
						font: {
							size: 14
						}
					}
				},
				tooltip: {
					enabled: false
				},
				zoom: {
					pan: {
						enabled: true,
						mode: "x",
					},
					zoom: {
						pinch: {
							enabled: true
						},
						wheel: {
							enabled: false 
						},
						mode: "x"
					},
					limits: {
						x: {
							min: new Date(dataset.temperature.values[0].x).valueOf(),
							max: new Date(dataset.temperature.values[dataset.temperature.values.length - 1].x).valueOf()
						}
					},
				},
				annotation: {
					annotations: night_boxes
				},
			}
		},
		plugins: [{
			afterDraw: function(chart) {
				if (chart.tooltip?._active?.length) {
					let x = chart.tooltip._active[0].element.x;
					let idx = chart.tooltip._active[0].index;

					const date_opts = {
						hour: "numeric",
						day: "numeric",
						month: "short"
					}
					
					var ts = new Date(dataset.temperature.values[idx].x);
					ts = ts.toLocaleDateString("en-US", date_opts);
					document.getElementById("date-value").innerHTML = ts 
					document.getElementById("temperature-value").innerHTML = dataset.temperature
						.values[idx].y + " &deg;C";
					document.getElementById("humidity-value").innerHTML = dataset.humidity
						.values[idx].y + " %";
					document.getElementById("wind-value").innerHTML = dataset.wind.speed[idx].y + 
						" kn " + dataset.wind.direction[idx].y;

					chart.ctx.save();
					chart.ctx.beginPath();
					chart.ctx.moveTo(x, chart.scales.y.top);
					chart.ctx.lineTo(x, chart.scales.y.bottom);
					chart.ctx.lineWidth = 2;
					chart.ctx.strokeStyle = '#000000';
					chart.ctx.stroke();
					chart.ctx.restore();
				}
				else {
					const idx = dataset.temperature.values.length - 1;
					var ts = new Date(dataset.temperature.values[idx].x);
					ts = ts.toLocaleDateString("en-US", date_opts);
					document.getElementById("date-value").innerHTML = ts 
					document.getElementById("temperature-value").innerHTML = dataset.temperature
						.values[idx].y + " &deg;C";
					document.getElementById("humidity-value").innerHTML = dataset.humidity
						.values[idx].y + " %";
					document.getElementById("wind-value").innerHTML = dataset.wind.speed[idx].y + 
						" kn " + dataset.wind.direction[idx].y;
				}
			},

		}]
	};

	const date_opts = {
		hour: "numeric",
		day: "numeric",
		month: "short"
	}

	const idx = dataset.temperature.values.length - 1;
	var ts = new Date(dataset.temperature.values[idx].x);
	ts = ts.toLocaleDateString("en-US", date_opts);
	document.getElementById("date-value").innerHTML = ts 
	document.getElementById("temperature-value").innerHTML = dataset.temperature
		.values[idx].y + " &deg;C";
	document.getElementById("humidity-value").innerHTML = dataset.humidity
		.values[idx].y + " %";
	document.getElementById("wind-value").innerHTML = dataset.wind.speed[idx].y + 
		" kn " + dataset.wind.direction[idx].y;
	document.getElementById("table-info").innerHTML = "&darr; decreasing, &#8212; stable, &uarr; increasing (based on data from the past hour)";

	return config;
}

