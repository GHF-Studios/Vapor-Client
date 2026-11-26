# Active Development Policy — PRE-ALPHA

Applies across GHF-Studios development, especially Vapor, Spacetime Engine, and
Loo Cast. This is the current owner-provided operating policy and should be
consulted when recovering work after a context change.

**Primary objective:** Maximize meaningful development progress per unit of time and AI usage.

- Prefer decisive implementation over prolonged deliberation.
- Perform substantial, cohesive architectural refactors when justified.
- Commit and push frequently, using meaningful checkpoints.
- Deploy and test aggressively in authorized development environments.
- Do not introduce mandatory PRs, reviews, compatibility guarantees, or heavyweight process without a concrete benefit.
- Favor working end-to-end mechanisms over premature polish and formalization.
- Avoid repeatedly requesting approval for ordinary, authorized, reversible development actions.
- Preserve unrelated work, credentials, irreplaceable data and external resources.
- Use targeted validation appropriate to the actual risk of each operation.

**Operating principle:** Engineering rigor should scale with project maturity and the consequences of failure, not with arbitrary conventions.

**Phase transitions:** Explicit decisions, never automatically inferred from the version number alone.

**Pre-alpha release version:** Keep the compatibility target pinned at `0.5.0`.
Standalone releases use `0.5.0-pre-alpha-<inner SemVer>` to identify the phase
while the inner version records rolling product changes without moving the
compatibility target. The hosted default-branch workflow currently advances
the inner patch component on its `0.1` release train. Package, content, and
schema versions keep their own contracts.

**Next milestone:** Alpha — integrated, comprehensive tech demos of Vapor,
Spacetime Engine, and Loo Cast. Phase transitions are explicit owner decisions;
a version string alone does not change the phase.
