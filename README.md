# downforce

Downforce is MapReduce library implemented in rust. Mainly this project is for me to better learn rust, explore topics deeper into computer systems topics from OMSCS, and practice TDD. Currently I am implementing this to run on a single machine but would like to down the road make it able to run on multiple nodes.

### Configuration
Downforce uses a `yaml` config file for setting up the master and worker nodes. Below are the required fields:

* **workers_count:** Number of workers to run 

* **worker_addresses:** Port numbers for workers (all will be ran on `localhost`)

* **shard_size:** Size in bytes for shards

* **input_dir:** Where the input files to be processed are stored

* **output_dir:** Where output files and temp files will be written to

* **output_file_count:** Number of expected 

For an example `config` see [test_config.yaml](https://github.com/smortime/downforce/blob/master/tests/test_config.yaml)