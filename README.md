# RustEvolve

Brain Inputs:

1. The tile type the creature is on
2. Each eye cell
    1. The inverse of the distance to the closest of each tile type + creatures
        * Done for empty floors as well, to look for floors past hazards
    2. The count of each tile type + creatures

Experiment Ideas:

* N% of frames, creatures lose sight
* Creatures survival while in the middle, with no indication that they are in the middle