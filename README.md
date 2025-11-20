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
- `rosetta` - Run Rosetta protocol
- `score` - Run Rosetta score command

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

## Examples

### Score a single structure

```bash
rc run rosetta score \
    -out:file:scorefile my_scores.sc \
    -in:file:s my_protein.pdb
```

### Using with different working directory

```bash
rc run -w /data/structures rosetta score \
    -out:file:scorefile results/scores.sc \
    -in:file:s protein.pdb
```

### Using Singularity instead of Docker

```bash
rc run -e singularity rosetta score \
    -in:file:s structure.pdb
```

## Verbose Mode

Enable verbose output with the `-v` flag:

```bash
rc -v run rosetta score -in:file:s structure.pdb
```

## Requirements

- One of the supported container engines (Docker, Singularity, or Apptainer)
- Appropriate container images for the applications you want to run

## License

See LICENSE file for details.

## Author

Sergey Lyskov