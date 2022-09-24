from nanopub_py import Nanopub

np = Nanopub(rdf="<http://s> <http://p> <http://o> .")

print(np.get_rdf())