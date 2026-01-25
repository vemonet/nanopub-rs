//! Provides ready to use [`NamedNodeRef`](oxrdf::NamedNodeRef)s for basic RDF vocabularies.

pub mod np {
    //! [Nanopub](http://nanopub.org/guidelines/working_draft) vocabulary.
    use oxrdf::NamedNodeRef;

    /// np:Nanopublication rdf:type owl:Class.
    pub const NANOPUBLICATION: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://www.nanopub.org/nschema#Nanopublication");
    /// np:Assertion rdfs:subClassOf rdfg:Graph.
    pub const ASSERTION: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://www.nanopub.org/nschema#Assertion");
    /// np:Provenance rdfs:subClassOf rdfg:Graph.
    pub const PROVENANCE: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://www.nanopub.org/nschema#Provenance");
    /// np:PublicationInfo rdfs:subClassOf rdfg:Graph.
    pub const PUBLICATION_INFO: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://www.nanopub.org/nschema#PublicationInfo");
    /// np:hasAssertion a owl:FunctionalProperty; rdfs:domain np:Nanopublication; rdfs:range np:Assertion.
    pub const HAS_ASSERTION: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://www.nanopub.org/nschema#hasAssertion");
    /// np:hasProvenance a owl:FunctionalProperty; rdfs:domain np:Nanopublication; rdfs:range np:Provenance.
    pub const HAS_PROVENANCE: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://www.nanopub.org/nschema#hasProvenance");
    /// np:hasPublicationInfo a owl:FunctionalProperty; rdfs:domain np:Nanopublication; rdfs:range np:PublicationInfo.
    pub const HAS_PUBLICATION_INFO: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://www.nanopub.org/nschema#hasPublicationInfo");
}

pub mod npx {
    //! [Nanopub-X](http://purl.org/nanopub/x/) vocabulary (subset only).
    use oxrdf::NamedNodeRef;

    /// npx:hasAlgorithm rdf:type owl:DatatypeProperty, owl:FunctionalProperty; rdfs:domain npx:CryptoElement; rdfs:range xsd:string.
    pub const HAS_ALGORITHM: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://purl.org/nanopub/x/hasAlgorithm");
    /// npx:hasPublicKey rdf:type owl:DatatypeProperty, owl:FunctionalProperty; rdfs:domain npx:CryptoElement; rdfs:range xsd:string.
    pub const HAS_PUBLIC_KEY: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://purl.org/nanopub/x/hasPublicKey");
    /// npx:hasSignatureTarget rdf:type rdf:Property, owl:FunctionalProperty; rdfs:domain npx:NanopubSignatureElement; rdfs:range np:Nanopublication.
    pub const HAS_SIGNATURE_TARGET: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://purl.org/nanopub/x/hasSignatureTarget");
    /// npx:hasSignature rdf:type owl:DatatypeProperty, owl:FunctionalProperty; rdfs:domain npx:NanopubSignatureElement.
    pub const HAS_SIGNATURE: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://purl.org/nanopub/x/hasSignature");
    /// npx:declaredBy rdf:type rdf:Property; rdfs:domain npx:KeyDeclaration; rdfs:range foaf:Agent.
    pub const DECLARED_BY: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://purl.org/nanopub/x/declaredBy");
}

pub mod pav {
    //! [PAV](https://github.com/pav-ontology/pav/wiki/) vocabulary (subset only).
    use oxrdf::NamedNodeRef;

    /// Created by.
    pub const CREATED_BY: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://purl.org/pav/createdBy");
}

pub mod foaf {
    //! [FOAF](http://xmlns.com/foaf/0.1/) vocabulary (subset only).
    use oxrdf::NamedNodeRef;

    /// A name for some thing.
    pub const NAME: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://www.nanopub.org/nschema#name");
}

pub mod dct {
    //! [DCMI Metadata Terms](http://purl.org/dc/terms/) vocabulary (subset only).
    use oxrdf::NamedNodeRef;

    /// Date Created.
    pub const CREATED: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://purl.org/dc/terms/created");
    /// Creator.
    pub const CREATOR: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://purl.org/dc/terms/creator");
}

pub mod prov {
    //! [PROV](http://www.w3.org/ns/prov#) vocabulary (subset only).
    use oxrdf::NamedNodeRef;

    ///  Attribution is the ascribing of an entity to an agent.
    pub const WAS_ATTRIBUTED_TO: NamedNodeRef<'_> =
        NamedNodeRef::new_unchecked("http://www.w3.org/ns/prov#wasAttributedTo");
}
