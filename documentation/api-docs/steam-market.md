## GET /api/steam/market/search

Returns Steam Market search results with standardized API response.

### Request Example

```http
GET /api/steam/market/search?appid=730&query=karambit&sort=default_desc&page=1
```

#### Query Parameters
- **appid** (string, optional): Steam app/game ID
- **query** (string, optional): Search term
- **sort** (string, optional): Sort order (default: `default_desc`)
- **page** (integer, optional): Page number (default: 1)

### Response Example

```json
{
  "apiversion": "v0.0.1cb",
  "code": 200,
  "success": true,
  "message": "OK",
  "size": 10,
  "data": [
    {
      "app_icon": "https://...",
      "app_name": "Counter-Strike 2",
      "item_details": {
        "appid": 730,
        "background_color": "",
        "classid": "313225800",
        "commodity": 0,
        "icon_url": "https://community.fastly.steamstatic.com/economy/image/i0CoZ81Ui0m-9KwlBY1L_18myuGuq1wfhWSaZgMttyVfPaERSR0Wqmu7LAocGIGz3UqlXOLrxM-vMGmW8VNxu5Dx60noTyL6kJ_m-B1Q7uCvZaZkNM-AHliUwP5mvORWQiy3nAgq_WnWytagIH2QOgJxWZYmE-dZskPpltbiM-nrtgLYg4oWyS352nwdvHt1o7FVEyKITis",
        "instanceid": "188530139",
        "market_hash_name": "★ Karambit | Scorched (Field-Tested)",
        "market_name": "★ Karambit | Scorched (Field-Tested)",
        "name": "★ Karambit | Scorched (Field-Tested)",
        "name_color": "8650AC",
        "tradable": 1,
        "type": "★ Covert Knife"
      },
      "hash_name": "★ Karambit | Scorched (Field-Tested)",
      "name": "★ Karambit | Scorched (Field-Tested)",
      "sale_price_text": "$1,498.39",
      "sell_listings": 2,
      "sell_price": 156649,
      "sell_price_text": "$1,566.49"
    }
    // ...
  ],
  "timestamp": "2025-07-24T09:56:19.014810900+00:00",
  "error": null
}
```

### Returned Fields

- **apiversion**: API version string
- **code**: HTTP-like status code
- **success**: Indicates if request was successful
- **message**: Status message
- **size**: Number of items returned (page size)
- **data**: Array of Steam Market results
- **timestamp**: Response timestamp (RFC3339)
- **error**: Error message (if any)

#### Each item in `data` contains:
- **app_icon**: Game icon URL
- **app_name**: Game name
- **item_details**: Steam asset details
- **hash_name**: Steam Market hash name
- **name**: Item name
- **sale_price_text**: Sale price (formatted)
- **sell_listings**: Number of listings
- **sell_price**: Price in Steam currency units
- **sell_price_text**: Price (formatted)

### Example Error Response

```json
{
  "apiversion": "v0.0.1cb",
  "code": 400,
  "success": false,
  "message": "Invalid request or unsuccessful response",
  "size": null,
  "data": null,
  "timestamp": "2025-07-24T09:56:19.014810900+00:00",
  "error": "success != true"
}
```