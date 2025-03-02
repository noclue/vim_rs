use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::PerfCompositeMetric;
use crate::types::structs::PerfCounterInfo;
use crate::types::structs::PerfInterval;
use crate::types::structs::PerfMetricId;
use crate::types::structs::PerfProviderSummary;
use crate::types::structs::PerfQuerySpec;
use crate::types::structs::PerformanceDescription;
use crate::types::structs::PerformanceManagerCounterLevelMapping;
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
pub struct PerformanceManager {
    client: Arc<Client>,
    mo_id: String,
}
impl PerformanceManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
    pub async fn create_perf_interval(&self, interval_id: &PerfInterval) -> Result<()> {
        let input = CreatePerfIntervalRequestType {interval_id, };
        let path = format!("/PerformanceManager/{moId}/CreatePerfInterval", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn query_available_perf_metric(&self, entity: &ManagedObjectReference, begin_time: Option<&str>, end_time: Option<&str>, interval_id: Option<i32>) -> Result<Option<Vec<PerfMetricId>>> {
        let input = QueryAvailablePerfMetricRequestType {entity, begin_time, end_time, interval_id, };
        let path = format!("/PerformanceManager/{moId}/QueryAvailablePerfMetric", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn query_perf_composite(&self, query_spec: &PerfQuerySpec) -> Result<PerfCompositeMetric> {
        let input = QueryPerfCompositeRequestType {query_spec, };
        let path = format!("/PerformanceManager/{moId}/QueryPerfComposite", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn query_perf_counter(&self, counter_id: &[i32]) -> Result<Option<Vec<PerfCounterInfo>>> {
        let input = QueryPerfCounterRequestType {counter_id, };
        let path = format!("/PerformanceManager/{moId}/QueryPerfCounter", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn query_perf_counter_by_level(&self, level: i32) -> Result<Vec<PerfCounterInfo>> {
        let input = QueryPerfCounterByLevelRequestType {level, };
        let path = format!("/PerformanceManager/{moId}/QueryPerfCounterByLevel", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn query_perf_provider_summary(&self, entity: &ManagedObjectReference) -> Result<PerfProviderSummary> {
        let input = QueryPerfProviderSummaryRequestType {entity, };
        let path = format!("/PerformanceManager/{moId}/QueryPerfProviderSummary", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn query_perf(&self, query_spec: &[PerfQuerySpec]) -> Result<Option<Vec<Box<dyn crate::types::traits::PerfEntityMetricBaseTrait>>>> {
        let input = QueryPerfRequestType {query_spec, };
        let path = format!("/PerformanceManager/{moId}/QueryPerf", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
        let path = format!("/PerformanceManager/{moId}/RemovePerfInterval", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
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
        let path = format!("/PerformanceManager/{moId}/ResetCounterLevelMapping", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn update_counter_level_mapping(&self, counter_level_map: &[PerformanceManagerCounterLevelMapping]) -> Result<()> {
        let input = UpdateCounterLevelMappingRequestType {counter_level_map, };
        let path = format!("/PerformanceManager/{moId}/UpdateCounterLevelMapping", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn update_perf_interval(&self, interval: &PerfInterval) -> Result<()> {
        let input = UpdatePerfIntervalRequestType {interval, };
        let path = format!("/PerformanceManager/{moId}/UpdatePerfInterval", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// The static description strings.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<PerformanceDescription> {
        let path = format!("/PerformanceManager/{moId}/description", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// A list of *intervals* configured on the
    /// system.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn historical_interval(&self) -> Result<Option<Vec<PerfInterval>>> {
        let path = format!("/PerformanceManager/{moId}/historicalInterval", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// A list of all supported performance counters in the system.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn perf_counter(&self) -> Result<Option<Vec<PerfCounterInfo>>> {
        let path = format!("/PerformanceManager/{moId}/perfCounter", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreatePerfIntervalRequestType<'a> {
    #[serde(rename = "intervalId")]
    interval_id: &'a PerfInterval,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryAvailablePerfMetricRequestType<'a> {
    entity: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "beginTime")]
    begin_time: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endTime")]
    end_time: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "intervalId")]
    interval_id: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPerfCompositeRequestType<'a> {
    #[serde(rename = "querySpec")]
    query_spec: &'a PerfQuerySpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPerfCounterRequestType<'a> {
    #[serde(rename = "counterId")]
    counter_id: &'a [i32],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPerfCounterByLevelRequestType {
    level: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPerfProviderSummaryRequestType<'a> {
    entity: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPerfRequestType<'a> {
    #[serde(rename = "querySpec")]
    query_spec: &'a [PerfQuerySpec],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemovePerfIntervalRequestType {
    #[serde(rename = "samplePeriod")]
    sample_period: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ResetCounterLevelMappingRequestType<'a> {
    counters: &'a [i32],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateCounterLevelMappingRequestType<'a> {
    #[serde(rename = "counterLevelMap")]
    counter_level_map: &'a [PerformanceManagerCounterLevelMapping],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdatePerfIntervalRequestType<'a> {
    interval: &'a PerfInterval,
}
