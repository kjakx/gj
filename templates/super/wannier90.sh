#!/bin/sh
#PBS -N {{ job_name }}
#PBS -q {{ queue }}
#PBS -l select={{ nodes }}
{%- if walltime %}
#PBS -l walltime={{ walltime }}
{%- endif %}
{%- if mail_address and mail_flags %}
#PBS -m {{ mail_flags }}
#PBS -M {{ mail_address }}
{%- endif %}

{% if use_workdir -%}
DIRNAME=`basename $PBS_O_WORKDIR`
WORKDIR=/work/$USER/$PBS_JOBID
mkdir -p $WORKDIR
cp -raf  $PBS_O_WORKDIR $WORKDIR
cd $WORKDIR/$DIRNAME
{%- else %}
cd ${PBS_O_WORKDIR}
{%- endif %}

aprun -n {{ nodes * ppn }} -N {{ ppn }} -j 1 {{ app.bin }} {{ job_name }}.in > {{ job_name }}.out 2> {{ job_name }}.err

{%- if use_workdir %}
cd; if cp -raf $WORKDIR/$DIRNAME $PBS_O_WORKDIR/.. ; then rm -rf $WORKDIR; fi
{%- endif -%}
