
use nanopub_rs::nanopub::Nanopub;

fn main() {
    // http://purl.org/np/RA5IWUwPmx_chibRuDOMfby6Sz8I0n76xnB3BiAm6ZP74
    let example = r#"
            @prefix this: <http://purl.org/nanopub/temp/mynanopub> .
            @prefix sub: <http://purl.org/nanopub/temp/mynanopub#> .
            @prefix drugbank: <http://identifiers.org/drugbank/> .
            @prefix np: <http://www.nanopub.org/nschema#> .
            @prefix pav: <http://purl.org/pav/> .
            @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
            @prefix infores: <https://w3id.org/biolink/infores/> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
            @prefix dcterms: <http://purl.org/dc/terms/> .
            @prefix orcid: <https://orcid.org/> .
            @prefix biolink: <https://w3id.org/biolink/vocab/> .
            @prefix pmid: <http://www.ncbi.nlm.nih.gov/pubmed/> .
            @prefix prov: <http://www.w3.org/ns/prov#> .
            @prefix npx: <http://purl.org/nanopub/x/> .

            sub:Head {
            this: np:hasAssertion sub:assertion;
                np:hasProvenance sub:provenance;
                np:hasPublicationInfo sub:pubInfo;
                a np:Nanopublication .
            }

            sub:assertion {
            drugbank:DB10771 a biolink:Drug;
                biolink:category biolink:Drug .

            <http://purl.obolibrary.org/obo/OMIM_130000> a biolink:Disease;
                biolink:category biolink:Disease .

            sub:association rdf:object <http://purl.obolibrary.org/obo/OMIM_130000>;
                rdf:predicate biolink:treats;
                rdf:subject drugbank:DB10771;
                a biolink:ChemicalToDiseaseOrPhenotypicFeatureAssociation;
                biolink:aggregator_knowledge_source infores:knowledge-collaboratory;
                biolink:category biolink:ChemicalToDiseaseOrPhenotypicFeatureAssociation;
                biolink:publications pmid:PMC3159979;
                biolink:relation <http://purl.obolibrary.org/obo/RO_0002606> .
            }

            sub:provenance {
            sub:assertion dcterms:created "2020-09-21T00:00:00"^^xsd:dateTime;
                prov:wasAttributedTo orcid:0000-0001-7769-4272 .
            }

            sub:pubInfo {
            sub:sig npx:hasAlgorithm "RSA";
                npx:hasPublicKey "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCR9fz0fKCdWOWC+pxhkQhEM/ppbdIYe5TLSdj+lJzSlv9mYBaPgrzVezSwwbmhlHBPDZa4/vHycU315BdmUGq+pXllp9+rWFfrb+kBJwhZjpG6BeyyXBsRFz4jmQVxl/ZYHilQTh/XalYzKkEAyTiEMPee4Kz61PaWOKH24CsnOQIDAQAB";
                npx:hasSignatureTarget this: .

            this: prov:generatedAtTime "2022-09-16T18:18:46.871040"^^xsd:dateTime;
                prov:wasAttributedTo orcid:0000-0002-1501-1082 .
            }
            "#;

    // let example = r#"
    //     @prefix this: <http://purl.org/np/RA5IWUwPmx_chibRuDOMfby6Sz8I0n76xnB3BiAm6ZP74> .
    //     @prefix sub: <http://purl.org/np/RA5IWUwPmx_chibRuDOMfby6Sz8I0n76xnB3BiAm6ZP74#> .
    //     @prefix np: <http://www.nanopub.org/nschema#> .
    //     this: np:hasAssertion sub:assertion;
    //         np:hasProvenance sub:provenance;
    //         np:hasPublicationInfo sub:pubInfo;
    //         a np:Nanopublication .
    //     "#;

    // let example = r#"
    //     <http://s> <http://p> <http://o> .
    //     "#;

    let np = Nanopub::new(example).unwrap_or_else(|error| {
        panic!("Problem parsing the RDF: {:?}", error);
    });

    // let np = Nanopub {
    //     rdf: String::from("toast"),
    // };
    println!("{}", np.get_rdf());
}