# ICEES DB

Tools for building the ICEES backend database(s).

## Instructions

Generate a `.env` file of the following sort:

```text
ICEES_PORT=5432
ICEES_HOST=db
ICEES_DB=sqlite
ICEES_DB_POOL_SIZE=10
ICEES_DB_MAX_OVERFLOW=0
DB_PATH=./db/data
DATA_PATH=./data
CONFIG_PATH=./config
COMPOSE_PROJECT_NAME=icees
MAX_OVERFLOW=0
POOL_SIZE=10
```

Add a `features.yml` file to your `CONFIG_PATH` of the following sort:

```yaml
table_0:
  feature_0:
    type: integer
    minimum: 0
    maximum: 1
  feature_1:
    type: string
    enum:
      - 'alligator'
      - 'baboon'
      - 'cockatoo'
```

Run:

```bash
./initdb.sh
```

Or, with Docker:

```bash
docker-compose -f docker-compose.postgres.yml up [-d]
```

or

```bash
docker-compose -f docker-compose.sqlite.yml up [-d]
```
