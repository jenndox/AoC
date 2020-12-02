Place Ruby unit tests here.

By default, this package is configured to use RSpec:
https://relishapp.com/rspec

You can run all tests with:

```bash
brazil-build spec
```

Run the tests in a specific file with:

```bash
brazil-build spec SPEC=spec/my_library_spec.rb
```

Or run one specific test with:

```bash
brazil-build spec SPEC_OPTS="-e 'MyLibrary#hello'"
```
