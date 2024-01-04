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

cd ${PBS_O_WORKDIR}

{%- else %}

DIRNAME=`basename $PBS_O_WORKDIR`
WORKDIR=/work/$USER/$PBS_JOBID
mkdir -p $WORKDIR
cp -raf $PBS_O_WORKDIR $WORKDIR
cd $WORKDIR/$DIRNAME

{%- endif %}

module load intel
export OMP_NUM_THREADS={{ run.threads }}
aprun -n {{ pbs.nodes * 36 / run.threads }} -d {{ run.threads }} -j {{ run.tpc }} --cc depth {{ app.bin }} > {{ run.stdout }} 2> {{ run.stderr }}

{%- if not pbs.cwd %}

cd; if cp -raf $WORKDIR/$DIRNAME $PBS_O_WORKDIR/.. ; then rm -rf $WORKDIR; fi
{%- endif -%}
