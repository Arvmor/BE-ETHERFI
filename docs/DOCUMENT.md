## Intro
All received responses have the below format:
```rust
{
  status: i16,
  message: Value,
}
```
where **status** is either `0` (Success), or `-1` (Failed)
<br>and the **message** contains the desired data
<br>

## API DOCs

### Auctions
#### POST `/auctions` - Create an Auction
```bash
curl --request POST \
  --url http://localhost:1337/auctions \
  --header 'Content-Type: application/json' \
  --data '{
  "name": "Auction Name Test",
  "starting_price": 100,
  "end_date": 17333333333
}'
```
Response: <br>
```json
{
  "status": 0,
  "message": {
    "id": "52dc9e6c-b0c8-4d50-bc3b-867da4290376",
    "name": "Auction Name Test",
    "bids": [],
    "starting_price": 100,
    "winner": null,
    "end_date": 17333333333
  }
}
```
#### DELETE `/auctions/{uuid}` - Delete an Auction
```bash
curl --request DELETE \
  --url http://localhost:1337/auctions/59384b6e-92bb-45a1-a04d-52c160ac0913 \
  --header 'Content-Type: application/json'
```
Response: <br>
```json
{
  "status": 0,
  "message": {
    "id": "59384b6e-92bb-45a1-a04d-52c160ac0913",
    "name": "TEst",
    "bids": [],
    "starting_price": 100,
    "winner": null,
    "end_date": 12333333333
  }
}
```
#### GET `/auctions` - Get all Auctions
```bash
curl --request GET \
  --url http://localhost:1337/auctions \
  --header 'Content-Type: application/json'
```
Response: <br>
```json
{
  "status": 0,
  "message": {
    "count": 0,
    "auctions": []
  }
}
```

#### GET `/auctions/{uuid}` - Get Specific Auction
```bash
curl --request GET \
  --url http://localhost:1337/auctions/718589a8-28c4-4db2-9c76-0d688040debc \
  --header 'Content-Type: application/json'
```
Response: <br>
```json
{
  "status": 0,
  "message": {
    "id": "718589a8-28c4-4db2-9c76-0d688040debc",
    "name": "TEst",
    "bids": [],
    "starting_price": 100,
    "winner": null,
    "end_date": 12333333333
  }
}
```

### Bids
#### POST `/bids/{uuid}` - Create a Bid for an Auction
```bash
curl --request POST \
  --url http://localhost:1337/bids/718589a8-28c4-4db2-9c76-0d688040debc \
  --header 'Content-Type: application/json' \
  --data '{
  "name": "User X",
  "amount": 100
}'
```
Response: <br>
```json
{
  "status": 0,
  "message": {
    "id": "718589a8-28c4-4db2-9c76-0d688040debc",
    "name": "TEst",
    "bids": [
      {
        "id": "158f47bb-5ac8-4f84-8bfd-50c9d73f2b55",
        "name": "User X",
        "amount": 1000,
        "timestamp": 1717772850
      }
    ],
    "starting_price": 100,
    "winner": {
      "id": "158f47bb-5ac8-4f84-8bfd-50c9d73f2b55",
      "name": "User X",
      "amount": 1000,
      "timestamp": 1717772850
    },
    "end_date": 12333333333
  }
}
```
