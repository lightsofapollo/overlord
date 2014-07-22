A primary design goal is to allow "nested" overlord configurations so
very large projects can have their own isolated suites _or_ add
additional files to the same "suite" that is defined as a higher level.


Explict manifest loading:

```toml
# Sub manifests can conform to whatever conventiton makes you happy.
manifests = ["relative_path/to_another/manifest.toml"]
```

There are two primary use cases for manifest paths:

  - mapping individual files to suites.
  - running entire suites.

Both cases must be correctly optimized but primarly mapping individual
files to suites.

## Algorithms for matching paths to suites
