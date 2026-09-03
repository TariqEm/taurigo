/** @type {import('@commitlint/types').UserConfig} */
module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    // Standard Conventional Commits type set — keeps history readable and leaves room to wire
    // up changelog/version-bump tooling (e.g. release-please) later without a history rewrite.
    'type-enum': [
      2,
      'always',
      ['feat', 'fix', 'docs', 'style', 'refactor', 'perf', 'test', 'build', 'ci', 'chore', 'revert'],
    ],
    // Scope is optional and free-form (desktop, sidecar, ui, types, db, deps, repo, ...) rather
    // than a fixed enum — this is a young monorepo and a hardcoded scope list would need
    // constant upkeep for no real benefit at this stage.
    'scope-empty': [0],
  },
};
