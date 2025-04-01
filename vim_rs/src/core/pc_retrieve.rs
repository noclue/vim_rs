use std::sync::Arc;
use crate::core::client::Client;
use crate::core::pc_helpers;
use crate::core::pc_helpers::{BoxableError, Error, Queriable};
use crate::mo::{ContainerView, PropertyCollector, ViewManager};
use crate::types::enums::MoTypesEnum;
use crate::types::structs::{ManagedObjectReference, ObjectContent, ObjectSpec};

/// A trait for objects that can be retrieved using the PropertyCollector utilities. In essence they
/// provide a `PropertySpec` for the object type and implement TryFrom<ObjectContent> to convert
/// from the `PropertyCollector::retrieve_properties_ex` API response to the object instances.
pub trait Retrievable: Queriable + TryFrom<ObjectContent>
where
    Self::Error: BoxableError
{}

/// Blanket implementation for Retrievable for all Queriable types that implement TryFrom<ObjectContent>.
impl<T: Queriable + TryFrom<ObjectContent, Error = E>, E: BoxableError> Retrievable for T {}

/// The ObjectRetriever struct is responsible for retrieving objects from the vSphere API using the
/// PropertyCollector and ViewManager. It provides methods to retrieve objects from a container or
/// to retrieve specific objects based on a set of ObjectSpecs.
pub struct ObjectRetriever {
    client: Arc<Client>,
    property_collector: PropertyCollector,
    view_manager: ViewManager,
}

impl ObjectRetriever {
    /// Creates a new ObjectRetriever instance. It initializes PropertyCollector and ViewManager
    /// instances using the provided client. It will fail if the client does not have a
    /// view manager.
    pub fn new(client: Arc<Client>) -> pc_helpers::Result<Self> {
        let pc_mo_id = &client.service_content().property_collector.value;
        let property_collector = PropertyCollector::new(client.clone(), pc_mo_id);
        let Some(view_manager_moref) = &client.service_content().view_manager else {
            return Err(Error::InternalError("cannot find view_manager".to_string()));
        };
        let view_manager = ViewManager::new(client.clone(), &view_manager_moref.value);
        Ok(Self {
            client,
            property_collector,
            view_manager,
        })
    }

    /// Retrieves objects of type T from the specified container. It creates a container view and
    /// uses the `retrieve_objects` method to get the objects. The view is destroyed after the
    /// retrieval is complete.
    ///
    /// Use the `vim_macros::vim_retrievable` macro to easily map PropertyCollector property
    /// paths to Rust objects. This macro generates a struct with the specified properties by
    /// resolving the property paths to the correct Rust types. The macro also generates the
    /// [`Retrievable`] trait implementation for the struct, allowing it to be used with the
    /// [`ObjectRetriever`] API.
    pub async fn retrieve_objects_from_container<T: Retrievable>(&self, container: &ManagedObjectReference) -> pc_helpers::Result<Vec<T>>
    where
        <T as TryFrom<crate::types::structs::ObjectContent>>::Error: BoxableError
    {
        let view_moref = self.view_manager.create_container_view(
            container,
            Some(&[T::prop_spec().r#type]),
            true,
        ).await?;

        let view = ContainerView::new(self.client.clone(), &view_moref.value);

        let object_set = vec![ObjectSpec {
            obj: view_moref.clone(),
            skip: Some(false),
            select_set: Some(vec![Box::new(crate::types::structs::TraversalSpec {
                name: Some("traverseEntities".to_string()),
                r#type: Into::<&str>::into(MoTypesEnum::ContainerView).to_string(),
                path: "view".to_string(),
                skip: Some(false),
                select_set: None,
            })]),
        }];

        let res = self.retrieve_objects(object_set).await;

        view.destroy_view().await?;

        res

    }

    /// Retrieves objects of type T based on the provided ObjectSpecs. It uses the
    /// `PropertyCollector` to retrieve the properties of the specified objects. The method
    /// handles pagination and continues to retrieve properties until all objects are fetched.
    ///
    /// Use the `vim_macros::vim_retrievable` macro to easily map PropertyCollector property
    /// paths to Rust objects. This macro generates a struct with the specified properties by
    /// resolving the property paths to the correct Rust types. The macro also generates the
    /// [`Retrievable`] trait implementation for the struct, allowing it to be used with the
    /// [`ObjectRetriever`] API.
    pub async fn retrieve_objects<T: Retrievable>(&self, object_set: Vec<ObjectSpec>) -> pc_helpers::Result<Vec<T>>
    where
        <T as TryFrom<crate::types::structs::ObjectContent>>::Error: BoxableError
    {
        let spec_set = vec![crate::types::structs::PropertyFilterSpec {
            object_set,
            prop_set: vec![T::prop_spec()],
            report_missing_objects_in_results: Some(true),
        }];
        let options = crate::types::structs::RetrieveOptions {
            max_objects: Some(100),
        };

        let mut vms: Vec<T> = Vec::new();

        let retrieve_result = self.property_collector.retrieve_properties_ex(&spec_set, &options).await?;
        let Some(mut res) = retrieve_result else {
            return Ok(Vec::new());
        };
        loop {
            for obj in res.objects {
                vms.push(obj.try_into().map_err(|e| Error::GenericError(Box::new(e)))?);
            };
            // Check for more results
            let Some(token) = res.token else {
                break;
            };
            res = self.property_collector.continue_retrieve_properties_ex(&token).await?;
        }
        Ok(vms)
    }
}