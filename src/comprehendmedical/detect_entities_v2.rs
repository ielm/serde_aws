use aws_sdk_comprehendmedical::operation::detect_entities_v2::DetectEntitiesV2Output;
use aws_sdk_comprehendmedical::types::{
    Attribute, AttributeName, Entity, EntitySubType, EntityType, RelationshipType, Trait,
    UnmappedAttribute,
};
use serde::{Deserialize, Serialize};

use crate::comprehendmedical::primitives::sealed_enum_unknown::UnknownVariantValue_;

#[derive(Serialize, Deserialize, Default)]
pub struct DetectEntitiesV2Output_ {
    pub entities: Vec<Entity_>,
    pub unmapped_attributes: Option<Vec<UnmappedAttribute_>>,
    pub pagination_token: Option<String>,
    pub model_version: String,
}

impl From<DetectEntitiesV2Output> for DetectEntitiesV2Output_ {
    fn from(value: DetectEntitiesV2Output) -> Self {
        Self {
            entities: value.entities.into_iter().map(Entity_::from).collect(),
            unmapped_attributes: value.unmapped_attributes.map(|unmapped_attributes| {
                unmapped_attributes
                    .into_iter()
                    .map(UnmappedAttribute_::from)
                    .collect()
            }),
            pagination_token: value.pagination_token,
            model_version: value.model_version,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Entity_ {
    pub id: Option<i32>,
    pub begin_offset: Option<i32>,
    pub end_offset: Option<i32>,
    pub score: Option<f32>,
    pub text: Option<String>,
    pub category: Option<EntityType_>,
    #[serde(rename = "type")]
    pub type_: Option<EntitySubType_>,
    pub traits: Option<Vec<Trait_>>,
    pub attributes: Option<Vec<Attribute_>>,
}

impl From<Entity> for Entity_ {
    fn from(value: Entity) -> Self {
        Self {
            id: value.id,
            begin_offset: value.begin_offset,
            end_offset: value.end_offset,
            score: value.score,
            text: value.text,
            category: value.category.map(EntityType_::from),
            type_: value.r#type.map(EntitySubType_::from),
            traits: value
                .traits
                .map(|traits| traits.into_iter().map(Trait_::from).collect()),
            attributes: value
                .attributes
                .map(|attributes| attributes.into_iter().map(Attribute_::from).collect()),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum EntityType_ {
    Anatomy,
    BehavioralEnvironmentalSocial,
    MedicalCondition,
    Medication,
    ProtectedHealthInformation,
    TestTreatmentProcedure,
    TimeExpression,
    Unknown(UnknownVariantValue_),
}

impl Default for EntityType_ {
    fn default() -> Self {
        EntityType_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<EntityType> for EntityType_ {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::Anatomy => EntityType_::Anatomy,
            EntityType::BehavioralEnvironmentalSocial => EntityType_::BehavioralEnvironmentalSocial,
            EntityType::MedicalCondition => EntityType_::MedicalCondition,
            EntityType::Medication => EntityType_::Medication,
            EntityType::ProtectedHealthInformation => EntityType_::ProtectedHealthInformation,
            EntityType::TestTreatmentProcedure => EntityType_::TestTreatmentProcedure,
            EntityType::TimeExpression => EntityType_::TimeExpression,
            other if other.as_str() == "NewFeature" => {
                EntityType_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => EntityType_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum EntitySubType_ {
    Acuity,
    Address,
    Age,
    AlcoholConsumption,
    Allergies,
    Amount,
    BrandName,
    ContactPoint,
    Date,
    Direction,
    Dosage,
    Duration,
    DxName,
    Email,
    Form,
    Frequency,
    Gender,
    GenericName,
    Id,
    Identifier,
    Name,
    PhoneOrFax,
    ProcedureName,
    Profession,
    Quality,
    Quantity,
    RaceEthnicity,
    Rate,
    RecDrugUse,
    RouteOrMode,
    Strength,
    SystemOrganSite,
    TestName,
    TestUnit,
    TestUnits,
    TestValue,
    TimeExpression,
    TimeToDxName,
    TimeToMedicationName,
    TimeToProcedureName,
    TimeToTestName,
    TimeToTreatmentName,
    TobaccoUse,
    TreatmentName,
    Url,
    Unknown(UnknownVariantValue_),
}

impl Default for EntitySubType_ {
    fn default() -> Self {
        EntitySubType_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<EntitySubType> for EntitySubType_ {
    fn from(value: EntitySubType) -> Self {
        match value {
            EntitySubType::Acuity => EntitySubType_::Acuity,
            EntitySubType::Address => EntitySubType_::Address,
            EntitySubType::Age => EntitySubType_::Age,
            EntitySubType::AlcoholConsumption => EntitySubType_::AlcoholConsumption,
            EntitySubType::Allergies => EntitySubType_::Allergies,
            EntitySubType::Amount => EntitySubType_::Amount,
            EntitySubType::BrandName => EntitySubType_::BrandName,
            EntitySubType::ContactPoint => EntitySubType_::ContactPoint,
            EntitySubType::Date => EntitySubType_::Date,
            EntitySubType::Direction => EntitySubType_::Direction,
            EntitySubType::Dosage => EntitySubType_::Dosage,
            EntitySubType::Duration => EntitySubType_::Duration,
            EntitySubType::DxName => EntitySubType_::DxName,
            EntitySubType::Email => EntitySubType_::Email,
            EntitySubType::Form => EntitySubType_::Form,
            EntitySubType::Frequency => EntitySubType_::Frequency,
            EntitySubType::Gender => EntitySubType_::Gender,
            EntitySubType::GenericName => EntitySubType_::GenericName,
            EntitySubType::Id => EntitySubType_::Id,
            EntitySubType::Identifier => EntitySubType_::Identifier,
            EntitySubType::Name => EntitySubType_::Name,
            EntitySubType::PhoneOrFax => EntitySubType_::PhoneOrFax,
            EntitySubType::ProcedureName => EntitySubType_::ProcedureName,
            EntitySubType::Profession => EntitySubType_::Profession,
            EntitySubType::Quality => EntitySubType_::Quality,
            EntitySubType::Quantity => EntitySubType_::Quantity,
            EntitySubType::RaceEthnicity => EntitySubType_::RaceEthnicity,
            EntitySubType::Rate => EntitySubType_::Rate,
            EntitySubType::RecDrugUse => EntitySubType_::RecDrugUse,
            EntitySubType::RouteOrMode => EntitySubType_::RouteOrMode,
            EntitySubType::Strength => EntitySubType_::Strength,
            EntitySubType::SystemOrganSite => EntitySubType_::SystemOrganSite,
            EntitySubType::TestName => EntitySubType_::TestName,
            EntitySubType::TestUnit => EntitySubType_::TestUnit,
            EntitySubType::TestUnits => EntitySubType_::TestUnits,
            EntitySubType::TestValue => EntitySubType_::TestValue,
            EntitySubType::TimeExpression => EntitySubType_::TimeExpression,
            EntitySubType::TimeToDxName => EntitySubType_::TimeToDxName,
            EntitySubType::TimeToMedicationName => EntitySubType_::TimeToMedicationName,
            EntitySubType::TimeToProcedureName => EntitySubType_::TimeToProcedureName,
            EntitySubType::TimeToTestName => EntitySubType_::TimeToTestName,
            EntitySubType::TimeToTreatmentName => EntitySubType_::TimeToTreatmentName,
            EntitySubType::TobaccoUse => EntitySubType_::TobaccoUse,
            EntitySubType::TreatmentName => EntitySubType_::TreatmentName,
            EntitySubType::Url => EntitySubType_::Url,
            other if other.as_str() == "NewFeature" => {
                EntitySubType_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => EntitySubType_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Trait_ {
    pub name: Option<AttributeName_>,
    pub score: Option<f32>,
}

impl From<Trait> for Trait_ {
    fn from(value: Trait) -> Self {
        Self {
            name: value.name.map(AttributeName_::from),
            score: value.score,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Attribute_ {
    #[serde(rename = "type")]
    pub type_: Option<EntitySubType_>,
    pub score: Option<f32>,
    pub relationship_score: Option<f32>,
    pub relationship_type: Option<RelationshipType_>,
    pub id: Option<i32>,
    pub begin_offset: Option<i32>,
    pub end_offset: Option<i32>,
    pub text: Option<String>,
    pub category: Option<EntityType_>,
    pub traits: Option<Vec<Trait_>>,
}

impl From<Attribute> for Attribute_ {
    fn from(value: Attribute) -> Self {
        Self {
            type_: value.r#type.map(EntitySubType_::from),
            score: value.score,
            relationship_score: value.relationship_score,
            relationship_type: value.relationship_type.map(RelationshipType_::from),
            id: value.id,
            begin_offset: value.begin_offset,
            end_offset: value.end_offset,
            text: value.text,
            category: value.category.map(EntityType_::from),
            traits: value
                .traits
                .map(|traits| traits.into_iter().map(Trait_::from).collect()),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum RelationshipType_ {
    Acuity,
    AdministeredVia,
    Amount,
    Direction,
    Dosage,
    Duration,
    Every,
    For,
    Form,
    Frequency,
    Negative,
    Overlap,
    Quality,
    Rate,
    RouteOrMode,
    Strength,
    SystemOrganSite,
    TestUnit,
    TestUnits,
    TestValue,
    Usage,
    WithDosage,
    Unknown(UnknownVariantValue_),
}

impl Default for RelationshipType_ {
    fn default() -> Self {
        RelationshipType_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<RelationshipType> for RelationshipType_ {
    fn from(value: RelationshipType) -> Self {
        match value {
            RelationshipType::Acuity => RelationshipType_::Acuity,
            RelationshipType::AdministeredVia => RelationshipType_::AdministeredVia,
            RelationshipType::Amount => RelationshipType_::Amount,
            RelationshipType::Direction => RelationshipType_::Direction,
            RelationshipType::Dosage => RelationshipType_::Dosage,
            RelationshipType::Duration => RelationshipType_::Duration,
            RelationshipType::Every => RelationshipType_::Every,
            RelationshipType::For => RelationshipType_::For,
            RelationshipType::Form => RelationshipType_::Form,
            RelationshipType::Frequency => RelationshipType_::Frequency,
            RelationshipType::Negative => RelationshipType_::Negative,
            RelationshipType::Overlap => RelationshipType_::Overlap,
            RelationshipType::Quality => RelationshipType_::Quality,
            RelationshipType::Rate => RelationshipType_::Rate,
            RelationshipType::RouteOrMode => RelationshipType_::RouteOrMode,
            RelationshipType::Strength => RelationshipType_::Strength,
            RelationshipType::SystemOrganSite => RelationshipType_::SystemOrganSite,
            RelationshipType::TestUnit => RelationshipType_::TestUnit,
            RelationshipType::TestUnits => RelationshipType_::TestUnits,
            RelationshipType::TestValue => RelationshipType_::TestValue,
            RelationshipType::Usage => RelationshipType_::Usage,
            RelationshipType::WithDosage => RelationshipType_::WithDosage,
            other if other.as_str() == "NewFeature" => {
                RelationshipType_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => RelationshipType_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum AttributeName_ {
    Diagnosis,
    Future,
    Hypothetical,
    LowConfidence,
    Negation,
    PastHistory,
    PertainsToFamily,
    Sign,
    Symptom,
    Unknown(UnknownVariantValue_),
}

impl Default for AttributeName_ {
    fn default() -> Self {
        AttributeName_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<AttributeName> for AttributeName_ {
    fn from(value: AttributeName) -> Self {
        match value {
            AttributeName::Diagnosis => AttributeName_::Diagnosis,
            AttributeName::Future => AttributeName_::Future,
            AttributeName::Hypothetical => AttributeName_::Hypothetical,
            AttributeName::LowConfidence => AttributeName_::LowConfidence,
            AttributeName::Negation => AttributeName_::Negation,
            AttributeName::PastHistory => AttributeName_::PastHistory,
            AttributeName::PertainsToFamily => AttributeName_::PertainsToFamily,
            AttributeName::Sign => AttributeName_::Sign,
            AttributeName::Symptom => AttributeName_::Symptom,
            other if other.as_str() == "NewFeature" => {
                AttributeName_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => AttributeName_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct UnmappedAttribute_ {
    #[serde(rename = "type")]
    pub type_: Option<EntityType_>,
    pub attribute: Option<Attribute_>,
}

impl From<UnmappedAttribute> for UnmappedAttribute_ {
    fn from(value: UnmappedAttribute) -> Self {
        Self {
            type_: value.r#type.map(EntityType_::from),
            attribute: value.attribute.map(Attribute_::from),
        }
    }
}
