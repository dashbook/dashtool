# Documentation

## Commands

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

## Configuration

Dashtool uses the `dashtool.json` file to store connection and authentication parameters for the current project.
The configuration file has two sections, one for the Iceberg catalog and one for the cloud provider.

### Catalog

The catalog section of the configuration file contains parameters related to the Iceberg catalog.
The field "catalog" defines which catalog to use.

| Field | Type | Description |
| --- | --- | --- |
| **catalog** | String | Name of the catalog. Can be: "sql" |

#### Sql catalog

| Field | Type | Description |
| --- | --- | --- |
| **catalogUrl** | String | Connection string to the database. Will substitute any variable $VAR with the according environment variable. For example: "postgres://username:$PASSWORD@host:5432/database" |
| **bucket** | String | The bucket to store the data. For example: "s3://bucket" |
| **secrets** | "{ String: { String: String }}"| A nested map that maps a kubernetes secret name to a map from a secret key to the environmant variable. Defines which secrets should be injected into the containers. For example : { "postgres-secret": { "password": "POSTGRES_PASSWORD" }, }|
| **env** | "{ String : String }"| A map from environment variable name to value. For example: { "POSTGRES_PASSWORD": "postgres" }|

### Object Storage

#### AWS S3

| Field | Type | Description |
| --- | --- | --- |
| **awsAccessKeyId** | String | AWS_ACCESS_KEY_ID |
| **awsSecretAccessKey** | String | AWS_SECRET_ACCESS_KEY |
| **awsRegion** | String | AWS_REGION |
| **awsEndpoint** | String | AWS_ENDPOINT |
| **awsAllowHttp** | String | Allow a http connection |

