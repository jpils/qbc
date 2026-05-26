import os
from typing import List
import numpy as np
from ase import Atom, Atoms
from ase.io import iread
from numpy._core.numeric import ndarray
import torch
from upet.calculator import UPETCalculator

MODEL = "pet-mad-s"

def get_device() -> str:
    device = ""
    if torch.cuda.is_available():
        device = "cuda"
        print("Using CUDA")
    else:
        device = "cpu"
        os.environ["WARP_DEVICE"] = "cpu"
        print("Using CPU")
    return device

calc = UPETCalculator(
    model = MODEL,
    version = "1.5.0",
    device = get_device()
)

def get_force_uncertainty(atoms: Atoms) -> ndarray:
    forces = atoms.get_forces()
    return forces

def main():
    frames = iread(filename="./dump.out", format="lammps-dump-text")

    energy_uncertainties: List[np.ndarray] = []
    force_uncertainties: List[np.ndarray] = []

    for idx, atoms in enumerate(frames):
        atoms.calc = calc
        energy_uc = atoms.calc.get_energy_uncertainty(atoms)
        force_uc = get_force_uncertainty(atoms)

        energy_uncertainties.append(energy_uc)
        force_uncertainties.append(force_uc)

    print(len(energy_uncertainties))

if __name__ == "__main__":
    main()
