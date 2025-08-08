

# Steam Store API

## Endpoints

### `/api/steam/store/game/<appid>`


Returns detailed information about a specific Steam game, using its AppID.

#### Query Parameters
| Parameter | Type   | Required | Description                       |
|-----------|--------|----------|-----------------------------------|
| language  | string | No       | Language (e.g., portuguese)       |
| cc        | string | No       | Country code (e.g., BR)           |

**Request example:**
```
GET /api/steam/store/game/730
```

**Response example:**
```json
{
  "apiversion": "v0.0.1cb",
  "code": 200,
  "success": true,
  "message": "OK",
  "size": 1,
  "data": { /* ...detailed game fields... */ },
  "timestamp": "...",
  "error": null
}
```

---

### `/api/steam/store/games`

Returns a list of Steam Store games, with scraped and enriched data.


#### Query Parameters
| Parameter | Type    | Required | Description                                   |
|-----------|---------|----------|-----------------------------------------------|
| query     | string  | No       | Search term (e.g., Final Fantasy)             |
| page      | integer | No       | Page number (default: 1)                      |
| count     | integer | No       | Number of results (minimum 25)                |
| cc        | string  | No       | Country code (e.g., BR)                       |
| language  | string  | No       | Language (e.g., portuguese)                   |

**Request example:**
```
GET /api/steam/store/games?query=Final+Fantasy&cc=BR&language=portuguese&page=1
```

**Response example:**
```json
{
  "apiversion": "v0.0.1cb",
  "code": 200,
  "success": true,
  "message": "OK",
  "size": 25,
  "data": [
    {
      "appid": 1173770,
      "title": "FINAL FANTASY",
      "url": "https://store.steampowered.com/app/1173770/FINAL_FANTASY/?snr=1_7_7_151_150_1",
      "img": "https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/1173770/capsule_sm_120.jpg?t=1746071255",
      "img_large": "https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/1173770/header.jpg",
      "price_final": "R$45,00",
      "price_original": null,
      "discount_pct": "0%",
      "discounted": false,
      "discount": 0,
      "bundle_discount": "0",
      "released": "12 Aug. 2021",
      "review": "Muito positivas\u003Cbr\u003E87% das 175,576 análises de utilizadores sobre este jogo são positivas.",
      "platforms": ["windows"],
      "tags": "[4434,122,3964,4325,10695,3871,3834]",
      "descids": null,
      "crtrids": "[1012195,34459938]",
      "itemkey": "App_1173770",
      "steamdeck": "true"
    },
    // ...more games...
  ],
  "timestamp": "...",
  "error": null
}
```

---


## Main fields

- `appid`: Steam AppID
- `title`: game name
- `url`: link to the game page
- `img`: small thumbnail
- `img_large`: large header image
- `price_final`: final price
- `price_original`: original price (if any)
- `discount_pct`: discount as string (e.g. "-30%")
- `discounted`: boolean, whether the game is discounted
- `discount`: numeric discount value (e.g. 30)
- `bundle_discount`: bundle discount
- `released`: release date
- `review`: review summary
  - **Note:** The review field is returned exactly as provided by Steam, in the language specified by the `language` parameter.
- `platforms`: supported platforms
- `tags`, `descids`, `crtrids`, `itemkey`, `steamdeck`: extra attributes
