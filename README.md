# MegaMix

- I don't know why I kept this name. (Probably will add stuff in future so to make sense of title)
- Not the best rust code
- Current state
  - API endpoints using [rocket.rs](https://rocket.rs/)
    - Basic endpoints so I don't forget how to code in rust and use rocket after weeks of not looking at them
    - Static file server endpoint
      - Basic one
      - Special image serving endpoint with image dimension query parameters
  

## API Reference

### Basic endpoints
#### Get Hello, world! or Hello, <name>!

```http
  GET /hello/<name>
```

| URL Parameter | Type     | Description                  |
|:--------------|:---------|:-----------------------------|
| `name`        | `string` | Some name (Default: "world") |

```http
  GET /hello?name=<name>
```

| Query Parameter | Type     | Description |
|:----------------|:---------|:------------|
| `name`          | `string` | Some name   |


#### Post name and age

```http
  POST /hello
```

| Json Body Parameter | Type     | Description               |
|:--------------------|:---------|:--------------------------|
| `name`              | `string` | **Required**. Some name   |
| `age`               | `int`    | **Required**. Some number |


### Static file server (serve files from src/static directory)

#### Get files
```http
  GET /public/<sub_directory>/<file_name>
```

| URL Parameter   | Type     | Description                   |
|:----------------|:---------|:------------------------------|
| `sub_directory` | `string` | Sub directory path (Optional) |
| `file_name`     | `string` | File name (**Required**)      |

#### Get images (with size query)
```http
  GET /public/images/<sub_directory>/<file_name>?width=<width>&height=<height>
```

| URL Parameter   | Type     | Description                   |
|:----------------|:---------|:------------------------------|
| `sub_directory` | `string` | Sub directory path (Optional) |
| `file_name`     | `string` | File name (**Required**)      |

| Query Parameter | Type  | Description                     |
|:----------------|:------|:--------------------------------|
| `width`         | `int` | Required image width in pixels  |
| `height`        | `int` | Required image height in pixels |
