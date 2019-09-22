// Copyright 2019 Diggory Hardy
// Copyright (c) 2013, Sébastien Crozet
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Most code adapted from ncollide implementations for
// ncollide3d::shape::HeightField. Note that we don't use HeightField directly
// because it assumes a different coordinate system and because of issues
// with more than ~100x100 points.

use nalgebra as na;
use na::{convert, RealField, geometry::Point3, Unit};
use ncollide3d::shape::{Shape, FeatureId, Triangle};
use ncollide3d::math::{Isometry, Vector};
use ncollide3d::query::{Ray, RayCast, RayIntersection, PointQuery};
use ncollide3d::bounding_volume::{self, AABB, BoundingSphere, HasBoundingVolume};

use super::Heightmap;


impl<F: RealField> HasBoundingVolume<F, AABB<F>> for Heightmap<F> {
    #[inline]
    fn bounding_volume(&self, m: &Isometry<F>) -> AABB<F> {
        self.local_bounding_volume().transform_by(m)
    }

    #[inline]
    fn local_bounding_volume(&self) -> AABB<F> {
        AABB::new(
            Point3::new(F::zero(), F::zero(), self.range.0),
            Point3::new(self.size.0, self.size.1, self.range.1)
        )
    }
}

impl<F: RealField> Shape<F> for Heightmap<F> {
    #[inline]
    fn aabb(&self, m: &Isometry<F>) -> AABB<F> {
        bounding_volume::aabb(self, m)
    }

    #[inline]
    fn local_aabb(&self) -> AABB<F> {
        bounding_volume::local_aabb(self)
    }

    #[inline]
    fn bounding_sphere(&self, _m: &Isometry<F>) -> BoundingSphere<F> {
        unimplemented!()
    }

    #[inline]
    fn as_ray_cast(&self) -> Option<&dyn RayCast<F>> {
        Some(self)
    }

    #[inline]
    fn as_point_query(&self) -> Option<&dyn PointQuery<F>> {
        unimplemented!()
//         Some(self)
    }
    
    fn tangent_cone_contains_dir(
        &self,
        _fid: FeatureId,
        _m: &Isometry<F>,
        _deformations: Option<&[F]>,
        _dir: &Unit<Vector<F>>,
    ) -> bool
    {
        unimplemented!()
    }

    fn subshape_containing_feature(&self, _id: FeatureId) -> usize {
        unimplemented!()
    }
}

impl<F: RealField> RayCast<F> for Heightmap<F> {
    #[inline]
    fn toi_and_normal_with_ray(
        &self,
        m: &Isometry<F>,
        ray: &Ray<F>,
        solid: bool,
    ) -> Option<RayIntersection<F>>
    {
        let dim = self.dim;
        let len_frac = self.len_frac;
        
        let aabb = self.local_bounding_volume();
        let ls_ray = ray.inverse_transform_by(m);
        let is_pos = (ls_ray.dir.x > F::zero(), ls_ray.dir.y > F::zero());
        let (min_t, max_t) = aabb.clip_ray_parameters(&ls_ray)?;
        
        // Algorithm: iterate over all cells along the 2D projection of the ray.
        // Note that multiple interceptions are possible and we must find the
        // first, so a guess-and-search method is not appropriate.
        
        let p = ls_ray.point_at(min_t);
        let mut cell = self.cell_at_coord(p.x, p.y)?;

        loop {
            if cell.0 + 1 == dim.0 || cell.1 + 1 == dim.1 {
                continue;   // on edge, not a cell
            }
            let tris = self.triangles_at(cell.0, cell.1);
            let inter1 = tris.0.toi_and_normal_with_ray(m, ray, solid);
            let inter2 = tris.1.toi_and_normal_with_ray(m, ray, solid);

            match (inter1, inter2) {
                (Some(inter1), Some(inter2)) => {
                    if inter1.toi < inter2.toi {
                        return Some(inter1);
                    } else {
                        return Some(inter2);
                    }
                }
                (Some(inter), None) => {
                    return Some(inter);
                }
                (None, Some(inter)) => {
                    return Some(inter);
                }
                (None, None) => {}
            }

            let toi_x = if is_pos.0 {
                let x = convert::<_, F>((cell.0 + 1) as f64) * len_frac.0;
                (x - ls_ray.origin.x) / ls_ray.dir.x
            } else if ls_ray.dir.x < F::zero() {
                let x = convert::<_, F>((cell.0) as f64) * len_frac.0;
                (x - ls_ray.origin.x) / ls_ray.dir.x
            } else {
                F::max_value()
            };

            let toi_y = if is_pos.1 {
                let y = convert::<_, F>((cell.1 + 1) as f64) * len_frac.0;
                (y - ls_ray.origin.y) / ls_ray.dir.y
            } else if ls_ray.dir.z < F::zero() {
                let y = convert::<_, F>((cell.1) as f64) * len_frac.0;
                (y - ls_ray.origin.y) / ls_ray.dir.y
            } else {
                F::max_value()
            };

            if toi_x > max_t && toi_y > max_t {
                break;
            }

            if toi_x >= F::zero() && toi_x < toi_y {
                if is_pos.0 && cell.0 + 2 < dim.0 {
                    cell.0 += 1
                } else if !is_pos.0 && cell.0 > 0 {
                    cell.0 -= 1
                } else {
                    break
                }
            } else if toi_y >= F::zero() {
                if is_pos.1 && cell.1 + 2 < dim.1 {
                    cell.1 += 1
                } else if !is_pos.1 && cell.1 > 0 {
                    cell.1 -= 1
                } else {
                    break
                }
            } else {
                break
            }
        }

        None
    }
}


impl<F: RealField> Heightmap<F> {
    /// The two triangles of the cell (cx, cy).
    fn triangles_at(&self, cx: u32, cy: u32) -> (Triangle<F>, Triangle<F>) {
        assert!(cx + 1 < self.dim.0);
        assert!(cy + 1 < self.dim.1);
        
        let (x0, y0) = self.coord_of(cx, cy);
        let (x1, y1) = self.coord_of(cx+1, cy+1);
        
        let p00 = Point3::new(x0, y0, self.get(cx, cy));
        let p01 = Point3::new(x1, y0, self.get(cx, cy + 1));
        let p10 = Point3::new(x0, y1, self.get(cx + 1, cy));
        let p11 = Point3::new(x1, y1, self.get(cx + 1, cy + 1));

        let tri1 = Triangle::new(p01, p00, p11);
        let tri2 = Triangle::new(p00, p10, p11);
        (tri1, tri2)
    }
}
