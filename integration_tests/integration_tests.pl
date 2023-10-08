foreign_resource('integration_tests', [rust_main]).

foreign(rust_main, c, rust_main).

:- load_foreign_resource(integration_tests).
