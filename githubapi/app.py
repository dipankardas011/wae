from typing import override
from githubapi_guest.project import exports
import urllib.request
import json


class Releases(exports.Releases):
    @override
    def fetch_latest(self, org: str, proj: str) -> str:
        print(f"Fetching latest release for {org}:{proj}@latest")
        url = f"https://api.github.com/repos/{org}/{proj}/releases/latest"
        print(f"Url to reach out for {url}")
        with urllib.request.urlopen(url) as response:
            data = response.read()
            release_info = json.loads(data)
            return str(release_info["tag_name"])
