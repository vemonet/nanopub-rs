const CACHE_NAME = 'nanopub-cache';
const urlsToCache = [
  '/',
  '/index.html',
  'https://cdn.tailwindcss.com',
  'https://cdn.tailwindcss.com?plugins=typography',
  'https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css',
  'assets/orcid-widget.js',
  'https://kjur.github.io/jsrsasign/jsrsasign-latest-all-min.js',
  'assets/Solarized-dark.json',
  'assets/nanopub-json-schema.json',
  'assets/nanopub-example.trig',
  'assets/nanopub-example.json',
  'https://upload.wikimedia.org/wikipedia/commons/0/06/ORCID_iD.svg',
  'https://cdn.jsdelivr.net/npm/@monaco-editor/loader@1.4.0/+esm',
  'https://unpkg.com/@nanopub/sign',
];

self.addEventListener('install', function(event) {
  // Perform install steps
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(function(cache) {
        console.log('Opened cache');
        return cache.addAll(urlsToCache);
      })
  );
});

self.addEventListener('fetch', function(event) {
  event.respondWith(
    caches.match(event.request)
      .then(function(response) {
        // Cache hit - return response
        if (response) {
          return response;
        }
        return fetch(event.request);
      }
    )
  );
});
