use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This describes the arguments to *GuestProcessManager.StartProgramInGuest*.
pub trait GuestProgramSpecTrait : super::data_object_trait::DataObjectTrait {
    /// The absolute path to the program to start.
    /// 
    /// For Linux guest operating systems, /bin/bash is used to start the
    /// program.
    /// 
    /// For Solaris guest operating systems, /bin/bash is used to start
    /// the program if it exists.
    /// Otherwise /bin/sh is used. If /bin/sh is used, then the process ID
    /// returned by *GuestProcessManager.StartProgramInGuest* will be that of the shell used
    /// to start the program, rather than the program itself, due to the
    /// differences in how /bin/sh and /bin/bash work. This PID will
    /// still be usable for watching the process with
    /// *GuestProcessManager.ListProcessesInGuest* to
    /// find its exit code and elapsed time.
    fn get_program_path(&self) -> &str;
    /// The arguments to the program.
    /// 
    /// In Linux and Solaris guest operating
    /// systems, the program will be executed by a guest shell.
    /// This allows stdio redirection, but may also
    /// require that characters which must be escaped to the shell also
    /// be escaped on the command line provided.
    /// 
    /// For Windows guest operating systems, prefixing the command with
    /// "cmd /c" can provide stdio redirection.
    fn get_arguments(&self) -> &str;
    /// The absolute path of the working directory for the program to be
    /// run.
    /// 
    /// VMware recommends explicitly setting the working directory
    /// for the program to be run. If this value is unset or is an empty
    /// string, the behavior depends on the guest operating system.
    /// For Linux guest operating systems, if this value is unset or is
    /// an empty string, the working directory will be the home directory
    /// of the user associated with the guest authentication.
    /// For other guest operating systems, if this value is unset, the
    /// behavior is unspecified.
    fn get_working_directory(&self) -> &Option<String>;
    /// An array of environment variables, specified
    /// in the guest OS notation (eg PATH=c:\\bin;c:\\windows\\system32
    /// or LD\_LIBRARY\_PATH=/usr/lib:/lib), to be set for the program
    /// being run.
    /// 
    /// Note that these are not additions to the default
    /// environment variables; they define the complete set available to
    /// the program. If none are specified the values are guest dependent.
    fn get_env_variables(&self) -> &Option<Vec<String>>;
}
impl<'s> serde::Serialize for dyn GuestProgramSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn GuestProgramSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(GuestProgramSpecVisitor)
            }
        }

struct GuestProgramSpecVisitor;

impl<'de> de::Visitor<'de> for GuestProgramSpecVisitor {
    type Value = Box<dyn GuestProgramSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid GuestProgramSpecTrait JSON object with a _typeName field")
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

impl GuestProgramSpecTrait for GuestProgramSpec {
    fn get_program_path(&self) -> &str { &self.program_path }
    fn get_arguments(&self) -> &str { &self.arguments }
    fn get_working_directory(&self) -> &Option<String> { &self.working_directory }
    fn get_env_variables(&self) -> &Option<Vec<String>> { &self.env_variables }
}
impl GuestProgramSpecTrait for GuestWindowsProgramSpec {
    fn get_program_path(&self) -> &str { &self.program_path }
    fn get_arguments(&self) -> &str { &self.arguments }
    fn get_working_directory(&self) -> &Option<String> { &self.working_directory }
    fn get_env_variables(&self) -> &Option<Vec<String>> { &self.env_variables }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn GuestProgramSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestProgramSpec => Some(from.as_any_ref().downcast_ref::<GuestProgramSpec>()?),
            StructType::GuestWindowsProgramSpec => Some(from.as_any_ref().downcast_ref::<GuestWindowsProgramSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestProgramSpec => Ok(from.as_any_box().downcast::<GuestProgramSpec>()?),
            StructType::GuestWindowsProgramSpec => Ok(from.as_any_box().downcast::<GuestWindowsProgramSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
