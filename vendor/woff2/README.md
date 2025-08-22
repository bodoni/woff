# woff2 Vendored

Original Repo: https://github.com/google/woff2  
Current Commit: `0f4d304faa1c62994536dc73510305c7357da8d4`

## To update and re-apply patch
```bash
cd vendor/woff2/
rm -rf source/
git clone https://github.com/google/woff2.git source
cd source
git checkout <NEW_COMMIT_HASH>
rm -rf .git
cd ..
patch -p1 < local-patches.patch
```
