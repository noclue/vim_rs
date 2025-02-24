use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Thrown when an operation is denied because of privileges
/// not held on managed object(s).
pub trait NoPermissionTrait : super::security_error_trait::SecurityErrorTrait {
    /// Deprecated as of vSphere 8.0, use the *NoPermission.missingPrivileges* field.
    /// 
    /// The managed object on which a permission is required.
    fn get_object(&self) -> &Option<ManagedObjectReference>;
    /// Deprecated as of vSphere 8.0, use the *NoPermission.missingPrivileges* field.
    /// 
    /// The privilege identifier required
    fn get_privilege_id(&self) -> &Option<String>;
    /// List of entities and missing privileges for each entity
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.2
    fn get_missing_privileges(&self) -> &Option<Vec<NoPermissionEntityPrivileges>>;
}
impl<'s> serde::Serialize for dyn NoPermissionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn NoPermissionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(NoPermissionVisitor)
            }
        }

struct NoPermissionVisitor;

impl<'de> de::Visitor<'de> for NoPermissionVisitor {
    type Value = Box<dyn NoPermissionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid NoPermissionTrait JSON object with a _typeName field")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let deserializer = de::value::MapAccessDeserializer::new(&mut map);
        let any: VimAny = de::Deserialize::deserialize(deserializer)?;
        match any {
            VimAny::Object(obj) => Ok(CastFrom::from_box(obj)
                .map_err(|_| de::Error::custom("Internal error converting to trait type"))?),
            VimAny::Value(value) => Err(de::Error::custom(format!(
                "expected object not wrapped value: {:?}",
                value))),
        }
    }
}

impl NoPermissionTrait for NoPermission {
    fn get_object(&self) -> &Option<ManagedObjectReference> { &self.object }
    fn get_privilege_id(&self) -> &Option<String> { &self.privilege_id }
    fn get_missing_privileges(&self) -> &Option<Vec<NoPermissionEntityPrivileges>> { &self.missing_privileges }
}
impl NoPermissionTrait for NotAuthenticated {
    fn get_object(&self) -> &Option<ManagedObjectReference> { &self.object }
    fn get_privilege_id(&self) -> &Option<String> { &self.privilege_id }
    fn get_missing_privileges(&self) -> &Option<Vec<NoPermissionEntityPrivileges>> { &self.missing_privileges }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn NoPermissionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::NoPermission => Some(from.as_any_ref().downcast_ref::<NoPermission>()?),
            StructType::NotAuthenticated => Some(from.as_any_ref().downcast_ref::<NotAuthenticated>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::NoPermission => Ok(from.as_any_box().downcast::<NoPermission>()?),
            StructType::NotAuthenticated => Ok(from.as_any_box().downcast::<NotAuthenticated>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
