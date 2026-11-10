# Steam Packagepack Product Design

## Intent

The Loo Cast Steam app installs one logical runnable product: the Loo Cast
Packagepack. Steam remains responsible for acquiring and updating it, while
Vapor remains an independently installed runtime and authoring tool.

## Steam payload contract

- The common depot contains `Packagepack.vapor.toml` only.
- Each platform depot contains the Packagepack executable for that platform.
- A platform depot may also contain explicitly allowlisted runtime libraries
  required by that executable, such as Windows DLLs. These are runtime
  sidecars of the same product, not additional products.
- The Steam payload does not contain `App.vapor.toml`, a Vapor launcher or
  runtime, docs, tools, source checkouts, build caches, or writable state.
- `App-Source.vapor.toml` describes source-side depot policy and is never
  included in the depots.
- The Packagepack executable composes the Spacetime Engine and Loo Cast Game;
  Steam does not distribute separate executable products for them.

## Ownership and release behavior

Steam owns installation and update of the Packagepack payload. Vapor owns its
own installation, update, repair, and uninstall lifecycle. The Packagepack
Steam release uses Vapor's shared allowlist and review pipeline, but its
validation is product-specific: require the Packagepack manifest and one
executable per selected platform; accept only explicitly included runtime
sidecars; do not require Vapor's own `App.vapor.toml` or documentation bundle.

The release is not uploaded during local validation. The owner will verify the
Steam launch and update behavior on Windows and Linux before the first release.

## Approaches considered

1. Treat every Steam app as a Vapor runtime app. This reuses today's smoke
   checks but wrongly forces `App.vapor.toml`, Vapor docs, launcher binaries,
   and other runtime files into Loo Cast.
2. Publish the Packagepack outside Vapor's root release pipeline. This avoids
   runtime assumptions but duplicates target selection, allowlist validation,
   review fingerprints, and SteamPipe staging.
3. Declare the Steam product kind in `App-Source.vapor.toml` and keep the same
   release pipeline with product-specific validation. This preserves the
   existing Steam authorization and staging machinery while making payload
   requirements match the product. **Chosen.**

## Constraints

- Product kind is explicit and unknown values fail closed.
- The Packagepack manifest is the shared product identity; the source/depot
  manifest remains out of the payload.
- Every executable or runtime sidecar still comes from a required,
  manifest-allowlisted include.
- No compatibility path for the former three-product Workshop layout is added.
