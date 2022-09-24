from nanopub_py import Nanopub

np = Nanopub(
    rdf="<http://s> <http://p> <http://o> .",
    public_key="PUBLICKEY",
    private_key="PRIVATEKEY",
    orcid="https://orcid.org/0000-0000-0000-0000",
    # TODO: Defaults don't work
    server_url='',
    publish=False,
)

print(np.get_rdf())