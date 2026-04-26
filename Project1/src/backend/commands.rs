use crate::backend::simulation::SimulationSettings;
use crate::backend::snapshot::CrystalSnapshot;

#[derive(Debug, Clone)]
pub enum UiCommand {
    // We must to stop simulation, if this occured. It means, simulation parameter is changed.
    ParameterUpdated,

    StartSimulation(SimulationSettings),
    StopSimulation,
}

#[derive(Debug, Clone)]
pub enum EngineEvent {
    AlgorithmPassed(CrystalSnapshot),
    Snapshot(CrystalSnapshot),
    SimulationFinished,
}
