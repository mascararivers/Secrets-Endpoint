# Usage

Usage format is `https://secrets-endpoint.onrender.com/secrets?uuid=uuid&api_key=key`. No headers required!
It includes a 45-second cache, so it returns an int value when result is new, or int + " (cached)" (ie. `208 (cached)`) when the result comes from the cache.
