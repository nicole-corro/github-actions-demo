# Branch Protection Setup

How to enforce that PRs cannot merge unless the pipeline succeeds.

## GitHub Settings

Go to **Settings > Branches > Add branch protection rule** for `main`:

### Required settings

| Setting | Value |
|---|---|
| Branch name pattern | `main` |
| Require a pull request before merging | Enabled |
| Require status checks to pass before merging | Enabled |
| Require branches to be up to date before merging | Enabled (optional) |
| Status checks that are required | `CI Gate` and `PR Gate` |

### How required status checks work

GitHub evaluates the **conclusion** of the check run. Only
`success` allows the PR to merge. All other states block it:

| State | Merge allowed? |
|---|---|
| `success` | Yes |
| `failure` | No |
| `cancelled` | No |
| `timed_out` | No |
| `action_required` | No |
| `in_progress` / queued | No (not yet concluded) |

This means:

- If someone cancels a running pipeline, the PR stays blocked.
- If a job times out, the PR stays blocked.
- The only way to unblock is a new successful run.

### Gate pattern

The workflows use a "gate job" pattern. Instead of requiring every
individual job as a status check (which breaks when you rename or
add jobs), a single `CI Gate` / `PR Gate` job aggregates results:

```yaml
ci-gate:
  if: always()           # run even if upstream jobs fail
  needs: [fmt, clippy, test, coverage]
  steps:
    - run: |
        # fail if any required job didn't succeed
        if [[ "$FMT_RESULT" != "success" || ... ]]; then
          exit 1
        fi
```

You only need to add `CI Gate` and `PR Gate` as required checks.

## Bitbucket comparison

### Pipeline triggers and control flow

| Feature | GitHub | Bitbucket |
|---|---|---|
| Required status checks | Per-check granularity | All or nothing |
| Concurrency control | Built-in `concurrency` key | Not available |
| Cancel in-progress runs | `cancel-in-progress: true` | `auto-cancel` (limited) |
| Scheduled runs | Cron syntax, multiple schedules | Scheduled pipelines (limited) |
| Manual triggers with inputs | `workflow_dispatch` with typed inputs (bool, choice, environment) | Manual triggers, no typed inputs |
| Draft PR detection | `pull_request.draft` field, skip jobs automatically | Not available |
| PR event type filtering | `types: [opened, synchronize, ready_for_review]` | Not available |
| Path filters on triggers | `paths` / `paths-ignore` per workflow | `changesets` with `includePaths` (step-level only) |
| Per-job timeouts | `timeout-minutes` per job | Global 120 min only |
| Conditional job execution | `if:` expressions on any job | Limited to step-level conditions |

### Build and test

| Feature | GitHub | Bitbucket |
|---|---|---|
| Matrix builds | Native `strategy.matrix` with includes/excludes | Not available (must duplicate steps) |
| Feature-flag testing | Matrix over feature combos | Hardcoded per-step |
| Service containers | `services:` with health checks, port mapping | `services:` without health checks |
| Multi-platform Docker | Buildx builds ARM + x86 in one step | Manual architecture switching per step |
| Dependency caching | `actions/cache` with hash-based keys | Bitbucket Caches (basic, no hash keys) |
| Docker layer caching | `cache-from: type=gha` (native) | Must disable BuildKit (`DOCKER_BUILDKIT=0`) |

### Artifacts and publishing

| Feature | GitHub | Bitbucket |
|---|---|---|
| Artifact passing | `upload-artifact` / `download-artifact` across jobs | `artifacts:` within same pipeline only |
| Artifact retention | Configurable per-artifact `retention-days` | Fixed retention policy |
| Build-once pattern | Build in job A, download in job B (no rebuild) | Must rebuild in every step |
| Multi-crate publish | Matrix with `max-parallel: 1` for dependency order | Duplicated steps per crate |
| Dry-run on PR | `if: github.event_name == 'pull_request'` | Separate pipeline definition |

### Security and permissions

| Feature | GitHub | Bitbucket |
|---|---|---|
| OIDC to AWS | First-class `id-token: write` + action | Custom OIDC scripts (duplicated across repos) |
| Token scoping | `permissions:` block per workflow/job | Not available |
| Environment approvals | Native environment protection rules with reviewers | Not available |
| Secret scoping | Per-environment, per-repo, per-org secrets | Workspace/repo variables only |

### Reusability

| Feature | GitHub | Bitbucket |
|---|---|---|
| Shared pipelines | `workflow_call` — call workflows across repos | Not available |
| Reusable steps | Composite actions, reusable workflows | YAML anchors within same file only |
| Marketplace actions | 20,000+ community actions | Bitbucket Pipes (limited selection) |
| Custom actions | Write your own (JS, Docker, composite) | Pipes only (Docker-based) |

### Pain points this demo solves

These are real issues observed in the current Bitbucket pipelines:

| Problem (Bitbucket) | Solution (GitHub) | Affected repos |
|---|---|---|
| OIDC setup scripts duplicated across repos | `aws-actions/configure-aws-credentials` action | API repos |
| Feature flag combos hardcoded per step | `strategy.matrix.features` | Library repos |
| Multi-crate publish steps duplicated | Matrix with `max-parallel: 1` | Multi-crate repos |
| Architecture switching (ARM/x86) per step | Multi-platform Buildx builds | Docker image and infra repos |
| No cancel-in-progress for PRs | `concurrency.cancel-in-progress` | Most repos |
| Rebuilding in every step | Artifact passing between jobs | API repos |
| No per-job timeout control | `timeout-minutes` per job | All repos (global 120 min) |
| Inconsistent artifact storage across repos | Unified `upload-artifact` + environment-based publish | API repos |
| Manual approval as blocking step | Environment protection rules with reviewers | Infra repos |
