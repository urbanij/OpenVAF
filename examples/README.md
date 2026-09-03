# OpenVAF Examples

Each directory contains Verilog-A models and ngspice netlists that load them
via the OSDI interface.

| Directory    | Description                                              |
|--------------|----------------------------------------------------------|
| `hello_world`| Minimal Verilog-A model that prints "Hello World!"       |
| `example_1`  | HICUM/L2 v3.0.0 bipolar transistor, DC output curve sweep |
| `example_2`  | PSP 103.8 MOSFET (incl. JUNCAP200), CMOS inverter transient |

## Usage

Compile the model, then run the netlist with ngspice (ngspice >= 39 with OSDI
support, e.g. `brew install ngspice`):

    cd hello_world
    openvaf hello.va
    ngspice -b hello.sp

`pre_osdi` paths inside a netlist resolve relative to the netlist file, so the
`.osdi` file is expected next to it (which is where `openvaf` writes it).
