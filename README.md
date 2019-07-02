Terr
====

[![Build Status](https://travis-ci.org/dhardy/terr.svg)](https://travis-ci.org/dhardy/terr)

Procedural terrain generation algorithms.

[Changelog](CHANGELOG.md)

Examples:

-   `flat`: just flat
-   `noise`: uncorrelated noise
-   `fractal-md`: fractal generation using the midpoint displacement algorithm
    
    ![Example](/fractal.png?raw=true)
-   `fractal-ds`: fractal generation using the diamond-square algorithm
-   `voronoi`: generate simple features via a modified Voronoi diagram
-   `voronoi-ds`: voronoi + diamond-square terrain
    
    ![Example](/voronoi-ds.png?raw=true)
-   `perlin`: generate from a single layer of Perlin noise
-   `perlin-octaves`: generate from multiple octaves of Perlin noise, with
    exponential slopes
    
    ![Example](/perlin-octaves.png?raw=true)

These are all very simple algorithms. Hopefully this library will accumulate
more, and better, techniques, along with mesh optimisation and texturing
support.
