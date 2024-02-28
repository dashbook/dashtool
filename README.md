# Dashtool

Dashtool is a Lakehouse build tool that builds Iceberg tables from declarative SQL statements and generates Kubernetes workflows to keep these tables up-to-date.
It handles Ingestion, Transformation and Orchestration.

## Features

- Uses declarative SQL select statements as input
- git inspired data version control
- Interoperability through [Apache Iceberg](https://iceberg.apache.org/) Table format
- Data ingestion through [Singer.io](https://www.singer.io/)
- Data processing based on [Datafusion](https://arrow.apache.org/datafusion/)
- Workflow orchestration in Kubernetes through [Argo Workflows](https://argoproj.github.io/workflows/)

## How it works

![dashtool](dashtool.svg)

Dashtool constructs a DAG by analyzing all `.sql` files in a directory structure and creates an Iceberg Materialized View for every file.
Each file contains a `SELECT` statement for the Materialized View definition.
Additionally, dashtool can use the DAG to create an Argo Workflow that refreshs the Materialized Views.
During the workflow execution Argo starts Docker containers that run Datafusion to perform the refresh operation.

## Examples

- [Postgres example](https://github.com/dashbook/dashtool-postgres-example)

## Usage

Check out the [Documentation](Documentation) for a detailed description.

### Build

The `build` command analyzes all `.sql` files in the subdirectories of the current directory and creates the corresponding Iceberg Materialized Views in the catalog.

```shell
dashtool build
```

### Create Workflow

The `workflow` command creates a lineage DAG based on the `.sql` files and constructs an Argo workflow based on it. It stores the Workflow configuration file in `argo/workflow.yaml`.

```shell
dashtool workflow
```

### Apply Workflow to the Kubernetes cluster

To apply the latest version of the workflow to the Kubernetes cluster run the following command:

```shell
kubectl apply -f argo/workflow.yaml
```

## Installation

### Homebrew

### Cargo
