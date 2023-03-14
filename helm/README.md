
## Helm Chart

This chart provides a way to deploy the service to a Kubernetes environment.

To deploy to the default namespace, you can use a command similar to the following:

```bash
helm install log-scraper ./helm --namespace default \
  --set service.newRelicAccountId='1234567' \
  --set service.newRelicApiKey='<my-api-key>' \
  --set service.logExtension='log' \
  --set service.logDirectory=/usr/src/app/logs \
  --set service.pollSchedule='0 1/5 * * * *' \
  --set service.redisURL='redis-release-master.default:6379' \
  --set service.redisPassword='somePassword' \
  --set service.port=3333 \
  --set service.logPrefix=my-app-logs \
  --set service.redisKeyName=last_seen_timestamp

# NAME: log-scraper
# LAST DEPLOYED: Sun Mar 12 19:07:24 2023
# NAMESPACE: default 
# STATUS: deployed
# REVISION: 1
# NOTES:
# 1. Get the application URL by running these commands:
#   http://log-scraper.local/
```

You can show the release with the `helm ls` command:

```bash
helm list

# NAME         	NAMESPACE	REVISION	UPDATED                             	STATUS  	CHART            	APP VERSION
# log-scraper  	default 	1       	2023-03-12 19:07:24.542044 -0500 CDT	deployed	log-scraper-0.1.1	0.3.0
```

Or teardown the release completely with the delete command:

```bash
helm delete log-scraper

# release "log-scraper" uninstalled

helm list
# NAME         	NAMESPACE	REVISION	UPDATED                             	STATUS  	CHART            	APP VERSION

```


### Upgrading

You can run the upgrade command to apply new updates or change values to configuration settings.

Note: the `--values` param is optional and you can also specify the values via `--set` instead. One thing
to keep in mind is that you need to specify your overrides explicitly every time you upgrade as well as at the
time the app was first deployed.

```bash
helm upgrade log-scraper ./helm \
  --values /path/to/my/custom/values.yaml \
  --namespace <my-namespace>
```

### List Release Versions (Revisions)

```bash
# show past releases
helm history log-scraper --namespace <my-namespace>
```

### Rollback

To revert a bad deployment (undo a bad upgrade).

```bash
# revert to last release
helm rollback log-scraper --namespace <my-namespace>

# revert to specific release
helm rollback log-scraper <RELEASE> [REVISION] --namespace <my-namespace>
```


### Teardown

Deletes the resources deployed from the helm chart.

```bash
# delete redis release
helm delete redis-release

# delete log-scraper release
helm delete log-scraper --namespace <my-namespace>
```
