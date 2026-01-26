//! Provides ready to use [`Iri<String>`](sophia::iri::Iri)s for basic RDF vocabularies.

pub mod np {
    //! [Nanopub](http://nanopub.org/guidelines/working_draft) vocabulary.
    use sophia::iri::Iri;

    /// np:Nanopublication rdf:type owl:Class.
    pub const NANOPUBLICATION: Iri<&'static str> =
        Iri::new_unchecked_const("http://www.nanopub.org/nschema#Nanopublication");
    /// np:Assertion rdfs:subClassOf rdfg:Graph.
    pub const ASSERTION: Iri<&'static str> =
        Iri::new_unchecked_const("http://www.nanopub.org/nschema#Assertion");
    /// np:Provenance rdfs:subClassOf rdfg:Graph.
    pub const PROVENANCE: Iri<&'static str> =
        Iri::new_unchecked_const("http://www.nanopub.org/nschema#Provenance");
    /// np:PublicationInfo rdfs:subClassOf rdfg:Graph.
    pub const PUBLICATION_INFO: Iri<&'static str> =
        Iri::new_unchecked_const("http://www.nanopub.org/nschema#PublicationInfo");
    /// np:hasAssertion a owl:FunctionalProperty; rdfs:domain np:Nanopublication; rdfs:range np:Assertion.
    pub const HAS_ASSERTION: Iri<&'static str> =
        Iri::new_unchecked_const("http://www.nanopub.org/nschema#hasAssertion");
    /// np:hasProvenance a owl:FunctionalProperty; rdfs:domain np:Nanopublication; rdfs:range np:Provenance.
    pub const HAS_PROVENANCE: Iri<&'static str> =
        Iri::new_unchecked_const("http://www.nanopub.org/nschema#hasProvenance");
    /// np:hasPublicationInfo a owl:FunctionalProperty; rdfs:domain np:Nanopublication; rdfs:range np:PublicationInfo.
    pub const HAS_PUBLICATION_INFO: Iri<&'static str> =
        Iri::new_unchecked_const("http://www.nanopub.org/nschema#hasPublicationInfo");
}

pub mod npx {
    //! [Nanopub-X](http://purl.org/nanopub/x/) vocabulary (subset only).
    use sophia::iri::Iri;

    /// npx:hasAlgorithm rdf:type owl:DatatypeProperty, owl:FunctionalProperty; rdfs:domain npx:CryptoElement; rdfs:range xsd:string.
    pub const HAS_ALGORITHM: Iri<&'static str> =
        Iri::new_unchecked_const("http://purl.org/nanopub/x/hasAlgorithm");
    /// npx:hasPublicKey rdf:type owl:DatatypeProperty, owl:FunctionalProperty; rdfs:domain npx:CryptoElement; rdfs:range xsd:string.
    pub const HAS_PUBLIC_KEY: Iri<&'static str> =
        Iri::new_unchecked_const("http://purl.org/nanopub/x/hasPublicKey");
    /// npx:hasSignatureTarget rdf:type rdf:Property, owl:FunctionalProperty; rdfs:domain npx:NanopubSignatureElement; rdfs:range np:Nanopublication.
    pub const HAS_SIGNATURE_TARGET: Iri<&'static str> =
        Iri::new_unchecked_const("http://purl.org/nanopub/x/hasSignatureTarget");
    /// npx:hasSignature rdf:type owl:DatatypeProperty, owl:FunctionalProperty; rdfs:domain npx:NanopubSignatureElement.
    pub const HAS_SIGNATURE: Iri<&'static str> =
        Iri::new_unchecked_const("http://purl.org/nanopub/x/hasSignature");
    /// npx:declaredBy rdf:type rdf:Property; rdfs:domain npx:KeyDeclaration; rdfs:range foaf:Agent.
    pub const DECLARED_BY: Iri<&'static str> =
        Iri::new_unchecked_const("http://purl.org/nanopub/x/declaredBy");
}

pub mod pav {
    //! [PAV](https://github.com/pav-ontology/pav/wiki/) vocabulary (subset only).
    use sophia::iri::Iri;

    /// Created by.
    pub const CREATED_BY: Iri<&'static str> =
        Iri::new_unchecked_const("http://purl.org/pav/createdBy");
}

pub mod foaf {
    //! [FOAF](http://xmlns.com/foaf/0.1/) vocabulary (subset only).
    use sophia::iri::Iri;

    /// A name for some thing.
    pub const NAME: Iri<&'static str> =
        Iri::new_unchecked_const("http://www.nanopub.org/nschema#name");
}

pub mod dct {
    //! [DCMI Metadata Terms](http://purl.org/dc/terms/) vocabulary (subset only).
    use sophia::iri::Iri;

    /// Date Created.
    pub const CREATED: Iri<&'static str> =
        Iri::new_unchecked_const("http://purl.org/dc/terms/created");
    /// Creator.
    pub const CREATOR: Iri<&'static str> =
        Iri::new_unchecked_const("http://purl.org/dc/terms/creator");
}

pub mod prov {
    //! [PROV](http://www.w3.org/ns/prov#) vocabulary (subset only).
    use sophia::iri::Iri;

    ///  Attribution is the ascribing of an entity to an agent.
    pub const WAS_ATTRIBUTED_TO: Iri<&'static str> =
        Iri::new_unchecked_const("http://www.w3.org/ns/prov#wasAttributedTo");
}

pub mod rdf {
    //! [RDF](https://www.w3.org/TR/rdf11-concepts/) vocabulary (subset only).
    use sophia::iri::Iri;

    /// The subject is an instance of a class.
    pub const TYPE: Iri<&'static str> =
        Iri::new_unchecked_const("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
}

pub mod xsd {
    //! [XML Schema](http://www.w3.org/2001/XMLSchema#) vocabulary (subset only).
    use sophia::iri::Iri;

    /// Date and time with or without timezone.
    pub const DATE_TIME: Iri<&'static str> =
        Iri::new_unchecked_const("http://www.w3.org/2001/XMLSchema#dateTime");
}
