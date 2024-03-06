# 🐍 Use from Python

[![PyPI](https://img.shields.io/pypi/v/nanopub-sign)](https://pypi.org/project/nanopub-sign/)

You can use this toolkit to sign and publish Nanopublications from Python with the [`nanopub`](https://pypi.org/project/nanopub-sign/) pip package.

!!! info "Build a Nanopublication"
    This package takes an already prepared Nanopublication RDF string as input. If you want to build a Nanopublication programmatically, use the [`nanopub`](https://fair-workflows.github.io/nanopub) pip package. You can then feed the serialized RDF of the built Nanopub to this package functions.

## 📥️ Install

Install the package with pip:

```bash
pip install nanopub-sign
```

## ✍️ Sign Nanopubs

This process involves signing a Nanopublication RDF string using a specified RSA private key passed through the profile. The signing operation ensures that the Nanopub is authentically created by the holder of the private key.

!!! success "Get a private key"
    You can easily create and register a new private key on the [playground page](https://vemonet.github.io/nanopub-rs/playground.html) after login with your ORCID.

```python title="sign.py"
from nanopub_sign import Nanopub, NpProfile

# Change the RDF and private key as you wish
rdf_str = """@prefix : <http://purl.org/nanopub/temp/mynanopub#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix dc: <http://purl.org/dc/terms/> .
@prefix pav: <http://purl.org/pav/> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix np: <http://www.nanopub.org/nschema#> .
@prefix npx: <http://purl.org/nanopub/x/> .
@prefix ex: <http://example.org/> .

:Head {
	: np:hasAssertion :assertion ;
		np:hasProvenance :provenance ;
		np:hasPublicationInfo :pubinfo ;
		a np:Nanopublication .
}

:assertion {
	ex:mosquito ex:transmits ex:malaria .
}

:provenance {
	:assertion prov:hadPrimarySource <http://dx.doi.org/10.3233/ISU-2010-0613> .
}

:pubinfo {
	: a npx:ExampleNanopub .
}"""
private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc="

# Instantiate a nanopub profile
profile = NpProfile(
    private_key=private_key,
    orcid_id="https://orcid.org/0000-0000-0000-0000",
    name="Your Name",
    introduction_nanopub_uri=""
)

# Sign a nanopub
np = Nanopub(rdf_str)
np = np.sign(profile=profile)
print("Signed info dict:", np.info())
```

Run the script:

```bash
python sign.py
```

## 📬 Publish Nanopubs

Signed Nanopubs can be published to a Nanopub server. This makes the Nanopub accessible to others in the network.

Use the `publish` function on a Nanopub, the 2 arguments are optional:

- `profile` is required if you want to also sign the nanopub, it is not required if you provide a signed nanopub
- If the `server_url` is null it will be published to the test server

```typescript
np = Nanopub(rdf_str).publish(profile=profile, server_url=None)
print("Published info dict:", np.info())
print(np.get_rdf())
```

!!! tip "Provide the nanopub signed or unsigned"
    - [x] If signed nanopub and profile not provided, we publish the signed nanopub as it is
    - [x] If signed nanopub and profile provided, we re-sign the nanopub (signature triples are updated)
    - [x] If unsigned nanopub and profile provided, we sign the nanopub
    - [ ] If unsigned nanopub and profile not provided, we throw an error

### 🧪 Test and production servers

If the provided `server_url` is empty, the nanopub will be published to the [test server](https://np.test.knowledgepixels.com/). In this case the nanopub will not be available at [https://w3id.org/np/](https://w3id.org/np/), but at [https://np.test.knowledgepixels.com/](https://np.test.knowledgepixels.com/), e.g. [https://np.test.knowledgepixels.com/RAKObyGXmbgTYWj2iN0XGgJv0yWNDQd_DTmAWUouGfIsM](https://np.test.knowledgepixels.com/RAKObyGXmbgTYWj2iN0XGgJv0yWNDQd_DTmAWUouGfIsM)

To publish to a production server use `get_np_server(true)`. With true for a random server in the network, and false for the [main nanopub server](https://server.np.trustyuri.net/), defaults to true.

```python
from nanopub_sign import Nanopub, NpProfile, get_np_server

np = Nanopub.publish(
    rdf=rdf_str,
    profile=profile,
    server_url=get_np_server(),
)
```

## ☑️ Verify Nanopubs

This operation involves checking the integrity of Nanopubs. It ensures that a Nanopub is valid, regardless of whether it is signed or unsigned.

```python
np = Nanopub(rdf_str).check()
print("Checked info dict:", np.info())
```

## 📡 Fetch Nanopubs

This function allows you to retrieve Nanopubs from the network using their URI. It's useful for accessing and using Nanopubs created by others.

```python
from nanopub_sign import Nanopub

np = Nanopub.fetch("https://w3id.org/np/RAltRkGOtHoj5LcBJZ62AMVOAVc0hnxt45LMaCXgxJ4fw")
print(np.info())
```

## 🔑 Generate private key and publish introduction

You can generate a new private/public key pair, and publish a nanopub introduction to register this key under your ORCID in the Nanopublications network:

```python
from nanopub_sign import Nanopub, NpProfile, KeyPair, get_np_server

# Randomly generate a new private/public key pair
keypair = KeyPair()

# Create profile with new private key
new_profile = NpProfile(
    private_key=keypair.private,
    orcid_id="https://orcid.org/0000-0000-0000-0000",
    name="",
    introduction_nanopub_uri=""
)

# Publish nanopub introduction for this profile
np = Nanopub.publish_intro(new_profile, get_np_server())
print(np.info())

# Publish to the production network:
# np = Nanopub.publish_intro(new_profile, get_np_server())
```
