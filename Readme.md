# SCALE Codec Decoding with Vara Network and Unity

This repository provides an example of how to use the SCALE (Simple Concatenated Aggregate Little-Endian) codec to decode data in a development environment with the Vara Network and Unity.

## Description

The SCALE codec is a serialization format used in the Substrate and Polkadot ecosystem for efficient data encoding. This project demonstrates how to decode SCALE-encoded data for use in Unity, allowing developers to integrate blockchain functionalities into their applications.

# Unity
## Installation

To use this template in your Unity project, follow these steps:

1. Clone the repository:

   ```sh
   git clone https://github.com/Vara-Lab/SDK-Unity-Vara-Template.git

2. Open the project in Unity:

    1. Open Unity Hub.
    2. Select "Add" and navigate to the cloned repository folder.
    3. Select the folder and click "Open".

## Step 1: Open Contract on Gitpod

<p align="center">
  <a href="https://gitpod.io/#https://github.com/Vara-Lab/SCALE-codec-Vara-Unity.git" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>

## Step 2: Compile and Deploy the Smart Contract

### Compile the smart contract by running the following command:

```bash
cd GamingProgram
cargo build --release
```

Once the compilation is complete, locate the `*.opt.wasm` file in the `target/wasm32-unknown-unknown/release` directory.

## Step 3: Interact with Your Contract on Vara Network

1. Access [Gear IDE](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Frpc.vara.network) using your web browser.
2. Connect your Substrate wallet to Gear IDE.
3. Upload the `*.opt.wasm` and `metadata.txt` files by clicking the "Upload Program" button.

## Step 4: Copy and Paste Examples in Unity for ReadState using SCALE Decoding

Follow the examples provided in the repository to implement ReadState functionality in Unity using SCALE decoding.

## License
This project is licensed under the MIT License. See the LICENSE file for details.
