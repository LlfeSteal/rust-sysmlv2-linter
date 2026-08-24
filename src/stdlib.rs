//! Bibliothèque standard SysML v2 / KerML — **fichier généré**.
//!
//! Ne pas éditer à la main : régénérer avec `tools/gen-stdlib.py`
//! (et `--fetch` pour rafraîchir l'index vendu dans `spec/`).
//!
//! Le vérificateur ne charge pas la bibliothèque ; il sait seulement quels
//! noms lui appartiennent et dans quel paquet, afin de ne pas signaler comme
//! « inconnu » ce qui en vient légitimement — et, à l'inverse, de repérer un
//! nom qui *ressemble* à un type standard sans en être un.
//!
//! Source : https://github.com/Systems-Modeling/SysML-v2-Release

/// Paquets de la bibliothèque standard (racines d'import valides).
pub const LIBRARY_ROOTS: &[&str] = &[
    "Actions",
    "Allocations",
    "AnalysisCases",
    "AnalysisTooling",
    "Attributes",
    "Base",
    "BaseFunctions",
    "BooleanFunctions",
    "Calculations",
    "Cases",
    "CausationConnections",
    "CauseAndEffect",
    "Clocks",
    "CollectionFunctions",
    "Collections",
    "ComplexFunctions",
    "Connections",
    "Constraints",
    "ControlFunctions",
    "ControlPerformances",
    "Core",
    "DataFunctions",
    "DerivationConnections",
    "FeatureReferencingPerformances",
    "Flows",
    "ISQ",
    "ISQAcoustics",
    "ISQAtomicNuclear",
    "ISQBase",
    "ISQCharacteristicNumbers",
    "ISQChemistryMolecular",
    "ISQCondensedMatter",
    "ISQElectromagnetism",
    "ISQInformation",
    "ISQLight",
    "ISQMechanics",
    "ISQSpaceTime",
    "ISQThermodynamics",
    "ImageMetadata",
    "IntegerFunctions",
    "Interfaces",
    "Items",
    "KerML",
    "Kernel",
    "Links",
    "MeasurementRefCalculations",
    "MeasurementReferences",
    "Metadata",
    "Metaobjects",
    "ModelingMetadata",
    "NaturalFunctions",
    "NumericalFunctions",
    "Objects",
    "Observation",
    "OccurrenceFunctions",
    "Occurrences",
    "ParametersOfInterestMetadata",
    "Parts",
    "Performances",
    "Ports",
    "Quantities",
    "QuantityCalculations",
    "RationalFunctions",
    "RealFunctions",
    "RequirementDerivation",
    "Requirements",
    "RiskMetadata",
    "Root",
    "SI",
    "SIPrefixes",
    "SampledFunctions",
    "ScalarFunctions",
    "ScalarValues",
    "SequenceFunctions",
    "ShapeItems",
    "SpatialFrames",
    "SpatialItems",
    "StandardViewDefinitions",
    "StatePerformances",
    "StateSpaceRepresentation",
    "States",
    "StringFunctions",
    "SysML",
    "Systems",
    "TensorCalculations",
    "Time",
    "TradeStudies",
    "Transfers",
    "TransitionPerformances",
    "TrigFunctions",
    "Triggers",
    "USCU",
    "USCustomaryUnits",
    "UseCases",
    "VectorCalculations",
    "VectorFunctions",
    "VectorValues",
    "VerificationCases",
    "Views",
];

