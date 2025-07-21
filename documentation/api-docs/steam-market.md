## GET /api/steam/market

This endpoint returns the featured items from the Steam Market, including image, name, game, price, currency, available quantity, appid, and direct link to the item.

### Request example

```http
GET /api/steam/market
```

### Response example

```json
[
  {
    "image": "https://...",
    "name": "Clutch Case",
    "game": "Counter-Strike 2",
    "price": 1.51,
    "currency": "1",
    "qty": 19687,
    "appid": "730",
    "item_link": "https://steamcommunity.com/market/listings/730/Clutch%20Case"
  },
  // ...
]
```

### Returned fields

- **image**: Item image URL
- **name**: Item name
- **game**: Game name
- **price**: Item price (float)
- **currency**: Steam currency code
- **qty**: Available quantity
- **appid**: Steam game ID
- **item_link**: Direct link to the item on Steam Market