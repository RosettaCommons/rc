# rc

A command line tool to run various biomolecular structural prediction, analysis and design applications using containerized environments.

## Overview

`rc` provides a unified interface for running Rosetta and other biomolecular modeling applications through container engines like Docker, Singularity, or Apptainer.

## Installation

```bash
cargo install --path .
```

## Basic Usage

### Running Rosetta Score

Score a PDB structure file using Rosetta:

```bash
rc run -w /path/to/working/directory rosetta score \
    -out:file:scorefile output.sc \
    -in:file:s structure.pdb
```

This command will:
- Use the default Docker container engine
- Mount the working directory into the container
- Run the Rosetta score application
- Output the score file to `output.sc`
- Log the executed command to a log file for reproducibility

### Specifying a Container Engine

You can specify which container engine to use with the `-e` flag:

```bash
rc run -e singularity rosetta score -in:file:s structure.pdb
```

Supported container engines:
- `docker` (default)
- `singularity`
- `apptainer`
- `none` (run natively without containers)

### Working Directory

The `-w` flag specifies the working directory that will be mounted into the container:

```bash
rc run -w ./data rosetta score -in:file:s input.pdb
```

If not specified, the current directory (`.`) is used by default.

## Command Logging

Every command executed by `rc` is automatically logged to `<working-dir>/.NNNN.rc.log`, where `NNNN` is a sequential number incremented with each command run in that directory. This provides:
- **Reproducibility** - Review and replay exact commands that were run
- **Debugging** - Trace what commands were executed in case of issues
- **Documentation** - Keep a record of all operations performed

Each log file contains:
- The exact command line used to invoke `rc`
- Full output logs from the executed application
- Timestamp and execution details

For example, your first run creates `.0000.rc.log`, the second creates `.0001.rc.log`, and so on.

## Commands

### `run`

Run an application with optional arguments.

```bash
rc run [OPTIONS] <APP> [ARGS]...
```

**Options:**
- `-w, --working-dir <PATH>` - Input directory path (default: current directory)
- `-e, --container-engine <ENGINE>` - Container engine to use (default: docker)

**Available Apps:**
- `rosetta` - Run Rosetta protocols and applications
- `score` - Run Rosetta score command (shorthand for common scoring tasks)
- `pyrosetta` - Execute PyRosetta Python scripts with PyRosetta environment
- `rfdiffusion` - Run RFdiffusion for protein structure generation
- `proteinmpnn` - Run ProteinMPNN for protein sequence design

### `install`

Install an application (not yet implemented).

```bash
rc install <APP>
```

### `clean`

Clean an app installation (not yet implemented).

```bash
rc clean <APP>
```

## App Usage Examples

### Rosetta

#### Score a single structure

```bash
rc run rosetta score \
    -out:file:scorefile my_scores.sc \
    -in:file:s my_protein.pdb
```

#### Run other Rosetta protocols

```bash
rc run rosetta relax \
    -in:file:s input.pdb \
    -relax:fast
```

### PyRosetta

#### Running PyRosetta Scripts

Execute PyRosetta Python scripts directly using the `-c` flag for inline code:

```bash
rc run pyrosetta -c 'import pyrosetta; pyrosetta.init(); pose=pyrosetta.pose_from_pdb("1brs.pdb"); print("SCORE:", pyrosetta.get_score_function()(pose) )'
```

Or run a Python script file:

```bash
rc run pyrosetta my-pyrosetta-script.py
```

#### Run a PyRosetta script file

```bash
rc run pyrosetta design_script.py
```

### RFdiffusion

#### Generate a protein backbone

```bash
rc run rfdiffusion inference.py \
    inference.output_prefix=output/sample \
    inference.num_designs=10
```

#### Conditional generation with a motif

```bash
rc run rfdiffusion inference.py \
    inference.output_prefix=output/motif_scaffold \
    inference.input_pdb=motif.pdb \
    'contigmap.contigs=[10-40/A163-181/10-40]'
```

### ProteinMPNN

#### Design sequences for a protein structure

```bash
rc run proteinmpnn \
    --pdb_path structure.pdb \
    --pdb_path_chains "A B"
```

This will generate designed sequences in the `seqs/` directory within your working directory.

#### Design with custom parameters

```bash
rc run proteinmpnn \
    --pdb_path structure.pdb \
    --pdb_path_chains "A" \
    --num_seq_per_target 10 \
    --sampling_temp 0.1
```

### General Options

#### Using with different working directory

```bash
rc run -w /data/structures rosetta score \
    -out:file:scorefile results/scores.sc \
    -in:file:s protein.pdb
```

#### Using Singularity instead of Docker

```bash
rc run -e singularity rosetta score \
    -in:file:s structure.pdb
```

## Verbose Mode

Enable verbose output with the `-v` flag:

```bash
rc -v run rosetta score -in:file:s structure.pdb
```

This will show detailed information including the exact command being executed and where it's being logged.

## Requirements

- One of the supported container engines (Docker, Singularity, or Apptainer)
- Appropriate container images for the applications you want to run

## License

See LICENSE file for details.

## Author

Sergey Lyskov
