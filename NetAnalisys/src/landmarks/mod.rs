pub mod basic_landmarks;
pub mod bfs_landmarks;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LandmarkStrategy {
    Random,
    HighestDegree,
    Coverage,
}
