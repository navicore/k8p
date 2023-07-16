A CLI for Inspecting Containers in Kubernetes
============

Currently, the command will use a local kubecontext to access a cluster
and get all the Prometheus data available from pods annotated for Prometheus
using the convention:

```
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/path: "/actuator/prometheus"
        prometheus.io/port: "8081"
```

The cli supports exporting the db to both N-Triple and Turtle RDF files.

Planned - formatted output and liveliness and readyness probe info
in aggregated report form.

Install
----------

```bash
#latest stable version via https://crates.io/crates/k8p
cargo install k8p

#or from this repo:
cargo install --path .
```

Usage
---------

from `k8p -h`

```
A cli tool for inspecting containers in Kubernetes

Usage: k8p [OPTIONS] <COMMAND>

Commands:
  scan-metrics
  export-triples
  export-turtle
  report
  help            Print this message or the help of the given subcommand(s)

Options:
  -t, --ttl-rdf-filename <TTL_RDF_FILENAME>  export Turtle RDF file [default: k8p.ttl]
  -r, --rdf-filename <RDF_FILENAME>          export N-Triples RDF file [default: k8p.nt]
  -n, --namespace <NAMESPACE>                Name of the namespace to walk
  -d, --db-location <DB_LOCATION>            [default: /tmp/k8p.db]
  -h, --help                                 Print help
  -V, --version                              Print version
```
