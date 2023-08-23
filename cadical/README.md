# rustsat-cadical - Interface to the CaDiCaL SAT Solver for RustSAT

Armin Biere's SAT solver [CaDiCaL](https://github.com/arminbiere/cadical) be used with the [RustSAT](https://github.com/chrjabs/rustsat) library.

## CaDiCaL Versions

CaDiCaL versions can be selected via cargo crate features.
The following CaDiCaL versions are available:
- `v1-7-0`: [Version 1.7.0](https://github.com/arminbiere/cadical/releases/tag/rel-1.7.0)
- `v1-6-0`: [Version 1.6.0](https://github.com/arminbiere/cadical/releases/tag/rel-1.6.0)
- `v1-5-6`: [Version 1.5.6](https://github.com/arminbiere/cadical/releases/tag/rel-1.5.6)
- `v1-5-5`: [Version 1.5.5](https://github.com/arminbiere/cadical/releases/tag/rel-1.5.5)
- `v1-5-4`: [Version 1.5.4](https://github.com/arminbiere/cadical/releases/tag/rel-1.5.4)
- `v1-5-3`: [Version 1.5.3](https://github.com/arminbiere/cadical/releases/tag/rel-1.5.3)
- `v1-5-2`: [Version 1.5.2](https://github.com/arminbiere/cadical/releases/tag/rel-1.5.2)
- `v1-5-1`: [Version 1.5.1](https://github.com/arminbiere/cadical/releases/tag/rel-1.5.1)
- `v1-5-0`: [Version 1.5.0](https://github.com/arminbiere/cadical/releases/tag/rel-1.5.0)

Without any features selected, the newest version will be used.
If conflicting CaDiCaL versions are requested, the newest requested version will be selected.