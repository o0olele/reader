# Source fixtures

Fixtures are intentionally kept outside the application bundle. The rule
analyzer and selector regression suites can add source-specific HTML and JSON
under this directory as Step 2 progresses.

`bookSources.json` is the field-level import regression fixture copied from
the reference project's `defaultData`. It intentionally contains a source
with JSON rules, URL option objects, login JavaScript, and explore rules so
the importer is checked against the full legado record shape.
