use aws_sdk_comprehendmedical::operation::infer_icd10_cm::InferIcd10CmOutput;
use aws_sdk_comprehendmedical::types::{
    Icd10CmAttribute, Icd10CmAttributeType, Icd10CmConcept, Icd10CmEntity, Icd10CmEntityCategory,
    Icd10CmEntityType, Icd10CmRelationshipType, Icd10CmTrait, Icd10CmTraitName,
};
use serde::{Deserialize, Serialize};

use crate::comprehendmedical::primitives::sealed_enum_unknown::UnknownVariantValue_;

#[derive(Serialize, Deserialize, Default)]
pub struct InferIcd10CmOutput_ {
    pub entities: Vec<Icd10CmEntity_>,
    pub pagination_token: Option<String>,
    pub model_version: Option<String>,
}

impl From<InferIcd10CmOutput> for InferIcd10CmOutput_ {
    fn from(output: InferIcd10CmOutput) -> Self {
        InferIcd10CmOutput_ {
            entities: output
                .entities
                .into_iter()
                .map(|entity| entity.into())
                .collect(),
            pagination_token: output.pagination_token,
            model_version: output.model_version,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Icd10CmEntity_ {
    pub id: Option<i32>,
    pub text: Option<String>,
    pub category: Option<Icd10CmEntityCategory_>,
    pub type_: Option<Icd10CmEntityType_>,
    pub score: Option<f32>,
    pub begin_offset: Option<i32>,
    pub end_offset: Option<i32>,
    pub attributes: Option<Vec<Icd10CmAttribute_>>,
    pub traits: Option<Vec<Icd10CmTrait_>>,
    pub icd10_cm_concepts: Option<Vec<Icd10CmConcept_>>,
}

impl From<Icd10CmEntity> for Icd10CmEntity_ {
    fn from(entity: Icd10CmEntity) -> Self {
        Icd10CmEntity_ {
            id: entity.id,
            text: entity.text,
            category: entity.category.map(|category| category.into()),
            type_: entity.r#type.map(|type_| type_.into()),
            score: entity.score,
            begin_offset: entity.begin_offset,
            end_offset: entity.end_offset,
            attributes: entity.attributes.map(|attributes| {
                attributes
                    .into_iter()
                    .map(|attribute| attribute.into())
                    .collect()
            }),
            traits: entity
                .traits
                .map(|traits| traits.into_iter().map(|trait_| trait_.into()).collect()),
            icd10_cm_concepts: entity
                .icd10_cm_concepts
                .map(|concepts| concepts.into_iter().map(|concept| concept.into()).collect()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Icd10CmEntityCategory_ {
    MedicalCondition,
    Unknown(UnknownVariantValue_),
}

impl Default for Icd10CmEntityCategory_ {
    fn default() -> Self {
        Icd10CmEntityCategory_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<Icd10CmEntityCategory> for Icd10CmEntityCategory_ {
    fn from(value: Icd10CmEntityCategory) -> Self {
        match value {
            Icd10CmEntityCategory::MedicalCondition => Icd10CmEntityCategory_::MedicalCondition,
            other if other.as_str() == "NewFeature" => {
                Icd10CmEntityCategory_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => Icd10CmEntityCategory_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Icd10CmEntityType_ {
    DxName,
    TimeExpression,
    Unknown(UnknownVariantValue_),
}

impl Default for Icd10CmEntityType_ {
    fn default() -> Self {
        Icd10CmEntityType_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<Icd10CmEntityType> for Icd10CmEntityType_ {
    fn from(value: Icd10CmEntityType) -> Self {
        match value {
            Icd10CmEntityType::DxName => Icd10CmEntityType_::DxName,
            Icd10CmEntityType::TimeExpression => Icd10CmEntityType_::TimeExpression,
            other if other.as_str() == "NewFeature" => {
                Icd10CmEntityType_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => Icd10CmEntityType_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Icd10CmAttribute_ {
    pub type_: Option<Icd10CmAttributeType_>,
    pub score: Option<f32>,
    pub relationship_score: Option<f32>,
    pub id: Option<i32>,
    pub begin_offset: Option<i32>,
    pub end_offset: Option<i32>,
    pub text: Option<String>,
    pub traits: Option<Vec<Icd10CmTrait_>>,
    pub category: Option<Icd10CmEntityType_>,
    pub relationship_type: Option<Icd10CmRelationshipType_>,
}

impl From<Icd10CmAttribute> for Icd10CmAttribute_ {
    fn from(attribute: Icd10CmAttribute) -> Self {
        Icd10CmAttribute_ {
            type_: attribute.r#type.map(|type_| type_.into()),
            score: attribute.score,
            relationship_score: attribute.relationship_score,
            id: attribute.id,
            begin_offset: attribute.begin_offset,
            end_offset: attribute.end_offset,
            text: attribute.text,
            traits: attribute
                .traits
                .map(|traits| traits.into_iter().map(|trait_| trait_.into()).collect()),
            category: attribute.category.map(|category| category.into()),
            relationship_type: attribute
                .relationship_type
                .map(|relationship_type| relationship_type.into()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Icd10CmAttributeType_ {
    Acuity,
    Direction,
    Quality,
    Quantity,
    SystemOrganSite,
    TimeExpression,
    TimeToDxName,
    Unknown(UnknownVariantValue_),
}

impl Default for Icd10CmAttributeType_ {
    fn default() -> Self {
        Icd10CmAttributeType_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<Icd10CmAttributeType> for Icd10CmAttributeType_ {
    fn from(value: Icd10CmAttributeType) -> Self {
        match value {
            Icd10CmAttributeType::Acuity => Icd10CmAttributeType_::Acuity,
            Icd10CmAttributeType::Direction => Icd10CmAttributeType_::Direction,
            Icd10CmAttributeType::Quality => Icd10CmAttributeType_::Quality,
            Icd10CmAttributeType::Quantity => Icd10CmAttributeType_::Quantity,
            Icd10CmAttributeType::SystemOrganSite => Icd10CmAttributeType_::SystemOrganSite,
            Icd10CmAttributeType::TimeExpression => Icd10CmAttributeType_::TimeExpression,
            Icd10CmAttributeType::TimeToDxName => Icd10CmAttributeType_::TimeToDxName,
            other if other.as_str() == "NewFeature" => {
                Icd10CmAttributeType_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => Icd10CmAttributeType_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Icd10CmTrait_ {
    pub name: Option<Icd10CmTraitName_>,
    pub score: Option<f32>,
}

impl From<Icd10CmTrait> for Icd10CmTrait_ {
    fn from(trait_: Icd10CmTrait) -> Self {
        Icd10CmTrait_ {
            name: trait_.name.map(|name| name.into()),
            score: trait_.score,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Icd10CmTraitName_ {
    Diagnosis,
    Hypothetical,
    LowConfidence,
    Negation,
    PertainsToFamily,
    Sign,
    Symptom,
    Unknown(UnknownVariantValue_),
}

impl Default for Icd10CmTraitName_ {
    fn default() -> Self {
        Icd10CmTraitName_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<Icd10CmTraitName> for Icd10CmTraitName_ {
    fn from(value: Icd10CmTraitName) -> Self {
        match value {
            Icd10CmTraitName::Diagnosis => Icd10CmTraitName_::Diagnosis,
            Icd10CmTraitName::Hypothetical => Icd10CmTraitName_::Hypothetical,
            Icd10CmTraitName::LowConfidence => Icd10CmTraitName_::LowConfidence,
            Icd10CmTraitName::Negation => Icd10CmTraitName_::Negation,
            Icd10CmTraitName::PertainsToFamily => Icd10CmTraitName_::PertainsToFamily,
            Icd10CmTraitName::Sign => Icd10CmTraitName_::Sign,
            Icd10CmTraitName::Symptom => Icd10CmTraitName_::Symptom,
            other if other.as_str() == "NewFeature" => {
                Icd10CmTraitName_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => Icd10CmTraitName_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Icd10CmRelationshipType_ {
    Overlap,
    Quality,
    SystemOrganSite,
    Unknown(UnknownVariantValue_),
}

impl Default for Icd10CmRelationshipType_ {
    fn default() -> Self {
        Icd10CmRelationshipType_::Unknown(UnknownVariantValue_("Unknown".to_string()))
    }
}

impl From<Icd10CmRelationshipType> for Icd10CmRelationshipType_ {
    fn from(value: Icd10CmRelationshipType) -> Self {
        match value {
            Icd10CmRelationshipType::Overlap => Icd10CmRelationshipType_::Overlap,
            Icd10CmRelationshipType::Quality => Icd10CmRelationshipType_::Quality,
            Icd10CmRelationshipType::SystemOrganSite => Icd10CmRelationshipType_::SystemOrganSite,
            other if other.as_str() == "NewFeature" => {
                Icd10CmRelationshipType_::Unknown(UnknownVariantValue_(other.as_str().to_string()))
            }
            _ => Icd10CmRelationshipType_::Unknown(UnknownVariantValue_("Unknown".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Icd10CmConcept_ {
    pub description: Option<String>,
    pub code: Option<String>,
    pub score: Option<f32>,
}

impl From<Icd10CmConcept> for Icd10CmConcept_ {
    fn from(concept: Icd10CmConcept) -> Self {
        Icd10CmConcept_ {
            description: concept.description,
            code: concept.code,
            score: concept.score,
        }
    }
}
