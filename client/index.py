from solders.pubkey import Pubkey # type: ignore
from solana.rpc.api import Client
import json
import base64


def get_token_amounts_example():
    endpoint = 'https://api.mainnet-beta.solana.com'
    solana_client = Client(endpoint)
    pool_address = 'PQ1sBpkZYTkXo2aDRqWU2sEJGUj7yaeJ3GePoCRYU4V' 
    pool = Pubkey.from_string(pool_address)
    info = json.loads(solana_client.get_account_info_json_parsed(pool).to_json())
    print(info)
    data = info['result']['value']['data']
    data_64 = base64.b64decode(data[0])
    print(data_64)
    

get_token_amounts_example()