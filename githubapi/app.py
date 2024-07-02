from typing import override
from project import exports
from project.imports import outgoing_http
import json
import traceback


class Releases(exports.Releases):
    @override
    def get_latest_release(self, org: str, proj: str) -> str:
        url = f"https://api.github.com/repos/{org}/{proj}/releases/latest"
        try:
            http_res = outgoing_http.get_request("GET", [
                outgoing_http.RequestHeader(
                    key="Content-Type", value="application/json",
                ),
            ], url, None)
            if http_res.status_code != 200:
                return f"StatusCode: {http_res.status_code}, Reason: {http_res.body}"

            data = json.loads(http_res.body)
            return data["tag_name"]
        except Exception as e:
            print(f"Caught Exception: {e}")
            traceback.print_exc()
            return "Failed to get the response"

    @override
    def get_contributors(self, org: str, proj: str) -> str:
        url = f"https://api.github.com/repos/{org}/{proj}/contributors"
        try:
            http_res = outgoing_http.get_request("GET", [
                outgoing_http.RequestHeader(
                    key="Content-Type", value="application/json",
                ),
            ], url, None)
            if http_res.status_code != 200:
                return f"StatusCode: {http_res.status_code}, Reason: {http_res.body}"
            data = json.loads(http_res.body)
            githubId = [item['login'] for item in data]
            githubIdContributions = [item['contributions'] for item in data]

            ret = ""
            for i in range(0, len(githubId)):
                id = githubId[i]
                c = githubIdContributions[i]
                ret += f"github_id={id}, no_of_contributions={c}\n"

            return ret
        except Exception as e:
            print(f"Caught Exception: {e}")
            traceback.print_exc()
            return "Failed to get the response"

    @override
    def get_stars(self, org: str, proj: str) -> int:
        url = f"https://api.github.com/repos/{org}/{proj}/stargazers?per_page=10"
        try:
            http_res = outgoing_http.get_request("GET", [
                outgoing_http.RequestHeader(
                    key="X-GitHub-Api-Version", value="2022-11-28",
                ),
                outgoing_http.RequestHeader(
                    key="Accept", value="application/vnd.github+json",
                )
            ], url, None)
            if http_res.status_code != 200:
                print(f"StatusCode: {http_res.status_code}, Reason: {http_res.body}")
                return -999
            data = json.loads(http_res.body)
            githubId = [item['login'] for item in data]

            ret = ""
            for i in range(0, len(githubId)):
                id = githubId[i]
                ret += f"github_id={id}\n"

            print(f"Top 10 latest stars\n{ret}")

            return len(githubId)
        except Exception as e:
            print(f"Caught Exception: {e}")
            traceback.print_exc()
            print("Failed to get the response")
            return -999
