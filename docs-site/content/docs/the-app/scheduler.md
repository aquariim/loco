+++
title = "Scheduler"
description = ""
date = 2024-11-09T18:10:00+00:00
updated = 2024-11-09T18:10:00+00:00
draft = false
weight = 17
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ""
toc = true
top = false
flair =[]
+++


Loco simplifies the traditional, often cumbersome `crontab` system, making it easier and more elegant to schedule cron jobs. The scheduler job can execute either a shell script command or run a registered [task](./task.md).


## Setting Up
Scheduler jobs can be configured via a dedicated YAML file or as part of an environment YAML file.


### 1. Dedicated File
Using a dedicated file provides a centralized place to configure all your scheduler jobs, making it easier to manage and maintain. You can start by generating a template file using the Loco generator command:

```sh
cargo loco generate scheduler
```

This command creates a `scheduler.yaml` file under the `config` folder. You can then configure your jobs within this file.

### 2. Environment Configuration File
You can also configure scheduler jobs per environment by adding the scheduler section to your environment's YAML configuration file:
```yaml
jobs:
  add text:
    run: "echo loco >> ./scheduler.txt"
    shell: true
    cron: "*/1 * * * * *"
    tags:
        - base
        - infra

  Run command:
    run: "foo path:/tmp/scheduler.txt"
    cron: "*/5 * * * * *"

  list if users:
    run: "user_report"
    cron: "*/7 * * * * *"
    tags:
        - base
        - users

```

## Scheduler Configuration

The scheduler configuration consists of the following elements:

* `scheduler.output` (Optional): Sets the default output location for all jobs.
    * `stdout:` Output to the console (default).
    * `silent:` Suppress all output.
* `scheduler.jobs:` A object of jobs to be scheduled, the object key describe the job name. Each job has:
    * `cron`: The cron expression that defines the job’s schedule.
        The cron get an english that convert to cron syntax or cron syntax itself. 

        ##### ***English to cron***
        Examples:
        * Run every 15 seconds                                     
        * Run every 15 sec                                         
        * Every 15 seconds, only on Friday                         
        * Run every 3 hours                                        
        * Run at 10:00 am every day                                
        * Run at 6:00 pm every Monday through Friday               
        * Run every 10 minutes Monday through Friday               
        * 7pm every Thursday                                       
        * Midnight on Tuesdays                                     
        * Every day at 17:25                                       
        * Every 3rd day at 2:55 am from January to August in 2019 and 2020 


        ##### ***Cron Syntax format:***
        ```sh
        sec   min   hour   day of month   month   day of week   year
        *     *     *      *              *       *             *
        ```
    * `run`: Cronjob command to run. 
        * `Task:` The task name (with variables e.x `seed refresh:true` ). Note that the `shell` field should be false.
        * `Shell`: Run a shell command (e.x `"echo loco >> ./scheduler.txt"`). Note that the `shell` field should be true.
    * `shell`: by default `false` meaning executing the the `run` value as a task. if `true` execute the `run` value as shell command
    * `tags` (Optional): A list of tags to categorize and manage the job.
    * `output` (Optional): Overrides the global `scheduler.output` for this job.


## Verifying the Configuration
After setting up your jobs, you can verify the configuration to ensure everything is correct.

### 1. When using a dedicated file:
Run the following command to list the jobs from your scheduler file:
<!-- <snip id="scheduler-list-from-file-command" inject_from="yaml"  template="sh"> -->
```sh
cargo loco scheduler --path config/scheduler.yaml --list
```
<!-- </snip> -->

### 2. When using environment-based configuration:
To list jobs from the environment configuration, run:
<!-- <snip id="scheduler-list-from-env-setting-command" inject_from="yaml"  template="sh"> -->
```sh
LOCO_ENV=production cargo loco scheduler --list
```
<!-- </snip> -->


## Running the Scheduler
Once the configuration is verified, you can remove the `--list` flag to start running the scheduler. The scheduler will continuously execute jobs based on their schedule until a shutdown signal is received. When a signal is received, it gracefully terminates all running tasks and shuts down safely.

### Important Notes:
* When a job is running, `Loco` spawns it in a new process, and all environment variables will propagate to the new job process.
* For tasks, ensure you run the scheduler with a valid environment by using the `--environment` flag or setting the `LOCO_ENV` environment variable. This ensures the correct environment and configuration are loaded for the task.
* You can pass variables to tasks by using the vars object in the task configuration.


## Running a Single Scheduled Job by Name
To run a specific scheduler job by its name, use the --name flag. This will execute a single job with the provided name.
<!-- <snip id="scheduler-run-job-by-name-command" inject_from="yaml"  template="sh"> -->
```sh
LOCO_ENV=production cargo loco scheduler --name 'JOB_NAME'
```
<!-- </snip> -->

This command will locate the job named `"Run command"` in your scheduler.yaml file and run it.

## Running Scheduled Jobs by Tag
You can also run multiple jobs that share the same tag. Tags are useful for grouping related jobs together. For example, you might have several jobs that perform different types of maintenance tasks—such as database cleanup, cache invalidation, and log rotation—that you want to run together. Assigning them the same tag, like `maintenance`, allows you to execute them all at once.
<!-- <snip id="scheduler-run-job-by-tag-command" inject_from="yaml"  template="sh"> -->
```sh
LOCO_ENV=production cargo loco scheduler --tag 'TAG'
```
<!-- </snip> -->


This command runs all jobs that have been tagged with `maintenance`, ensuring that all related jobs are executed in one go.


