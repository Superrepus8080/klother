"""
CocoIndex flow — semantic code search index for the Klother project.

Indexes all source files (Kotlin, Rust, JTE templates, SQL, TOML, Gradle, etc.)
using sentence-transformers embeddings stored in PostgreSQL (pgvector).

Usage:
  cd /home/user/klother
  cocoindex setup  cocoindex_flow.py   # create/migrate DB schema (run once)
  cocoindex update cocoindex_flow.py   # index / re-index all files
  cocoindex server cocoindex_flow.py   # start REST query server on :8000
"""

import cocoindex

# ── Global settings (DB connection) ──────────────────────────────────────────

cocoindex.init(
    cocoindex.Settings(
        database=cocoindex.setting.DatabaseConnectionSpec(
            url="postgresql://127.0.0.1:5432/klother",
            user="klother",
            password="klother",
        )
    )
)

# ── Flow definition ───────────────────────────────────────────────────────────

@cocoindex.flow_def(name="KlotherCodeIndex")
def klother_code_index(
    flow_builder: cocoindex.FlowBuilder,
    data_scope: cocoindex.DataScope,
):
    """
    Read every source file → split into chunks → embed → store in Postgres.

    Splitting uses blank-line boundaries so function/class bodies stay together.
    Embedding uses all-MiniLM-L6-v2 (384-dim, runs locally, no API key needed).
    """

    # 1. Source: all relevant source files in the repo
    data_scope["files"] = flow_builder.add_source(
        cocoindex.sources.LocalFile(
            path="/home/user/klother",
            included_patterns=[
                "**/*.kt",
                "**/*.rs",
                "**/*.jte",
                "**/*.sql",
                "**/*.toml",
                "**/*.gradle",
                "**/*.kts",
                "**/*.yaml",
                "**/*.yml",
                "**/*.py",
                "**/*.ts",
                "**/*.js",
                "**/*.html",
                "**/*.css",
            ],
            excluded_patterns=[
                "**/target/**",
                "**/build/**",
                "**/.gradle/**",
                "**/node_modules/**",
                "**/output.css",
                "**/*.lock",
                "**/package-lock.json",
            ],
            max_file_size=512 * 1024,  # 512 KB cap
        )
    )

    # 2. Collector: accumulates (filename, location, text, embedding) rows
    collector = data_scope.add_collector()

    # 3. For each file: split → embed → collect
    with data_scope["files"].row() as file:
        # Split on blank lines — keeps function/class bodies together.
        # keep_separator=None means don't attach the separator to either side.
        file["chunks"] = file["content"].transform(
            cocoindex.functions.SplitBySeparators(
                separators_regex=[r"\n\n+"],
                keep_separator=None,
            )
        )

        with file["chunks"].row() as chunk:
            # Embed with a lightweight local model (no API key required)
            chunk["embedding"] = chunk["text"].transform(
                cocoindex.functions.SentenceTransformerEmbed(
                    model="sentence-transformers/all-MiniLM-L6-v2"
                )
            )

            collector.collect(
                filename=file["filename"],
                location=chunk["location"],
                text=chunk["text"],
                embedding=chunk["embedding"],
            )

    # 4. Export → Postgres table with HNSW cosine vector index
    collector.export(
        "klother_code_chunks",
        cocoindex.targets.Postgres(),
        primary_key_fields=["filename", "location"],
        vector_indexes=[
            cocoindex.index.VectorIndexDef(
                field_name="embedding",
                metric=cocoindex.index.VectorSimilarityMetric.COSINE,
            )
        ],
    )


if __name__ == "__main__":
    cocoindex.cli.main()
