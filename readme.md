# rast2vec-rs

**Work in progress!**

Tool for converting raster image to vector.

## Purpose

Convert simple images to clean vector in a second.

### Simple image

*(for now)*
Image that can be closely approximated with svg paths.

### Clean vector

*(for now)*
SVG without reduntant paths.

## State

Algorithm exists and it should work - there's some basic working example.

With enabled logging it took about 25 sec for example.

## In future

1. Fix errors in areas calculation (they are skipped now).
2. Simplify paths in produced result.
3. Use differentiable rasterizer to tune produced result as temporary solution.
4. Check (and fix if necessary) arguments for edge detector.
5. Apply algorithm on difference with origin several times as temporary solution for covering areas that wasn't processed during first pass.
6. Speed up algo by fixing `areas` fucntion.
7. Speed up algo by switching from graphs algorithms to convolutions with precalculated matrixes on local steps.
8. Get rid of python by using implemented in rust `canny edge detector` and diff rasterizer.
9. Implement edge detector basing on `canny` (or smth else) which upscales image (to get rid of diff rasterizer) and provides more information for more precise path reconstruction.
10. ...
