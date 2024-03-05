# 🟨 Use from JavaScript

[![npm](https://img.shields.io/npm/v/@nanopub/sign)](https://www.npmjs.com/package/@nanopub/sign)

You can use this toolkit to sign and publish Nanopublications from JavaScript or TypeScript with the [`@nanopub/sign`](https://www.npmjs.com/package/@nanopub/sign) NPM package.

```admonish example title="Demo"
Visit the **[demo page](https://vemonet.github.io/nanopub-rs/demo.html)** to sign nanopubs, or generate and register a new key pair, directly in your browser using this NPM package. You can checkout the [`demo.html`](https://github.com/vemonet/nanopub-rs/blob/main/lib/docs/demo.html) file as an example to use the package directly from HTML/JS.
```

```admonish info title="Build a Nanopublication"
This package takes an already prepared Nanopublication RDF string as input. If you want to easily display nanopubs checkout the [`@nanopub/display`](https://nanopublication.github.io/nanopub-js/modules/_nanopub_display.html) package, if you want to fetch and manipulate nanopubs check the [`@nanopub/utils`](https://nanopublication.github.io/nanopub-js/modules/_nanopub_utils.html) package.
```

## 📥️ Install

Install the `npm` package (use `yarn` or `pnpm` if you prefer) to use it from your favorite framework:

```bash
npm install --save @nanopub/sign
```

Or directly import from a CDN in JavaScript code:

```typescript
import init, { Nanopub, NpProfile, getNpServer } from "https://unpkg.com/@nanopub/sign";
```

## ℹ️ How it works

This package provides several functionalities related to the handling of Nanopublications, including signing, publishing, verifying, and fetching them:

### ✍️ Sign Nanopubs

This process involves signing a Nanopublication RDF string using a specified RSA private key passed through the profile. The signing operation ensures that the Nanopub is authentically created by the holder of the private key.

~~~admonish success title="Get a private key"
You can easily create and register a new private key on the [demo page](https://vemonet.github.io/nanopub-rs/demo.html) after login with your ORCID.
~~~

```typescript
import init, { Nanopub, NpProfile, getNpServer } from "@nanopub/sign";

// Change the RDF and private key as you wish
const rdfStr = `@prefix : <http://purl.org/nanopub/temp/mynanopub#> .
@prefix np: <http://www.nanopub.org/nschema#> .
@prefix npx: <http://purl.org/nanopub/x/> .
@prefix prov: <http://www.w3.org/ns/prov#> .
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
}`
const privateKey=`MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=`;

// Instantiate Nanopub profile
const profile = new NpProfile(privateKey, "https://orcid.org/0000-0000-0000-0000", "Your Name", "");

// Sign the Nanopub RDF
const signed = new Nanopub(rdfStr).sign(profile);
console.log("Signed:", signed.info());
```

### 📬 Publish Nanopubs

Signed Nanopubs can be published to a Nanopub server. This makes the Nanopub accessible to others in the network.

Use the `publish` function on a Nanopub, the 2 arguments are optional:

- `profile` is required if you want to also sign the nanopub, it is not required if you provide a signed nanopub
- If the `server_url` is null it will be published to the test server

```typescript
import { Nanopub, NpProfile } from "@nanopub/sign";

const profile = new NpProfile(privateKey, "https://orcid.org/0000-0000-0000-0000", "Your Name", "");
const np = await new Nanopub(rdfStr).publish(profile, null);
console.log("Published:", np.info());
```

~~~admonish tip title="Provide the nanopub signed or unsigned"
- If signed nanopub and profile not provided, we publish the signed nanopub as it is
- If signed nanopub and profile provided, we re-sign the nanopub (only the triples related to the signature are changed)
- If unsigned nanopub and profile provided, we sign the nanopub
- If unsigned nanopub and profile not provided, we throw an error
~~~

#### 🧪 Test and productions servers

If the the last argument of `publish()` is null the nanopub will be published to the [test server](https://np.test.knowledgepixels.com/). In this case the nanopub will not be available at https://w3id.org/np/, but at https://np.test.knowledgepixels.com/, e.g. https://np.test.knowledgepixels.com/RAKObyGXmbgTYWj2iN0XGgJv0yWNDQd_DTmAWUouGfIsM

You can publish to the production network by getting the URL of a server using `getNpServer(true)` (true will pick a random nanopub server on the production network, while false will pick the [main nanopub server](https://server.np.trustyuri.net/)):

```typescript
import init, { Nanopub, NpProfile, getNpServer } from "@nanopub/sign";

const np = await new Nanopub(rdfStr).publish(profile, getNpServer(true));
```

### ☑️ Verify Nanopubs

This operation involves checking the integrity of Nanopubs. It ensures that a Nanopub is valid, regardless of whether it is signed or unsigned.

```typescript
const checked = new Nanopub(rdfStr).check();
```

### 📡 Fetch Nanopubs

This function allows you to retrieve Nanopubs from the network using their URI. It's useful for accessing and using Nanopubs created by others.

```typescript
const npUri = "https://w3id.org/np/RAltRkGOtHoj5LcBJZ62AMVOAVc0hnxt45LMaCXgxJ4fw";
const np = await Nanopub.fetch(npUri);
console.log(np.info())
```

### 🔑 Generate private key and publish introduction

You can generate a new private/public key pair, and publish a nanopub introduction to register this key under your ORCID in the Nanopublications network:

```typescript
import init, { Nanopub, NpProfile, KeyPair } from "@nanopub/sign";

// Randomly generate a new private/public key pair, and convert it to a JS object
let keypair = new KeyPair();
keypair = keypair.toJs();

// Create profile with new private key
const orcid = "https://orcid.org/0000-0000-0000-0000"
const profile = new NpProfile(keypair.private, orcid, "Your Name", "");

// Publish nanopub introduction for this profile
Nanopub.publish_intro(profile, getNpServer(false))
    .then(np => {
        console.log("Published Introduction Nanopub:", np.info());
    })
    .catch(err => {
        console.error("Error publishing the Nanopub Introduction:", err);
    });
```

## 🚀 Use it in bare HTML files

You can easily import the NPM package from a CDN, and sign a Nanopublication from a simple `index.html` file. There is no need to install the `npm` package in this case, but you will need to initialize the WebAssembly binary when using it on the client (for JS running in the browser):

```html
<!DOCTYPE html>
<html lang="en-US">
  <head>
    <meta charset="utf-8" />
    <title>Testing Nanopub JS</title>
  </head>
  <body>
    <pre><code id="rdf-text"></code></pre>

    <script type="module">
      import init, { Nanopub, NpProfile } from "https://unpkg.com/@nanopub/sign";

      // Change the RDF and private key as you wish
      const rdfStr = `@prefix : <http://purl.org/nanopub/temp/mynanopub#> .
@prefix np: <http://www.nanopub.org/nschema#> .
@prefix npx: <http://purl.org/nanopub/x/> .
@prefix prov: <http://www.w3.org/ns/prov#> .
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
}`
      const privateKey=`MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=`;

      async function main() {
        // WebAssembly binary needs to be initialized only if the JS runs on the client
        await init();
        const rdfText = document.getElementById('rdf-text');
        const serverUrl = "";

        // Instantiate nanopub profile
        const profile = new NpProfile(privateKey, "https://orcid.org/0000-0000-0000-0000", "Your Name", "");

        // Check a nanopub RDF string
        const checked = new Nanopub(rdfStr).check();
        console.log("Checked info dict:", checked.info());

        // Sign a nanopub
        const signed = new Nanopub(rdfStr).sign(profile)
        console.log("Signed info dict:", signed.info());

        // Sign & publish a nanopub
        const np = await new Nanopub(rdfStr).publish(profile, serverUrl);
        console.log("Published info dict:", np.info());
        rdfText.innerText = np.get_rdf();
      }
      main()
    </script>
  </body>
</html>
```

Then just start the web server from the directory where the HTML file is with:

```bash
npx http-server
# Or:
python -m http.server
```


## ⚛️ Use from any JavaScript framework

It can be used from any JavaScript framework, or NodeJS.

For example, to use it in a nextjs react app to publish a nanopub defined in JSON-LD:

1. Create the project and `cd` into your new app folder

    ```bash
    npx create-next-app@latest --typescript
    ```

2. Add the `@nanopub/sign` dependency to your project:

    ```bash
    npm install --save @nanopub/sign
    ```

3. Add code to sign the nanopub, e.g. in `src/app/page.tsx`:

    ```typescript
    'use client'
    import { useEffect, useState } from 'react';
    import init, { Nanopub, NpProfile } from "@nanopub/sign";

    export default function Home() {
      const [rdfOutput, setRdfOutput] = useState('');
      useEffect(() => {
        // ℹ️ You can also provide JSON-LD objects!
        const rdf = {
        "@context": {
            "@base": "http://purl.org/nanopub/temp/mynanopub#",
            "np" : "http://www.nanopub.org/nschema#",
            "npx": "http://purl.org/nanopub/x/",
            "rdf": "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
            "rdfs": "http://www.w3.org/2000/01/rdf-schema#",
            "xsd": "http://www.w3.org/2001/XMLSchema#",
            "foaf": "http://xmlns.com/foaf/0.1/",
            "dct": "http://purl.org/dc/terms/",
            "prov": "http://www.w3.org/ns/prov#",
            "pav": "http://purl.org/pav/",
            "orcid": "https://orcid.org/",
            "schema": "https://schema.org/",
        },
        "@id": "#Head",
        "@graph" : {
            "@id" : "#",
            "@type": "np:Nanopublication",
            "np:hasAssertion" : {
                "@id" : "#assertion",
                "@context": {
                    "ex": "http://example.org/"
                },
                "@graph" : [
                    {
                        "@id": "ex:mosquito",
                        "ex:transmits": {"@id": "ex:malaria"}
                    }
                ]
            },
            "np:hasProvenance" : {
                "@id" : "#provenance",
                "@graph" : [
                    {
                        "@id": "#assertion",
                        "prov:hadPrimarySource": {"@id": "http://dx.doi.org/10.3233/ISU-2010-0613"}
                    }
                ]
            },
            "np:hasPublicationInfo" : {
                "@id" : "#pubinfo",
                "@graph" : [
                    {
                        "@id": "#",
                        "@type": "npx:ExampleNanopub"
                    }
                ]
            }
        }
    };
        const privateKey = `MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=`;

        // WebAssembly binary needs to be initialized only if the JS runs on the client
        init().then(async () => {
          const serverUrl = "";
          const profile = new NpProfile(privateKey, "https://orcid.org/0000-0000-0000-0000", "User Name", "");

          const np = await new Nanopub(rdf).publish(profile, serverUrl)
          console.log("Published info dict:", np.info());
          setRdfOutput(np.get_rdf());
        });
      }, []);

      return (
        <main>
          <h1>Nanopublication RDF Output:</h1>
          <pre><code>{rdfOutput}</code></pre>
        </main>
      );
    }
    ```

4. Start in dev:

    ```bash
    npm run dev
    ```
