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

module load intel
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:{{ app.dir }}/spglib/install_dir/lib
{% if use_workdir -%}
DIRNAME=`basename $PBS_O_WORKDIR`
WORKDIR=/work/$USER/$PBS_JOBID
mkdir -p $WORKDIR
cp -raf  $PBS_O_WORKDIR $WORKDIR
cd $WORKDIR/$DIRNAME
{%- else %}
cd ${PBS_O_WORKDIR}
{%- endif %}

export OMP_NUM_THREADS={{ ppn }}
{%- if app.version == "1.1.0" %}
aprun -d {{ ppn }} -j 1 --cc depth {{ app.bin }} {{ job_name }}.in > {{ job_name }}.out 2> {{ job_name }}.err
{%- else %}
aprun -n 1 -d {{ ppn }} -N 1 -j 1 --cc depth {{ app.bin }} {{ job_name }}.in > {{ job_name }}.out 2> {{ job_name }}.err
{%- endif %}

{%- if use_workdir %}
cd; if cp -raf $WORKDIR/$DIRNAME $PBS_O_WORKDIR/.. ; then rm -rf $WORKDIR; fi
{%- endif -%}
