# Project Proposal

In this project, we aim to improve the functionality of the MeTA toolkit and and MeTApy python library.

### Team

- Kevin Vece (vece2@illinois.edu, NETID: vece2)

### System

- MeTA/MeTApy

### What is the problem being solved?

The MeTA and MeTApy libraries have not been updated in five years (as of writing, Oct 2022). This means that users of the MeTApy python library are currently forced to used a separate python environment with an older version of python, preventing easy and seamless integration with python applications as the MeTApy author intended. This project aims to update MeTApy to be usable with python versions 3.5-3.10, and add a build pipeline to verify compatibility through future updates. We also seek to begin adding proper documentation for MeTApy python bindings so users are not required to refer to the original C++ documentation and translate it into the equivalent python code.

### Project Breakdown

This project will consist of three main components:

1. Creation of Python build pipeline
1. Update of MeTA build code and MeTApy python code
1. Update of MeTApy python documentation

##### Python build pipeline

This component will consist of building an automatic test environment to ensure MeTApy builds properly in different python environments using github hooks. It should verify that the current version of MeTApy works correctly on python versions 3.5-3.7 and fails to work on python versions 3.8+. We will then leverage this to verify that the updated MeTA/MeTApy code works correctly for all python versions later. This portion of the project will largely rely on python code and yaml configurations and should take a couple hours.

##### Update of MeTA/MeTApy build code

This component will consist of updating the MeTA and MeTApy libraries so they are able to work on more versions of python (and explore possible operating system limitations, limited to Windows 11, Mac OS X, and Ubuntu 22.04). This will involve understanding the build system for both libraries, but actual amount of work is to be determined based on complexity. Knowledge of python, C++, and the CMake build system will be required to accomplish this task.

##### Update of Python documentation

This component will involve comparing the C++ MeTA documentation to the MeTApy python implementation and creating a relevant readthedocs documentation website. In addition to understanding the functionality exposed by the MeTApy python module, this will require understanding of either the sphinx or mkdocs ecosystem, and an analysis of an efficient way to export all of the objects exposed by pybind11 to the documentation system. The author has no experience with these systems and there are over 2000 lines of code in the MeTApy library to go over, so this component is expected to take up to or exceed 20 hours to complete. Depending on the time commitment, only a subset of the components may be documented, such as those necessary for future students in the class.
