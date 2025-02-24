use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *ProfileExecuteResult* data object contains the results from a
/// *HostProfile*.*HostProfile.ExecuteHostProfile*
/// operation.
pub trait ProfileExecuteResultTrait : super::data_object_trait::DataObjectTrait {
    /// Status of the profile execution operation.
    /// 
    /// The value is a string that contains
    /// one of the *ProfileExecuteResultStatus_enum* enumerations.
    fn get_status(&self) -> &str;
    /// Host configuration specification.
    /// 
    /// This data is valid only if
    /// the <code>status</code> value is <code>success</code>.
    /// See *ProfileExecuteResultStatus_enum*.
    /// 
    /// Use this data object when you apply the configuration
    /// to a host. See the <code>configSpec</code> parameter to the
    /// *HostProfileManager*.*HostProfileManager.ApplyHostConfig_Task*
    /// method.
    fn get_config_spec(&self) -> &Option<HostConfigSpec>;
    /// List of property paths.
    /// 
    /// Each path identifies a policy that does not apply
    /// to this host. For example, if the precheck policies for a port group are not satisfied,
    /// the port group will not be created when you apply the profile to the host.
    /// Based on this information, the client might not display that part of the profile tree.
    fn get_inapplicable_path(&self) -> &Option<Vec<String>>;
    /// List that describes the required input for host configuration and identifies
    /// any policy options that still require parameter data.
    /// 
    /// Each entry in the list
    /// specifies the path to a policy and a parameter list. If the call to
    /// *HostProfile.ExecuteHostProfile* includes deferred parameters,
    /// the <code>requireInput</code> entries
    /// (<code>requireInput\[\].</code>*ProfileDeferredPolicyOptionParameter.parameter*\[\])
    /// will be populated with the parameter data that was passed to the execute method.
    /// For policies that still require input data, the parameter list in the corresponding
    /// entry will be null.
    /// 
    /// A vSphere client that displays a GUI can use this information to show the host-specific
    /// configuration policy options. The client can highlight required input fields
    /// and ask the user for data in increments instead of collecting all of the input at once.
    /// For example, in the first pass, the client collects a minimum of user input and
    /// sends that to the Server. The Server evaluates the profile and might decide to
    /// invalidate a particular part of the subtree or enable a new
    /// subtree in the profile. This would result in a new set of invalid paths
    /// (*ProfileExecuteResult.inapplicablePath*\[\]) and
    /// required input property paths
    /// (*ProfileDeferredPolicyOptionParameter*.*ProfileDeferredPolicyOptionParameter.inputPath*).
    /// The client can make a series of calls to the method until it achieves a success status.
    /// 
    /// When *HostProfile.ExecuteHostProfile* returns a success status,
    /// the <code>requireInput</code> list contains the complete list of parameters,
    /// consisting of the following data:
    /// - Deferred parameter values resolved through successive calls to
    ///   *HostProfile.ExecuteHostProfile*.
    /// - Default parameter values from the host configuration.
    /// - User-specified values that override the defaults.
    ///   
    /// You can specify the returned <code>requireInput</code> list in the
    /// <code>userInput</code> parameter to the
    /// *HostProfileManager*.*HostProfileManager.ApplyHostConfig_Task*
    /// method. The Server will use the list to update the *AnswerFile*
    /// associated with the host.
    fn get_require_input(&self) -> &Option<Vec<ProfileDeferredPolicyOptionParameter>>;
    /// List of errors that were encountered during execute.
    /// 
    /// This field will be set if status is set to error.
    fn get_error(&self) -> &Option<Vec<ProfileExecuteError>>;
}
impl<'s> serde::Serialize for dyn ProfileExecuteResultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ProfileExecuteResultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ProfileExecuteResultVisitor)
            }
        }

struct ProfileExecuteResultVisitor;

impl<'de> de::Visitor<'de> for ProfileExecuteResultVisitor {
    type Value = Box<dyn ProfileExecuteResultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ProfileExecuteResultTrait JSON object with a _typeName field")
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

impl ProfileExecuteResultTrait for ProfileExecuteResult {
    fn get_status(&self) -> &str { &self.status }
    fn get_config_spec(&self) -> &Option<HostConfigSpec> { &self.config_spec }
    fn get_inapplicable_path(&self) -> &Option<Vec<String>> { &self.inapplicable_path }
    fn get_require_input(&self) -> &Option<Vec<ProfileDeferredPolicyOptionParameter>> { &self.require_input }
    fn get_error(&self) -> &Option<Vec<ProfileExecuteError>> { &self.error }
}
impl ProfileExecuteResultTrait for ApplyHostProfileConfigurationSpec {
    fn get_status(&self) -> &str { &self.status }
    fn get_config_spec(&self) -> &Option<HostConfigSpec> { &self.config_spec }
    fn get_inapplicable_path(&self) -> &Option<Vec<String>> { &self.inapplicable_path }
    fn get_require_input(&self) -> &Option<Vec<ProfileDeferredPolicyOptionParameter>> { &self.require_input }
    fn get_error(&self) -> &Option<Vec<ProfileExecuteError>> { &self.error }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ProfileExecuteResultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfileExecuteResult => Some(from.as_any_ref().downcast_ref::<ProfileExecuteResult>()?),
            StructType::ApplyHostProfileConfigurationSpec => Some(from.as_any_ref().downcast_ref::<ApplyHostProfileConfigurationSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfileExecuteResult => Ok(from.as_any_box().downcast::<ProfileExecuteResult>()?),
            StructType::ApplyHostProfileConfigurationSpec => Ok(from.as_any_box().downcast::<ApplyHostProfileConfigurationSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
