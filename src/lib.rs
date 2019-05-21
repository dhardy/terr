// Copyright 2019 Diggory Hardy
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Terrain tools

/*
For now, we will focus only on simple height-maps.

Input source:
-   load from image data

Procedural sources:
-   flat
-   noise
-   fractal algorithms

Optimisation:
-   cull unnecessary triangles: http://www.shamusyoung.com/twentysidedtale/?p=142
    Peter Lindstrom's paper: Terrain Simplification Simplified, May 2002

References:
-   https://14mul8.wordpress.com/2018/10/13/procedural-terrain-generation-part-2-midpoint-displacement-algorithm/

    Covers fractal generation via the mid-point displacement algorithm.

-   http://www.shamusyoung.com/twentysidedtale/?p=143
    
    -   optimisation pp 2, 4-5
    -   textures pp 3, 7-9
    -   source: Terrain
*/

pub mod heightmap;
