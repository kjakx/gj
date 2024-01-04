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

module load intel
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:{{ app.dir }}/spglib/install_dir/lib
cd ${PBS_O_WORKDIR}

{%- else %}

module load intel
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:{{ app.dir }}/spglib/install_dir/lib
DIRNAME=`basename $PBS_O_WORKDIR`
WORKDIR=/work/$USER/$PBS_JOBID
mkdir -p $WORKDIR
cp -raf $PBS_O_WORKDIR $WORKDIR
cd $WORKDIR/$DIRNAME

{%- endif %}

export OMP_NUM_THREADS={{ run.threads }}
{%- if app.version == "1.1.0" %}
aprun -d {{ run.threads }} -j {{ run.tpc }} --cc depth {{ app.bin }} {{ run.input }} > {{ run.stdout }} 2> {{ run.stderr }}
{%- else %}
aprun -n {{ pbs.nodes * 36 / run.threads }} -d {{ run.threads }} -N {{ 36 / run.threads }} -j {{ run.tpc }} --cc depth {{ app.bin }} {{ run.input }} > {{ run.stdout }} 2> {{ run.stderr }}
{%- endif %}

{%- if not pbs.cwd %}

cd; if cp -raf $WORKDIR/$DIRNAME $PBS_O_WORKDIR/.. ; then rm -rf $WORKDIR; fi
{%- endif -%}
