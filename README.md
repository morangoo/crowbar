<p align="center">
  <img src="./public/crowbar.png" width="220px">
</p>

<h1 align="center">Crowbar</h1>

<p align="center">
  A community-maintained API for accessing and integrating with Steam.<br>
  Provides real-time data on games, users, inventories, and more.
</p>

## Notes

- To get Steam Market data, you need to **fetch** the page that returns a fully **rendered HTML document**.
There is no direct JSON response; therefore, you must **parse the HTML** to extract item information. This involves identifying the HTML elements containing the desired data and extracting their content.