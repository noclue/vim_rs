use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type provides the service interface for obtaining
/// statistical data about various aspects of system performance, as generated
/// and maintained by the system's performance providers.
/// 
/// A "performance
/// provider" (*PerfProviderSummary*) is any managed object
/// that generates utilization or other performance metrics. Performance
/// providers include *managed entities*, such as *hosts*, *virtual machines*, *compute resources*, *resource
/// pools*, *datastores*, and *networks*.
/// Performance providers also include physical or virtual devices associated
/// with these objects, such as virtual host-bus adapters and network-interface
/// controllers (NICs)
/// 
/// <a name="counterTables"></a>**Performance Counters**  
/// Each
/// performance provider&#151;the instrumented device or entity&#151;has its own
/// set of *counters* that provides
/// metadata about its available *metrics*. Each counter has a unique *PerfCounterInfo.key*, referred to as the counterId. The
/// actual performance metrics generated at runtime are identified by this
/// *PerfMetricId.counterId*. Counters are organized by
/// *groups* of finite system
/// resources, such as <a href="memory_counters.html">memory</a>, <a href="cpu_counters.html">CPU</a>, <a href="disk_counters.html">disk</a>, and
/// so on. The links in this list contain documentation for performance
/// counters, by *group*. Each
/// page contains a table that includes data extracted from instances of the
/// *PerfCounterInfo* data object, including the counter
/// name, its Label, Unit, StatsType, RollupType, and Level:
/// - <a href="cluster_services_counters.html">Cluster Services
/// - <a href="cpu_counters.html">CPU</a>
/// - <a href="hbr_counters.html">Host-Based Replication</a>
/// - <a href="mgmt_agent_counters.html">Management Agent</a>
/// - <a href="memory_counters.html">Memory</a>
/// - <a href="network_counters.html">Network</a>
/// - <a href="power_counters.html">Power</a>
/// - <a href="resource_scheduler_counters.html">Resource Scheduler</a>
/// - Storage Capacity:
///   - <a href="disk_storutil_counters.html">Datastore / Virtual
///     Machine</a>
/// - Storage I/O:
///   - <a href="datastore_counters.html">Datastore</a>
///   - <a href="disk_counters.html">Disk</a>
///   - <a href="virtual_disk_counters.html">Virtual Disk</a>
///   - <a href="storage_adapter_counters.html">Storage Adapter</a>
///   - <a href="storage_path_counters.html">Storage Path</a>
/// - <a href="system_counters.html">System</a>
/// - <a href="vcres_counters.html">vCenter Resource</a>
/// - <a href="vmop_counters.html">Virtual Machine Operations</a>
///   
/// Other performance-counter groups, in addition to those listed here,
/// exist on the system. However, only the counter groups listed are considered
/// of possible interest to third-party developers.
/// 
/// **Obtaining Metadata and Metrics**  
/// This interface provides these
/// query operations:
/// - *PerformanceManager.QueryPerfProviderSummary*, for obtaining metatdata about *performance providers*
/// - *PerformanceManager.QueryPerfCounter* and *PerformanceManager.QueryPerfCounterByLevel* for obtaining
///   metadata about supported counters.
/// - *PerformanceManager.QueryPerf*, *PerformanceManager.QueryAvailablePerfMetric*, and *PerformanceManager.QueryPerfComposite* for obtaining statistics for one or more
///   entities:
///   - Use *PerformanceManager.QueryPerf* to obtain metrics for multiple entities in a
///     single call&#46;
///   - Use *PerformanceManager.QueryPerfComposite* to obtain statistics for a single
///     entity with its descendent objects&#151;statistics for a *host* and all its *virtual
///     machines*, for example. 
///     
/// **Product and Version Specifics**  
/// Some differences between ESX
/// and vCenter Server implementations of this interface include:
/// - For ESX systems, this interface provides access to real-time data, and
///   to real-time data that has been rolled up into "PastDay" statistics (if
///   applicable for the specific counter).
/// - For vCenter Server systems, this interface provides access to real-time
///   and historical data. vCenter Server collects statistics on a regular
///   basis from all ESX systems that it manages, and aggregates the results
///   based on the level settings for the server.
/// - Default sampling interval is product- and version-specific:
///   - ESX 3&#46;x (and subsequent) systems: 20 second interval
///   - ESX 2&#46;x systems: 60 second interval
/// - VirtualCenter Server 2&#46;5 (and subsequent vCenter Server) systems
///   initially collect statistics data 10 minutes after system startup, and
///   then hourly thereafter&#46;
///   
/// See the Programming Guide for more information about using *PerformanceManager*&#46;
#[derive(Clone)]
pub struct PerformanceManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl PerformanceManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated as of API 2.5, use *PerformanceManager.UpdatePerfInterval*. The
    /// default historical intervals can be modified, but they cannot be created.
    /// 
    /// Adds a new historical interval.
    /// 
    /// Sampling period for new interval must be
    /// a multiple of an existing interval; must comprise a longer period of
    /// time; and must be uniquely named.
    /// 
    /// ***Required privileges:*** Performance.ModifyIntervals
    ///
    /// ## Parameters:
    ///
    /// ### interval_id
    /// A custom interval, specified as the number of seconds to hold data in the
    /// database, a user-specified unique name, and a sampling period (in
    /// seconds).
    pub async fn create_perf_interval(&self, interval_id: &crate::types::structs::PerfInterval) -> Result<()> {
        let input = CreatePerfIntervalRequestType {interval_id, };
        self.client.invoke_void("", "PerformanceManager", &self.mo_id, "CreatePerfInterval", Some(&input)).await
    }
    /// Retrieves all performance counters for the specified *managed object* generated during a specified
    /// period of time.
    /// 
    /// The time period can be specified using beginTime,
    /// endTime, or by interval ID.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The *managed object* that
    /// provides performance metrics.
    ///
    /// ### begin_time
    /// Starting time (server time) for a period of time from which to return
    /// available metrics. If not specified, defaults to oldest available metric
    /// for the specified entity.
    ///
    /// ### end_time
    /// Ending time (server time) for a period of time from which to return
    /// available performance metrics. If not specified, defaults to the most
    /// recently generated metric for the specified entity.
    ///
    /// ### interval_id
    /// Period of time from which to retrieve metrics, defined by intervalId
    /// (rather than beginTime or endTime). Valid intervalIds include:
    /// - For real-time counters, the *refreshRate* of
    ///   the *performance
    ///   provider*.
    /// - For historical counters, the *samplingPeriod* of the *historical interval*. 
    ///   
    /// If this parameter is not specified, the system returns available metrics
    /// for historical statistics&#46;
    ///
    /// ## Returns:
    ///
    /// An array of metrics, each of which comprises a
    /// *PerfMetricId.counterId* and an
    /// *name*.
    pub async fn query_available_perf_metric(&self, entity: &crate::types::structs::ManagedObjectReference, begin_time: Option<&str>, end_time: Option<&str>, interval_id: Option<i32>) -> Result<Option<Vec<crate::types::structs::PerfMetricId>>> {
        let input = QueryAvailablePerfMetricRequestType {entity, begin_time, end_time, interval_id, };
        let bytes_opt = self.client.invoke_optional("", "PerformanceManager", &self.mo_id, "QueryAvailablePerfMetric", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Retrieves a *PerfCompositeMetric* data object
    /// that comprises statistics for the specified entity and its children
    /// entities.
    /// 
    /// Only metrics for the first level of descendents are included in
    /// the *PerfCompositeMetric* object.
    /// 
    /// Use this operation to obtain statistics for a *host* and its associated *virtual machines*, for
    /// example.
    /// 
    /// Requires **system.read** privilege for every virtual machine on
    /// the target host, or the query fails with the &#147;NoPermission&#148;
    /// fault. Suported for *HostSystem* managed entities only.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### query_spec
    /// A *PerfQuerySpec* object specifying the query
    /// parameters. This *PerfQuerySpec* object specifies a
    /// managed object for which composite statistics should be retrieved, with
    /// specific optional criteria for filtering the results.
    /// 
    /// This *PerfQuerySpec* requires a valid *PerfQuerySpec.metricId* property that specifies a metric
    /// that is available, in common, to the entity and its children. If the
    /// specified metricId is not available to the entity and its children, it is
    /// ignored.
    ///
    /// ## Returns:
    ///
    /// The metric values for the specified entity and its associated
    /// entities for a single interval.
    pub async fn query_perf_composite(&self, query_spec: &crate::types::structs::PerfQuerySpec) -> Result<crate::types::structs::PerfCompositeMetric> {
        let input = QueryPerfCompositeRequestType {query_spec, };
        let bytes = self.client.invoke("", "PerformanceManager", &self.mo_id, "QueryPerfComposite", Some(&input)).await?;
        let result: crate::types::structs::PerfCompositeMetric = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Retrieves counter information for the specified list of counter IDs.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### counter_id
    /// An array of one or more *counterIds* representing performance counters for which information is
    /// being retrieved.
    ///
    /// ## Returns:
    ///
    /// An array consisting of performance counter information for the
    /// specified counterIds.
    pub async fn query_perf_counter(&self, counter_id: &[i32]) -> Result<Option<Vec<crate::types::structs::PerfCounterInfo>>> {
        let input = QueryPerfCounterRequestType {counter_id, };
        let bytes_opt = self.client.invoke_optional("", "PerformanceManager", &self.mo_id, "QueryPerfCounter", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Retrieves the set of counters that are available at a specified
    /// collection *PerfInterval.level*.
    /// 
    /// The collection level
    /// determines the statistics that get stored in VirtualCenter. See *PerfInterval* for more information about VirtualCenter Server
    /// historical statistics collection.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### level
    /// A number between 1 and 4 that specifies the collection level.
    ///
    /// ## Returns:
    ///
    /// An array of *PerfCounterInfo* objects that
    /// define the set of counters having the specified level number available
    /// for the entity.
    pub async fn query_perf_counter_by_level(&self, level: i32) -> Result<Vec<crate::types::structs::PerfCounterInfo>> {
        let input = QueryPerfCounterByLevelRequestType {level, };
        let bytes = self.client.invoke("", "PerformanceManager", &self.mo_id, "QueryPerfCounterByLevel", Some(&input)).await?;
        let result: Vec<crate::types::structs::PerfCounterInfo> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Retrieves the *PerfProviderSummary* data object that
    /// defines the capabilities of the specified managed object with respect to
    /// statistics, such as whether it supports current or summary
    /// statistics&#46;
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// Reference to a managed object that provides performance data. If the
    /// entity specified by managed object reference is not a performance
    /// provider, an "InvalidArgument" exception is thrown.
    ///
    /// ## Returns:
    ///
    /// A data object containing metadata about the entity as a
    /// performance provider, such as the type of metrics (real-time, summary, or
    /// both) it generates and the *PerfProviderSummary.refreshRate*.
    pub async fn query_perf_provider_summary(&self, entity: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::PerfProviderSummary> {
        let input = QueryPerfProviderSummaryRequestType {entity, };
        let bytes = self.client.invoke("", "PerformanceManager", &self.mo_id, "QueryPerfProviderSummary", Some(&input)).await?;
        let result: crate::types::structs::PerfProviderSummary = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Retrieves the performance metrics for the specified entity (or entities)
    /// based on the properties specified in the *PerfQuerySpec* data object.
    /// 
    /// **Query Performance for VirtualCenter Server**  
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### query_spec
    /// An array of *PerfQuerySpec* objects. Each *PerfQuerySpec* object specifies a managed object reference
    /// for an entity, plus optional criteria for filtering results. Only metrics
    /// for entities that can be resolved and that are valid *performance providers* are returned in
    /// any result.
    /// 
    /// Each *PerfQuerySpec* object in the array
    /// submitted in this operation can query for different metrics. Or, select
    /// all types of statistics for a single managed entity.
    /// 
    /// Raw data feed workaround: Normally, QueryPerf will return performance
    /// statistics stored in the VirtualCenter database. However this may not be
    /// suitable for certain applications. For example, applications that treat
    /// VirtualCenter as a raw data source, query for performance statistics
    /// regularly (say every 5 minutes) and extract the data for external
    /// archival and reporting. Such applications need better query performance.
    /// These applications should query statistics using QueryPerf for the base
    /// historical interval (5 minutes by default) having a start and end time
    /// range within 30 minutes from the current VirtualCenter server system
    /// time. These QueryPerf calls will have better performance than other
    /// QueryPerf calls.
    ///
    /// ## Returns:
    ///
    /// The metric values for the specified entity or entities.
    pub async fn query_perf(&self, query_spec: &[crate::types::structs::PerfQuerySpec]) -> Result<Option<Vec<Box<dyn crate::types::traits::PerfEntityMetricBaseTrait>>>> {
        let input = QueryPerfRequestType {query_spec, };
        let bytes_opt = self.client.invoke_optional("", "PerformanceManager", &self.mo_id, "QueryPerf", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Deprecated as of API 2.5, use *PerformanceManager.UpdatePerfInterval*.
    /// Historical intervals cannot be removed.
    /// 
    /// Removes an interval from the list.
    /// 
    /// ***Required privileges:*** Performance.ModifyIntervals
    ///
    /// ## Parameters:
    ///
    /// ### sample_period
    /// The sampling period, in seconds, for the specified interval being
    /// removed.
    pub async fn remove_perf_interval(&self, sample_period: i32) -> Result<()> {
        let input = RemovePerfIntervalRequestType {sample_period, };
        self.client.invoke_void("", "PerformanceManager", &self.mo_id, "RemovePerfInterval", Some(&input)).await
    }
    /// Restores a set of performance counters to the default level of data
    /// collection.
    /// 
    /// See the <a href="#counterTables">performance counter
    /// tables</a> for the default collection level for individual counters.
    /// 
    /// ***Required privileges:*** Performance.ModifyIntervals
    ///
    /// ## Parameters:
    ///
    /// ### counters
    /// An array of counter ids.
    pub async fn reset_counter_level_mapping(&self, counters: &[i32]) -> Result<()> {
        let input = ResetCounterLevelMappingRequestType {counters, };
        self.client.invoke_void("", "PerformanceManager", &self.mo_id, "ResetCounterLevelMapping", Some(&input)).await
    }
    /// Changes the level of data collection for a set of performance counters.
    /// 
    /// See the <a href="#counterTables">performance counter tables</a>
    /// for the default collection level for individual counters.
    /// 
    /// **Important:**
    /// 
    /// Consider the performance and storage consequences of using this
    /// method. You may cause a significant increase in data collection and
    /// storage, along with a corresponding decrease in performance. vCenter
    /// Server performance and database storage requirements depend on the
    /// collection levels defined for the performance intervals
    /// (PerformanceManager.*PerformanceManager.historicalInterval*)
    /// and the collection levels specified for individual performance counters
    /// (*PerfCounterInfo*.*PerfCounterInfo.level*).
    /// 
    /// <u>Performance Counter Data Collection</u>
    /// 
    /// vSphere defines four levels of data collection for performance
    /// counters. Each performance counter specifies a level for collection. The
    /// historical performance intervals (PerformanceManager.*PerformanceManager.historicalInterval*) define the sampling period
    /// and length for a particular collection level.
    /// 
    /// The amount of data collected for a performance counter depends
    /// on the performance interval and on the type of entity for which
    /// the counter is defined. For example, a datastore counter such as
    /// datastoreIops (the aggregate number of IO operations on the datastore)
    /// will generate a data set that corresponds to the number of datastores
    /// on a host. If a vCenter Server manages a large number of hosts
    /// with a large number of datastores, the Server will collect
    /// a large amount of data.
    /// 
    /// There are other counters for which the vCenter Server collects
    /// a relatively smaller amount of data. For example, memory counters
    /// are collected as a single counter per virtual machine and a single
    /// counter per host.
    /// 
    /// <u>Performance Counter Data Storage</u>
    /// 
    /// The performance interval collection *PerfCounterInfo.level* defines the set of counters for
    /// which the vCenter Server stores performance data. The Server will store
    /// data for counters at the specified level and for counters at all lower
    /// levels.
    /// 
    /// By default, all the performance intervals specify collection level
    /// one. Using these defaults, the vCenter Server stores performance counter
    /// data in the vCenter database for all counters that specify collection
    /// level one. It does not store data for counters that specify collection
    /// levels two through four.
    /// 
    /// <u>Performance Manager Method Interaction</u>
    /// 
    /// You can use the UpdateCounterLevelMapping method to change the
    /// collection level for individual counters. You can also use the *PerformanceManager.UpdatePerfInterval* method to change the
    /// collection level for the system-defined performance intervals. These
    /// methods can cause a significant increase in the amount of data collected
    /// and stored in the vCenter database.
    /// 
    /// You may cause a significant increase in data collection and storage
    /// along with a corresponding decrease in performance under the following
    /// conditions:
    /// - By default the system-defined performance intervals use collection
    ///   level one, storing data for all counters that specify collection
    ///   level one. If you use the UpdateCounterLevelMapping method to change
    ///   the collection level of performance counters to level one, you will
    ///   increase the amount of stored performance data.
    /// - If you use the *PerformanceManager.UpdatePerfInterval* method to increase
    ///   the collection level for the system-defined performance intervals,
    ///   you will increase the amount of stored performance data. 
    ///   
    /// To restore counter levels to default settings use the *PerformanceManager.ResetCounterLevelMapping* method.
    /// 
    /// ***Required privileges:*** Performance.ModifyIntervals
    ///
    /// ## Parameters:
    ///
    /// ### counter_level_map
    /// An array of *PerformanceManagerCounterLevelMapping* objects. The
    /// levels for the counters passed in are changed to the passed in values. If
    /// the optional aggregateLevel field is left unset then only the
    /// perDeviceLevel is configured. If the optional perDeviceLevel is left
    /// unset then only the aggregateLevel is configured. If there are multiple
    /// entries in the passed in array for the same counterId being updated then
    /// the last entry containing the counterId takes effect.
    pub async fn update_counter_level_mapping(&self, counter_level_map: &[crate::types::structs::PerformanceManagerCounterLevelMapping]) -> Result<()> {
        let input = UpdateCounterLevelMappingRequestType {counter_level_map, };
        self.client.invoke_void("", "PerformanceManager", &self.mo_id, "UpdateCounterLevelMapping", Some(&input)).await
    }
    /// Modifies VirtualCenter Server's built-in *historical intervals*, within certain limits.
    /// 
    /// **Supported Modifications**
    /// <table border="1"width="100%">
    /// <tr>
    /// <th>key</th>
    /// <th>samplingPeriod</th>
    /// <th>length</th>
    /// <th>name</th>
    /// <th>level \[1\]</th>
    /// <th>enabled \[2\]</th>
    /// </tr>
    /// <tr>
    /// <td>1</td>
    /// <td>300 \[3\]</td>
    /// <td>86400 \[4\]</td>
    /// <td>Past&nbsp;day</td>
    /// <td>1</td>
    /// <td>true</td>
    /// </tr>
    /// <tr>
    /// <td>2</td>
    /// <td>1800</td>
    /// <td>604800</td>
    /// <td>Past&nbsp;week</td>
    /// <td>1</td>
    /// <td>true</td>
    /// </tr>
    /// <tr>
    /// <td>3</td>
    /// <td>7200</td>
    /// <td>2592000</td>
    /// <td>Past&nbsp;month</td>
    /// <td>1</td>
    /// <td>true</td>
    /// </tr>
    /// <tr>
    /// <td>4</td>
    /// <td>86400</td>
    /// <td>31536000 \[5\]</td>
    /// <td>Past&nbsp;year</td>
    /// <td>1</td>
    /// <td>true</td>
    /// </tr>
    /// </table>
    /// 
    /// **\[1\]**&nbsp; The collection level for the *historical intervals* can be changed. However,
    /// the level specified for a lower-numbered interval cannot be smaller
    /// than that of a larger interval.  
    /// **\[2\]**&nbsp; An interval can be disabled. By default, all four
    /// intervals are enabled. Disabling an interval disables all higher
    /// intervals. For example, disabling interval 3 (&#147;Past month&#148;)
    /// also disables interval 4 (&#147;Past year&#148;).  
    /// **\[3\]**&nbsp; Can reduce this interval&#146;s *PerfInterval.samplingPeriod* from 5 minutes to 1, 2, or 3
    /// minutes.  
    /// **\[4\]**&nbsp; Can increase this interval&#146;s *PerfInterval.length* from 1 day to 2 or 3 days.  
    /// **\[5\]**&nbsp; Can increase interval&#146;s *PerfInterval.length* from 1 year to 2 or 3 years.  
    /// 
    /// See *PerfInterval* for information about the four default
    /// intervals for VirtualCenter Server.
    /// 
    /// ***Required privileges:*** Performance.ModifyIntervals
    ///
    /// ## Parameters:
    ///
    /// ### interval
    /// The *historical interval* being modified, a
    /// complete data object comprising values for *PerfInterval.enabled*, *interval ID*,
    /// *PerfInterval.length* of time to maintain statistics for this
    /// interval in the database, *PerfInterval.level*, *PerfInterval.name*, and *PerfInterval.samplingPeriod*
    /// properties.
    pub async fn update_perf_interval(&self, interval: &crate::types::structs::PerfInterval) -> Result<()> {
        let input = UpdatePerfIntervalRequestType {interval, };
        self.client.invoke_void("", "PerformanceManager", &self.mo_id, "UpdatePerfInterval", Some(&input)).await
    }
    /// The static description strings.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<crate::types::structs::PerformanceDescription> {
        let pv_opt = self.client.fetch_property_raw("", "PerformanceManager", &self.mo_id, "description").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property description was empty".to_string()))?;
        let result: crate::types::structs::PerformanceDescription = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// A list of *intervals* configured on the
    /// system.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn historical_interval(&self) -> Result<Option<Vec<crate::types::structs::PerfInterval>>> {
        let pv_opt = self.client.fetch_property_raw("", "PerformanceManager", &self.mo_id, "historicalInterval").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// A list of all supported performance counters in the system.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn perf_counter(&self) -> Result<Option<Vec<crate::types::structs::PerfCounterInfo>>> {
        let pv_opt = self.client.fetch_property_raw("", "PerformanceManager", &self.mo_id, "perfCounter").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct CreatePerfIntervalRequestType<'a> {
    interval_id: &'a crate::types::structs::PerfInterval,
}

impl<'a> miniserde::Serialize for CreatePerfIntervalRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreatePerfIntervalRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreatePerfIntervalRequestTypeSer<'b, 'a> {
    data: &'b CreatePerfIntervalRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreatePerfIntervalRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreatePerfIntervalRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("intervalId"), &self.data.interval_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryAvailablePerfMetricRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
    begin_time: Option<&'a str>,
    end_time: Option<&'a str>,
    interval_id: Option<i32>,
}

impl<'a> miniserde::Serialize for QueryAvailablePerfMetricRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryAvailablePerfMetricRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryAvailablePerfMetricRequestTypeSer<'b, 'a> {
    data: &'b QueryAvailablePerfMetricRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryAvailablePerfMetricRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryAvailablePerfMetricRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.begin_time else { continue; };
                    return Some((std::borrow::Cow::Borrowed("beginTime"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.end_time else { continue; };
                    return Some((std::borrow::Cow::Borrowed("endTime"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.interval_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("intervalId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryPerfCompositeRequestType<'a> {
    query_spec: &'a crate::types::structs::PerfQuerySpec,
}

impl<'a> miniserde::Serialize for QueryPerfCompositeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPerfCompositeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPerfCompositeRequestTypeSer<'b, 'a> {
    data: &'b QueryPerfCompositeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryPerfCompositeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPerfCompositeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("querySpec"), &self.data.query_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryPerfCounterRequestType<'a> {
    counter_id: &'a [i32],
}

impl<'a> miniserde::Serialize for QueryPerfCounterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPerfCounterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPerfCounterRequestTypeSer<'b, 'a> {
    data: &'b QueryPerfCounterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryPerfCounterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPerfCounterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("counterId"), &self.data.counter_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryPerfCounterByLevelRequestType {
    level: i32,
}

impl miniserde::Serialize for QueryPerfCounterByLevelRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPerfCounterByLevelRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPerfCounterByLevelRequestTypeSer<'b> {
    data: &'b QueryPerfCounterByLevelRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for QueryPerfCounterByLevelRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPerfCounterByLevelRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("level"), &self.data.level as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryPerfProviderSummaryRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryPerfProviderSummaryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPerfProviderSummaryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPerfProviderSummaryRequestTypeSer<'b, 'a> {
    data: &'b QueryPerfProviderSummaryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryPerfProviderSummaryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPerfProviderSummaryRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryPerfRequestType<'a> {
    query_spec: &'a [crate::types::structs::PerfQuerySpec],
}

impl<'a> miniserde::Serialize for QueryPerfRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPerfRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPerfRequestTypeSer<'b, 'a> {
    data: &'b QueryPerfRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryPerfRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPerfRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("querySpec"), &self.data.query_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemovePerfIntervalRequestType {
    sample_period: i32,
}

impl miniserde::Serialize for RemovePerfIntervalRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemovePerfIntervalRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemovePerfIntervalRequestTypeSer<'b> {
    data: &'b RemovePerfIntervalRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for RemovePerfIntervalRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemovePerfIntervalRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("samplePeriod"), &self.data.sample_period as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ResetCounterLevelMappingRequestType<'a> {
    counters: &'a [i32],
}

impl<'a> miniserde::Serialize for ResetCounterLevelMappingRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ResetCounterLevelMappingRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ResetCounterLevelMappingRequestTypeSer<'b, 'a> {
    data: &'b ResetCounterLevelMappingRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ResetCounterLevelMappingRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ResetCounterLevelMappingRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("counters"), &self.data.counters as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateCounterLevelMappingRequestType<'a> {
    counter_level_map: &'a [crate::types::structs::PerformanceManagerCounterLevelMapping],
}

impl<'a> miniserde::Serialize for UpdateCounterLevelMappingRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateCounterLevelMappingRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateCounterLevelMappingRequestTypeSer<'b, 'a> {
    data: &'b UpdateCounterLevelMappingRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateCounterLevelMappingRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateCounterLevelMappingRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("counterLevelMap"), &self.data.counter_level_map as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdatePerfIntervalRequestType<'a> {
    interval: &'a crate::types::structs::PerfInterval,
}

impl<'a> miniserde::Serialize for UpdatePerfIntervalRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdatePerfIntervalRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdatePerfIntervalRequestTypeSer<'b, 'a> {
    data: &'b UpdatePerfIntervalRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdatePerfIntervalRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdatePerfIntervalRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("interval"), &self.data.interval as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
