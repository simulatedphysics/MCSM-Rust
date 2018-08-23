# MCSM
Monte Carlo for Statistical Mechanics

The goal of this project is to create a physics simulation software in Rust. For now, we are focusing on a 
Monte Carlo simulation of an Ising Model on a square lattice. Soon, we plan to generalize this to a few 
well-known models on several lattices.

While the Monte Carlo runs, observables are collected at each temperature. These observables are average energy, heat capacity, average magnetization, and susceptibility.

These plots are plotted using Plotly and Javascript.