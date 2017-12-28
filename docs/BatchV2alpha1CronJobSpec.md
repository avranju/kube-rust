# BatchV2alpha1CronJobSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**concurrency_policy** | **String** | Specifies how to treat concurrent executions of a Job. Defaults to Allow. | [optional] [default to null]
**failed_jobs_history_limit** | **i32** | The number of failed finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified. | [optional] [default to null]
**job_template** | [***::models::BatchV2alpha1JobTemplateSpec**](io.k8s.kubernetes.pkg.apis.batch.v2alpha1.JobTemplateSpec.md) | Specifies the job that will be created when executing a CronJob. | [default to null]
**schedule** | **String** | The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron. | [default to null]
**starting_deadline_seconds** | **i64** | Optional deadline in seconds for starting the job if it misses scheduled time for any reason.  Missed jobs executions will be counted as failed ones. | [optional] [default to null]
**successful_jobs_history_limit** | **i32** | The number of successful finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified. | [optional] [default to null]
**suspend** | **bool** | This flag tells the controller to suspend subsequent executions, it does not apply to already started executions.  Defaults to false. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