/// `(nom, paquets)` pour chaque élément de la bibliothèque, **trié par nom**
/// (invariant requis par la recherche dichotomique ci-dessous).
///
/// Les paquets sont ceux depuis lesquels le nom est visible : celui qui le
/// définit en tête, puis ceux qui le ré-exportent via `public import`.
/// `MassValue` est ainsi joignable par `ISQBase` (sa définition) comme par
/// `ISQ` (qui ré-exporte `ISQBase`).
const LIBRARY_INDEX: &[(&str, &[&str])] = &[
    (
        "AbsoluteActivityValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AbsorbedDoseRateUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AbsorbedDoseRateValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AbsorbedDoseUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AbsorbedDoseValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AbsorptanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AbsorptionNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AccelerationUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AccelerationValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("AcceptAction", &["Actions"]),
    ("AcceptMessageAction", &["Actions"]),
    ("AcceptPerformance", &["Transfers"]),
    (
        "AcceptorDensityUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AcceptorDensityValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AcousticImpedanceUnit",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AcousticImpedanceValue",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Action", &["Actions"]),
    ("ActionFlowView", &["StandardViewDefinitions"]),
    (
        "ActionQuantityUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ActionQuantityValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ActivityCoefficientValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ActivityDensityUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ActivityDensityValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ActivityFactorValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ActivityOfSoluteValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ActivityOfSolventValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AdmittanceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AdmittanceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("AffineTransformationMatrix3d", &["MeasurementReferences"]),
    (
        "AffinityOfAChemicalReactionUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AffinityOfAChemicalReactionValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Allocation", &["Allocations"]),
    (
        "AmountOfSubstanceConcentrationUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AmountOfSubstanceConcentrationValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AmountOfSubstanceFractionMoleFractionValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AmountOfSubstanceUnit",
        &["ISQBase", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AmountOfSubstanceValue",
        &["ISQBase", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("AnalysisCase", &["AnalysisCases"]),
    (
        "AngularAccelerationUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularAccelerationValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularFrequencyUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularFrequencyValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularImpulseUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularImpulseValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularMeasureUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularMeasureValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularMomentumUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularMomentumValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularReciprocalLatticeVectorMagnitudeUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularReciprocalLatticeVectorMagnitudeValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularRepetencyUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularRepetencyValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularVelocityUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AngularVelocityValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Anything", &["Base"]),
    (
        "ArchimedesNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AreaUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AreaValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Array", &["Collections", "CollectionFunctions"]),
    (
        "ArrheniusNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("AssignObservations", &["Observation", "Triggers"]),
    ("AssignmentAction", &["Actions"]),
    (
        "AtomicAttenuationCoefficientUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AtomicAttenuationCoefficientValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AtomicScatteringFactorValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AttenuationUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AttenuationValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AtwoodNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AverageEnergyLossPerElementaryChargeProducedUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AverageEnergyLossPerElementaryChargeProducedValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AverageInformationRateUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AverageInformationRateValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AverageLogarithmicEnergyDecrementValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AverageTransinformationRateUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "AverageTransinformationRateValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Bag", &["Collections", "CollectionFunctions"]),
    (
        "BagnoldNumberForSolidParticlesValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BagnoldNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("BasicClock", &["Clocks", "Triggers"]),
    ("BasicDurationOf", &["Clocks", "Triggers"]),
    ("BasicTimeOf", &["Clocks", "Triggers"]),
    (
        "BatchelorNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BejanNumberForEntropyValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BejanNumberForHeatTransferValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BejanNumberForMassTransferValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BejanNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("BinaryConnection", &["Connections"]),
    (
        "BinaryDigitRateUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BinaryDigitRateValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("BinaryInterface", &["Interfaces"]),
    (
        "BindingFractionValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BinghamNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BiotNumberForMassTransferValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BiotNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BlakeNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BodensteinNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BoltzmannNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "BondNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Boolean",
        &[
            "ScalarValues",
            "BooleanFunctions",
            "ComplexFunctions",
            "IntegerFunctions",
            "NaturalFunctions",
            "NumericalFunctions",
            "RationalFunctions",
            "RealFunctions",
            "ScalarFunctions",
            "StringFunctions",
        ],
    ),
    ("BooleanEvaluation", &["Performances"]),
    (
        "BooleanEvaluationResultMonitorPerformance",
        &["FeatureReferencingPerformances"],
    ),
    (
        "BooleanEvaluationResultToMonitorPerformance",
        &["FeatureReferencingPerformances"],
    ),
    (
        "BrinkmanNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("BrowserView", &["StandardViewDefinitions"]),
    ("Calculation", &["Calculations"]),
    (
        "CallIntensityUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CallIntensityValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CanonicalPartitionFunctionValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CapacitanceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CapacitanceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CapillaryNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CarnotNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Cartesian3dMomentOfInertiaMeasurementReference",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Cartesian3dMomentOfInertiaTensor",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Cartesian3dStrainMeasurementReference",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Cartesian3dStrainTensor",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Cartesian3dStressMeasurementReference",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Cartesian3dStressTensor",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianAcceleration3dCoordinateFrame",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianAcceleration3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianAngularAcceleration3dCoordinateFrame",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianAngularAcceleration3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianAngularImpulse3dCoordinateFrame",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianAngularImpulse3dVector",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianAngularMomentum3dCoordinateFrame",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianAngularMomentum3dVector",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianAngularReciprocalLattice3dCoordinateFrame",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianAngularReciprocalLattice3dVector",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianAngularVelocity3dCoordinateFrame",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianAngularVelocity3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianBurgers3dVector",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("CartesianCurrentDisplacementOf", &["SpatialFrames"]),
    ("CartesianCurrentPositionOf", &["SpatialFrames"]),
    (
        "CartesianDisplacement3dVector",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianDisplacementCurrentDensity3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianDisplacementCurrentDensity3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("CartesianDisplacementOf", &["SpatialFrames"]),
    (
        "CartesianDragForce3dVector",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianElectricCurrentDensity3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianElectricCurrentDensity3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianElectricDipoleMoment3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianElectricDipoleMoment3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianElectricFieldStrength3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianElectricFieldStrength3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianElectricFluxDensity3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianElectricFluxDensity3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianElectricPolarization3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianElectricPolarization3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianEquilibriumPosition3dVector",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianForce3dCoordinateFrame",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianForce3dVector",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianFundamentalLattice3dVector",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianFundamentalReciprocalLattice3dCoordinateFrame",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianFundamentalReciprocalLattice3dVector",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianImpulse3dCoordinateFrame",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianImpulse3dVector",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianKineticFrictionForce3dVector",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianLattice3dVector",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianLinearElectricCurrentDensity3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianLinearElectricCurrentDensity3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagneticDipoleMoment3dCoordinateFrame",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagneticDipoleMoment3dVector",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagneticFieldStrength3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagneticFieldStrength3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagneticFluxDensity3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagneticFluxDensity3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagneticMoment3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagneticMoment3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagneticPolarization3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagneticPolarization3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagneticVectorPotential3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagneticVectorPotential3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagnetization3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMagnetization3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMassFlow3dCoordinateFrame",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMassFlow3dVector",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMomentOfForce3dCoordinateFrame",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMomentOfForce3dVector",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMomentum3dCoordinateFrame",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianMomentum3dVector",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianParticleCurrentDensity3dCoordinateFrame",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianParticleCurrentDensity3dVector",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianParticlePosition3dVector",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianPosition3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("CartesianPositionOf", &["SpatialFrames"]),
    (
        "CartesianPoynting3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianPoynting3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianRollingResistance3dVector",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianSoundIntensity3dCoordinateFrame",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianSoundIntensity3dVector",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianSoundParticleAcceleration3dVector",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianSoundParticleDisplacement3dVector",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianSoundParticleVelocity3dVector",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianSpatial3dCoordinateFrame",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("CartesianSpatialFrame", &["SpatialFrames"]),
    (
        "CartesianSpin3dCoordinateFrame",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianSpin3dVector",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianStaticFrictionForce3dVector",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("CartesianThreeVectorOf", &["VectorFunctions"]),
    (
        "CartesianThreeVectorValue",
        &["VectorValues", "VectorFunctions"],
    ),
    (
        "CartesianTotalAngularMomentum3dCoordinateFrame",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianTotalAngularMomentum3dVector",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianTotalCurrentDensity3dCoordinateFrame",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianTotalCurrentDensity3dVector",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("CartesianVectorOf", &["VectorFunctions"]),
    ("CartesianVectorValue", &["VectorValues", "VectorFunctions"]),
    (
        "CartesianVelocity3dCoordinateFrame",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianVelocity3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianWave3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianWaveVector3dCoordinateFrame",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CartesianWeight3dVector",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Case", &["Cases"]),
    (
        "CauchyNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Causation", &["CausationConnections", "CauseAndEffect"]),
    ("CausationMetadata", &["CauseAndEffect"]),
    ("CausationSemanticMetadadata", &["CauseAndEffect"]),
    ("CauseMetadata", &["CauseAndEffect"]),
    (
        "CavitationNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CelsiusTemperatureUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CelsiusTemperatureValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ChandrasekharNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("ChangeMonitor", &["Observation", "Triggers"]),
    ("ChangeSignal", &["Observation", "Triggers"]),
    (
        "ChannelCapacityPerCharacterUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ChannelCapacityPerCharacterValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ChannelTimeCapacityUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ChannelTimeCapacityValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CharacterMeanEntropyUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CharacterMeanEntropyValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CharacterMeanTransinformationContentUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CharacterMeanTransinformationContentValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ChargeNumberValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ChemicalPotentialUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ChemicalPotentialValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Circle", &["ShapeItems"]),
    ("CircularCone", &["ShapeItems"]),
    ("CircularCylinder", &["ShapeItems"]),
    ("CircularDisc", &["ShapeItems"]),
    (
        "ClausiusNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Clock", &["Time"]),
    (
        "CoefficientOfHeatTransferUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CoefficientOfHeatTransferValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CoercivityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CoercivityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Collection", &["Collections", "CollectionFunctions"]),
    (
        "CompletedCallIntensityUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CompletedCallIntensityValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Complex",
        &[
            "ScalarValues",
            "BooleanFunctions",
            "ComplexFunctions",
            "IntegerFunctions",
            "NaturalFunctions",
            "NumericalFunctions",
            "RationalFunctions",
            "RealFunctions",
            "ScalarFunctions",
            "StringFunctions",
        ],
    ),
    (
        "CompressibilityNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CompressibilityUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CompressibilityValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("ConcernCheck", &["Requirements"]),
    (
        "ConditionalEntropyUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ConditionalEntropyValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ConditionalInformationContentUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ConditionalInformationContentValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ConductanceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ConductanceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ConductivityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ConductivityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Cone", &["ShapeItems"]),
    ("ConeOrCylinder", &["ShapeItems"]),
    ("ConicSection", &["ShapeItems"]),
    ("ConicSurface", &["ShapeItems"]),
    ("Connection", &["Connections"]),
    ("ConstraintCheck", &["Constraints"]),
    (
        "ContinuousStateSpaceDynamics",
        &["StateSpaceRepresentation"],
    ),
    ("ControlAction", &["Actions"]),
    ("ConversionByConvention", &["MeasurementReferences"]),
    ("ConversionByPrefix", &["MeasurementReferences"]),
    ("ConvertQuantity", &["QuantityCalculations"]),
    ("CoordinateFrame", &["MeasurementReferences"]),
    ("CoordinateFramePlacement", &["MeasurementReferences"]),
    ("CoordinateTransformation", &["MeasurementReferences"]),
    ("CountValue", &["MeasurementReferences"]),
    (
        "CouplingFactorValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CowlingNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CubicExpansionCoefficientUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CubicExpansionCoefficientValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Cuboid", &["ShapeItems"]),
    ("CuboidOrTriangularPrism", &["ShapeItems"]),
    ("CurrentDisplacementOf", &["SpatialItems"]),
    ("CurrentPositionOf", &["SpatialItems"]),
    (
        "CurvatureUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CurvatureValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("CyclicRatioScale", &["MeasurementReferences"]),
    ("Cylinder", &["ShapeItems"]),
    (
        "CylindricalDisplacement3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CylindricalPosition3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "CylindricalSpatial3dCoordinateFrame",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DampingCoefficientUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DampingCoefficientValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DarcyFrictionFactorValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("DataValue", &["Base"]),
    ("Date", &["Time"]),
    ("DateTime", &["Time"]),
    (
        "DeanNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DeborahNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DebyeWallerFactorValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DecayConstantUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DecayConstantValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("DecisionAction", &["Actions"]),
    (
        "DecisionContentValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("DecisionPerformance", &["ControlPerformances"]),
    ("DecisionTransitionAction", &["Actions"]),
    ("DefaultFrameLife", &["SpatialFrames"]),
    ("DefaultMonitorLife", &["Observation", "Triggers"]),
    ("DefinitionalQuantityValue", &["MeasurementReferences"]),
    (
        "DegeneracyValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DegreeOfDissociationValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DensityOfHeatFlowRateUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DensityOfHeatFlowRateValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DensityOfVibrationalStatesUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DensityOfVibrationalStatesValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Derivation",
        &["DerivationConnections", "RequirementDerivation"],
    ),
    ("DerivationMetadata", &["RequirementDerivation"]),
    ("DerivedRequirementMetadata", &["RequirementDerivation"]),
    ("DerivedUnit", &["MeasurementReferences"]),
    ("DesignConstraintCheck", &["Requirements"]),
    (
        "DiffusionCoefficientUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DiffusionCoefficientValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("DimensionOneUnit", &["MeasurementReferences"]),
    ("DimensionOneValue", &["MeasurementReferences"]),
    (
        "DirectionAndEnergyDistributionOfCrossSectionUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DirectionAndEnergyDistributionOfCrossSectionValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DirectionDistributionOfCrossSectionUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DirectionDistributionOfCrossSectionValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Disc", &["ShapeItems"]),
    ("DiscreteStateSpaceDynamics", &["StateSpaceRepresentation"]),
    (
        "Displacement3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DisplacementCurrentDensityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DisplacementCurrentDensityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("DisplacementOf", &["SpatialItems"]),
    ("Domain", &["SampledFunctions"]),
    (
        "DonorDensityUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DonorDensityValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DoseEquivalentUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DoseEquivalentValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DragCoefficientValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("DurationOf", &["Time"]),
    (
        "DurationUnit",
        &["ISQBase", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DurationValue",
        &["ISQBase", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DynamicCapillaryNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DynamicViscosityUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "DynamicViscosityValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("EccentricCone", &["ShapeItems"]),
    ("EccentricCylinder", &["ShapeItems"]),
    (
        "EckertNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("EffectMetadata", &["CauseAndEffect"]),
    (
        "EkmanNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElasticityNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricChargeDensityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricChargeDensityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricChargeUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricChargeValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricConstantUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricConstantValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricCurrentDensityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricCurrentDensityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricCurrentUnit",
        &["ISQBase", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricCurrentValue",
        &["ISQBase", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricDipoleMomentUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricDipoleMomentValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricFieldParameterValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricFieldStrengthUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricFieldStrengthValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricFluxDensityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricFluxDensityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricFluxUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricFluxValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricPolarizationUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricPolarizationValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricPotentialDifferenceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricPotentialDifferenceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricPotentialUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricPotentialValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectricSusceptibilityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectrolyticConductivityUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectrolyticConductivityValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectromagneticEnergyDensityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectromagneticEnergyDensityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectronDensityUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ElectronDensityValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Ellipse", &["ShapeItems"]),
    ("Ellipsoid", &["ShapeItems"]),
    (
        "EmissivityAtASpecifiedWavelengthValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EmissivityValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EnergyDensityOfStatesUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EnergyDensityOfStatesValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EnergyDistributionOfCrossSectionUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EnergyDistributionOfCrossSectionValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EnergyFluenceRateUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EnergyFluenceRateValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EnergyFluenceUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EnergyFluenceValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EnergyUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EnergyValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EntropyForInformationScienceUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EntropyForInformationScienceValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EntropyUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EntropyValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EquilibriumConstantOnConcentrationBasisUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EquilibriumConstantOnConcentrationBasisValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EquilibriumConstantOnPressureBasisUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EquilibriumConstantOnPressureBasisValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EquivalentBinaryDigitRateUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EquivalentBinaryDigitRateValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EquivalentBinaryStorageCapacityUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EquivalentBinaryStorageCapacityValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EquivocationUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EquivocationValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ErrorProbabilityValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "EulerNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Evaluation", &["Performances"]),
    ("EvaluationFunction", &["TradeStudies"]),
    (
        "EvaluationResultMonitorPerformance",
        &["FeatureReferencingPerformances"],
    ),
    (
        "ExpansionNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ExposureRateUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ExposureRateValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ExposureUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ExposureValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "FanningNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "FastFissionFactorUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "FastFissionFactorValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "FeatureAccessPerformance",
        &["FeatureReferencingPerformances"],
    ),
    (
        "FeatureMonitorPerformance",
        &["FeatureReferencingPerformances"],
    ),
    ("FeatureReadEvaluation", &["FeatureReferencingPerformances"]),
    (
        "FeatureReferencingPerformance",
        &["FeatureReferencingPerformances"],
    ),
    (
        "FeatureWritePerformance",
        &["FeatureReferencingPerformances"],
    ),
    ("Flow", &["Flows"]),
    ("FlowTransfer", &["Transfers"]),
    ("FlowTransferBefore", &["Transfers"]),
    ("ForLoopAction", &["Actions"]),
    (
        "ForceUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ForceValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("ForkAction", &["Actions"]),
    (
        "FourierNumberForMassTransferValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "FourierNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "FrequencyUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "FrequencyValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "FroudeNumberForHeatTransferValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "FroudeNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "FugacityUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "FugacityValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("FunctionalRequirementCheck", &["Requirements"]),
    (
        "FundamentalReciprocalLatticeVectorMagnitudeUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "FundamentalReciprocalLatticeVectorMagnitudeValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "GFactorOfNucleusOrNuclearParticleValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "GalileiNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("GeneralView", &["StandardViewDefinitions"]),
    ("GeometryView", &["StandardViewDefinitions"]),
    ("GetDerivative", &["StateSpaceRepresentation"]),
    ("GetDifference", &["StateSpaceRepresentation"]),
    ("GetNextState", &["StateSpaceRepresentation"]),
    ("GetOutput", &["StateSpaceRepresentation"]),
    (
        "GoertlerNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "GraetzNumberForMassTransferValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "GraetzNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "GrandCanonicalPartitionFunctionValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("GraphicalRendering", &["Views"]),
    (
        "GrashofMagneticNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "GrashofNumberForMassTransferValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "GrashofNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("GridView", &["StandardViewDefinitions"]),
    (
        "GyromagneticRatioOfTheElectronUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "GyromagneticRatioOfTheElectronValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "GyromagneticRatioUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "GyromagneticRatioValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HagenNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HallCoefficientUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HallCoefficientValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HallNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("HappensLink", &["Occurrences"]),
    (
        "HartmannNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HartreeEnergyUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HartreeEnergyValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HeatCapacityUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HeatCapacityValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HeatFlowRateUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HeatFlowRateValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HeatTransferNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HoleDensityUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HoleDensityValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "HookeNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Hyperbola", &["ShapeItems"]),
    ("Hyperboloid", &["ShapeItems"]),
    ("Icon", &["ImageMetadata"]),
    ("IfElsePerformance", &["ControlPerformances"]),
    ("IfPerformance", &["ControlPerformances"]),
    ("IfThenAction", &["Actions"]),
    ("IfThenElseAction", &["Actions"]),
    ("IfThenElsePerformance", &["ControlPerformances"]),
    ("IfThenPerformance", &["ControlPerformances"]),
    (
        "IlluminanceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IlluminanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Image", &["ImageMetadata"]),
    (
        "ImpedanceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ImpedanceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ImpulseUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ImpulseValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("IncomingTransferSort", &["Occurrences"]),
    (
        "InductanceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "InductanceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "InfiniteMultiplicationFactorUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "InfiniteMultiplicationFactorValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "InformationContentUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "InformationContentValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Input", &["StateSpaceRepresentation"]),
    (
        "Integer",
        &[
            "ScalarValues",
            "BooleanFunctions",
            "ComplexFunctions",
            "IntegerFunctions",
            "NaturalFunctions",
            "NumericalFunctions",
            "RationalFunctions",
            "RealFunctions",
            "ScalarFunctions",
            "StringFunctions",
        ],
    ),
    ("Integrate", &["StateSpaceRepresentation"]),
    ("InterconnectionView", &["StandardViewDefinitions"]),
    ("Interface", &["Interfaces"]),
    ("InterfaceRequirementCheck", &["Requirements"]),
    (
        "InternalConversionFactorValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Interpolate", &["SampledFunctions"]),
    ("IntervalScale", &["MeasurementReferences"]),
    (
        "IntrinsicCarrierDensityUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IntrinsicCarrierDensityValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IonNumberDensityUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IonNumberDensityValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IonicStrengthUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IonicStrengthValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IrradianceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IrradianceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IrrelevanceUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IrrelevanceValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IsentropicCompressibilityUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IsentropicCompressibilityValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IsentropicExponentValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Iso8601DateTime", &["Time"]),
    ("Iso8601DateTimeEncoding", &["Time"]),
    ("Iso8601DateTimeStructure", &["Time"]),
    (
        "IsothermalCompressibilityUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "IsothermalCompressibilityValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Issue", &["ModelingMetadata"]),
    ("Item", &["Items"]),
    (
        "JFactorValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("JoinAction", &["Actions"]),
    (
        "JointInformationContentUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "JointInformationContentValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "JouleMagneticNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "JouleThomsonCoefficientUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "JouleThomsonCoefficientValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "KermaRateUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "KermaRateValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "KermaUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "KermaValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("KeyValuePair", &["Collections", "CollectionFunctions"]),
    (
        "KinematicViscosityUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "KinematicViscosityValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "KineticFrictionFactorValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "KnudsenNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LagrangeNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LandauGinzburgNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LandeFactorValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LaplaceNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LarmorFrequencyUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LarmorFrequencyValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LavalNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LeakageFactorValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Length", &["StringFunctions"]),
    ("LengthUnit", &["ISQBase", "ISQ", "SI", "USCustomaryUnits"]),
    ("LengthValue", &["ISQBase", "ISQ", "SI", "USCustomaryUnits"]),
    (
        "LethargyValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Level", &["RiskMetadata"]),
    (
        "LewisNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LiftCoefficientValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Line", &["ShapeItems"]),
    ("Linear", &["SampledFunctions"]),
    (
        "LinearAbsorptionCoefficientUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearAbsorptionCoefficientValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearAttenuationCoefficientForIonizingRadiationUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearAttenuationCoefficientForIonizingRadiationValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearAttenuationCoefficientUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearAttenuationCoefficientValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearDensityOfElectricChargeUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearDensityOfElectricChargeValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearElectricCurrentDensityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearElectricCurrentDensityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearEnergyTransferUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearEnergyTransferValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearExpansionCoefficientUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearExpansionCoefficientValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearIonizationUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearIonizationValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearMassDensityUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinearMassDensityValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Link", &["Links"]),
    (
        "LinkedFluxUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LinkedFluxValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("List", &["Collections", "CollectionFunctions"]),
    ("LiteralBooleanEvaluation", &["Performances"]),
    ("LiteralEvaluation", &["Performances"]),
    ("LiteralIntegerEvaluation", &["Performances"]),
    ("LiteralRationalEvaluation", &["Performances"]),
    ("LiteralStringEvaluation", &["Performances"]),
    (
        "LockhartMartinelliParameterValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LogarithmicDecrementValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LogarithmicFrequencyRangeUnit",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LogarithmicFrequencyRangeValue",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("LogarithmicScale", &["MeasurementReferences"]),
    (
        "LongRangeOrderParameterValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("LoopAction", &["Actions"]),
    ("LoopPerformance", &["ControlPerformances"]),
    (
        "LorentzNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LorenzCoefficientUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LorenzCoefficientValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LossFactorValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LossProbabilityValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminanceFactorValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminanceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousAbsorptanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousEfficacyOfASourceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousEfficacyOfASourceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousEfficacyOfRadiationUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousEfficacyOfRadiationValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousEfficiencyValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousEnergyUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousEnergyValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousExitanceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousExitanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousExposureUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousExposureValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousFluxUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousFluxValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousIntensityUnit",
        &["ISQBase", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousIntensityValue",
        &["ISQBase", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousReflectanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LuminousTransmittanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "LundquistNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MachNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticConstantUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticConstantValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticDipoleMomentUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticDipoleMomentValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticFieldStrengthUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticFieldStrengthValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticFluxDensityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticFluxDensityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticFluxUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticFluxValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticMomentUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticMomentValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticPolarizationUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticPolarizationValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticPressureNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticSusceptibilityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticVectorPotentialUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagneticVectorPotentialValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagnetizationUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagnetizationValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagnetomotiveForceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MagnetomotiveForceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Map", &["Collections", "CollectionFunctions"]),
    (
        "MarangoniNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassAbsorptionCoefficientUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassAbsorptionCoefficientValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassAttenuationCoefficientForIonizingRadiationUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassAttenuationCoefficientForIonizingRadiationValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassAttenuationCoefficientUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassAttenuationCoefficientValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassChangeRateUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassChangeRateValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassConcentrationOfWaterUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassConcentrationOfWaterValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassConcentrationOfWaterVapourAbsoluteHumidityUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassConcentrationOfWaterVapourAbsoluteHumidityValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassConcentrationUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassConcentrationValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassDensityUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassDensityValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassEnergyTransferCoefficientUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassEnergyTransferCoefficientValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassFlowRateUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassFlowRateValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassFlowUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassFlowValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassFractionOfDryMatterValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassFractionOfWaterValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassFractionValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassRatioOfWaterToDryMatterValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassRatioOfWaterVapourToDryGasValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassTransferFactorValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("MassUnit", &["ISQBase", "ISQ", "SI", "USCustomaryUnits"]),
    ("MassValue", &["ISQBase", "ISQ", "SI", "USCustomaryUnits"]),
    (
        "MassieuFunctionUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MassieuFunctionValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("MaximizeObjective", &["TradeStudies"]),
    (
        "MaximumEntropyUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MaximumEntropyValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MaximumLuminousEfficacyUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MaximumLuminousEfficacyValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MaximumThermalEfficiencyValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MeanMassRangeUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MeanMassRangeValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MeanQueueLengthValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MeanTransinformationContentUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MeanTransinformationContentValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("MeasureOfEffectiveness", &["ParametersOfInterestMetadata"]),
    ("MeasureOfPerformance", &["ParametersOfInterestMetadata"]),
    ("MeasurementScale", &["MeasurementReferences"]),
    ("MeasurementUnit", &["MeasurementReferences"]),
    (
        "MechanicalEfficiencyValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("MergeAction", &["Actions"]),
    ("MergePerformance", &["ControlPerformances"]),
    ("Message", &["Flows"]),
    ("MessageAction", &["Flows"]),
    ("MessageTransfer", &["Transfers"]),
    ("MetadataAccessEvaluation", &["Performances"]),
    ("MetadataItem", &["Metadata"]),
    ("Metaobject", &["Metaobjects"]),
    ("MinimizeObjective", &["TradeStudies"]),
    (
        "MobilityRatioValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MobilityUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MobilityValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ModulationRateUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ModulationRateValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ModulusOfAdmittanceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ModulusOfAdmittanceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ModulusOfCompressionUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ModulusOfCompressionValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ModulusOfElasticityUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ModulusOfElasticityValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ModulusOfImpedanceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ModulusOfImpedanceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ModulusOfRigidityUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ModulusOfRigidityValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolalityUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolalityValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarAbsorptionCoefficientUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarAbsorptionCoefficientValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarAttenuationCoefficientUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarAttenuationCoefficientValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarConductivityUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarConductivityValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarEnthalpyUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarEnthalpyValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarEntropyUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarEntropyValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarGasConstantUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarGasConstantValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarGibbsEnergyUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarGibbsEnergyValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarHeatCapacityUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarHeatCapacityValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarHelmholtzEnergyUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarHelmholtzEnergyValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarInternalEnergyUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarInternalEnergyValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarMassUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarMassValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarOpticalRotatoryPowerUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarOpticalRotatoryPowerValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarVolumeUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolarVolumeValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MolecularPartitionFunctionValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MomentOfForceUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MomentOfForceValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MomentOfInertiaUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MomentOfInertiaValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MomentumUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MomentumValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MortonNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Multicausation",
        &["CausationConnections", "CauseAndEffect"],
    ),
    ("MulticausationSemanticMetadata", &["CauseAndEffect"]),
    (
        "MultiplicationFactorUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "MultiplicationFactorValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "NapierianAbsorbanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Natural",
        &[
            "ScalarValues",
            "BooleanFunctions",
            "ComplexFunctions",
            "IntegerFunctions",
            "NaturalFunctions",
            "NumericalFunctions",
            "RationalFunctions",
            "RealFunctions",
            "ScalarFunctions",
            "StringFunctions",
        ],
    ),
    (
        "NazeNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "NonLeakageProbabilityUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "NonLeakageProbabilityValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("NonStateTransitionPerformance", &["TransitionPerformances"]),
    (
        "NormalStressUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "NormalStressValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "NuclearActivityUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "NuclearActivityValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "NuclearQuadrupoleMomentUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "NuclearQuadrupoleMomentValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("NullEvaluation", &["Performances"]),
    ("NullTransformation", &["MeasurementReferences"]),
    (
        "Number",
        &[
            "ScalarValues",
            "BooleanFunctions",
            "ComplexFunctions",
            "IntegerFunctions",
            "NaturalFunctions",
            "NumericalFunctions",
            "RationalFunctions",
            "RealFunctions",
            "ScalarFunctions",
            "StringFunctions",
        ],
    ),
    (
        "NumericalValue",
        &[
            "ScalarValues",
            "BooleanFunctions",
            "ComplexFunctions",
            "IntegerFunctions",
            "NaturalFunctions",
            "NumericalFunctions",
            "RationalFunctions",
            "RealFunctions",
            "ScalarFunctions",
            "StringFunctions",
        ],
    ),
    ("NumericalVectorValue", &["VectorValues", "VectorFunctions"]),
    (
        "NusseltElectricNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "NusseltNumberForMassTransferValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "NusseltNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Object", &["Objects"]),
    ("ObserveChange", &["Observation", "Triggers"]),
    ("Occurrence", &["Occurrences"]),
    (
        "OhnesorgeNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("OrderedCollection", &["Collections", "CollectionFunctions"]),
    ("OrderedMap", &["Collections", "CollectionFunctions"]),
    ("OrderedSet", &["Collections", "CollectionFunctions"]),
    ("OrdinalScale", &["MeasurementReferences"]),
    ("OriginalRequirementMetadata", &["RequirementDerivation"]),
    (
        "OsmoticFactorOfSolventValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "OsmoticPressureUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "OsmoticPressureValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Output", &["StateSpaceRepresentation"]),
    (
        "PackingFractionValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Parabola", &["ShapeItems"]),
    ("Paraboloid", &["ShapeItems"]),
    ("Part", &["Parts"]),
    (
        "PartialPressureUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PartialPressureValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleConcentrationUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleConcentrationValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleCurrentDensityUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleCurrentDensityValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleEmissionRateUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleEmissionRateValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleFluenceRateUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleFluenceRateValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleFluenceUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleFluenceValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleNumberDensityUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleNumberDensityValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleSourceDensityUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ParticleSourceDensityValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("PassIf", &["VerificationCases"]),
    ("Path", &["ShapeItems"]),
    ("Performance", &["Performances"]),
    ("PerformanceRequirementCheck", &["Requirements"]),
    (
        "PermeabilityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PermeabilityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PermeanceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PermeanceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PermittivityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PermittivityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhaseCoefficientUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhaseCoefficientValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhaseDifferenceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhaseDifferenceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhaseSpeedOfElectromagneticWavesUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhaseSpeedOfElectromagneticWavesValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhaseVelocityUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhaseVelocityValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonExitanceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonExitanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonExposureUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonExposureValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonFluxUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonFluxValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonIntensityUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonIntensityValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonIrradianceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonIrradianceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonNumberValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonRadianceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PhotonRadianceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("PhysicalRequirementCheck", &["Requirements"]),
    ("PlanarCurve", &["ShapeItems"]),
    ("PlanarSurface", &["ShapeItems"]),
    (
        "PlanckFunctionUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PlanckFunctionValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PlanetaryPosition3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PlanetarySpatial3dCoordinateFrame",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PoiseuilleNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PoissonNumberValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Polygon", &["ShapeItems"]),
    ("Polyhedron", &["ShapeItems"]),
    (
        "PomerantsevNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Port", &["Ports"]),
    (
        "Position3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("PositionOf", &["SpatialItems"]),
    (
        "Positive",
        &[
            "ScalarValues",
            "BooleanFunctions",
            "ComplexFunctions",
            "IntegerFunctions",
            "NaturalFunctions",
            "NumericalFunctions",
            "RationalFunctions",
            "RealFunctions",
            "ScalarFunctions",
            "StringFunctions",
        ],
    ),
    (
        "PowerFactorValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PowerNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PowerUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PowerValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PoyntingVectorMagnitudeUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PoyntingVectorMagnitudeValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PrandtlMagneticNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PrandtlNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PressureCoefficientUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PressureCoefficientValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PressureUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PressureValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PropagationCoefficientUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "PropagationCoefficientValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Pyramid", &["ShapeItems"]),
    ("Quadrilateral", &["ShapeItems"]),
    (
        "QualityFactorForIonizingRadiationUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "QualityFactorForIonizingRadiationValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "QualityFactorValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("QuantityDimension", &["Quantities"]),
    ("QuantityPowerFactor", &["Quantities"]),
    ("QuantityValueMapping", &["MeasurementReferences"]),
    (
        "QuantumNumberValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadianceFactorValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadianceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadianceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadiantEnergyDensityUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadiantEnergyDensityValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadiantExitanceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadiantExitanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadiantExposureUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadiantExposureValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadiantFluxUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadiantFluxValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadiantIntensityUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RadiantIntensityValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Range", &["SampledFunctions"]),
    (
        "RatioOfSpecificHeatCapacitiesValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Rational",
        &[
            "ScalarValues",
            "BooleanFunctions",
            "ComplexFunctions",
            "IntegerFunctions",
            "NaturalFunctions",
            "NumericalFunctions",
            "RationalFunctions",
            "RealFunctions",
            "ScalarFunctions",
            "StringFunctions",
        ],
    ),
    ("Rationale", &["ModelingMetadata"]),
    (
        "RayleighNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ReactanceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ReactanceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "Real",
        &[
            "ScalarValues",
            "BooleanFunctions",
            "ComplexFunctions",
            "IntegerFunctions",
            "NaturalFunctions",
            "NumericalFunctions",
            "RationalFunctions",
            "RealFunctions",
            "ScalarFunctions",
            "StringFunctions",
        ],
    ),
    (
        "RecombinationCoefficientUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RecombinationCoefficientValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Rectangle", &["ShapeItems"]),
    ("RectangularCuboid", &["ShapeItems"]),
    ("RectangularPyramid", &["ShapeItems"]),
    ("RectangularToroid", &["ShapeItems"]),
    (
        "RedundancyUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RedundancyValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ReechNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Refinement", &["ModelingMetadata"]),
    (
        "ReflectanceFactorValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ReflectanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RefractiveIndexValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativeAtomicMassValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativeEntropyValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativeHumidityValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativeLinearStrainValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativeMassConcentrationOfVapourValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativeMassDefectValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativeMassDensityValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativeMassExcessValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativeMassRatioOfVapourValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativePermeabilityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativePermittivityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativePressureCoefficientUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativePressureCoefficientValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativeRedundancyValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RelativeVolumeStrainValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ReluctanceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ReluctanceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Rendering", &["Views"]),
    (
        "RepetencyUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RepetencyValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("RequirementCheck", &["Requirements"]),
    ("RequirementConstraintCheck", &["Requirements"]),
    (
        "ResistanceToAlternatingCurrentUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ResistanceToAlternatingCurrentValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ResistanceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ResistanceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ResistivityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ResistivityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ResonanceEscapeProbabilityValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ReynoldsElectricNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ReynoldsMagneticNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ReynoldsNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RichardsonConstantUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RichardsonConstantValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RichardsonNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("RightCircularCone", &["ShapeItems"]),
    ("RightCircularCylinder", &["ShapeItems"]),
    ("RightTriangle", &["ShapeItems"]),
    ("RightTriangularPrism", &["ShapeItems"]),
    ("Risk", &["RiskMetadata"]),
    ("RiskLevel", &["RiskMetadata"]),
    (
        "RobertsNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RollingResistanceFactorValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RossbyNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Rotation", &["MeasurementReferences"]),
    (
        "RydbergConstantUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "RydbergConstantValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Sample", &["SampledFunctions"]),
    ("SamplePair", &["SampledFunctions"]),
    ("SampledFunction", &["SampledFunctions"]),
    ("ScalarMeasurementReference", &["MeasurementReferences"]),
    ("ScalarQuantityValue", &["Quantities"]),
    (
        "ScalarValue",
        &[
            "ScalarValues",
            "BooleanFunctions",
            "ComplexFunctions",
            "IntegerFunctions",
            "NaturalFunctions",
            "NumericalFunctions",
            "RationalFunctions",
            "RealFunctions",
            "ScalarFunctions",
            "StringFunctions",
        ],
    ),
    (
        "SchmidtNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SecondAxialMomentOfAreaUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SecondAxialMomentOfAreaValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SecondPolarMomentOfAreaUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SecondPolarMomentOfAreaValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SectionModulusUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SectionModulusValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SeebeckCoefficientForSubstancesAAndBUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SeebeckCoefficientForSubstancesAAndBValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("SemanticMetadata", &["Metaobjects"]),
    ("SendAction", &["Actions"]),
    ("SendPerformance", &["Transfers"]),
    ("SequenceView", &["StandardViewDefinitions"]),
    ("Set", &["Collections", "CollectionFunctions"]),
    (
        "ShearStrainValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ShearStressUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ShearStressValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Shell", &["ShapeItems"]),
    (
        "ShortRangeOrderParameterValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("SimpleUnit", &["MeasurementReferences"]),
    (
        "SlowingDownDensityUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SlowingDownDensityValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SolidAngularMeasureUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SolidAngularMeasureValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SommerfeldNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SoundEnergyDensityUnit",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SoundEnergyDensityValue",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SoundExposureLevelUnit",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SoundExposureLevelValue",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SoundExposureUnit",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SoundExposureValue",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SoundIntensityUnit",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SoundIntensityValue",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SoundPowerLevelUnit",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SoundPowerLevelValue",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SoundPressureLevelUnit",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SoundPressureLevelValue",
        &["ISQAcoustics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SourceVoltageUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SourceVoltageValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("SpaceLink", &["Occurrences"]),
    (
        "Spatial3dCoordinateFrame",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("SpatialFrame", &["SpatialFrames"]),
    ("SpatialItem", &["SpatialItems"]),
    (
        "SpecificActivityUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificActivityValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificEnergyUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificEnergyValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificEnthalpyUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificEnthalpyValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificEntropyUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificEntropyValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificGasConstantUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificGasConstantValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificHeatCapacityAtConstantPressureUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificHeatCapacityAtConstantPressureValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificHeatCapacityAtConstantVolumeUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificHeatCapacityAtConstantVolumeValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificHeatCapacityAtSaturatedVapourPressureUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificHeatCapacityAtSaturatedVapourPressureValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificHeatCapacityUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificHeatCapacityValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificOpticalRotatoryPowerUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificOpticalRotatoryPowerValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificVolumeUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpecificVolumeValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralIrradianceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralIrradianceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralLuminousEfficacyUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralLuminousEfficacyValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralLuminousEfficiencyValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadianceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadianceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantEnergyDensityInTermsOfWavelengthValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantEnergyDensityInTermsOfWavenumberValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantEnergyUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantEnergyValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantExitanceUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantExitanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantExposureUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantExposureValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantFluxUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantFluxValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantIntensityUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpectralRadiantIntensityValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpeedOfLightInAMediumUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpeedOfLightInAMediumValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpeedOfLightUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpeedOfLightValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpeedUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpeedValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Sphere", &["ShapeItems"]),
    (
        "SphericalDisplacement3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SphericalPosition3dVector",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SphericalSpatial3dCoordinateFrame",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpinUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SpinValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StandardAbsoluteActivityInMixtureValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StandardAbsoluteActivityInSolutionValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StandardAbsoluteActivityOfSolventValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StandardChemicalPotentialUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StandardChemicalPotentialValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StandardEquilibriumConstantValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StantonNumberForMassTransferValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StantonNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StarkNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("StateAction", &["States"]),
    ("StateDerivative", &["StateSpaceRepresentation"]),
    ("StatePerformance", &["StatePerformances"]),
    ("StateSpace", &["StateSpaceRepresentation"]),
    ("StateSpaceDynamics", &["StateSpaceRepresentation"]),
    ("StateSpaceEventDef", &["StateSpaceRepresentation"]),
    ("StateSpaceItem", &["StateSpaceRepresentation"]),
    ("StateTransitionAction", &["States"]),
    ("StateTransitionPerformance", &["StatePerformances"]),
    ("StateTransitionView", &["StandardViewDefinitions"]),
    (
        "StaticFrictionCoefficientValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("StatusInfo", &["ModelingMetadata"]),
    (
        "StefanNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StoichiometricNumberOfSubstanceValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StokesNumberForDragValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StokesNumberForGravityValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StokesNumberForRotameterValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StokesNumberForVibratingParticlesValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StokesNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StorageCapacityUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StorageCapacityValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StrainUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StrainValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StressUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StressValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "String",
        &[
            "ScalarValues",
            "BooleanFunctions",
            "ComplexFunctions",
            "IntegerFunctions",
            "NaturalFunctions",
            "NumericalFunctions",
            "RationalFunctions",
            "RealFunctions",
            "ScalarFunctions",
            "StringFunctions",
        ],
    ),
    (
        "StrouhalNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StructureFactorValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("StructuredCurve", &["Objects"]),
    ("StructuredPoint", &["Objects"]),
    ("StructuredSpaceObject", &["Objects"]),
    ("StructuredSurface", &["Objects"]),
    (
        "StuartElectricalNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "StuartNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Substring", &["StringFunctions"]),
    ("SuccessionFlow", &["Flows"]),
    (
        "SurfaceActivityDensityUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SurfaceActivityDensityValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SurfaceCoefficientOfHeatTransferUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SurfaceCoefficientOfHeatTransferValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SurfaceDensityOfElectricChargeUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SurfaceDensityOfElectricChargeValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SurfaceMassDensityUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SurfaceMassDensityValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SurfaceTensionUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SurfaceTensionValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SusceptanceUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "SusceptanceValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("SystemOfQuantities", &["Quantities"]),
    ("SystemOfUnits", &["MeasurementReferences"]),
    ("TabularRendering", &["Views"]),
    (
        "TaylorNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TemperatureDifferenceUnit",
        &["ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TemperatureDifferenceValue",
        &["ISQ", "SI", "USCustomaryUnits"],
    ),
    ("TensorMeasurementReference", &["MeasurementReferences"]),
    ("TensorQuantityValue", &["Quantities"]),
    ("TensorScalarMult", &["TensorCalculations"]),
    ("TensorScalarQuantityMult", &["TensorCalculations"]),
    ("TerminateAction", &["Actions"]),
    ("Tetrahedron", &["ShapeItems"]),
    ("TextualRendering", &["Views"]),
    (
        "ThermalConductanceUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalConductanceValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalConductivityUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalConductivityValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalDiffusionCoefficientUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalDiffusionCoefficientValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalDiffusionFactorValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalDiffusionRatioValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalDiffusivityUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalDiffusivityValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalEfficiencyValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalInsulanceUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalInsulanceValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalResistanceUnit",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalResistanceValue",
        &["ISQThermodynamics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalUtilizationFactorUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermalUtilizationFactorValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermodynamicTemperatureUnit",
        &["ISQBase", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThermodynamicTemperatureValue",
        &["ISQBase", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThomsonCoefficientUnit",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "ThomsonCoefficientValue",
        &["ISQCondensedMatter", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("ThreeVectorValue", &["VectorValues", "VectorFunctions"]),
    (
        "ThrustCoefficientValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("TimeInstantValue", &["Time"]),
    ("TimeOf", &["Time"]),
    ("TimeOfDay", &["Time"]),
    ("TimeScale", &["Time"]),
    ("TimeSignal", &["Triggers"]),
    ("ToBoolean", &["BooleanFunctions"]),
    ("ToComplex", &["ComplexFunctions"]),
    ("ToDimensionOneValue", &["QuantityCalculations"]),
    ("ToInteger", &["QuantityCalculations"]),
    ("ToNatural", &["IntegerFunctions"]),
    ("ToRational", &["QuantityCalculations"]),
    ("ToReal", &["QuantityCalculations"]),
    ("ToString", &["MeasurementRefCalculations"]),
    ("ToolExecution", &["AnalysisTooling"]),
    ("ToolVariable", &["AnalysisTooling"]),
    ("Toroid", &["ShapeItems"]),
    (
        "TorqueUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TorqueValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Torus", &["ShapeItems"]),
    (
        "TotalAngularMomentumUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TotalAngularMomentumValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TotalCurrentDensityUnit",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TotalCurrentDensityValue",
        &["ISQElectromagnetism", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TotalIonizationValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TotalLinearStoppingPowerUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TotalLinearStoppingPowerValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TotalMassStoppingPowerUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TotalMassStoppingPowerValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Touches", &["Items"]),
    ("TradeStudy", &["TradeStudies"]),
    ("TradeStudyObjective", &["TradeStudies"]),
    (
        "TrafficCarriedIntensityUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TrafficCarriedIntensityValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TrafficIntensityUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TrafficIntensityValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TrafficOfferedIntensityUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TrafficOfferedIntensityValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Transfer", &["Transfers"]),
    ("TransferBefore", &["Transfers"]),
    (
        "TransferRateUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TransferRateValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TransinformationContentUnit",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TransinformationContentValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("TransitionAction", &["Actions"]),
    ("TransitionPerformance", &["TransitionPerformances"]),
    ("Translation", &["MeasurementReferences"]),
    ("TranslationOrRotation", &["MeasurementReferences"]),
    ("TranslationRotationSequence", &["MeasurementReferences"]),
    (
        "TransmittanceOpticalDensityValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TransmittanceValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TransportNumberOfTheIonBValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("Triangle", &["ShapeItems"]),
    ("TriangularPrism", &["ShapeItems"]),
    ("TriggerAfter", &["Triggers"]),
    ("TriggerAt", &["Triggers"]),
    ("TriggerWhen", &["Triggers"]),
    (
        "TristimulusValuesForTheCie1931StandardColorimetricObserverUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TristimulusValuesForTheCie1931StandardColorimetricObserverValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TristimulusValuesForTheCie1964StandardColorimetricObserverUnit",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "TristimulusValuesForTheCie1964StandardColorimetricObserverValue",
        &["ISQLight", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("UniqueCollection", &["Collections", "CollectionFunctions"]),
    ("UnitBoundedReal", &["TrigFunctions"]),
    ("UnitConversion", &["MeasurementReferences"]),
    ("UnitPowerFactor", &["MeasurementReferences"]),
    ("UnitPrefix", &["MeasurementReferences"]),
    ("UniversalClockLife", &["Clocks", "Triggers"]),
    ("UseCase", &["UseCases"]),
    ("UtcTimeInstantValue", &["Time"]),
    ("VectorMeasurementReference", &["MeasurementReferences"]),
    ("VectorOf", &["VectorFunctions"]),
    ("VectorQuantityValue", &["Quantities"]),
    ("VectorValue", &["VectorValues", "VectorFunctions"]),
    ("VerificationCase", &["VerificationCases"]),
    ("VerificationMethod", &["VerificationCases"]),
    ("VerifyUnitPowerFactors", &["MeasurementReferences"]),
    ("View", &["Views"]),
    ("ViewpointCheck", &["Views"]),
    (
        "VolumeFlowRateUnit",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "VolumeFlowRateValue",
        &["ISQMechanics", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "VolumeFractionUnit",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "VolumeFractionValue",
        &["ISQChemistryMolecular", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "VolumeUnit",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "VolumeValue",
        &["ISQSpaceTime", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "VolumicCrossSectionUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "VolumicCrossSectionValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "VolumicTotalCrossSectionUnit",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "VolumicTotalCrossSectionValue",
        &["ISQAtomicNuclear", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "WaitingProbabilityValue",
        &["ISQInformation", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "WeberNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    (
        "WeissenbergNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("WhileLoopAction", &["Actions"]),
    (
        "WomersleyNumberValue",
        &["ISQCharacteristicNumbers", "ISQ", "SI", "USCustomaryUnits"],
    ),
    ("ZeroCrossingEventDef", &["StateSpaceRepresentation"]),
    ("abs", &["QuantityCalculations"]),
    ("add", &["SequenceFunctions"]),
    ("addAt", &["SequenceFunctions"]),
    ("addNew", &["OccurrenceFunctions"]),
    ("addNewAt", &["OccurrenceFunctions"]),
    ("afv", &["StandardViewDefinitions"]),
    ("all", &["Links"]),
    ("allSubstatePerformances", &["StatePerformances"]),
    ("allSubtransitionPerformances", &["StatePerformances"]),
    ("allTrue", &["ControlFunctions"]),
    ("angle", &["VectorCalculations"]),
    ("anyTrue", &["ControlFunctions"]),
    ("arccos", &["TrigFunctions"]),
    ("arcsin", &["TrigFunctions"]),
    ("arctan", &["TrigFunctions"]),
    ("arg", &["ComplexFunctions"]),
    ("bv", &["StandardViewDefinitions"]),
    ("cartesianAngle", &["VectorFunctions"]),
    ("cartesianInner", &["VectorFunctions"]),
    ("cartesianNorm", &["VectorFunctions"]),
    ("cartesianScalarVectorMult", &["VectorFunctions"]),
    ("cartesianVectorScalarMult", &["VectorFunctions"]),
    ("causation", &["CauseAndEffect"]),
    ("cause", &["CauseAndEffect"]),
    ("collect", &["ControlFunctions"]),
    ("contains", &["CollectionFunctions"]),
    ("containsAll", &["CollectionFunctions"]),
    ("cos", &["TrigFunctions"]),
    ("cot", &["TrigFunctions"]),
    ("create", &["OccurrenceFunctions"]),
    ("def", &["ModelingMetadata"]),
    ("deg", &["TrigFunctions"]),
    ("denom", &["RationalFunctions"]),
    ("derivation", &["RequirementDerivation"]),
    ("derive", &["RequirementDerivation"]),
    ("destroy", &["OccurrenceFunctions"]),
    ("effect", &["CauseAndEffect"]),
    ("equals", &["SequenceFunctions"]),
    ("excludes", &["SequenceFunctions"]),
    ("excluding", &["SequenceFunctions"]),
    ("excludingAt", &["SequenceFunctions"]),
    ("excludingOnce", &["Interfaces"]),
    ("exists", &["ControlFunctions"]),
    ("floor", &["QuantityCalculations"]),
    ("forAll", &["ControlFunctions"]),
    ("gcd", &["RationalFunctions"]),
    ("gev", &["StandardViewDefinitions"]),
    ("grv", &["StandardViewDefinitions"]),
    ("gv", &["StandardViewDefinitions"]),
    ("head", &["CollectionFunctions"]),
    ("im", &["ComplexFunctions"]),
    ("includes", &["SequenceFunctions"]),
    ("includesOnly", &["SequenceFunctions"]),
    ("including", &["SequenceFunctions"]),
    ("includingAt", &["SequenceFunctions"]),
    ("index", &["CollectionFunctions"]),
    ("inner", &["VectorCalculations"]),
    ("intersection", &["SequenceFunctions"]),
    ("isCartesianZeroVector", &["VectorFunctions"]),
    ("isDuring", &["OccurrenceFunctions"]),
    ("isEmpty", &["CollectionFunctions"]),
    ("isUnit", &["QuantityCalculations"]),
    ("isUnitTensorQuantity", &["TensorCalculations"]),
    ("isUnitVectorQuantity", &["VectorCalculations"]),
    ("isZero", &["QuantityCalculations"]),
    ("isZeroTensorQuantity", &["TensorCalculations"]),
    ("isZeroVector", &["VectorFunctions"]),
    ("isZeroVectorQuantity", &["VectorCalculations"]),
    ("iv", &["StandardViewDefinitions"]),
    ("last", &["CollectionFunctions"]),
    ("max", &["QuantityCalculations"]),
    ("maximize", &["ControlFunctions"]),
    ("min", &["QuantityCalculations"]),
    ("minimize", &["ControlFunctions"]),
    ("moe", &["ParametersOfInterestMetadata"]),
    ("mop", &["ParametersOfInterestMetadata"]),
    ("multicausation", &["CauseAndEffect"]),
    ("norm", &["VectorCalculations"]),
    ("notEmpty", &["CollectionFunctions"]),
    ("numer", &["RationalFunctions"]),
    ("original", &["RequirementDerivation"]),
    ("outer", &["VectorCalculations"]),
    ("polar", &["ComplexFunctions"]),
    ("product", &["QuantityCalculations"]),
    ("product1", &["NumericalFunctions"]),
    ("rad", &["TrigFunctions"]),
    ("rat", &["RationalFunctions"]),
    ("re", &["ComplexFunctions"]),
    ("rect", &["ComplexFunctions"]),
    ("reduce", &["ControlFunctions"]),
    ("refinement", &["ModelingMetadata"]),
    ("reject", &["ControlFunctions"]),
    ("remove", &["SequenceFunctions"]),
    ("removeAt", &["SequenceFunctions"]),
    ("removeOld", &["OccurrenceFunctions"]),
    ("removeOldAt", &["OccurrenceFunctions"]),
    ("round", &["QuantityCalculations"]),
    ("same", &["SequenceFunctions"]),
    ("scalarQuantityTensorMult", &["TensorCalculations"]),
    ("scalarQuantityVectorMult", &["VectorCalculations"]),
    ("scalarTensorMult", &["TensorCalculations"]),
    ("scalarVectorMult", &["VectorCalculations"]),
    ("select", &["ControlFunctions"]),
    ("selectOne", &["ControlFunctions"]),
    ("sin", &["TrigFunctions"]),
    ("size", &["CollectionFunctions"]),
    ("sqrt", &["QuantityCalculations"]),
    ("struct", &["Objects"]),
    ("stv", &["StandardViewDefinitions"]),
    ("subsequence", &["SequenceFunctions"]),
    ("sum", &["QuantityCalculations"]),
    ("sum0", &["NumericalFunctions"]),
    ("sv", &["StandardViewDefinitions"]),
    ("tail", &["CollectionFunctions"]),
    ("tan", &["TrigFunctions"]),
    ("tensorTensorMult", &["TensorCalculations"]),
    ("tensorVectorMult", &["TensorCalculations"]),
    ("transform", &["TensorCalculations"]),
    ("union", &["SequenceFunctions"]),
    ("vectorScalarDiv", &["VectorCalculations"]),
    ("vectorScalarMult", &["VectorCalculations"]),
    ("vectorScalarQuantityDiv", &["VectorCalculations"]),
    ("vectorScalarQuantityMult", &["VectorCalculations"]),
    ("vectorTensorMult", &["TensorCalculations"]),
];

pub fn is_library_root(name: &str) -> bool {
    LIBRARY_ROOTS.binary_search(&name).is_ok()
}

pub fn is_library_leaf(name: &str) -> bool {
    !exposing_packages(name).is_empty()
}

/// Paquet standard qui **définit** `name`, s'il en existe un.
pub fn suggest_import_for(name: &str) -> Option<&'static str> {
    exposing_packages(name).first().copied()
}

/// Tous les noms de la bibliothèque, triés — pour les suggestions par
/// proximité orthographique.
pub fn all_names() -> impl Iterator<Item = &'static &'static str> {
    LIBRARY_INDEX.iter().map(|(n, _)| n)
}

/// Tous les paquets depuis lesquels `name` est visible : celui qui le définit,
/// puis ses ré-exportateurs. Vide si `name` n'appartient pas à la bibliothèque.
pub fn exposing_packages(name: &str) -> &'static [&'static str] {
    LIBRARY_INDEX
        .binary_search_by(|(n, _)| (*n).cmp(name))
        .map(|i| LIBRARY_INDEX[i].1)
        .unwrap_or(&[])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_are_sorted() {
        assert!(LIBRARY_ROOTS.windows(2).all(|w| w[0] < w[1]));
        assert!(LIBRARY_INDEX.windows(2).all(|w| w[0].0 < w[1].0));
    }

    #[test]
    fn known_names_resolve_to_their_package() {
        assert_eq!(suggest_import_for("String"), Some("ScalarValues"));
        assert_eq!(suggest_import_for("Flow"), Some("Flows"));
        assert_eq!(suggest_import_for("Connection"), Some("Connections"));
    }

    #[test]
    fn reexports_are_followed() {
        // `MassValue` est défini dans `ISQBase` et ré-exporté par `ISQ` :
        // les deux imports doivent le rendre visible.
        let pkgs = exposing_packages("MassValue");
        assert_eq!(pkgs.first().copied(), Some("ISQBase"));
        assert!(pkgs.contains(&"ISQ"), "{pkgs:?}");
    }

    #[test]
    fn roots_are_recognised() {
        assert!(is_library_root("Connections"));
        assert!(is_library_root("ScalarValues"));
    }

    #[test]
    fn flow_connection_is_not_a_standard_type() {
        // `FlowConnection` n'existe nulle part dans la bibliothèque : le type
        // de flux standard est `Flows::Flow`. Régression pour un faux négatif
        // plausible — le nom *semble* standard mais ne l'est pas.
        assert!(!is_library_leaf("FlowConnection"));
        assert!(is_library_leaf("Flow"));
    }
}
