/*
 * Create a table of latest readings.
 */
async function createValuesTable() {

	var dataset = {
		buoys: [],
		temperature: {
			values: [],
			trends: [],
			colors: []
		},
		salinity: {
			values: [],
			trends: [],
			colors: []
		},
		timestamp: {
			values: [],
			colors: []
		}
	};

	const temperature_data = {
		ids: [
			"620c34a236478d1df15fcd03",
			"6181b4e2852f0907a66ca28b",
			"61788ec5dc917002aa2562e2",
			"61788eb9a73c3401fa330fbf",
			"6174a4cddc9170000d0c6be1",
			"61732bf7852f09000e22a105",
			"617318d1852f09000e22a102",
			"6171f4d686f43b04efae2d48",
			"616e4a88810cbd039c60af03",
			"616e493d810cbd03d916fa69",
			"616e476e41ac9d03d99b67ed"
		]
	};

	var ts_body = { 
		variables: temperature_data.ids,
		columns: ["device.name", "value.value", "timestamp"],
		join_dataframes: false,
		start: Math.floor(Date.now() - 3600000)
	};

	var options = {
		method: "POST",
		headers: {"x-auth-token": "BBAU-VOMMw42nHGcLPKVfBMQxYXDUiL78ln",
			"Content-Type": "application/json"
		},
		body: JSON.stringify(ts_body),
	};
	
	var response = await fetch("https://industrial.api.ubidots.com.au/api/v1.6/data/raw/series", 
			options).then(res => res.json());

	response.results.map(function(r, i) {
		if(r.length != 0) {

			var n = r.length;
			var sum = 0;
			var device_name = "Error";
			var init_ts = false;
			var ts = Math.floor(Date.now() - 3600000);

			var temp_values = [];
			r.map(function(v, i) {
				var trend = null;
				device_name = v[0];
				sum += v[1];
				temp_values.push(v[1]);
				if (init_ts == false) {
					ts = v[2];
					dataset.timestamp.colors.push("green");
					init_ts = true;
				}
			});

			var trend = " &#8212;";
			var diff = 0;
			if (temp_values.length > 1) {
				diff = temp_values[0] - temp_values[temp_values.length - 1];
			}
			if (diff > 0) {
				trend = " &darr;";
			} else if (diff < 0){
				trend = " &uarr;";
			}

			dataset.temperature.trends.push(trend);

			var average = (sum / n);
			if(average >= 28) {
				dataset.temperature.colors.push("red");
			} else if (average > 24 && average < 28) {
				dataset.temperature.colors.push("orange");
			} else {
				dataset.temperature.colors.push("#0645ad");
			}

			const ts_options = {
				hour: "numeric",
				minute: "numeric",
				day: "numeric",
				month: "short"
			};

			if(init_ts == false) {
				dataset.timestamp.colors.push("red");
			}

			ts = new Date(ts);
			ts = ts.toLocaleDateString("en-US", ts_options);
			dataset.temperature.values.push(average.toFixed(1));
			dataset.timestamp.values.push(ts);
			dataset.buoys.push(device_name);
		}
	})


	const salinity_data = {
		ids: [
			"620c34a036478d1df15fcd01",
			"6181b4e0a73c34080beda0d7",
			"61788ec4a73c3402b041fb61",
			"61788eb8a73c34027320e15e",
			"6174a4cc852f09011427403b",
			"61732bf6852f09000d3d1bff",
			"617318d0dc9170000d0c6bbc",
			"6171f4d5810cbd25005c19e4",
			"616e4a8741ac9d039cfa39f8",
			"616e493c810cbd03d916fa68",
			"616e476d41ac9d03d99b67ec"
		]
	}

	ts_body = { 
		variables: salinity_data.ids,
		columns: ["device.name", "value.value", "timestamp"],
		join_dataframes: false,
		start: Math.floor(Date.now() - 3600000)
	};


	options = {
		method: "POST",
		headers: {"x-auth-token": "BBAU-VOMMw42nHGcLPKVfBMQxYXDUiL78ln",
			"Content-Type": "application/json"
		},
		body: JSON.stringify(ts_body),
	};

	response = await fetch("https://industrial.api.ubidots.com.au/api/v1.6/data/raw/series", 
			options).then(res => res.json());

	response.results.map(function(r, i) {
		if(r.length != 0) {
			var n = r.length;
			var sum = 0;
			var temp_values = [];
			r.map(function(v, i) {
				sum += v[1];
				temp_values.push(v[1]);
			});

			var trend = " &#8212;";
			var diff = 0;
			if (temp_values.length > 1) {
				diff = temp_values[0] - temp_values[temp_values.length - 1];
			}
			if (diff > 0) {
				trend = " &darr;";
			} else if (diff < 0){
				trend = " &uarr;";
			}

			dataset.salinity.trends.push(trend);

			var average = (sum / n);
			if(average >= 22 && average < 50) {
				dataset.salinity.colors.push("green");
			} else if (average > 16 && average < 22) {
				dataset.salinity.colors.push("orange")
			} else {
				dataset.salinity.colors.push("red");
			}

			dataset.salinity.values.push(average.toFixed(1));
		}
	})
	
	var table_head = document.getElementById("data-head");
	var head = table_head.insertRow(0);
	head.insertCell(0).innerHTML = "Buoy";
	head.insertCell(1).innerHTML = "Salinity";
	head.insertCell(2).innerHTML = "Temperature";
	head.insertCell(3).innerHTML = "Date";

	dataset.buoys.map(function(b, i) {
		const table = document.getElementById("data-body");
		var row = table.insertRow(0);
		var buoy = row.insertCell(0);
		var salinity = row.insertCell(1);
		var temperature = row.insertCell(2);
		var date = row.insertCell(3);
		buoy.innerHTML = b;
		salinity.innerHTML = dataset.salinity.values[i] + dataset.salinity.trends[i];
		salinity.style.color = dataset.salinity.colors[i];
		temperature.innerHTML = dataset.temperature.values[i] + dataset.temperature.trends[i];
		temperature.style.color = dataset.temperature.colors[i];
		date.innerHTML = dataset.timestamp.values[i];
		date.style.color = dataset.timestamp.colors[i];
	});

	let table = new DataTable("#data-table", {
		"order": [[0, 'asc']],
		"lengthChange": false,
		"info": false,
		"searching": false,
		"paging": false
	});
}
