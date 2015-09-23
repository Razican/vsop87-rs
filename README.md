# VSOP87 Rust implementation #
[![Coverage Status](https://coveralls.io/repos/Razican/vsop87-rs/badge.svg?branch=develop&service=github)](https://coveralls.io/github/Razican/vsop87-rs?branch=develop)
[![Build Status](https://travis-ci.org/Razican/vsop87-rs.svg?branch=develop)](https://travis-ci.org/Razican/vsop87-rs)

This library implements the VSOP87 algorithm in Rust. The use can be seen in the
[documentation](http://razican.github.io/vsop87-rs). The library currently is divided in one module
per VSOP87 version implementation. The implemented ones are basic VSOP87 algorithm, VSOP87A,
VSOP87B, VSOP87C and VSOP87D. The VSOP87E version will be implemented in the future.

## What are the python files for? ##

Well, currently there are two python files in the repository. One would be *tests.py* and the other
one *data.py*. Those are for generating the data needed for the algorithm along with the tests. They
take the original VSOP87 files from the *data* directory and create the needed constants and tests
for the library. They will be removed once the library is stabilized. They try to create perfect
Rust code.
