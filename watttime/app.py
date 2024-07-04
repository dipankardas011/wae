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
                return False
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

            encoded_data = base64.b64encode(f"{token_usr}:{token_pass}".encode('utf-8'))
            print(encoded_data.decode('utf-8'))
            return "ABCDXYZ"

        except Exception as e:
            text = colored(f"Caught Exception: {e}", "red", attrs=["reverse", "blink"])

            print(f"{text}")
            traceback.print_exc()
