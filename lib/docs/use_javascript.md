# 🟨 Use from JavaScript

[![npm](https://img.shields.io/npm/v/@nanopub/sign)](https://www.npmjs.com/package/@nanopub/sign)

You can easily publish Nanopubs from JavaScript, or TypeScript with the [`@nanopub/sign`](https://www.npmjs.com/package/@nanopub/sign) NPM package.

```admonish example title="Demo"
Visit the **[demo page](https://vemonet.github.io/nanopub-rs/demo.html)** to sign nanopubs, or generate and register a new key pair, directly in your browser using this NPM package. You can checkout the [`demo.html`](https://github.com/vemonet/nanopub-rs/blob/main/lib/docs/demo.html) file as an example to use the package directly from HTML/JS.
```

```admonish info title="Build a Nanopublication"
This package takes an already prepared Nanopublication RDF string as input. If you want to easily display nanopubs checkout the [`@nanopub/display`](https://nanopublication.github.io/nanopub-js/modules/_nanopub_display.html) package, if you want to fetch and manipulate nanopubs check the [`@nanopub/utils`](https://nanopublication.github.io/nanopub-js/modules/_nanopub_utils.html) package.
```

## 🚀 Use it in bare HTML files

You can easily import the NPM package from a CDN, and sign a Nanopublication from a simple `index.html` file:

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
      const private_key=`MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=`;

      async function main() {
        // WebAssembly binary needs to be initialized
        await init();
        const rdfText = document.getElementById('rdf-text');
        const serverUrl = "";

        // Instantiate nanopub profile
        const profile = new NpProfile("https://orcid.org/0000-0000-0000-0000", "Your Name", private_key, "");

        // Check
        const checked = new Nanopub(rdfStr).check();
        console.log("Checked info dict:", checked.info());

        // Sign
        const signed = new Nanopub(rdfStr).sign(profile)
        console.log("Signed info dict:", signed.info());

        // Publish
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

The nanopub test server is used if the last argument of `Nanopub.publish` is an empty string, to easily publish to a production server use `getNpServer()`:

```typescript
import init, { Nanopub, NpProfile, getNpServer } from "https://unpkg.com/@nanopub/sign";

const np = await new Nanopub(rdfStr).publish(profile, getNpServer());
```

## 📥️ Install

Install the `npm` package (use `yarn` or `pnpm` if you prefer) to use it from your favorite framework:

```bash
npm install @nanopub/sign
```

## ⚛️ Use from any JavaScript framework

It can be used from any JavaScript framework, or NodeJS.

For example, to use it in a nextjs react app:

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
            "hasAssertion": {"@id": "np:hasAssertion"},
            "hasProvenance": {"@id": "np:hasProvenance"},
            "hasPublicationInfo": {"@id": "np:hasPublicationInfo"},
            "Nanopublication": {"@id": "np:Nanopublication"}
        },
        "@id": "#Head",
        "@graph" : {
            "@id" : "#",
            "@type": "Nanopublication",
            "hasAssertion" : {
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
            "hasProvenance" : {
                "@id" : "#provenance",
                "@graph" : [
                    {
                        "@id": "#assertion",
                        "prov:hadPrimarySource": {"@id": "http://dx.doi.org/10.3233/ISU-2010-0613"}
                    }
                ]
            },
            "hasPublicationInfo" : {
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

        // Initialize the wasm library and use it
        init().then(async () => {
          const serverUrl = "";
          const profile = new NpProfile("https://orcid.org/0000-0000-0000-0000", "User Name", privateKey, "");

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
