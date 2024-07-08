# Rest API for Stock and ETF Performance Figures

## Overview
This service provides two core functionalities:
- **prices**: Simulates a connection to a data provider's REST API.
- **performance_figures**: Calculates and returns performance figures for stocks and ETFs.

### Current Implementation:
- Provides daily returns, YTD returns, and price history.
- Sharpe ratio calculation is pending implementation.
- Date range filtering (from_date, to_date) is not yet supported.

## How to Run

### Data: 
Download the date from https://www.kaggle.com/datasets/jacksoncrow/stock-market-dataset and extract it into the directory `data`, following the folder structure in place. In the current files there is some sample data GOOG for stocks and the DAX for etfs. 

### Requirements:
- Cargo

### Steps:
1. Navigate to the project directory.
2. Run `cargo run`.
3. Access the API via `http://localhost:8080`.

### Example:
#### Bash
```bash
curl http://localhost:8080/price_history/GOOG
```

## API Endpoints
```plaintext
GET:
/daily_returns/{id}
Retrieves daily return data for the given asset.

GET:
/ytd_return/{id}
Calculates and retrieves YTD return for the given asset.
Optional Query Parameter: as_of_date (YYYY-MM-DD)

GET:
/price_history/{id}
Fetches the price history for the given asset.
```

Sample Response
```json
{
  "data": {
    "id": "GOOG",
    "prices": [49.982655, 53.95277, 54.495735, ...],
    "dates": ["2004-08-19", "2004-08-20", ...]
  },
  "error": null
}
```

## Future Enhancements
- Implement Sharpe ratio calculation.
- Add support for date range filtering.