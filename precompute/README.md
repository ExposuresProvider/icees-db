# ICEES correlation pre-computation

### Need:

* `features.yml`
* CSV data

### Example:
```bash
cd precomp
cat ../test_data.csv | cargo run --release -- --features ../config/features.yml > pvals.csv
```
