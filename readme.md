### Create Database `beer_tap_list`

```sql
CREATE DATABASE beer_tap_list;

\c beer_tap_list;
```

### Create DB Table `beers`

```sql
CREATE TABLE beers (
    id                SERIAL PRIMARY KEY,
    name              VARCHAR(100) NOT NULL,
    type              VARCHAR(100) NOT NULL,
    abv               VARCHAR(5) NOT NULL,
    ibu               VARCHAR(3),
    serving_size      VARCHAR(3),
    cost              VARCHAR(2) NOT NULL,
    brewery_name      VARCHAR(60) NOT NULL,
    brewery_city      VARCHAR(60),
    brewery_state     VARCHAR(2),
    brewery_img_url   VARCHAR(100), 
    keg_id            VARCHAR(3) NOT NULL,
    keg_size          INTEGER NOT NULL, 
    keg_amount_left   INTEGER NOT NULL,
    updated_ts        INTEGER NOT NULL,
    created_ts        INTEGER NOT NULL,
);
```
