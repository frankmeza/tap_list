### Create Database `beer_tap_list`

```sql
CREATE DATABASE beer_tap_list;

\c beer_tap_list;
```

### Create DB Table `beers`

```sql
CREATE TABLE beers (
    id                VARCHAR(40) NOT NULL,
    sort_order        INTEGER NOT NULL,
    name              VARCHAR(100) NOT NULL,
    type              VARCHAR(100) NOT NULL,
    abv               VARCHAR(5) NOT NULL,
    ibu               VARCHAR(3),
    serving_size      VARCHAR(3),
    cost              VARCHAR(2) NOT NULL,
    brewery_name      VARCHAR(60) NOT NULL,
    brewery_city      VARCHAR(60),
    brewery_state     VARCHAR(2),
    brewery_img_url   VARCHAR(255), 
    keg_id            VARCHAR(3) NOT NULL,
    keg_size          INTEGER NOT NULL, 
    keg_amount_left   INTEGER NOT NULL,
    updated_ts        INTEGER NOT NULL,
    created_ts        INTEGER NOT NULL
);
```

## INSERT INTO Beers

```sql
INSERT INTO beers (
    id,
    sort_order,
    name,
    type,
    abv,
    ibu,
    serving_size,
    cost,
    brewery_name,
    brewery_city,
    brewery_state,
    brewery_img_url,
    keg_id,
    keg_size,
    keg_amount_left,
    updated_ts,
    created_ts
) VALUES (
  'ID STRING',
  0,
  'TUAN BEER',
  'STOUT',
  '10.0',
  '20',
  '16',
  '5',
  'TUAN BREWED IT',
  'LONG BEACH',
  'CA',
  'IMAGE URL',
  'KEG',
  100,
  90,
  0,
  0
);
```
