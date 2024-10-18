import json
import urllib.request
from pprint import pprint


def main():
    url = 'https://crates.io/api/v1/crates?page=1&per_page=100&sort=recent-downloads'
    with urllib.request.urlopen(url) as response:
        body = response.read().decode("utf8")
    pprint([
        (crate['id'], crate['recent_downloads'], crate['repository'])
        for crate in json.loads(body)['crates']])


if __name__ == "__main__":
    main()
