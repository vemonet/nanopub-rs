# 🐍 Use from Python

[![PyPI](https://img.shields.io/pypi/v/nanopub-sign)](https://pypi.org/project/nanopub-sign/)

You can easily publish Nanopubs from Python.


~~~admonish info title="Build a Nanopublication"
This package takes an already prepared Nanopublication RDF string as input. If you want to build a Nanopublication programmatically, use the [`nanopub`](https://fair-workflows.github.io/nanopub) pip package. You can then feed the serialized RDF of the built Nanopub to this package functions.
~~~

## 📥️ Install

Install the `pip` package:

```bash
pip install nanopub-sign
```

## 🚀 Use

Create a `sign.py` file that takes a nanopub RDF and a private key as input.

```admonish success title="Get a private key"
You can easily create and register a new private key on the [demo page](https://vemonet.github.io/nanopub-rs/demo.html) after login with your ORCID.
```

```python
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

# Instantiate nanopub profile
profile = NpProfile(
    private_key=private_key,
    orcid_id="https://orcid.org/0000-0000-0000-0000",
    name="Your Name",
    introduction_nanopub_uri=""
)

# Check a nanopub RDF string
np = Nanopub(rdf_str).check()
print("Checked info dict:", np.info())

# Sign a nanopub
np = Nanopub(rdf_str)
np = np.sign(profile=profile)
print("Signed info dict:", np.info())

# Sign & publish
np = Nanopub(rdf_str).publish(profile=profile, server_url=None)
print("Published info dict:", np.info())
print(np.get_rdf())
```

Run the script:

```bash
python sign.py
```

## 🧪 Test and production servers

If the provided `server_url` is empty, the nanopub will be published to the [test server](https://np.test.knowledgepixels.com/). In this case the nanopub will not be available at https://w3id.org/np/, but at https://np.test.knowledgepixels.com/, e.g. https://np.test.knowledgepixels.com/RAKObyGXmbgTYWj2iN0XGgJv0yWNDQd_DTmAWUouGfIsM

To publish to a production server use `get_np_server(true)`. With true for a random server in the network, and false for the [main nanopub server](https://server.np.trustyuri.net/), defaults to true.

```python
from nanopub_sign import Nanopub, NpProfile, get_np_server

np = Nanopub.publish(
    rdf=rdf_str,
    profile=profile,
    server_url=get_np_server(),
)
```
