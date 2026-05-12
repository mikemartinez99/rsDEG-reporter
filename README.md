# rsDEG-reporter

Basic DEG filtering tool in Rust for practice...

Takes a `DESeq2` output file and outputs filtered differentially expressed genes with a column that tells if genes are up or downregulated in your reference level. 

## Build binary

```shell
cargo build
```

## Usage

```shell
./deg_reporter \
  --input-csv DESeq2_Results.csv \
  --output-csv Filtered_results.csv \
  --padj-thresh 0.05 \
  --log2fc-thresh 1 \
  --numerator Tumor \
  --denominator Normal
```



