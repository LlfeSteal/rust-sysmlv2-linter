//! Connaissance minimale de la bibliothèque standard SysML v2 / KerML.
//!
//! Le vérificateur ne charge pas la bibliothèque : il se contente de savoir
//! quelles racines et quels noms lui appartiennent, afin de ne pas signaler
//! comme « inconnu » ce qui vient légitimement de la bibliothèque.

/// Paquets racines de la bibliothèque standard.
pub const LIBRARY_ROOTS: &[&str] = &[
    // KerML — Kernel Semantic Library
    "Base",
    "Occurrences",
    "Objects",
    "Performances",
    "Transfers",
    "Links",
    "FeatureReferencingPerformances",
    "ControlPerformances",
    "StatePerformances",
    "TransitionPerformances",
    "Clocks",
    "Observation",
    "SpatialFrames",
    "Metaobjects",
    "KerML",
    // KerML — Data Type Library
    "ScalarValues",
    "VectorValues",
    "Collections",
    "ScalarFunctions",
    "BooleanFunctions",
    "NumericalFunctions",
    "StringFunctions",
    "DataFunctions",
    "BaseFunctions",
    "SequenceFunctions",
    "ControlFunctions",
    "TrigFunctions",
    "ComplexFunctions",
    "VectorFunctions",
    "CollectionFunctions",
    "IntegerFunctions",
    "NaturalFunctions",
    "RealFunctions",
    "RationalFunctions",
    "OccurrenceFunctions",
    "ObjectFunctions",
    // SysML — Systems Library
    "Parts",
    "Items",
    "Attributes",
    "Ports",
    "Connections",
    "Interfaces",
    "Flows",
    "Allocations",
    "Actions",
    "States",
    "Calculations",
    "Constraints",
    "Requirements",
    "Cases",
    "AnalysisCases",
    "UseCases",
    "VerificationCases",
    "Views",
    "Viewpoints",
    "Metadata",
    "SysML",
    "Systems",
    "Occurrence",
    "Definitions",
    "Usages",
    "Rendering",
    "Concerns",
    "Enumerations",
    // SysML — Quantities & Units
    "Quantities",
    "MeasurementReferences",
    "ISQ",
    "ISQBase",
    "SI",
    "SIPrefixes",
    "USCustomaryUnits",
    "UnitsAndScales",
    "Time",
    "SpatialItems",
    "Shapes",
    "ShapeItems",
    "Geometry",
    "CoordinateFrames",
    "GeometryFunctions",
    "AnalysisTooling",
    "Trades",
    "TradeStudies",
    "ImageDefinitions",
    "SampledFunctions",
];

/// Noms de types couramment utilisés sans qualification (via `import ... ::*`).
pub const LIBRARY_LEAVES: &[&str] = &[
    // ScalarValues
    "Anything",
    "DataValue",
    "Boolean",
    "String",
    "Number",
    "NumericalValue",
    "Complex",
    "Real",
    "Rational",
    "Integer",
    "Natural",
    "Positive",
    // Collections
    "Collection",
    "Array",
    "Set",
    "OrderedSet",
    "Bag",
    "List",
    "Map",
    "KeyValuePair",
    // Base / Occurrences
    "Base",
    "Occurrence",
    "Life",
    "SelfSameLifeOccurrence",
    "Object",
    "Performance",
    "Transfer",
    "Link",
    "BinaryLink",
    "SelfLink",
    // Systèmes
    "Part",
    "Item",
    "Port",
    "Connection",
    "Interface",
    "Action",
    "State",
    "Constraint",
    "Requirement",
    "Case",
    "UseCase",
    "AnalysisCase",
    "VerificationCase",
    "View",
    "Viewpoint",
    "Rendering",
    "Concern",
    "Stakeholder",
    "Allocation",
    "Flow",
    "Metadata",
    "SemanticMetadata",
    // Quantités & unités fréquentes
    "QuantityValue",
    "ScalarQuantityValue",
    "MeasurementReference",
    "MeasurementUnit",
    "SystemOfUnits",
    "SystemOfQuantities",
    "DimensionOneValue",
    "LengthValue",
    "MassValue",
    "TimeValue",
    "DurationValue",
    "TemperatureValue",
    "SpeedValue",
    "AccelerationValue",
    "ForceValue",
    "EnergyValue",
    "PowerValue",
    "PressureValue",
    "AngleValue",
    "AreaValue",
    "VolumeValue",
    "ElectricCurrentValue",
    "ElectricPotentialValue",
    "FrequencyValue",
    "VoltageValue",
    // Résultats de vérification
    "VerdictKind",
    "PassIf",
    "RequirementConstraintKind",
    "RequirementVerdictKind",
];

pub fn is_library_root(name: &str) -> bool {
    LIBRARY_ROOTS.contains(&name)
}

pub fn is_library_leaf(name: &str) -> bool {
    LIBRARY_LEAVES.contains(&name)
}

/// Paquet standard le plus probable pour un nom de type non qualifié.
pub fn suggest_import_for(name: &str) -> Option<&'static str> {
    match name {
        "Boolean" | "String" | "Number" | "NumericalValue" | "Complex" | "Real" | "Rational"
        | "Integer" | "Natural" | "Positive" | "DataValue" | "Anything" => Some("ScalarValues"),
        "Collection" | "Array" | "Set" | "OrderedSet" | "Bag" | "List" | "Map" => {
            Some("Collections")
        }
        "QuantityValue" | "ScalarQuantityValue" | "MeasurementReference" | "MeasurementUnit" => {
            Some("Quantities")
        }
        _ => {
            if name.ends_with("Value") {
                Some("ISQ")
            } else {
                None
            }
        }
    }
}
