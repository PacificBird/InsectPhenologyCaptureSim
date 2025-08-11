[![DOI](https://zenodo.org/badge/628125611.svg)](https://doi.org/10.5281/zenodo.16809301)

## Modeling the Relationship Between Phenology and Sampling Data as a Function of Sampling Efficacy

This repository contains three interlocking parts:
- The Rust-based population/capture simulation for calculating delay-adjusted trajectories
- The QGIS project and model for calculating trap densities and creating the map figures
- The Python-based Jupyter Notebook containing the statistical analysis and charts
as well as the rendered figures tables.

Direct questions to Izzy McCabe (izzym@pacificbird.dev)

### Simulation
The reference trajectories from the simulation are not packaged with this repository due to their >500MB size.
To run the simulation and generate the reference trajectories for use in the Python analysis,
you'll want an up to date version of the Rust nightly toolchain. You can find this either through your package manager
or at https://rustup.rs/. After installation run the commands `rustup install nightly` followed by `rustup default nightly`
After that, clone the repository and use `cargo run` to compile and run the program, which will automatically generate a file called `simdata.csv`.

### QGIS Project
The required data on trap densities is already packaged with this repository. However, if you desire to reproduce, audit, or reuse the methodology on your own trap data,
the QGIS project and included QGIS models contains all you need. Install an up to date version of QGIS from your package manager or from https://qgis.org/ and open up
`traps_geo.qgz`. Import your trap location data into QGIS _by year_, ensuring that each trap is annotated by field name with a data column called _field\_name_,
and use the included `buffer_dissolve_density` model to calculate polygons that have densities labeled by field names. The data associated with those polygons can be
exported to csv without geometry data by right clicking them in the Layers panel and selecting `Export -> Save Features As`, then switching format to
Comma Separated Values.

### Jupyter Notebook
The analysis document requires Jupyter Notebook as well as an installation of Python from https://www.python.org/ in addition
to a list of statistical packages. After installing Python, create a virtual environment within this repository
using `python -m venv venv_name`, where `venv_name` is a name of your choice. Then, run `.venv/bin/activate` for bash-based shells
or whichever activation script within the `bin` folder that matches your terminal shell. Once your virtual environment is
activated, running `pip install pandas numpy matplotlib scipy seaborn statsmodels jupyterlab` will install all the packages
required to run the document. Running `jupyter-lab` in the main directory will open your browser with an instance of
Jupyter Lab. Select `analysis.ipynb` in the left panel, and if you have ran the simulation to obtain `simdata.csv`,
pressing the double arrows (⏩) will rerun all cells and perform the analysis automatically.
