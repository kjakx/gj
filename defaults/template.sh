{%- if shell -%}
#!{{ shell }}
{% endif -%}
{%- if name -%} 
#PBS -N {{ name }}
{% endif -%}
{%- if queue -%}
#PBS -q {{ queue }}
{% endif -%}
{%- for resource in resources -%}
#PBS -l {{ resource }}
{% endfor -%}
{%- if email_address and email_flags -%}
#PBS -M {{ email_address }}
#PBS -m {{ email_flags }}
{% endif -%}

{% if cwd %}
cd ${PBS_O_WORKDIR}
{% endif -%}

{%- for command in commands %}
{{ command }}
{%- endfor -%}