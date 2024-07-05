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
    # TODO(dipankar): need to think of the dict type
    def get_region(self) -> str:
        # curl to ipinfo.io
        # { ...
        # "loc": "ABCD,EFGH",
        # ...}
        return "US"

    def get_forecast(self) -> dict:
        """
        https://docs.watttime.org/#tag/GET-Forecast/operation/get_current_forecast_v3_forecast_get
        """
        ...

    def get_range_of_historical_signal_data(self) -> dict:
        ...

    def get_current_CO2_MOER_index(self) -> dict:
        """
        https://docs.watttime.org/#tag/GET-Index/operation/get_signal_index_v3_signal_index_get
        """
        ...

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
