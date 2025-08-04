# Steam Market API

## Endpoints

### 1. Search Items on Steam Market

**GET** `/api/steam/market/search`

Searches for items on the Steam Community Market for a given game and search term.

#### Query Parameters
| Parameter | Type    | Required | Description                                                      |
|-----------|---------|----------|------------------------------------------------------------------|
| appid     | string  | Yes      | Game ID (e.g., 730 for CS2, 252490 for Rust)                     |
| query     | string  | No       | Search term (e.g., ak47)                                         |
| sort      | string  | No       | Sorting (e.g., default_desc, price_asc, price_desc, etc)         |
| page      | integer | No       | Results page (default: 1)                                        |

#### Example Request
```
GET /api/steam/market/search?appid=730&query=ak47
```

#### Example Response
```json
{
  "apiversion": "v0.0.1cb",
  "code": 200,
  "success": true,
  "message": "OK",
  "size": 10,
  "data": [
    {
      "app_icon": "https://cdn.fastly.steamstatic.com/steamcommunity/public/images/apps/730/8dbc71957312bbd3baea65848b545be9eae2a355.jpg",
      "app_name": "Counter-Strike 2",
      "hash_name": "Sticker | Hello AK-47",
      "item_details": {
        "appid": 730,
        "background_color": "",
        "classid": "4141791078",
        "commodity": 1,
        "icon_url": "https://community.fastly.steamstatic.com/economy/image/i0CoZ81Ui0m-9KwlBY1L_18myuGuq1wfhWSaZgMttyVfPaERSR0Wqmu7LAocGJai0ki7VeTHjNikNnSe6Rl0_9Oj1UviQhL4kti4_nsKvvf6avE0JqLDC2HIxLYk5rA5TC3mk01y62WBwt__Jy7CbFUlFNIuEiXV6npc",
        "instanceid": "519977179",
        "market_hash_name": "Sticker | Hello AK-47",
        "market_name": "Sticker | Hello AK-47",
        "market_url": "https://steamcommunity.com/market/listings/730/Sticker%20%7C%20Hello%20AK-47",
        "name": "Sticker | Hello AK-47",
        "name_color": "D2D2D2",
        "tradable": 1,
        "type": "High Grade Sticker"
      },
      "name": "Sticker | Hello AK-47",
      "sale_price_text": "$0.74",
      "sell_listings": 146,
      "sell_price": 77,
      "sell_price_text": "$0.77"
    }
    // ...more results
  ],
  "timestamp": "2025-08-04T14:20:21.216924300+00:00",
  "error": null
}
```

#### Notes
- The `market_url` field allows direct access to the item on the Steam Market.
- The `item_details` field contains all technical details of the item.

---

### 2. Get Details of a Specific Item

**POST** `/api/steam/market/item`

Retrieves full details of a specific Steam Market item, given the appid and hashname.

#### Body (JSON)
| Field     | Type   | Required | Description                              |
|-----------|--------|----------|------------------------------------------|
| appid     | string | Yes      | Game ID (e.g., 252490 for Rust)          |
| hashname  | string | Yes      | Item name/hash (e.g., Heat Seeker Mp5)   |

#### Example Request
```json
{
  "appid": "252490",
  "hashname": "Heat Seeker Mp5"
}
```

#### Example Response
```json
{
  "apiversion": "v0.0.1cb",
  "code": 200,
  "success": true,
  "message": "OK",
  "size": null,
  "data": {
    "actions": [
      {
        "link": "https://steamcommunity.com/sharedfiles/filedetails/?id=3483628701",
        "name": "View Workshop Item"
      }
    ],
    "amount": "0",
    "app_icon": "https://cdn.fastly.steamstatic.com/steamcommunity/public/images/apps/252490/820be4782639f9c4b64fa3ca7e6c26a95ae4fd1c.jpg",
    "appid": 252490,
    "background_color": "42413e",
    "classid": "7104051095",
    "commodity": 1,
    "contextid": "2",
    "currency": 0,
    "descriptions": [
      {
        "type": "html",
        "value": "<span style=\"color: #FF9800\">This is a skin for the <span style=\"color: #ffdba5\">MP5A4</span> item. You will be able to apply this skin at a repair bench or when you craft the item in game.</span><br><br><span style=\"color: #5098ce\">Breaks down into <span style=\"color: #ffffff\">1 x Metal</span></span>"
      }
    ],
    "icon_url": "6TMcQ7eX6E0EZl2byXi7vaVKyDk_zQLX05x6eLCFM9neAckxGDf7qU2e2gu64OnAeQ7835Vd7GLFfCk4nReh8DEiv5dYOaw7pLU_RPC9nJcdyp4",
    "icon_url_large": "6TMcQ7eX6E0EZl2byXi7vaVKyDk_zQLX05x6eLCFM9neAckxGDf7qU2e2gu64OnAeQ7835Vd7GLFfDY0jhyo8DEiv5dYOaw7pLU_RPC9nmOB87U",
    "id": "632304305006232521",
    "instanceid": "0",
    "market_hash_name": "Heat Seeker Mp5",
    "market_marketable_restriction": 7,
    "market_name": "Heat Seeker Mp5",
    "market_tradable_restriction": 7,
    "marketable": 1,
    "name": "Heat Seeker Mp5",
    "name_color": "f15840",
    "original_amount": "1",
    "owner": 0,
    "sealed": 0,
    "status": 4,
    "tradable": 1,
    "type": "Workshop Item",
    "unowned_contextid": "2",
    "unowned_id": "632310646322899217"
  },
  "timestamp": "2025-08-04T14:19:01.432888400+00:00",
  "error": null
}
```

#### Notes
- The `data` field contains all item details as provided by the Steam Market.
- The `market_url` field may be included for consistency if needed.
