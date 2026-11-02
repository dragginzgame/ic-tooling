# One admission rule for fresh discovery and saved-baseline recovery.
def completed: .status == "completed" and .conclusion == "success";

. as $baseline |
.schema == 1 and
(.source | type == "string" and test("^([0-9a-f]{40}|[0-9a-f]{64})$")) and
(.run | type == "number" and . > 0 and floor == .) and
(.jobs | type == "object" and (.jobs | type == "array") and
    .total_count == (.jobs | length)) and
all(["native (ubuntu-24.04, Linux, x86_64)",
    "native (macos-15-intel, Darwin, x86_64)",
    "native (macos-15, Darwin, arm64)"][];
    . as $name | [$baseline.jobs.jobs[] | select(.name == $name)] |
    length == 1 and all(.[];
        .run_id == $baseline.run and .head_sha == $baseline.source and completed and
        any(.steps[]; .name == "Run native CI and publication qualification" and .conclusion == "success") and
        any(.steps[]; .name == "Qualify focused release contracts" and .conclusion == "success") and
        any(.steps[]; .name == "Retain release-validation comparison" and .conclusion == "success") and
        any(.steps[]; .name == "Retain successful native qualification" and .conclusion == "success"))) and
any(.jobs.jobs[]; .name == "msrv" and completed)
