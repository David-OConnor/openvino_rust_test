## Preqrequisites
  - [Install Rust](https://www.rust-lang.org/tools/install)

  - [Install OpenVINO](https://docs.openvino.ai/2023.1/openvino_docs_install_guides_overview.html) Download the `Archives` version.
  - Unpack this to `C:\Program Files (x86)\Intel\openvino_2023`
  - Set the environment var `OPENVINO_INSTALL_DIR` to that same path.



## Build
  - Run `setupvars.bat`, in the openvino_2023 directory
  - Run `cargo b` (`cargo b --release` for an optimized / production build)