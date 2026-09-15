---
name: Release
about: Checklist for cutting a new version
title: 'chore(release): prepare and cut vX.Y.Z'
labels: chore
assignees: ''
---

### Context
<!-- Why are we cutting this release? (e.g., Reached a milestone, hotfix, initial launch). -->

<!-- (Optional) Use the block below for visual context: -->
<!--
<details>
  <summary>Expand for visual context</summary>

  [Paste media or logs here]

</details>
-->

### Objective
Prepare the repository for the `vX.Y.Z` release.

### Release Checklist
- [ ] Migrate `[Unreleased]` changes to `X.Y.Z` in `CHANGELOG.md`.
- [ ] Bump version to `vX.Y.Z` where applicable.
- [ ] Create git tag `vX.Y.Z`.
- [ ] Publish the GitHub release.

### Acceptance Criteria
- [ ] Release PR merged into `main`.
- [ ] Git tag `vX.Y.Z` pushed to remote.
- [ ] GitHub release published.
