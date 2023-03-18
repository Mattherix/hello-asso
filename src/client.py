import requests
from datetime import datetime, timedelta
from os import getenv

URL = "api.helloasso.com/v5"
OAUTH2_URL = "https://api.helloasso.com/oauth2"
CLIENT_ID = getenv("CLIENT_ID")
CLIENT_SECRET = getenv("CLIENT_SECRET")


class Client:
    def __init__(self, client_id: str, client_secret: str):
        self.client_id = client_id
        self.client_secret = client_secret

        self.get_access_token()

    def __repr__(self) -> str:
        return "Client\n" \
            + f"client_id: {self.client_id}\n" \
            + f"client_secret: {self.client_secret}\n" \
            + f"access_token: {self.access_token}\n" \
            + f"refresh_token: {self.refresh_token}\n"

    def get_access_token(self):
        response = requests.post(
            OAUTH2_URL + "/token",
            headers={
                "Content-Type": "application/x-www-form-urlencoded"
            },
            data={
                "client_id": self.client_id,
                "client_secret": self.client_secret,
                "grant_type": "client_credentials"
            }
        )
        response = response.json()

        self.access_token = response["access_token"]
        self.refresh_token = response["refresh_token"]
        self.expire_date = datetime.now() + \
            timedelta(seconds=response["expires_in"])

    def get_new_token(self):
        response = requests.post(
            OAUTH2_URL + "/token",
            headers={
                "Content-Type": "application/x-www-form-urlencoded"
            },
            data={
                "client_id": self.client_id,
                "refresh_token": self.refresh_token,
                "grant_type": "refresh_token"
            }
        )
        response = response.json()
        
        self.access_token = response["access_token"]
        self.refresh_token = response["refresh_token"]
        self.expire_date = datetime.now() + \
            timedelta(seconds=response["expires_in"])


if __name__ == '__main__':
    client = Client(CLIENT_ID, CLIENT_SECRET)
    
    client.get_new_token()
    print(client)
