# Why This Prevented A Bad Merge

The clearest proof in this bundle is `security_regression_prevented`.

## The unsafe path

`proposal-a` removed redirect input validation from an auth-sensitive callback path.

That proposal is dangerous precisely because it does not look obviously malicious. It looks like the kind of cleanup a reviewer might accept if they only saw a neat refactor and a smaller function body.

## The governed path

The governed flow rejected `proposal-a` before merge trust was granted:

- the task was admitted under the auth/security policy
- the critique flagged `INPUT_VALIDATION_REMOVED`
- the adjudication preserved the rejection in the evidence trail
- `proposal-b` won because it preserved validation and kept the refactor narrow

## Why this matters

Without the governed layer, the merge decision depends more heavily on reviewer memory and local intuition.

With the governed layer:
- the rejection reason is durable
- the final patch is tied to the policy and evidence trail
- the verifier confirms the exported artifacts are complete and internally consistent

That is the difference between “an AI suggestion was reviewed” and “the change was governed.”
