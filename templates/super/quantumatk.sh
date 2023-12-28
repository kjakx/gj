#!/bin/sh
#PBS -l select={{ nodes }}
#PBS -l atk=1 -l atkdp={{ nodes * ppn - 1 }}
#PBS -N {{ job_name }}
#PBS -q {{ queue }}
{%- if walltime %}
#PBS -l walltime={{ walltime }}
{%- endif %}
{%- if mail_address and mail_flags %}
#PBS -m {{ mail_flags }}
#PBS -M {{ mail_address }}
{%- endif %}

module load ccm
{%- if use_workdir %}
DIRNAME=`basename $PBS_O_WORKDIR`
WORKDIR=/work/$USER/$PBS_JOBID
mkdir -p $WORKDIR
cp -raf  $PBS_O_WORKDIR $WORKDIR
cd $WORKDIR/$DIRNAME
{%- else %}
cd ${PBS_O_WORKDIR}
{%- endif %}

aprun -n {{ nodes * ppn }} -N {{ ppn }} hostname | grep -v ^Applicati > hostfile
{%- if app.version == "2022.12-SP1" %}
ccmrun {{ app.dir }}/mpi/bin/mpiexec.hydra -n {{ nodes * ppn }} -f ./hostfile -genv I_MPI_FABRICS=shm:tcp {{ app.bin }} {{ job_name }}.py > {{ job_name }}.out 2> {{ job_name }}.err
{%- else %}
ccmrun {{ app.dir }}/libexec/mpiexec.hydra -n {{ nodes * ppn }} -f ./hostfile -genv I_MPI_FABRICS=shm:tcp {{ app.bin }} {{ job_name }}.py > {{ job_name }}.out 2> {{ job_name }}.err
{%- endif %}

{%- if use_workdir %}
cd; if cp -raf $WORKDIR/$DIRNAME $PBS_O_WORKDIR/.. ; then rm -rf $WORKDIR; fi
{%- endif -%}
