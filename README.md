# Sunnah Search Engine

Sunnah Search will be a search engine for hadiths from multiple sources and authenticity levels.

## Features

- Filter & Rank hadiths based on a query
- Support for fuzzy search

## Stack

- Rust
- tf-idf
- JSON

## **Strategy**

### Preprocessing

1. Compile a list of hadiths 
2. Serialize an index of the hadiths; e.g. `token → [id1, id25]`
3. Serialize a hashmap of the *idf* of each token; e.g. `token → 123`
4. Serialize a hashmap for the *term frequencies*; e.g. `hadith → [{token, tf}]`

```mermaid
flowchart TD

index_db[(Index)]
idf_db[(ID Freq.)]
tf_db[(Term Freq.)]

pp_s((Start)) --> hadiths
hadiths[/hadiths/] --> tokenizer

%% Index Database %%
tokenizer --> indexer
indexer --> index_db

%% IDF Database %%
tokenizer --> term_counter[term counter]
term_counter --> idf_db

%% TF Database %%
tokenizer --> tf_idf
index_db --- tf_idf
tf_idf[tf-idf] --> tf_db
```

### Querying

1. List all tokens in the query
2. Fuzzy search the index hashmap
3. Apply tf-idf
4. Return the result

```mermaid
flowchart LR
start((Start)) --> query[/query/]
query --> tokenizer
tokenizer --> fuzzy_search
index_db[(Index)] --- fuzzy_search
fuzzy_search ---> tf_idf[tf-idf]
tf_idf --> return([return])
```
