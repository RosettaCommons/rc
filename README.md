# rc

`rc` is a command-line tool for running and reproducing calculations with containerized biomolecular software. It simplifies everything from mounting local directories to logging executed commands, helping you run complex workflows reliably and repeatably. Designed for reproducible research, `rc` aims to become a seamless part of your daily computational workflow.

## Overview

`rc` provides a unified interface for running Rosetta and other biomolecular modeling applications through container engines like Docker, Singularity, or Apptainer.

**Available Apps:**
- [`rosetta`](https://docs.rosettacommons.org/docs/latest/Home) - Run Rosetta protocols and applications
- [`score`](https://docs.rosettacommons.org/docs/latest/application_documentation/analysis/score-commands) - Run Rosetta score command (shorthand for common scoring tasks)
- [`pyrosetta`](https://www.pyrosetta.org/) - Execute PyRosetta Python scripts with PyRosetta environment
- [`rfdiffusion`](https://sites.google.com/omsf.io/rfdiffusion) - Run RFdiffusion for protein structure generation
- [`proteinmpnn`](https://github.com/dauparas/ProteinMPNN) - Run ProteinMPNN for protein sequence design
- [`proteinmpnn-script`](https://github.com/dauparas/ProteinMPNN) - Run ProteinMPNN helper scripts for preprocessing and analysis
- [`ligandmpnn`](https://github.com/dauparas/LigandMPNN) - Run LigandMPNN for protein-ligand interface design
- [`foundry`](https://rosettacommons.github.io/foundry/) - Run Foundry toolkit (RFDiffusion3, LigandMPNN, RoseTTAFold3) for integrated protein design workflows

See [App Usage Examples](#app-usage-examples) for how to run each of these tools using `rc`.

**Container Engine Support:**

> [!NOTE]
> Here, 'Native' means that `rc` can work with a local (non-containerized) installation of a particular tool. 

| App | Docker | HPC Containers (Singularity/Apptainer) | Native |
|-----|--------|----------------------------------------|--------|
| `rosetta` | ✓ | ✓ |  |
| `score` | ✓ | ✓ |  |
| `pyrosetta` | ✓ | ✓ |  |
| `rfdiffusion` | ✓ | ✓ | ✓ |
| `proteinmpnn` | ✓ | ✓ | ✓ |
| `proteinmpnn-script` | ✓ | ✓ |  |
| `ligandmpnn` | ✓ | ✓ | |
| `foundry` | ✓ | ✓ | ✓ |

## Installation
`rc` uses `cargo`, Rust's package manager and build tool, for fast and seamless installation. To run the following command, you will need to have [Rust installed](https://rust-lang.org/tools/install/). 

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
- Use the default [Docker](https://www.docker.com/) container engine
- Mount the working directory into the container
- Run the Rosetta score application
- Output the score file to `output.sc`
- Log the executed command to a log file for reproducibility, see [Command Logging](#command-logging)

### Specifying a Container Engine

You can specify which container engine to use with the `-e` flag:

```bash
rc run -e singularity rosetta score -in:file:s structure.pdb
```

Supported container engines:
- [`docker`](https://docs.docker.com/engine/install/) (default)
- [`singularity`](https://docs.sylabs.io/guides/latest/user-guide/)
- [`apptainer`](https://apptainer.org/)
- `none` (run natively without containers - supported by RFDiffusion and Foundry)

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
The `[OPTIONS]` are `rc`-specific, while the `[ARGS]` are specific to the app you are running.

**Options:**
- `-w, --working-dir <PATH>` - Input directory path (default: current directory)
- `-e, --container-engine <ENGINE>` - Container engine to use (default: docker)

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

> [!IMPORTANT]
> A list of available Rosetta protocols/applications can be found [here](https://docs.rosettacommons.org/docs/latest/application_documentation/Application-Documentation). 

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

RFdiffusion supports native runs without containers ((uses `-e none`).

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

### Foundry

Foundry provides an integrated toolkit combining RFDiffusion3 (rfd3), LigandMPNN (mpnn), and RoseTTAFold3 (rf3) for comprehensive protein design workflows. It supports native runs without containers (uses `-e none`).

#### Complete workflow example

> [!NOTE]
> Each `rc run` invocation will produce a separate log file. So even though
> these commands are run as part of a workflow, each will produce its own log file.

First, create an input JSON file (e.g., `input.json`):

```bash
echo '{ "foundry": { "length": "10" } }' > input.json
```

Then run the three-step workflow:

**Step 1: Run RFDiffusion3 for structure generation**

```bash
rc run foundry rfd3 \
    out_dir=rfd3_out/ \
    inputs=input.json \
    skip_existing=False \
    prevalidate_inputs=True \
    n_batches=1 \
    diffusion_batch_size=1 \
    inference_sampler.num_timesteps=10 \
    low_memory_mode=True \
    global_prefix=design_
```

**Step 2: Run LigandMPNN for sequence design**

```bash
rc run foundry mpnn \
    --structure_path rfd3_out/design_foundry_0_model_0.cif.gz \
    --is_legacy_weights True \
    --model_type ligand_mpnn \
    --out_directory mpnn_out
```

**Step 3: Run RoseTTAFold3 for structure prediction**

```bash
rc run foundry rf3 fold \
    inputs=mpnn_out/design_foundry_0_model_0.cif_b0_d0.cif \
    diffusion_batch_size=1 \
    num_steps=10 \
    out_dir=rf3_out
```

#### Using explicit weights specification

You can specify custom checkpoint paths for the models:

```bash
# RFDiffusion3 with custom checkpoint
rc run foundry rfd3 \
    out_dir=rfd3_out/ \
    inputs=input.json \
    ckpt_path=/weights/rfd3_latest.ckpt \
    skip_existing=False \
    prevalidate_inputs=True \
    n_batches=1 \
    diffusion_batch_size=1 \
    inference_sampler.num_timesteps=10 \
    low_memory_mode=True \
    global_prefix=design_

# LigandMPNN with custom checkpoint
rc run foundry mpnn \
    --structure_path rfd3_out/design_foundry_0_model_0.cif.gz \
    --checkpoint_path /weights/ligandmpnn_v_32_010_25.pt \
    --is_legacy_weights True \
    --model_type ligand_mpnn \
    --out_directory mpnn_out

# RoseTTAFold3 with custom checkpoint
rc run foundry rf3 fold \
    inputs=mpnn_out/design_foundry_0_model_0.cif_b0_d0.cif \
    ckpt_path=/weights/rf3_foundry_01_24_latest_remapped.ckpt \
    diffusion_batch_size=1 \
    num_steps=10 \
    out_dir=rf3_out
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
