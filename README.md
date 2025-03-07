# merfix

Remove EXIF from supported image with the power of love and wasm.

## Requirements

### Install wasm-pack

```sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Available functions

| Function                     | Description                                                | Arguments          |
|------------------------------|------------------------------------------------------------|--------------------|
| ```supported_mime_types```   | Returns list of library supported mime types               | None               |
| ```supported_extensions```   | Returns list of library supported file extensions          | None               |
| ```detect_image_mime_type``` | Detects image mime type based on data content of the image | Uint8Array         |
| ```detect_image_extension``` | Detects image extension based on data content of the image | Uint8Array         |
| ```remove_exif```            | Removes all the EXIF metadata from the image               | Uint8Array, String |
| ```version```                | Retrieve version of the library                            | String             |

## CDN

```
https://cdn.jsdelivr.net/npm/merfix@0/merfix.min.js
https://cdn.jsdelivr.net/npm/merfix@0.1/merfix.min.js
https://cdn.jsdelivr.net/npm/merfix@0.1.35/merfix.min.js
```

NPM

```
https://www.npmjs.com/package/merfix
```

## Demo

```
https://dimitrmo.github.io/merfix/
```

## JavaScript Integration

See https://github.com/dimitrmo/merfix/blob/master/public/index.js
