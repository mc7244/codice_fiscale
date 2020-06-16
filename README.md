# codice_fiscale

This crate provides tools to manage the Italian *codice fiscale*, which
(for anyone who doesn't live in Italy) is a code associated to every
individual which helps with identification in public services.

We currently provide codice fiscale calculation and check.

For anyone interested, here's an explanation (Italian language) on how
to calculate the codice fiscale:
https://it.wikipedia.org/wiki/Codice_fiscale#Generazione_del_codice_fiscale

The crate will fail in case of omocody (i.e. fiscal code anti collision)
because it won't find the place corresponding to the Belfiore code

License: MIT
