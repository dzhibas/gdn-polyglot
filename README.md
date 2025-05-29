# gdn-polyglot (GDN for translations)

GDN reverse proxy which provides automatic translations of your website

GDN workings:

Request → Proxy Intercept → Content Parse → Translation Lookup → Response Modification → Delivery

Reverse Proxy with Rust Pingora (used in cloudfare)
Filters (excluding urls and stuff, and in that case nothing rewritten)
Content Parse > lol-html lexer/parser or html5ever
Multi-Level Caching - Redis (translation lookup)
Rewriter - HashMap lookup based on parsed source (source is has for source english content)
Translation provider populates translation lookup table

Write as simple proof of concept:

1. reverse proxy to random.com/about (upstream)
2. filter uri /about
3. choose one string to rewrite
4. lookup in simple hashmap
5. ingest if not in hashmap
6. respond to downstream

LLM translation gdn

you pass through and pipeline will translate it in background with using LLM (of your choice)
this distributed hashmap is eventually is translated into languages of choice

Product name:

HeyLingua
FluentGDN
Polyglot
HeyPolyglot
FlowGlot
HeyAiGlot
AiGlot
