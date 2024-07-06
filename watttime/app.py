from typing import override
from green import exports
from green.imports import outgoing_http
import json
import traceback
import os
from dotenv import load_dotenv
import base64
from termcolor import colored, cprint


class Watttime(exports.Watttime):
    @override
    def get_region(self, token: str, signal_type: str) -> str | None:
        try:
            http_res = outgoing_http.get_request(
                method="GET",
                headers=[
                    outgoing_http.RequestHeader(
                        key="Content-Type", value="application/json",
                    )
                ],
                url="https://ipinfo.io",
                body=None)

            if http_res.status_code != 200:
                raise Exception(f"StatusCode: {http_res.status_code}, Reason: {str(http_res.body)}")

            data = json.loads(http_res.body)
            loc = data['loc']
            lat_lon = loc.split(",")
            if signal_type == "":
                signal_type = "co2_moer"
            url = f"https://api.watttime.org/v3/region-from-loc?latitude={lat_lon[0]}&longitude={lat_lon[1]}&signal_type={signal_type}"
            http_res = outgoing_http.get_request(
                method="GET",
                headers=[
                    outgoing_http.RequestHeader(
                        key="Authorization", value=f"Bearer {token}",
                    )
                ],
                url=url,
                body=None)

            if http_res.status_code != 200:
                raise Exception(f"StatusCode: {http_res.status_code}, Reason: {str(http_res.body)}")

            data = json.loads(http_res.body)
            return data['region']

        except Exception as e:
            text = colored(f"Caught Exception: {e}", "red", attrs=["reverse", "blink"])
            print(f"{text}")
            traceback.print_exc()
            return None

    @override
    def get_forecast(self, token: str, region: str, signal_type) -> exports.watttime.Forecast | None:
        try:
            if signal_type == "":
                signal_type = "co2_moer"
            url = f"https://api.watttime.org/v3/forecast?region={region}&horizon_hours=1&signal_type={signal_type}"
            http_res = outgoing_http.get_request(
                method="GET",
                headers=[
                    outgoing_http.RequestHeader(
                        key="Authorization", value=f"Bearer {token}",
                    )
                ],
                url=url,
                body=None)

            if http_res.status_code == 403:
                data = json.loads(http_res.body)
                print(colored(f"Trying to recover and use default region 'CAISO_NORTH' {data['error']}=>{data['message']}", "light_yellow"))
                url = f"https://api.watttime.org/v3/forecast?region=CAISO_NORTH&horizon_hours=1&signal_type={signal_type}"
                http_res = outgoing_http.get_request(
                    method="GET",
                    headers=[
                        outgoing_http.RequestHeader(
                            key="Authorization", value=f"Bearer {token}",
                        )
                    ],
                    url=url,
                    body=None)
                if http_res.status_code != 200:
                    raise Exception(f"StatusCode: {http_res.status_code}, Reason: {str(http_res.body)}")

            elif http_res.status_code != 200:
                raise Exception(f"StatusCode: {http_res.status_code}, Reason: {str(http_res.body)}")

            data = json.loads(http_res.body)
            data_pointers = [exports.watttime.PointData(
                point_time=item['point_time'],
                value=item['value']) for item in data['data']]

            meta_pointers = data['meta']
            meta_warnings = [f"{item['type']}:{item['message']}" for item in meta_pointers['warnings']]

            obj = exports.watttime.Forecast(
                data=data_pointers,
                meta=exports.watttime.MetadataForecast(
                    data_point_period_seconds=meta_pointers['data_point_period_seconds'],
                    region=meta_pointers['region'],
                    signal_type=meta_pointers['signal_type'],
                    model=meta_pointers['model']['date'],
                    warnings=meta_warnings,
                    units=meta_pointers['units'],
                    generated_at=meta_pointers['generated_at'],
                    generated_at_period_seconds=meta_pointers['generated_at_period_seconds'],
                )
            )
            return obj

        except Exception as e:
            text = colored(f"Caught Exception: {e}", "red", attrs=["reverse", "blink"])
            print(f"{text}")
            traceback.print_exc()
            return None

    def get_current_co2_moer_index(self, token: str, region: str, signal_type: str) -> exports.watttime.Co2MoerIndex | None:
        try:
            if signal_type == "":
                signal_type = "co2_moer"
            url = f"https://api.watttime.org/v3/signal-index?region={region}&signal_type={signal_type}"
            http_res = outgoing_http.get_request(
                method="GET",
                headers=[
                    outgoing_http.RequestHeader(
                        key="Authorization", value=f"Bearer {token}",
                    )
                ],
                url=url,
                body=None)

            if http_res.status_code == 403:
                data = json.loads(http_res.body)
                print(colored(f"Trying to recover and use default region 'CAISO_NORTH' {data['error']}=>{data['message']}", "light_yellow"))
                url = f"https://api.watttime.org/v3/signal-index?region=CAISO_NORTH&signal_type={signal_type}"
                http_res = outgoing_http.get_request(
                    method="GET",
                    headers=[
                        outgoing_http.RequestHeader(
                            key="Authorization", value=f"Bearer {token}",
                        )
                    ],
                    url=url,
                    body=None)
                if http_res.status_code != 200:
                    raise Exception(f"StatusCode: {http_res.status_code}, Reason: {str(http_res.body)}")

            elif http_res.status_code != 200:
                raise Exception(f"StatusCode: {http_res.status_code}, Reason: {str(http_res.body)}")

            data = json.loads(http_res.body)
            data_pointers = [exports.watttime.PointData(
                point_time=item['point_time'],
                value=item['value']) for item in data['data']]

            meta_pointers = data['meta']
            meta_warnings = [f"{item['type']}:{item['message']}" for item in meta_pointers['warnings']]

            obj = exports.watttime.Co2MoerIndex(
                data=data_pointers[0],
                meta=exports.watttime.MetadataCo2MoerIndex(
                    data_point_period_seconds=meta_pointers['data_point_period_seconds'],
                    region=meta_pointers['region'],
                    signal_type=meta_pointers['signal_type'],
                    model=f"{meta_pointers['model']}",
                    warnings=meta_warnings,
                    units=meta_pointers['units'],
                )
            )
            return obj

        except Exception as e:
            text = colored(f"Caught Exception: {e}", "red", attrs=["reverse", "blink"])
            print(f"{text}")
            traceback.print_exc()
            return None

    @override
    def register(self, username: str, password: str, email: str) -> bool:

        try:
            body = {
                "username": username,
                "password": password,
                "email": email,
            }
            r_body = json.dumps(body)

            http_res = outgoing_http.get_request(
                method="POST",
                headers=[
                    outgoing_http.RequestHeader(
                        key="Content-Type", value="application/json",
                    )
                ],
                url="https://api.watttime.org/register",
                body=r_body.encode('utf-8'))

            if http_res.status_code != 200:
                raise Exception(f"StatusCode: {http_res.status_code}, Reason: {str(http_res.body)}")

            data = json.loads(http_res.body)
            print(f"Registration successful {data}")
            return True
        except Exception as e:
            text = colored(f"Caught Exception: {e}", "red", attrs=["reverse", "blink"])
            print(f"{text}")
            traceback.print_exc()
            return False

    @override
    def get_token(self) -> str | None:
        load_dotenv()
        try:

            token_tt = os.getenv("WATTTIME_TOKEN")
            if token_tt is not None:
                print(f"{colored("Using Token present in .env", "green")}")
                return token_tt

            token_usr = os.getenv("WATTTIME_USERNAME")
            if token_usr is None:
                raise Exception("env:$WATTTIME_USERNAME is not set")
            token_pass = os.getenv("WATTTIME_PASSWORD")
            if token_pass is None:
                raise Exception("env:$WATTTIME_PASSWORD is not set")

            encoded_data = base64.b64encode(f"{token_usr}:{token_pass}".encode('utf-8')).decode('utf-8')

            http_res = outgoing_http.get_request(
                method="GET",
                headers=[
                    outgoing_http.RequestHeader(
                        key="Authorization", value=f"Basic {encoded_data}",
                    )
                ],
                url="https://api.watttime.org/login",
                body=None)

            if http_res.status_code != 200:
                raise Exception(f"StatusCode: {http_res.status_code}, Reason: {str(http_res.body)}")

            data = json.loads(http_res.body)
            resp = data['token']
            return resp

        except Exception as e:
            text = colored(f"Caught Exception: {e}", "red", attrs=["reverse", "blink"])
            print(f"{text}")
            traceback.print_exc()
