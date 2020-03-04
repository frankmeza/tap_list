<!-- checkout markdown snippets: pagetop, goback, makelinks, atag, ltag for making links to top of page, and elsewhere -->

<sub><a name="top_of_page">top_of_page</a></sub>
 
# Table of Contents 

1. [Database](#database) - for database related things to know.
2. [Function Naming Convertion](#naming_functions) - how to name functions.

<sub><a name="database">database
  
<sub>[go back to top](#top_of_page)</sub>  

## 1. Database 

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

<sub><a name="naming_functions">naming_functions</a>  
  
<sub>[go back to top](#top_of_page)</sub>  

## Function Naming Convention

### Responders

Functions that act as Responders are prefixed with `fetch_`.

### Handlers, and utils

Functions that act as Handlers are prefixed with `handle_`.  
Functions that act as Handlers are prefixed with `collect_`.  

### Queries, and utils

Functions that return SQL Queries are prefixed with `get_`.  
Functions that return SQL Queries are prefixed with `generate_`.  

