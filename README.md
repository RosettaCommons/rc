# rc

A command line tool to run various biomolecular structural prediction, analysis and design applications using containerized environments.

## Overview

`rc` provides a unified interface for running Rosetta and other biomolecular modeling applications through container engines like Docker, Singularity, or Apptainer.

## Installation

`rc` relies on the [Rust programming language](https://rust-lang.org/). If you do not already
have Rust installed on your system, see [Rust's Getting Started page](https://rust-lang.org/learn/get-started/). 

```bash
cargo install --path .
```

## Basic Usage

`rc` is an interface for Rosetta Commons-supported containers, so using it is as simple
as adding `rc run -w` to the beginning of the command you would otherwise directly use
with a container.

### Usage Example: Running Rosetta Score

This command will:
- Use the default [Docker](https://www.docker.com/) container engine
    - You will need to have a Docker Daemon installed and running for this
    - If you cannot use Docker on your system, see the [next section](#specifying-a-container-engine)
    - If you would like to install a Docker Daemon see either the [Docker Engine Installation guide](https://docs.docker.com/engine/install/) or the [Docker Desktop documentation](https://docs.docker.com/desktop/)
- Mount the working directory into the container
- Run the Rosetta score application

<!--Score a PDB structure file using Rosetta:

```bash
rc run -w /path/to/working/directory rosetta score \
    -out:file:scorefile output.sc \
    -in:file:s structure.pdb
```
-->

Run the Rosetta score executable

```bash
rc run score
```
This should print out the different input and output options for Rosetta's score 
executable along with various other pieces of information relevant to how this
executable functions. For a more detailed example of this see the [Examples](examples) 
section. 

### Specifying a Container Engine

You can specify which container engine to use with the `-e` flag:

```bash
rc run -e singularity rosetta score -in:file:s structure.pdb
```

Supported container engines:
- [`docker`](https://www.docker.com/) (default)
- [`singularity`](https://docs.sylabs.io/guides/latest/user-guide/)
- [`apptainer`](https://apptainer.org/)
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
