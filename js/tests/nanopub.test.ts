import {describe, expect, test} from '@jest/globals';
import {Nanopub, NpProfile} from "../pkg/node";

const privKey=`MIICeAIBADANBgkqhkiG9w0BAQEFAASCAmIwggJeAgEAAoGBAPdEfIdHtZYoFh6/DWorzoHpFXMjugqW+CGpe9uk4BfUq54MToi2u7fgdGGtXLg4wsJFBYETdVeS0p1uA7EPe8LhwjHPktf5c6AZbO/lYpKM59e7/Ih4mvOy4iTIe/Dv+1OgasTSK0nXAbKUm/5iJ6LOYa82JQeE/QnT5gUw2e97AgMBAAECgYBbNQnyJINYpeSy5qoeFZaQ2Ncup2kCavmQASJMvJ5ka+/51nRJfY30n3iOZxIiad19J1SGbhUEfoXtyBzYfOubF2i2GJtdF5VyjdSoU6w/gOo2/vnbH+GCHnMclrWshohOADGQU/Y8pYhIvlQqcb6xEOts9m9C9g4uwvPXqjmhoQJBAPkmSFIZwF3i2UvJlHyeXi599L0jkGTUJy/Y4IjieUx5suwvAtG47ejhgIPKK06VtW49oGPHWjWc3cJAmnV+vTMCQQD+EPTvNtLpX9QiDEJD7b8woDwmVrvH/RUosP/cXpMQd7BUVgPlpffAlFJGDlOzwwjZjy+8kc6MYevh1kWqobSZAkEAyCs+nV99ErEHnYEFoB1oU3f0oeSpxKhCF4np03AIvi1kV6bpX+9wjNJnevp5UriqvDgc3S0zx7EQ5Vkb/1vkywJBAMMw59y4tAVT+DhITsi9aTvEfzG9RPt6trzSb2Aw0K/AJJpGkyvl/JfZ2/Oyoh/jYXM0DKrFIni76mtRIajcH1ECQQCJi6aXOaRkRPmf7FYY9cRaJdR1BtZkKZbDg6ZMD1bY97cGiM9STTMeldYcCtQBtyhVCTEObI/V6/0FAvY9Zi7w`;
const orcid="https://orcid.org/0000-0000-0000-0000";
const unsignedRdf = `@prefix : <http://purl.org/nanopub/temp/mynanopub#> .
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
	: dc:created "2014-07-24T18:05:11+01:00"^^xsd:dateTime ;
		pav:createdBy <http://orcid.org/0000-0002-1267-0234> ;
		a npx:ExampleNanopub .
}
`

describe('Tests for the @nanopub/sign npm package', () => {
  // NOTE: `await init()` only needed in browser environment

  test('sign nanopub', async () => {
    const profile = new NpProfile(privKey, orcid, "Your Name");
    const np = new Nanopub(unsignedRdf).sign(profile);
    // console.log({ profile, np });
    // console.log({ profile, nanopub: np.info() });
    // console.log({ profile, nanopub: nanopub.info(), signed: signed.info() });
    expect(np.info().trusty_hash.startsWith("RA")).toBe(true);
    expect(np.info().trusty_hash).toBe("RAe_LF_8hl-wFdzgbxnLS2T3zNWwic2jFiF-tjuWCdkr4");
  });

  test('sign nanopub and check', async () => {
    const profile = new NpProfile(privKey, orcid, "Your Name");
    let np = new Nanopub(unsignedRdf);
    np = np.sign(profile);
    expect(np.check());
    // np.sign(profile);
    // expect(np.info().trusty_hash.startsWith("RA")).toBe(true);
    // expect(np.info().trusty_hash).toBe("RAe_LF_8hl-wFdzgbxnLS2T3zNWwic2jFiF-tjuWCdkr4");
  });

  test('publish nanopub', async () => {
    const profile = new NpProfile(privKey, orcid, "Your Name");
    const np = await new Nanopub(unsignedRdf).publish(profile);
    expect(np.info().published).toBeDefined();
    // console.log({ profile, np });
    // console.log({ profile, nanopub: np.info() });
    // console.log({ profile, nanopub: nanopub.info(), signed: signed.info() });
    // expect(np.info().trusty_hash).toBe("RAE9traVUygMTJ-k8E1_pVNy3gtf7uUvtHJtPeU64WpA4");
  });

  test('publish nanopub default profile', async () => {
    const profile = new NpProfile(privKey);
    const np = await new Nanopub(unsignedRdf).publish(profile);
    expect(np.info().published).toBeDefined();
    // console.log({ profile, np });
    // console.log({ profile, nanopub: np.info() });
  });

  test('fetch nanopub', async () => {
    const npUri = "https://w3id.org/np/RAltRkGOtHoj5LcBJZ62AMVOAVc0hnxt45LMaCXgxJ4fw";
    const np = await Nanopub.fetch(npUri);
    expect(np.info().published).toBeDefined();
  });

});
