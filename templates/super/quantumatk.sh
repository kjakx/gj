#!/bin/sh
#PBS -q {{ pbs.queue }}
#PBS -N {{ pbs.jobname }}
#PBS -l select={{ pbs.nodes }}
{%- if pbs.ncpus -%}
:ncpus={{ pbs.ncpus }}
{%- endif -%}
{%- if pbs.ngpus -%}
:ngpus={{ pbs.ngpus }}
{%- endif -%}
{%- if pbs.walltime %}
#PBS -l walltime={{ pbs.walltime }}
{%- endif %}
{%- if pbs.mail_address and pbs.mail_flags %}
#PBS -m {{ pbs.mail_flags }}
#PBS -M {{ pbs.mail_address }}
{%- endif %}

{%- if pbs.cwd %}

module load ccm
cd ${PBS_O_WORKDIR}

{%- else %}

module load ccm
DIRNAME=`basename $PBS_O_WORKDIR`
WORKDIR=/work/$USER/$PBS_JOBID
mkdir -p $WORKDIR
cp -raf $PBS_O_WORKDIR $WORKDIR
cd $WORKDIR/$DIRNAME

{%- endif %}

aprun -n {{ run.nprocs }} -N {{ run.ppn }} hostname | grep -v ^Applicati > hostfile
{%- if app.version == "2022.12-SP1" %}
ccmrun {{ app.dir }}/mpi/bin/mpiexec.hydra -n {{ run.nprocs }} -f ./hostfile -genv I_MPI_FABRICS=shm:tcp {{ app.bin }} {{ pbs.jobname }}.py > {{ run.stdout }} 2> {{ run.stderr }}
{%- else %}
ccmrun {{ app.dir }}/libexec/mpiexec.hydra -n {{ run.nprocs }} -f ./hostfile -genv I_MPI_FABRICS=shm:tcp {{ app.bin }} {{ pbs.jobname }}.py > {{ run.stdout }} 2> {{ run.stderr }}
{%- endif %}

{%- if not pbs.cwd %}

cd; if cp -raf $WORKDIR/$DIRNAME $PBS_O_WORKDIR/.. ; then rm -rf $WORKDIR; fi
{%- endif -%}
