#!/bin/sh
#PBS -l select=1
#PBS -N {{ name }}
#PBS -q {{ queue }}
{%- if walltime %}
#PBS -l walltime={{ walltime }}
{%- endif %}
{%- if mail_address and mail_flags %}
#PBS -m {{ mail_flags }}
#PBS -M {{ mail_address }}
{%- endif %}

source {{ config }}
{%- if use_workdir %}
DIRNAME=`basename $PBS_O_WORKDIR`
WORKDIR=/work/$USER/$PBS_JOBID
mkdir -p $WORKDIR
cp -raf  $PBS_O_WORKDIR $WORKDIR
cd $WORKDIR/$DIRNAME
{%- else %}
cd ${PBS_O_WORKDIR}
{%- endif %}

aprun -j 1 -d {{ ppn }} g16 > {{ name }}.out 2> {{ name }}.err

{%- if use_workdir %}
cd; if cp -raf $WORKDIR/$DIRNAME $PBS_O_WORKDIR/.. ; then rm -rf $WORKDIR; fi
{%- endif %}
