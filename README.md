# gj
gj is a job script generator with configuration and template system.

This project is under development and currently available only on linux cluster and PBS job scheduler with limited options.

## Installation
Install [Rust](https://rust-lang.org/ja/tools/install/), clone this repo and run `cargo install --path .`.

## Usage
Run `gj` to generate job script:
```
$ gj --shell /bin/sh --queue workq > job.sh
$ cat job.sh
#!/bin/sh
#PBS -q workq

cd ${PBS_O_WORKDIR}

```

### Options
You can pass some options to `gj` command:
| option  				| description 		| value			| default	|
| :---:					| :---:				| :---:			| :-------:	|
| `--shell`				| path to the shell | path			|			|
| `--name`, `-N`		| job name 			| text			|			|
| `--queue`, `-q`		| queue name 		| text			|			|
| `--resources`, `-l`	| resources list (multiple) | list of "key=value" text | 			|
| `--email-address`, `-M`| email address 	| text 			| 			|
| `--email-flags`, `-m`	| email flags 		| a \| b \| e 	| 			|
| `--cwd` 				| go to current directory or not | bool | true	|
| `--commands` 			| commands list (multiple)| list of text | 		|
| `--template-path` 	| path to the template file 	| path | 		|

### example
```
$ gj --shell /bin/sh -N test -q cpu_112 \
  -l select=1:ncpus=112:mem=230gb -l walltime=24:00:00 \
  --commands "module load openmpi" \
  --commands "mpirun -np 112 ./a.out > stdout 2> stderr"
#!/bin/sh
#PBS -N test
#PBS -q cpu_112
#PBS -l select=1:ncpus=112:mem=230gb
#PBS -l walltime=24:00:00

cd ${PBS_O_WORKDIR}

module load openmpi
mpirun -np 112 ./a.out > stdout 2> stderr
```

## Config file
You can set profiles of your scripts in `~/.config/gj/config.toml`.
It will be automatically created at the first run. 

The root profile, which is on the head of the file, is applied globally.
You can write named profiles which are sectioned by `[profile_name]`. 
If you run gj with a profile name (like `gj gpu8` in the example below), the profile is read.

If no profile name is given, the `[default]` section is applied if it exists.

### example
An example of config file:
```
# The following lines are the root profile.
shell = "/bin/bash"
email_address = "example@example.com"
email_flags = "abe"

# named profile
[gpu8]
queue = "gpu_8slots"
resources = [
	"select=1:ncpus=8:ngpus=8",
	"walltime=08:00:00"
]
commands = [
	"module load nvhpc",
	"mpirun -np 8 ./a.out > stdout 2> stderr"
]

# default profile which is applied when no profile name is specified
[default]
queue = "cpu_112cores"
resources = [
	"select=1:ncpus=112:mem=230gb",
	"walltime=24:00:00"
]
commands = [
	"module load openmpi",
	"mpirun -np 112 ./a.out > stdout 2> stderr"
]
```

If you run `gj` with a profile name, the profile is applied like below:
```
$ gj gpu8
#!/bin/bash
#PBS -q gpu_8slots
#PBS -M example@example.com
#PBS -m abe

cd ${PBS_O_WORKDIR}

module load nvhpc
mpirun -np 8 ./a.out > stdout 2> stderr
```

If no profile name is given, the default profile is applied:
```
$ gj
#!/bin/bash
#PBS -q cpu_112cores
#PBS -M example@example.com
#PBS -m abe

cd ${PBS_O_WORKDIR}

module load openmpi
mpirun -np 112 ./a.out > stdout 2> stderr
```

### Options
The available options in the config file are:
| option  			| description 			| value			| default	|
| :---:				| :---:					| :---:			| :-------:	|
| `shell`			| path to the shell 	| path			|			|
| `name`			| job name 				| text			|			|
| `queue`			| queue name 			| text			|			|
| `resources`		| resource list 		| `[ "key1=value1", "key2=value2", ... ]` | `[]` |
| `email_address` 	| email address 		| text 			| 			|
| `email_flags`		| email flags 			| a \| b \| e | |
| `cwd`				| move current directory or not | bool | `true` |
| `commands` 		| commands to run in the script | `[ "command1", "command2", ... ]` | `[]` |
| `template_path` 	| path to the template file 	| path |  |

## Template
You can use your own template by passing the path to `template-path` command line option or `template_path` config option.
If no template path is specified, the default template (`defaults/template.sh`) is used.

**[Tera](https://github.com/Keats/tera)** is used as the template engine. Please see Tera's manual for the detail of its notation.

## Future List (someday)
 * Add more options
 * Support other job schedulers (Slurm, Grid Engine, etc.)
 * Support other template engine (Mustache looks good)
