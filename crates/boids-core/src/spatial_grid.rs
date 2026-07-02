use alloc::{vec, vec::Vec};

use crate::math::Vec2;

/// A list of boid indices that corresponds to the contents of a cell in a spatial grid
type BoidIndices = Vec<usize>;

/// Spatial Grid for efficient boid neighbour processing.
/// Cells are stored as a flattened 1D Vec which contain a Vec of boid indices.
/// 
/// Assume row-major order of cells. ie: (0,0), (1,0), (2,0), ..., (0,1), (1,1), ...
pub struct SpatialGrid {
    cell_size: f32,
    rows: usize,
    columns: usize,
    cells: Vec<BoidIndices>
}

// 1. Public spatial grid functions
impl SpatialGrid {
    pub fn new(area_size: Vec2, neighbour_radius: f32) -> Self {
        assert!(area_size.x.is_finite());
        assert!(area_size.y.is_finite());
        assert!(area_size.x > 0.0);
        assert!(area_size.y > 0.0);

        assert!(neighbour_radius.is_finite());
        assert!(neighbour_radius > 0.0);

        let rows = libm::ceilf(area_size.y/neighbour_radius) as usize;
        let columns = libm::ceilf(area_size.x/neighbour_radius) as usize;

        assert!(rows > 0);
        assert!(columns > 0);

        let cell_count = rows.checked_mul(columns).expect("Spatial grid count overflowed.");

        Self { 
            cell_size: neighbour_radius, 
            rows, 
            columns, 
            cells: vec![Vec::new(); cell_count]
        }
    }

    pub fn clear(&mut self) {
        // Loop through and clear each cell in cells, rather than reconstructing the cells
        for cell in &mut self.cells {
            cell.clear();
        }
    }

    /// Insert the given index of a boid into the correct cell based on its position
    pub fn insert(&mut self, boid_index: usize, boid_position: Vec2) {
        let boid_cell_coord = self.position_to_cell_coord(boid_position);
        // At this point the cell coord must be valid due to clamping inside the function
        if let Some(insert_index) = self.cell_coord_to_index(boid_cell_coord.0, boid_cell_coord.1) {
            self.cells[insert_index].push(boid_index);
        }
    }

    /// Return a vector of all boid indices that are in neighbouring cells to a given boid position.
    /// This does NOT return only neighbouring boids (ie only boids within radius), 
    /// rather also all candidate boids which are in neighbouring cells.
    pub fn nearby_boid_indices(&self, boid_position: Vec2, radius: f32) -> Vec<usize> {
        let nearby_boid_indices = &mut Vec::<usize>::new();

        // Calculate our cell search range. Also handles radius < 0
        let cell_search_range = self.cell_search_range_for_radius(radius);

        // Get the cell coord of our given boid
        let (cell_x, cell_y) = self.position_to_cell_coord(boid_position);
        // Use cell coords to derive neighbouring cells
        for row_offset in -cell_search_range..=cell_search_range {
            for column_offset in -cell_search_range..=cell_search_range {
                // Check neighbouring cell
                let neighbour_x = cell_x as isize + column_offset;
                let neighbour_y = cell_y as isize + row_offset;

                if !self.is_cell_in_bounds(neighbour_x, neighbour_y) {
                    continue
                }

                // We can safely cast to usize here since cell is in bounds so its not negative
                match self.cell_coord_to_index(neighbour_x as usize, neighbour_y as usize) {
                    Some(cell_index) => {
                        nearby_boid_indices.extend(&self.cells[cell_index]);
                    },
                    None => {continue;}
                }
            }
        }
        nearby_boid_indices.clone()
    } 
}

// 2. Private spatial grid helpers
impl SpatialGrid {
    /// Given a coordinate in the world, return the corresponding 2D spatial grid cell it is in
    fn position_to_cell_coord(&self, position: Vec2) -> (usize, usize) {
        // Corresponding cells is basic division, floored. Casting to usize trims the float, ie floors it 
        let column_index = ((position.x / self.cell_size) as usize).min(self.columns-1);
        let row_index = ((position.y / self.cell_size) as usize).min(self.rows-1);

        (column_index, row_index)
    }

    /// Given the cell coord return the corresponding flattened 1D row-major index
    fn cell_coord_to_index(&self, x: usize, y: usize) -> Option<usize> {
        // Flattening 2D to 1D in row-major order formula: 
        // column idx + (row idx * column length)
        // NOTE: x is the column idx and y is the row idx
        if !self.is_cell_in_bounds(x as isize, y as isize) {
            return None
        }
        Some(x + (y * self.columns))
    }

    /// Return if a given cell coord is in bounds of our spatial grid
    fn is_cell_in_bounds(&self, x: isize, y: isize) -> bool {
        // Function uses isize as for neighbour checking we may expect negative values
        // as we may request checking a non-existent cell
        x >= 0 && y >= 0 &&
        x < self.columns as isize &&
        y < self.rows as isize
    }

    /// Return the cell search range, given a neighbour radius value, since 
    /// neighbour radius is modifiable, but we do not want constantly update 
    /// our spatial grid. Returns an isize since it requires a negative sign to
    /// be used as the range.
    ///
    /// For example: typically our neigbour radius if our search range is equal 
    /// to cell size then 1 is the optimum radius. However if we change our neighbour 
    /// radius it would be sufficient to keep it the same if smaller than the cell size,
    /// but if it is larger than the cell size it must increase otherwise we will lose
    /// 'neighbour' boids.
    /// 
    /// NOTE: This function is necessary to avoid using libm ceil potentially every tick.
    fn cell_search_range_for_radius(&self, radius: f32) -> isize {
        // Handle radius <= 0 gracefully
        if radius <= 0.0 {
            return 0;
        }

        // Range can be calculated by division. However it requires a ceil
        let mut range = (radius/self.cell_size) as isize;

        // Makeshift ceil operation
        if (range as f32 * self.cell_size) < radius {
            range += 1;
        }
        range
    }

}

#[cfg(test)] 
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn new_panic_invalid_area_size() {
        let inputs = [
            Vec2::new(f32::NAN, 10.0f32),
            Vec2::new(10.0f32, f32::NAN),
            Vec2::new(f32::INFINITY, 10.0f32),
            Vec2::new(10.0f32, f32::NEG_INFINITY),
            Vec2::new(0.0, 10.0f32),
            Vec2::new(10.0f32, -0.0),
            Vec2::new(-10.0, 10.0f32),
            Vec2::new(10.0f32, -10.0)
        ];

        for input in inputs {
            let result = std::panic::catch_unwind(|| {
                SpatialGrid::new(input, 1.0f32);                
            });

            let debug_x = input.x;
            let debug_y = input.y;
            assert!(
                result.is_err(), 
                "Expected SpatialGrid::new([{debug_x}, {debug_y}], 1.0f32) to panic"
            );
        }
    }
}