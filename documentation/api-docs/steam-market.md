## GET /api/steam/market/search

Returns Steam Market search results with standardized API response.

### Request example

```http
GET /api/steam/market/search?appid=730&query=karambit&sort=default_desc&page=1
```

#### Query Parameters
- **appid** (string, optional): Steam app/game ID
- **query** (string, optional): Search term
- **sort** (string, optional): Sort order (default: `default_desc`)
- **page** (integer, optional): Page number (default: 1)

### Response example

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
      "asset_description": { ... },
      "hash_name": "★ Karambit | Scorched (Field-Tested)",
      "name": "★ Karambit | Scorched (Field-Tested)",
      "sale_price_text": "$1,498.39",
      "sell_listings": 2,
      "sell_price": 156649,
      "sell_price_text": "$1,566.49"
    },
    // ...
  ],
  "timestamp": "2025-07-24T09:56:19.014810900+00:00",
  "error": null
}
```

### Returned fields
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
- **asset_description**: Steam asset details (object)
- **hash_name**: Steam Market hash name
- **name**: Item name
- **sale_price_text**: Sale price (formatted)
- **sell_listings**: Number of listings
- **sell_price**: Price in Steam currency units
- **sell_price_text**: Price (formatted)

#### Example error response
```json
{
  "apiversion": "v0.0.1cb",
  "code": 400,
  "success": false,
  "message": "Invalid request or unsuccessful response",
  "size": null,
  "data": null,
  "timestamp": "2025-07-24T09:56:19.014810900+00:00",
  "error": 099
}
```