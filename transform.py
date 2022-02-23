import pandas as pd
import json
import os

class Transform:
    def __init__(self):
        self.output_df = pd.DataFrame()
        self.properties = {}
        self.files = self.list_datasets()
        self.run()
        
    def run(self):
        for file in self.files:
            self.fortnightly_summary(file)
        self.output_df.index.names = ["Location"]
        self.export_df_to_csv()
    
    def list_datasets(self, path = "data/"):
        files = []
        for file in os.listdir(path):
            if file.endswith(".csv"):
                files.append(file)
        return files
    
    def export_df_to_csv(self, path="data/transformed/"):
        self.output_df.to_csv(f"{path}transformed.csv")

    def fortnightly_summary(self, filename, path="data/"):
        df = pd.read_csv(f"{path}{filename}")

        # Exclude datasets where there is no data
        if df.empty:
            return

        # Localise datetime to Sydney
        df["ts"] = pd.to_datetime(df["timestamp"], unit="ms").dt.tz_localize('utc').dt.tz_convert("Australia/Sydney")
        df = df.sort_values(by="ts", ignore_index=True)

        # Calculate mean salinity values for each week
        salinity_mean = df["salinity"].groupby(df["ts"].map(lambda t: t.isocalendar().week)).mean().values
        last_week_salinity_mean = salinity_mean[0].round(2)
        this_week_salinity_mean = salinity_mean[1].round(2)
        salinity_mean_diff = (this_week_salinity_mean - last_week_salinity_mean).round(2)
        salinity_mean_dif_percent = None
        if last_week_salinity_mean != 0 and this_week_salinity_mean != 0:
            salinity_mean_dif_percent = (((salinity_mean[1] / salinity_mean[0]) * 100) - 100).round(2)

        # Calculate stdev salinity values for each week
        salinity_stdev = df["salinity"].groupby(df["ts"].map(lambda t: t.isocalendar().week)).std().values
        last_week_salinity_stdev = salinity_stdev[0].round(2)
        this_week_salinity_stdev = salinity_stdev[1].round(2)

        # Calculate mean temperature values for each week
        temperature_mean = df["temperature"].groupby(df["ts"].map(lambda t: t.isocalendar().week)).mean().values
        last_week_temperature_mean = temperature_mean[0].round(2)
        this_week_temperature_mean = temperature_mean[1].round(2)
        temperature_mean_diff = (this_week_temperature_mean - last_week_temperature_mean).round(2)
        temperature_mean_dif_percent = None
        if last_week_temperature_mean != 0 and this_week_temperature_mean != 0:
            temperature_mean_dif_percent = (((temperature_mean[1] / temperature_mean[0]) * 100) - 100).round(2)

        # Calculate stdev temperature values for each week
        temperature_stdev = df["temperature"].groupby(df["ts"].map(lambda t: t.isocalendar().week)).std().values
        last_week_temperature_stdev = temperature_stdev[0].round(2)
        this_week_temperature_stdev = temperature_stdev[1].round(2)

        # Get location from devices.json
        devices = json.load(open("devices.json"))
        location = [d["location"] for d in devices["buoys"] if d["name"] == filename[:-4]]
        buoy_number = [d["buoy_number"] for d in devices["buoys"] if d["name"] == filename[:-4]]

        # Add the logic for column colours (salinity)
        salinity_trend = f"{this_week_salinity_mean}"
        salinity_trend_color = "GREY"
        error = "No errors"
        error_color = "GREEN"
        if(salinity_mean_dif_percent != None):
            if last_week_salinity_mean > this_week_salinity_mean:
                salinity_trend = f"{salinity_trend} ^Decreasing {salinity_mean_dif_percent}%^"
                salinity_trend_color = "RED"
            else:
                salinity_trend = f"{salinity_trend} ^Increasing {salinity_mean_dif_percent}%^"
                salinity_trend_color = "GREEN"

            if last_week_salinity_mean == 0 or this_week_salinity_mean == 0 or last_week_salinity_mean > 45 or this_week_salinity_mean > 45:
                error = "Possible Error"
                error_color = "RED"
                salinity_trend_color = "GREY"
        else:
            error = "Error"
            error_color = "RED"
            salinity_trend = f"{salinity_trend} ^Salinity Read Error^"
        
        salinity_properties = {
            "Fortnightly salinity trend (ppt)": salinity_trend,
            "salinity_trend_color": salinity_trend_color
        }

        # Add the logic for column colours (temperature)
        temperature_trend = f"{this_week_temperature_mean}"
        temperature_trend_color = "GREY"
        temperature_error = "No errors"
        temperature_error_color = "GREEN"
        if(temperature_mean_dif_percent != None):
            if last_week_temperature_mean > this_week_temperature_mean:
                temperature_trend = f"{temperature_trend} ^Decreasing {temperature_mean_dif_percent}%^"
                temperature_trend_color = "GREEN"
            else:
                temperature_trend = f"{temperature_trend} ^Increasing {temperature_mean_dif_percent}%^"
                temperature_trend_color = "RED"
        else:
            error = "Error"
            error_color = "RED"
            temperature_trend = f"{temperature_trend} ^Temperature Read Error^"

        if last_week_temperature_mean == 0 or this_week_temperature_mean == 0 or last_week_temperature_mean > 35 or this_week_temperature_mean > 35:
            error = "Error"
            error_color = "RED"
            temperature_trend_color = "GREY"
            
        temperature_properties = {
            "Fortnightly temperature trend (\N{DEGREE SIGN}C)": temperature_trend,
            "temperature_trend_color": temperature_trend_color
        }
        
        device_error_properties = {
            "Device Error": error,
            "error_color": error_color
        }
            
        (sal_day, temp_day) = self.daily_summary(df)
        
         # Create a properties object that will be converted into a dataframe later
        self.properties = {
            location[0]: {
            }
        }
    
        # Update properties with values containing an average from each variable over the past week
        self.properties[location[0]].update(salinity_properties)
        self.properties[location[0]].update(sal_day)
        self.properties[location[0]].update(temperature_properties)
        self.properties[location[0]].update(temp_day)
        self.properties[location[0]].update(device_error_properties)

        temp_df = pd.DataFrame.from_dict(self.properties, orient="index")
        self.output_df = self.output_df.append(temp_df)

    def daily_summary(self, df):
        # Parse by day of week salinity
        salinity_mean_day = df["salinity"].groupby(df["ts"].map(lambda t: t.day)).mean().values
        sal_day = {}
        day_number = 1
        for value in salinity_mean_day[7:]:
            sal_day[f"sday{day_number}"] = value.round(2)
            day_number += 1

        # Same for temperature 
        temperature_mean_day = df["temperature"].groupby(df["ts"].map(lambda t: t.day)).mean().values
        temp_day = {}
        day_number = 1
        for value in temperature_mean_day[7:]:
            temp_day[f"tday{day_number}"] = value.round(2)
            day_number += 1
            
        return (sal_day, temp_day)
    
if __name__ == "__main__":
    Transform()
