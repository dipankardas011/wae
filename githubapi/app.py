from typing import override
from project import exports
from project.imports import outgoing_http
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
            print(f"Status_code: {http_res.status_code}")
            data = json.loads(http_res.body)
            return data["tag_name"]
        except Exception as e:
            print(f"Caught Exception: {e}")
            traceback.print_exc()
            return "Failed to get the response"
