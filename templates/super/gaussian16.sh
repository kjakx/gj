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

source {{ app.config }}
cd ${PBS_O_WORKDIR}

{%- else %}

source {{ app.config }}
DIRNAME=`basename $PBS_O_WORKDIR`
WORKDIR=/work/$USER/$PBS_JOBID
mkdir -p $WORKDIR
cp -raf $PBS_O_WORKDIR $WORKDIR
cd $WORKDIR/$DIRNAME

{%- endif %}

aprun -j {{ run.tpc }} -d {{ run.threads }} g16 > {{ run.stdout }} 2> {{ run.stderr }}

{%- if not pbs.cwd %}

cd; if cp -raf $WORKDIR/$DIRNAME $PBS_O_WORKDIR/.. ; then rm -rf $WORKDIR; fi
{%- endif -%}
