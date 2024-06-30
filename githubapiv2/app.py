from typing import override
from githubapiv2_guest.project.types import Ok
from githubapiv2_guest.project import exports
from githubapiv2_guest.project.imports import outgoing_http
import json
import traceback


class Releases(exports.Releases):
    @override
    def fetch_latest(self, org: str, proj: str) -> str:
        url = f"https://api.github.com/repos/{org}/{proj}/releases/latest"
        try:
            http_res = outgoing_http.get_request("GET", [
                outgoing_http.RequestHeader(
                    key="Content-Type", value="application/json",
                ),
            ], url)
            if isinstance(http_res, Ok):
                print(f"Status_code: {http_res.value.status_code}")
                data = json.loads(http_res.value.body)
                return data["tag_name"]
            else:
                return http_res.value.msg
        except Exception as e:
            print(f"Caught Exception: {e}")
            traceback.print_exc()
            return "Failed to get the response"
