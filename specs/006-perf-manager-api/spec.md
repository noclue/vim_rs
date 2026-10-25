# Feature Specification: Ergonomic Performance Manager API

**Feature Branch**: `006-perf-manager-api`

**Created**: 2026-10-24

**Status**: Draft

**Input**: User description: `docs/vim_rs-perf-spec.md` (canonical specification v5, Part I). This file is the tracked copy of that Part I. `docs/**` is gitignored, so planning and implementation use this spec. Part II of the source (signatures, algorithms, and the generator contract beyond the rules below) is input to `/speckit-plan` and is not repeated here. Normative tables in this spec must not be softened.

## Context & Problem

vSphere performance statistics are the primary time-series telemetry in the VIM API, and one of the least ergonomic surfaces to consume directly. A caller who wants “CPU and memory for these virtual machines” must today:

1. **Resolve per-server counter IDs.** Metrics are addressed on the wire by numeric IDs that differ per server. A measured vCenter 8.0.3 (build 24305161) has 732 counters, a standalone ESXi 8.0.3 (build 24414501) has 353, and `govc vcsim` (reporting 6.5.0) has 560. Of 353 names present on both vCenter and ESXi, 350 numeric keys differ (`cpu.usage.average` is `2` on that vCenter and `1` on that ESXi). `group.name.rollup` is not unique: `disk.scsiReservationCnflctsPct.average` exists twice on that vCenter (key `148`, absolute; key `358`, rate). Identity is `(group, name, rollup, stats kind)`.
2. **Decode polymorphic responses and keep parallel arrays aligned.** The requested format selects the response shape. One shape uses nested typed series plus a separate sample-info array. The other returns comma-separated timestamps and values that must be split and aligned. Empty CSV tokens and `-1` both mean “no sample”.
3. **Interpret sentinels and units that the wire does not explain.** In a realtime series, `-1` means “no sample” while the timestamp is still present. A power-off of a probed VM produced about 16 consecutive `-1` samples on `cpu.usage.average` on both ESXi and vCenter realtime; the vCenter 5-minute rollup covering that same window kept ordinary positive values and emitted no `-1`. Measured `percent` counters are hundredths of a percent (`10000` means `100.00%`). `kiloBytes` means 1024 bytes. `gigaBytes` is not a first-class unit token and arrives as an other-unit tagged `gigaBytes` (observed on `gpu.mem.*.gb.latest`). Across vCenter, ESXi, and vcsim, 16 distinct unit strings exist.
4. **Choose realtime versus historical correctly.** The query interval is the sampling period in seconds (`20` for realtime, or `300`, `1800`, `7200`, `86400` for historical), never the historical interval’s own key (`vcsim` reuses key `1` for three different periods). Standalone ESXi returns an empty historical-interval list and does not support summary statistics. Calling provider summary on `ComputeResource`, `Datacenter`, or `Datastore` on that ESXi throws invalid-argument.
5. **Avoid silent empties and wire traps.** Start time is strictly exclusive and end time is inclusive. Maximum sample count is ignored for historical statistics and is invalid on composite queries. A powered-off VM or a deleted object is omitted from a successful response (no fault, no `-1` series). A realtime query whose start is older than the roughly 60-minute retention window still returns the retained tail (about 180 samples at 20 seconds), not an error. On ESXi, collection level is unset on all 353 counters; unset means unpopulated, not level 1.
6. **Batch without asking the server for a limit it will not report.** On default vCenter 8.0.3, the stats max-query-metrics option is unset and querying it faults with an unknown name. Single specs of 300 counters, or 64 wildcard counters expanding to about 2,200 series, succeeded without faulting. Callers still need a client-side cap (default 256 requested metric IDs per call, packed in request order) and optional historical time slicing, because an unbounded query can pressure the vCenter database.

This feature adds one public performance module that hides those six sources of friction for diagnostic scripts, live inventory views, and metric exporters, while keeping compile-time checks on which counters apply to which managed-object kinds. It ships in `0.7.0` because `vim_rs::core::client::Error` is now `Clone`: `MethodFault` holds `Arc<MethodFault>` and `ReqwestError` holds `Arc<reqwest::Error>`. Callers on `^0.6.1` that take either payload by value cannot accept that. There is a single public module path (no second re-export under the core client module).

### Non-Goals

- Changing vCenter performance intervals or collection levels (those are administrative operations).
- Wrapping vSAN’s separate performance manager.
- Persisting the counter catalog across process restarts.
- Owning a background poll timer or stream inside the library. Callers own their timer and cancellation, and call a prepared query (one-shot or incremental) plus a sample watermark.
- Composite queries for resource pools, folders, or cluster compute resources. Version 1 composite queries cover one host and its child virtual machines only.

### Settled Product Decisions

These decisions are closed. Planning must not reopen them.

| ID | Decision | Why |
|---|---|---|
| **ADR-01** | Counter identity is `(group, name, rollup, stats kind)`. Names may contain dots (`cpu.capacity.contention.average` is group `cpu`, name `capacity.contention`, rollup `average`). | The SCSI reservation counter exists twice, differing only by stats kind. 247 of 732 counter names contain dots. |
| **ADR-02** | Every generated counter has a compile-time entity scope from the three-tier rule in the normative scope table. A scope override exists for inferred or group-union scopes and keeps the counter’s unit dimension. Dynamic queries still go to the server and record a scope-mismatch warning when the server returns nothing for an out-of-scope entity. | Direct availability covers 265 counters; sibling rollups, group unions, and inferred domains cover the rest so no generated counter is “any entity”. |
| **ADR-03** | Do not call the option-manager query for max query metrics. Pack entity specs greedily in request order with a default chunk of 256 metric IDs. | That option query always throws unknown-name on default vCenter 8.0.3 and on ESXi. |
| **ADR-04** | No missing-object retry that bisects the batch. | vCenter (both transports) and ESXi omit deleted references from a successful query without a fault. Omitted entities are “no samples”. |
| **ADR-05** | When one entity’s data arrives in multiple chunks or time slices, timestamps are the sorted unique union, and any series missing a slot is filled with `-1`. Differing sample intervals record an interval-substituted warning. | Callers see one aligned grid per entity. |
| **ADR-06** | Session-fatal and transport failures abort the whole fetch. Object-scoped permission errors, invalid argument, system errors, and parse failures fail only that chunk’s entities. | One bad chunk must not hide sibling results, and an auth failure must not continue. |
| **ADR-07** | Commit a sanitized vCenter 8.0.3 catalog in the generator data set and again under the library’s own test fixtures. | `docs/**` and the probe output directory are gitignored. Library tests must not reach into another crate. |
| **ADR-08** | Performance errors are their own type, wrapping the existing client error. If at least one entity can be queried, fetch succeeds and skipped entities keep a per-entity unsupported status. If none can be queried, fetch returns one “no supported entities” error. | Avoids overloading the client error, and avoids a batch-level error that contradicts per-entity status. |
| **ADR-09** | A named historical preset (`past day`, `past week`, `past month`, `past year`) with neither “latest N” nor an explicit time window uses that preset’s server retention length. On the measured vCenter, past day is 24 hours and past week is 7 days. | The preset name must match the window the caller gets. Length comes from the server interval, not from a sample cap. |

## User Scenarios & Testing *(mandatory)*

### User Story 1 - One-shot diagnostic query (Priority: P1)

A vSphere automation author writes a short diagnostic program. They name the counters they care about (or ask for whatever the server has for those objects), pass a slice of managed objects, and print human-readable values such as `12.34 %` or `1.21 GiB`. They do not fetch the counter catalog by hand, build an ID map, probe each virtual machine, or downcast response objects.

**Why this priority**: This is the smallest slice that removes the six sources of friction. A diagnostic script can ship on this alone.

**Independent Test**: Against a fixture or a live vCenter, request `cpu.usage.average`, `cpu.usagemhz.average`, `mem.usage.average`, and `mem.consumed.average` for a few virtual machines and confirm formatted values, gap handling, and a result row for every requested object.

**Acceptance Scenarios**:

1. **Given** a connected vCenter and a handful of virtual machines, **When** the author requests those four counters and prints scaled values, **Then** percents print as hundredths-corrected percentages and memory prints as byte-based binary units, with no hand-built counter map and no per-object provider loop.
2. **Given** a name that exists as both a rate and an absolute counter, **When** the author looks it up by `group.name.rollup` without a stats kind, **Then** the rate variant is selected; specifying the stats kind selects the absolute variant.
3. **Given** a powered-off virtual machine in the request, **When** the query succeeds, **Then** that object is still in the result as no-samples (not a fault and not a frozen previous value).
4. **Given** a realtime series that ends in `-1` gap slots, **When** the author asks for the newest valid sample and for the current slot, **Then** the newest valid sample is returned and the current slot is blank.

---

### User Story 2 - Live inventory poller (Priority: P1)

A terminal UI polls four counters (`cpu.usage.average`, `mem.usage.average`, `cpu.usagemhz.average`, `mem.consumed.average`) across a changing list of virtual machines, hosts, clusters, or resource pools about every 20 seconds. The poll is stateless and reusable: the same prepared query accepts a new object list each time. Wide inventories are packed into few calls. Each poll yields a fixed-width sparkline (six slots, gaps blank), an exact integer byte count for memory, and a current-slot value that is blank when the latest sample is a gap or the VM is off. A VM that disappears between polls overwrites the previous sparkline with an empty row.

**Why this priority**: This is the behavior a live inventory already implements by hand (about 422 lines in the external `vtui` poller). The library slice must replace that hand decoding, packing, padding, and stale-row logic. Updating the external repository is a follow-up; the behavior is in scope here.

**Independent Test**: Drive a prepared query with 100 virtual machines and 4 counters at the default chunk size and confirm two underlying queries (64 objects, then 36). Replay a gapped series and confirm the sparkline and the blank current slot. Omit one requested object from the response and confirm its row is no-samples.

**Acceptance Scenarios**:

1. **Given** 100 virtual machines, 4 counters, and the default chunk size of 256 metric IDs, **When** the poller fetches, **Then** exactly two performance queries are issued (64 objects, then 36), in request order.
2. **Given** raw values `[100, -1, 300]` and a window width of 5, **When** the sparkline is taken, **Then** it is `[blank, blank, 100, blank, 300]` from oldest to newest.
3. **Given** a series `[420, 410, -1, -1]`, **When** the poller reads the newest valid sample and the current slot, **Then** the valid sample is `410` and the current slot is blank.
4. **Given** a previous poll that showed a sparkline for a VM, **When** the next successful response omits that VM, **Then** the new row is no-samples and the old sparkline is not reused.
5. **Given** a mix of virtual machines and a datastore on ESXi with automatic interval selection, **When** the poller fetches, **Then** the virtual machine is queried at realtime and the datastore is marked unsupported, and the fetch still succeeds.

---

### User Story 3 - Statically checked counters (Priority: P2)

A developer holds a managed object whose kind is known (virtual machine, host, cluster, resource pool, datastore, or datacenter). They pick counters from a generated catalog, including per-kind listings. Asking for a CPU or memory counter on a datastore, or a host-only hardware counter on a virtual machine, is rejected before the program runs, with a diagnostic that names the mismatch. Counters whose scope was only inferred can be explicitly widened without losing their unit type.

**Why this priority**: The catalog is what makes 732 counters usable without memorizing server IDs. It is independently testable with the committed catalog and compile-time checks, after the one-shot query exists. It is not required to print a single formatted sample.

**Independent Test**: Build the generated catalog from the sanitized vCenter catalog and confirm all 732 constants resolve, none are scoped to every entity kind, datastore listings contain no CPU or memory group, and the named illegal combinations fail to compile.

**Acceptance Scenarios**:

1. **Given** the vCenter 8.0.3 catalog, **When** constants are generated, **Then** all 732 counters have a scope narrower than “any entity”, and `cpu.usage.maximum` admits only virtual machine, host, and cluster (inherited from `cpu.usage.average`).
2. **Given** a datastore-typed object, **When** a program requests `cpu.usage.average`, `cpu.usage.maximum`, or `mem.usage.average`, **Then** the program is rejected before it runs.
3. **Given** a virtual-machine-typed object, **When** a program requests host-only `disk.deviceReadLatency.average`, **Then** the program is rejected before it runs.
4. **Given** the per-entity listings, **When** a developer browses datastore counters, **Then** there is no CPU or memory group in that listing.

---

### User Story 4 - Incremental export (Priority: P2)

A telemetry exporter streams samples to a time-series database or a columnar frame. It remembers a high-water timestamp per object, counter, and instance. Each incremental pull asks only for samples after that object’s own watermark, does not send a maximum-sample cap on that pull, and skips negative gap slots. A powered-off VM in a 500-VM fleet does not force the other 499 VMs back to a full-hour refetch. Every valid `(object, counter, instance, timestamp)` sample is emitted once.

**Why this priority**: Exporters need the query and alignment behavior from Story 1, plus watermark rules. The poller in Story 2 does not need watermarks to be useful.

**Independent Test**: Seed a watermark from a first result that contains a `-1` gap, run a second result with one new valid sample and one gap, and confirm only the new valid sample is emitted and the gap does not move the watermark. Confirm two objects in one batch use independent start times.

**Acceptance Scenarios**:

1. **Given** a result that mixes valid samples and `-1` gaps, **When** rows or new-sample filtering run, **Then** gaps are omitted and do not advance the high-water mark.
2. **Given** a watermark that already covers one VM and has never seen another, **When** an incremental fetch runs, **Then** each object’s start time is that object’s own earliest tracked timestamp, or unset if that object has never reported.
3. **Given** an incremental pull whose start time is set, **When** the request is built, **Then** maximum sample count is omitted so a delayed tick cannot drop intermediate samples.
4. **Given** the caller stops consuming new rows early, **When** filtering returns, **Then** the watermark has already advanced for every new valid sample in that result.

---

### User Story 5 - Host and guest composite query (Priority: P3)

A caller asks for one host and its child virtual machines in a single composite query. The host reference must actually be a host. The result separates the host series from the child series. Resource pools, folders, and clusters are not composite roots in this version.

**Why this priority**: Composite is a distinct server operation with different wire limits. Stories 1–4 deliver the common query path without it.

**Independent Test**: Decode the committed composite fixture (one host and two child VMs) into a host series plus a child map. Attempt the same fetch with a non-host reference and confirm failure before any network call.

**Acceptance Scenarios**:

1. **Given** a host reference, **When** a composite fetch runs, **Then** the result contains that host’s series and each child virtual machine’s series, and maximum sample count is not sent.
2. **Given** a reference that is not a host, **When** composite fetch is called, **Then** it fails with an entity-kind mismatch before network I/O.
3. **Given** a resource pool, folder, or cluster, **When** a caller looks for a composite root of that kind, **Then** version 1 does not offer one.

---

### Edge Cases

- **Duplicate `group.name.rollup`.** `disk.scsiReservationCnflctsPct.average` resolves to the rate counter by name, and to the absolute counter only when stats kind is specified. Generated names keep both (rate unsuffixed, absolute suffixed). On the vCenter 8.0.3 catalog that pair is the only name with two stats kinds. Each delta counter is the only variant of its name, so bare lookup returns that delta. If rate is absent and more than one stats kind still matches, bare lookup fails instead of choosing one.
- **Dotted names and case collisions.** `cpu.capacity.contention.average` splits on the first and last dot. Eight `mem` pairs differ only by case (`swapIn` versus `swapin`) and stay distinct. `mem.zip.saved.latest` and `mem.zipSaved.latest` stay distinct.
- **Counter missing on this server.** A vCenter-only counter requested against ESXi, mixed with a counter ESXi does have, warns and continues. The fetch fails only when every requested counter is missing or unit-incompatible.
- **Unset collection level.** ESXi counters have no level. No “level too low” warning is emitted unless both the counter level and the interval level are present and the counter’s level is higher.
- **Realtime window older than retention.** The server returns the retained tail and the client records that the requested start was older than about 60 minutes. It does not fail the fetch.
- **Historical interval the server does not enable.** If the server interval list is non-empty and at least one object supports summary, a period that matches no enabled sampling period fails the whole fetch before any query. If the interval list is empty (standalone ESXi), objects are skipped as historically unsupported and, when none remain queryable, the fetch returns no-supported-entities.
- **Server substitutes a coarser interval.** The client keeps the first observed sample interval, aligns to the union grid, and records the requested versus actual period.
- **Empty successful response.** A query that returns no payload still lists every requested object as no-samples.
- **Deleted object during provider discovery.** If the first object of a kind faults with not-found while a later object of that kind succeeds, capabilities are learned from the later object. The deleted object becomes no-samples after the query (the server omits it), not a cached “this kind is unsupported”.
- **Provider summary fails for every object of one kind, but another kind succeeds.** The failed kind is marked failed; the other kind is still queried; the fetch succeeds.
- **Provider summary or a chunk hits authentication failure, cancellation, or a transport error.** The whole fetch aborts. An object-scoped permission error does not abort sibling chunks.
- **All chunks fail locally.** The fetch returns the first chunk error rather than a success full of failures.
- **Single object asks for more metric IDs than the chunk size.** That object is split across multiple specs, then aligned onto one timestamp grid with `-1` fill.
- **Time slicing a historical window.** Slices share boundary timestamps with neither gaps nor duplicates, because start is exclusive and end is inclusive. Time slicing is ignored for realtime.
- **Availability cache.** Empty availability results are not cached. The cache key is the object plus the interval id, with a capacity of 1024 and first-in eviction.
- **Relative “since ago”.** The duration is applied at each fetch, not frozen when the query is built. An incremental fetch overrides it.
- **Unknown unit token.** A fully dynamic dimension still returns a number and records that the unit could not be scaled. A typed dimension that rejects the unit warns with a dimension mismatch (and drops that counter) rather than inventing a scale.
- **Zero vCPUs, non-positive interval, or a per-device ready instance when more than one vCPU is requested.** Ready-percent for that slot is blank. A single vCPU instance is computed with a vCPU count of 1.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-01**: The counter registry MUST index the server catalog by `(group, name, rollup, stats kind)` and by numeric counter ID. Name lookup MUST treat the group as the text before the first dot, the rollup as the text after the last dot (case-insensitive), and the name as the middle (internal dots preserved). When one `group.name.rollup` matches both absolute and rate, resolving a metric MUST honor that metric’s stats kind when present; bare name lookup MUST return the rate variant (or the only variant); an explicit stats-kind lookup MUST disambiguate. On the committed vCenter catalog the only such pair is absolute plus rate, and every delta counter is the only variant of its name. If rate is absent and more than one stats kind still matches, bare lookup MUST fail instead of choosing one.
  - **Acceptance**: **Given** the 732-counter vCenter catalog and the 353-counter ESXi catalog, **When** a registry is built from each, **Then** every counter resolves, including `cpu.capacity.contention.average`, both `mem.zip.saved.latest` and `mem.zipSaved.latest`, and both SCSI reservation variants. That SCSI name is the only `group.name.rollup` with two stats kinds. `cpu.ready.summation` is delta and the only variant of its name, so bare lookup returns it.
- **FR-02**: If a requested counter is absent from the connected server, or its unit is incompatible with the requested dimension, the fetch MUST continue when at least one other requested counter resolves. It MUST record a counter-not-on-server warning or a dimension-mismatch warning. It MUST fail with unknown-counter or dimension-mismatch only when zero requested counters resolve.
  - **Acceptance**: **Given** an ESXi registry, **When** a query requests `cpu.usage.average` and a vCenter-only `vcResources` counter, **Then** the fetch succeeds with CPU data and a counter-not-on-server warning.
- **FR-03**: Collection level and per-device level MUST stay optional, including unset for all 353 ESXi counters and for custom metrics. A level-too-low warning MUST be recorded only when both the counter level and the interval level are present and the counter level is greater. The metric MUST still be included in the query. Unset levels MUST NOT warn.
  - **Acceptance**: **Given** an ESXi counter with no level, **When** it is queried at any interval, **Then** no level-too-low warning is emitted.
- **FR-04**: Every decoded series MUST expose exact raw integers and a unit-scaled view. Memory scaling MUST use exact integer multiplication (`kiloBytes` × 1024, `megaBytes` × 1024², other-unit `gigaBytes` × 1024³, `teraBytes` × 1024⁴) with no floating-point rounding. All 16 observed unit tokens MUST scale and display as in the unit table.
  - **Acceptance**: **Given** `mem.consumed.average = 1048576` kiloBytes and `gpu.mem.used.gb.latest = 16` gigaBytes, **When** the scaled latest values are read, **Then** they are 1,073,741,824 bytes and 17,179,869,184 bytes.
- **FR-05**: Series MUST distinguish the newest valid sample (skipping trailing values less than 0) from the current slot (only the final timestamp; blank when the series is empty or the final value is less than 0).
  - **Acceptance**: **Given** `[420, 410, -1, -1]`, **When** both accessors run, **Then** the newest valid sample is 410 and the current slot is blank.
- **FR-06**: A fixed-width window of N MUST return N slots, oldest to newest, using the newest N timestamps, left-padded with blanks when fewer than N exist, and mapping every negative value to blank.
  - **Acceptance**: **Given** `[100, -1, 300]` and N = 5, **When** the window is taken, **Then** the result is blank, blank, 100, blank, 300.
- **FR-07**: A CPU-ready helper on a time series MUST return one optional percentage per raw slot. It MUST be blank when the sample is negative, the vCPU count is 0, the interval is not positive, or the instance is not the aggregate while the vCPU count is greater than 1. Otherwise it MUST convert the raw ready time to milliseconds and compute `(ready_ms / (interval_secs × 1000 × vcpus)) × 100`. Callers inspecting one per-vCPU instance pass a vCPU count of 1.
  - **Acceptance**: **Given** an aggregate ready sample of 1000 milliseconds, a 20-second interval, and 2 vCPUs, **When** the helper runs, **Then** that slot is 2.5%. **Given** the same sample on a non-aggregate instance with 2 vCPUs, **Then** that slot is blank.
- **FR-08**: Automatic wire format MUST select CSV for SOAP (so each sample is not wrapped in its own element) and CSV for JSON, except when the caller explicitly asked for the latest single sample, in which case JSON uses the normal (non-CSV) format.
  - **Acceptance**: **Given** a SOAP client, **When** automatic format is used, **Then** the query format is CSV. **Given** a JSON client with latest-1, **Then** the format is normal.
- **FR-09**: The decoder MUST accept both normal and CSV entity metrics and produce the same entities, statuses, timestamps, values, and warnings (request time excluded). Both the CSV token `-1` and an empty CSV token MUST become `-1`. When the query returns no payload or an empty list, every requested entity MUST appear as no-samples.
  - **Acceptance**: **Given** the paired normal JSON, CSV JSON, and SOAP XML fixtures, **When** each is decoded, **Then** entities, timestamps, values, and statuses match.
- **FR-10**: On first use of a managed-object kind, the client MUST probe provider summary and cache whether realtime is supported, whether historical summary is supported, and the refresh rate. Probe rules:
  - A successful summary is cached for that kind.
  - Invalid-argument is cached as neither realtime nor summary, with no refresh rate (ESXi does this for compute resource, datacenter, and datastore).
  - Transport failure, cancellation, or not-authenticated / invalid-login aborts the fetch immediately.
  - Any other fault (not-found, permission, system error, and similar) is NOT cached. The probe retries on the next requested object of that kind. If every object of that kind fails the probe, those objects are marked failed and other kinds continue. The fetch returns the client error only when every object in the batch failed the probe.
  - After a capability is known, each object is judged by the interval decision table: neither capability means not-a-provider; realtime requested but realtime unsupported (or refresh rate not positive) means realtime-unsupported; historical requested but summary unsupported or the server has no intervals means historical-unsupported.
  - If at least one object is queryable, or some objects already failed probing while another kind can proceed, the fetch queries the supported objects and returns success, keeping unsupported status on the skipped objects. If every object was skipped as not-a-provider, realtime-unsupported, or historical-unsupported, the fetch returns “no supported entities” and does not call the performance query.
  - **Acceptance**:
    1. **Given** ESXi (no historical intervals; a VM with realtime only; a datastore that is not a provider) and the pair `[vm, datastore]`, **When** fetch uses automatic or realtime interval, **Then** the VM is queried at 20 seconds and the datastore is not-a-provider, and the fetch succeeds. **When** the same pair uses the past-day preset, **Then** both are skipped and the fetch returns no-supported-entities.
    2. **Given** a deleted VM whose summary probe returns not-found and a live VM of the same kind, **When** fetch runs, **Then** the probe is retried on the live VM, the kind’s capabilities are cached, the live VM is queried, and the deleted VM (omitted by the server) is no-samples.
- **FR-11**: Automatic interval MUST be resolved per object using the decision table. When an object falls back from automatic to historical, the chosen period is the finest enabled sampling period whose retention still covers the requested start, or the finest enabled period when start is unset, or the coarsest enabled period when start is older than every retention window. That fallback MUST record an automatic-interval-fallback warning naming the object kind and the chosen period.
  - **Acceptance**: **Given** a virtual machine and a cluster, automatic interval, and no start time, **When** fetch runs on vCenter, **Then** the VM uses interval 20 and the cluster uses interval 300, with a fallback warning for the cluster.
- **FR-12**: An explicit historical period MUST mark objects historically unsupported when that object has no summary support or the server interval list is empty. If no object remains queryable, the fetch returns no-supported-entities. If the interval list is non-empty, at least one object supports summary, and the requested period matches no enabled sampling period, the fetch MUST return invalid-interval before any performance query.
  - **Acceptance**: **Given** standalone ESXi, **When** the caller asks for the past-day preset, **Then** the fetch returns no-supported-entities. **Given** vCenter intervals 300, 1800, 7200, and 86400, **When** the caller asks for historical period 999 on a VM, **Then** the fetch returns invalid-interval 999.
- **FR-13**: Start time is exclusive and end time is inclusive on the server. Maximum sample count is not sent for historical intervals. When the resolved interval is historical:
  - Latest-N with no start time MUST set start to now minus `(N + 2)` periods (a two-period rollup cushion), end to now, omit maximum sample count, and keep only the newest N slots after decoding.
  - Neither latest-N nor a time window MUST set start to now minus that interval’s retention length and end to now (24 hours for past day, 7 days for past week, and the matching retention for past month and past year).
  - Realtime with neither latest-N nor a time window MUST leave start, end, and maximum sample count unset (the server’s retained realtime window).
- **FR-14**: “Since ago” MUST store a duration and subtract it from the clock at each fetch, not at query build. Incremental fetch MUST override any since-ago start.
- **FR-15**: The chunker MUST pack query specs in request order so the sum of metric IDs in one performance call is at most the chunk size (default 256). Objects that fit together MUST share one call (64 objects × 4 metrics = one call of 256, not 64 calls). Interval, start, end, and maximum sample count MAY differ across specs in that call and MUST NOT by themselves force another call. One object with M metric IDs above the chunk size MUST be split into `ceil(M / chunk size)` specs.
  - **Acceptance**: **Given** 100 virtual machines × 4 metrics and chunk size 256, **When** fetch runs against a mock, **Then** exactly two performance calls are issued.
- **FR-16**: A historical time-chunk duration on a bounded window MUST split the range into contiguous slices `(t0, t1], (t1, t2], …` with each cut at most one chunk duration after the previous cut, and the last slice ending at the caller’s end. Because start is exclusive and end is inclusive, using the previous cut as start and the next cut as end neither drops nor duplicates the boundary sample. Time chunking MUST be ignored for realtime.
- **FR-17**: Series for one object that arrive from several metric chunks or time slices MUST share one timestamp grid: the sorted, unique union of timestamps for that object. Missing slots MUST be `-1`. The object’s interval is the first observed sample interval, falling back to the requested interval. A differing interval MUST record interval-substituted with the requested and actual periods. Every series on that object MUST share that same timestamp grid.
  - **Acceptance**: **Given** chunk A timestamps `[t1, t2]` for metric 1 and chunk B `[t2, t3]` for metric 2 on the same VM, **When** merged, **Then** the grid is `[t1, t2, t3]`, metric 1 is `[v1, v2, -1]`, and metric 2 is `[-1, w2, w3]`.
- **FR-18**: Chunks MUST run with bounded concurrency (default 4) inside the caller’s fetch, and dropping that fetch MUST cancel in-flight calls. Classification follows the fault table: transport errors, cancellation, and session authentication faults abort remaining chunks and fail the fetch; other method faults and parse errors fail only that chunk’s objects. If every chunk failed, the fetch returns the first of those errors.
- **FR-19**: An object that was sent in a chunk that succeeded, and was omitted or returned with no timestamps or values, MUST be no-samples.
- **FR-20**: The availability cache MUST hold at most 1024 entries, keyed by the full object reference plus interval id, evicting the oldest entry at capacity. It MUST NOT store an empty metric-id list.
- **FR-21**: Building a prepared query MUST be synchronous and infallible. Metric resolution against the registry MUST happen on the first fetch and MUST be cached with the registry generation. Refreshing the registry MUST bump that generation so the next fetch resolves again. Generation MUST be monotonic so a recreated registry cannot be mistaken for the previous one.
- **FR-22**: Row export and watermark filtering MUST skip negative raw slots. Watermark filtering MUST advance high-water marks for every valid new sample before returning rows, so abandoning the row iterator cannot leave a partial watermark. Incremental fetch MUST set each object’s start time from that object’s own minimum high-water timestamp (or leave it unset if the object has never reported) and MUST omit maximum sample count for any object whose start time is set.
- **FR-23**: Composite query MUST require a host, failing with entity-kind mismatch before network I/O otherwise. It MUST force the normal (non-CSV) format, omit maximum sample count, and decode into an optional host series, a map of child series, and warnings.
- **FR-24**: Regenerating bindings MUST read the committed sanitized catalog beside the OpenAPI spec and emit the performance counter constants into the library source, checked in like other generated surfaces. An ordinary library build MUST NOT overwrite that file. Every generated constant MUST receive a compile-time scope from the three-tier rule so none use the any-entity scope. Per-kind listings MUST re-export only counters whose scope includes that kind, excluding inferred-tier groups until real sample rows exist, and MUST omit empty groups (datastore has no CPU or memory listing).
- **FR-25**: The default library build MUST gain no new dependencies. Optional helpers for civil time and for columnar frames MUST be separate opt-in features, off by default.
- **FR-26**: The feature is successful when all of the following hold:
  1. The in-tree performance snippet no longer downcasts response objects, builds a counter-id map, or loops provider calls per virtual machine.
  2. The behaviors the external inventory poller implements by hand (counter resolution, unit multipliers, sparkline padding, current-slot blanking, and clearing omitted or powered-off rows) are provided by one prepared query. Changing that external repository is not a merge gate.
  3. All 732 vCenter counters and all 353 ESXi counters resolve in tests against the committed fixtures.
  4. All 732 generated constants exclude at least one entity kind, and the illegal combinations named in Story 3 fail before the program runs.
  5. Normal and CSV fixtures decode to equivalent results.

### Follow-on performance requirements

[fastpath.md](./fastpath.md) specifies two internal cost reductions for large ongoing polls: an allocation-free `series()` lookup, and a single-payload decoder that skips the timestamp grid. Both are implemented. Decoded results stay under FR-09 and FR-17, and prepared-query resolution stays under FR-21.

### Normative Wire & Probe Rules

These tables are verified vSphere 8.0.3 behavior. Planning and implementation must preserve them.

#### Counter identity, name splitting, and scope tiers

| Rule | Specification | Verified case |
|---|---|---|
| Canonical identity | `(group, name, rollup, stats kind)` | `disk.scsiReservationCnflctsPct.average` is key 148 (absolute) and key 358 (rate). |
| Name split | First dot and last dot, with at least one character of name between them. Group is before the first dot, rollup is after the last dot (lowercased), name is the middle. | 247 of 732 names contain dots. `cpu.capacity.contention.average` → group `cpu`, name `capacity.contention`, rollup `average`. |
| Case-distinct constants | Eight `mem` pairs differ only by case (`swapIn` versus `swapin`, each with average, maximum, minimum, and none). | Generated names stay distinct: `SWAP_IN_*` versus `SWAPIN_*`. |
| Dot versus camelCase | When a dotted name and a non-dotted camelCase name in the same group and rollup would produce the same constant, the non-dotted name omits internal camelCase underscores. | `mem.zip.saved.latest` (key 389) stays `ZIP_SAVED_LATEST`; `mem.zipSaved.latest` (key 471) becomes `ZIPSAVED_LATEST`. Zero collisions remain across 732 counters. |
| Duplicate group.name.rollup | The rate counter keeps the unsuffixed constant; the absolute counter appends an absolute suffix. | Rate key 358 unsuffixed; absolute key 148 suffixed. |
| Tier 1 | Direct availability: bitwise union of entity-kind bits for kinds whose availability list contains this counter. 265 counters. | Includes all 17 `rescpu` counters and all 20 `vmop` counters. |
| Tier 2 | If tier 1 is empty, union of tier-1 masks of counters that share `(group, name)`. 96 counters. | `cpu.usage.maximum` (key 4, level 4) inherits virtual machine, host, and cluster from `cpu.usage.average` (key 2). Querying it on a datastore is rejected before the program runs. |
| Tier 3a | If tier 2 is empty and the group has any tier-1 mask, use the union of tier-1 masks in that group. 197 counters across 13 groups. | Group unions below. |
| Tier 3b | If the group has no tier-1 rows, use the inferred domain mask, marked inferred in documentation, and overridable without changing the unit dimension. 174 counters across 11 groups. | Inferred masks below. |

Tier 3a group unions (measured):

| Groups | Admitted kinds |
|---|---|
| `cpu`, `mem`, `power` | Virtual machine, host, cluster, resource pool |
| `virtualDisk` | Virtual machine |
| `storageAdapter`, `storagePath`, `hbr` | Host |
| `net`, `sys`, `clusterServices` | Virtual machine, host |
| `datastore`, `disk` | Datastore, virtual machine, host |
| `gpu` | Cluster, resource pool |

Tier 3b inferred masks (no active samples in the lab dump; overridable):

| Groups (counter count) | Admitted kinds |
|---|---|
| `vmx` (10) | Virtual machine |
| `managementAgent` (5), `vflashModule` (1), `vmotion` (6), `vcResources` (16), `vcDebugInfo` (39) | Host |
| `pmem` (2), `lwd` (1) | Virtual machine, host |
| `nfs` (13), `vvol` (65), `vsanDomObj` (16) | Datastore, virtual machine, host |

Entity-kind bits used by those scopes: virtual machine, host, cluster compute resource, resource pool (including virtual app), datastore (including storage pod), and datacenter. The any-entity mask is only for caller-defined custom metrics, never for a generated constant. Per-kind listings include tier 1, tier 2, and tier 3a counters whose mask contains that kind, and omit tier 3b until real sample rows exist.

#### Unit scaling and display (all 16 observed tokens)

A dimension accepts a unit exactly when scaling that unit succeeds. Percent raw values are hundredths of a percent.

| # | Wire unit | Dimension | Scaled form | Conversion from raw integer | Display |
|---:|---|---|---|---|---|
| 1 | `percent` | ratio | percent | raw × 0.01 | two-decimal percent, e.g. `45.23 %` |
| 2 | `kiloBytes` | memory | bytes (exact wide integer) | raw × 1024 | binary IEC: `B`, `KiB`, `MiB`, `GiB`, else `TiB` (integers under 1024 bytes, otherwise two decimals) |
| 3 | `megaBytes` | memory | bytes | raw × 1,048,576 | same as bytes |
| 4 | other-unit `gigaBytes` | memory | bytes | raw × 1,073,741,824 | same as bytes |
| 5 | `teraBytes` | memory | bytes | raw × 1,099,511,627,776 | same as bytes |
| 6 | `kiloBytesPerSecond` | throughput | bytes per second | raw × 1024 | binary IEC per second: `B/s`, `KiB/s`, `MiB/s`, else `GiB/s` |
| 7 | `megaBytesPerSecond` | throughput | bytes per second | raw × 1,048,576 | same as bytes per second |
| 8 | `megaHertz` | frequency | hertz | raw × 1,000,000 | whole megahertz below 1e9 hertz, otherwise two-decimal gigahertz |
| 9 | `nanosecond` | time | seconds | raw × 1e-9 | `ns`, `µs`, `ms`, or `s` by magnitude |
| 10 | `microsecond` | time | seconds | raw × 1e-6 | same as seconds |
| 11 | `millisecond` | time | seconds | raw × 1e-3 | same as seconds |
| 12 | `second` | time | seconds | raw as seconds | same as seconds |
| 13 | `watt` | power | watts | raw as watts | one decimal watts |
| 14 | `joule` | energy | joules | raw as joules | one decimal joules |
| 15 | `celsius` | temperature | celsius | raw as celsius | one decimal degrees Celsius |
| 16 | `number` | count | number | raw as a number | whole number when fractional part is zero, otherwise two decimals |
| — | any unit via the unconstrained dimension | unconstrained | number | dispatch through rows 1–16 and take the numeric value; unknown other-units use the raw number and record an unscalable-unit warning | same as number |

#### Interval, time bounds, and maximum sample count (first match wins)

Evaluated top to bottom for each requested object.

- If at least one object is queryable (rows 3, 4, 7, 8, 9, 10, 11), fetch queries those objects and returns success, preserving unsupported status for skipped objects.
- If every object matched rows 1, 2, or 5, fetch returns no-supported-entities and does not query.

| Row | Requested interval | Object capabilities | Server intervals | Caller time / latest-N | Resolved query | Diagnostic |
|---:|---|---|---|---|---|---|
| 1 | Any | realtime unsupported and summary unsupported | Any | Any | Skip as not-a-provider | If all skipped: no-supported-entities |
| 2 | Realtime | realtime unsupported, or refresh rate not positive | Any | Any | Skip as realtime-unsupported | If all skipped: no-supported-entities |
| 3 | Realtime | realtime supported, refresh rate R > 0 | Any | None | interval R; start, end, and max samples unset | None |
| 4 | Realtime | realtime supported, refresh rate R > 0 | Any | latest-N and/or a time window | interval R; start and end as requested; max samples is N unless this is an incremental pull, in which case max samples is omitted | If start is older than now minus 3600 seconds: realtime-window-too-old |
| 5 | Historical period P | summary unsupported, or server interval list empty | Any | Any | Skip as historical-unsupported | If all skipped: no-supported-entities |
| 6 | Historical period P | summary supported | Non-empty, P not enabled | Any | Abort before the call | invalid-interval P |
| 7 | Historical period P | summary supported | P enabled, retention L | explicit window (between, since, since-ago, or incremental) | interval P; start and end set; max samples omitted (if latest-N is also set on a non-incremental query, truncate to the newest N slots after decode) | level-too-low / per-device level-too-low when both levels are present and the counter’s level is higher |
| 8 | Historical period P | summary supported | P enabled, retention L | latest-N only | interval P; start = now − (N + 2) × P; end = now; max samples omitted; truncate to newest N slots | same level warnings |
| 9 | Historical period P | summary supported | P enabled, retention L | neither window nor latest-N | interval P; start = now − L; end = now; max samples omitted | same level warnings |
| 10 | Automatic | realtime supported, refresh rate R > 0 | Any | start unset, or start within the last 3600 seconds, or the server has no intervals, or summary is unsupported | Treat as realtime (rows 3–4) | None |
| 11 | Automatic | summary supported, and either realtime is unsupported or start is older than 3600 seconds | Non-empty | Any | Finest enabled period whose retention covers start; if start is unset, the finest enabled period; if start is older than every retention, the coarsest enabled period. Then apply rows 7–9 | Always record automatic-interval-fallback with the object kind and chosen period |

Named presets select the enabled historical interval for that horizon. On the measured vCenter, past day retains 24 hours and past week retains 7 days. Past month and past year use the matching server interval’s retention length. The period sent on the wire is that interval’s sampling period, not its key.

#### Fault classification for multi-chunk fetch

| Error | Class | Action |
|---|---|---|
| Not-authenticated, invalid-login, or caller cancellation | Session / fatal | Abort remaining chunks and fail the fetch. An object-scoped permission error is not in this class. |
| Transport, HTTP, or connection failure (anything that is not a method fault or a parse error) | Transport / fatal | Abort remaining chunks and fail the fetch. |
| Any other method fault (including object-scoped permission, invalid argument, system error, not supported) or a parse error | Chunk-local | Mark only that chunk’s objects failed. Sibling chunks continue. If every chunk failed, return the first error. |

#### Committed catalog and fixtures

Library tests must be self-contained. Catalog bytes are sanitized (`server` replaced with a documentation host, instance identity zeroed, sample object IDs replaced with synthetic IDs).

| Committed artifact | What it contains | Requirements it locks |
|---|---|---|
| Generator catalog and library `vcenter_catalog` fixture (same sanitized vCenter 8.0.3 document, format `vim_rs.perf-catalog/1`) | 732 counters, 4 intervals, 12 availability masks | FR-01, FR-24 |
| Library `esxi_catalog` fixture | 353 counters, null levels, empty interval list | FR-01, FR-02, FR-03, FR-12 |
| `query_perf_csv` JSON | CSV entity metrics, 5 samples of the four diagnostic counters | FR-04, FR-09 |
| `query_perf_normal` JSON | Normal-format equivalent of the CSV fixture | FR-09 |
| `query_perf_gaps` JSON | Trailing and mid-series `-1` gaps, including `[38, 41, -1, -1]` | FR-05, FR-06, FR-22 |
| `query_perf_soap` XML | SOAP query response matching the normal and CSV fixtures | FR-08, FR-09 |
| `query_composite` JSON | One host metric plus two child VM metrics | FR-23 |

### Key Entities

- **Counter identity**: Group, name, rollup, and stats kind (absolute, delta, or rate). Stable across servers; the numeric ID is not.
- **Counter definition**: Server catalog entry: numeric ID, unit, optional collection level, optional per-device level, and whether instances exist.
- **Counter registry**: Session catalog plus the server’s historical intervals, with a monotonic generation so a refresh invalidates prepared resolutions.
- **Metric request**: A counter identity plus a unit dimension and an entity scope. Generated requests carry all four identity fields; custom requests may omit stats kind.
- **Provider capabilities**: Per object kind: realtime supported, summary supported, refresh rate. Cached only from a successful probe or a definitive invalid-argument.
- **Entity series**: One managed object’s timestamp grid, interval, and aligned raw series.
- **Entity status**: Ok, no samples, unsupported (not a provider, realtime unsupported, or historical unsupported), or failed with a shared error.
- **Performance result**: Per requested object, a status plus series when present, and a list of warnings (missing counter, dimension mismatch, level too low, interval substituted, realtime window too old, automatic fallback, scope mismatch, unscalable unit).
- **Prepared query**: A reusable, synchronously built description of counters, interval, chunk size, and optional time chunk, resolved against the registry on fetch.
- **Sample watermark**: Per object, counter, and instance high-water timestamp. Independent per object so one idle object does not widen the window for the others.
- **Composite result**: Optional host series, child series keyed by object, and warnings.
- **Scaled value**: The human-facing quantity for a dimension (percent, exact bytes, hertz, seconds, bytes per second, watts, joules, celsius, or a plain number).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A diagnostic program can print scaled CPU and memory values for a set of virtual machines in one call sequence, with no caller-built counter-id map, no per-object provider loop, and no response downcasts. The in-tree snippet demonstrates this.
- **SC-002**: A poll of 100 virtual machines and 4 counters at the default chunk size issues exactly 2 performance queries (64 objects, then 36).
- **SC-003**: 100% of 732 vCenter counters and 100% of 353 ESXi counters resolve from the committed catalogs, including every dotted name and both SCSI reservation variants, with zero constant-name collisions.
- **SC-004**: 100% of generated counters admit a strict subset of entity kinds. Requesting CPU or memory usage (average or CPU maximum) on a datastore, or host-only device read latency on a virtual machine, is rejected before the program runs.
- **SC-005**: The normal JSON fixture, the CSV JSON fixture, and the SOAP XML fixture decode to equal objects, timestamps, series values, and statuses.
- **SC-006**: For raw values `[420, 410, -1, -1]`, the newest valid sample is 410 and the current slot is blank. A width-5 window over `[100, -1, 300]` is blank, blank, 100, blank, 300.
- **SC-007**: Scaling `1048576` kiloBytes yields exactly 1,073,741,824 bytes, and scaling `16` gigaBytes yields exactly 17,179,869,184 bytes.
- **SC-008**: A mixed ESXi batch of one realtime virtual machine and one non-provider datastore succeeds, with the datastore marked unsupported. The same pair on a past-day historical preset returns no-supported-entities and does not query.
- **SC-009**: An incremental export of a fleet emits each valid sample once, never emits a negative gap, and does not move one object’s start time because a different object is powered off.
- **SC-010**: The default library build adds zero dependencies. Civil-time and columnar-frame helpers are off unless the caller opts in.
- **SC-011**: A past-day query with no explicit window and no latest-N requests a start 24 hours back and an end of now, and does not send a maximum sample count.

## Assumptions

- Target release is `vim_rs` 0.7.0. `vim_rs::core::client::Error` is `Clone`. `MethodFault` holds `Arc<MethodFault>` and `ReqwestError` holds `Arc<reqwest::Error>`. The performance module is otherwise additive. There is no second public module path.
- Measured wire behavior is from vCenter 8.0.3 build 24305161, ESXi 8.0.3 build 24414501, and vcsim reporting 6.5.0. Those measurements override generic API prose where they differ (interval key versus sampling period, omitted powered-off objects, ESXi invalid-argument on some provider summaries, unset ESXi levels, `gigaBytes` as an other-unit).
- Realtime retention is about 60 minutes (3600 seconds). Historical presets use the server interval’s retention length. On the measured vCenter the enabled periods are 300, 1800, 7200, and 86400 seconds.
- Percent counters are hundredths of a percent. Memory units are powers of 1024, not powers of 1000.
- The default chunk size is 256 metric IDs and the default in-flight chunk concurrency is 4. Callers can lower the chunk size. The availability cache holds 1024 non-empty entries.
- Callers own poll timers and cancellation. Dropping an in-flight fetch cancels its calls; the library does not schedule the next poll.
- Sanitized catalogs and fixtures contain no live hostnames, instance identities, or real object IDs and are safe to commit. The generator catalog and the library fixture are the same vCenter document in two places so tests do not cross crate boundaries.
- Ordinary library builds do not regenerate constants. Regeneration is an explicit generator run, and the emitted file is committed.
- The external inventory application is a downstream consumer. This feature must make its hand-rolled poller replaceable; updating that repository is not part of this change. The in-tree diagnostic snippet is in scope.
- Custom metrics (not in the generated catalog) are allowed, use the any-entity scope, and may omit stats kind (preferring rate when both exist).
- Both VI/JSON and SOAP/XML must decode to the same series. SOAP is available only when that transport feature is enabled. Automatic format follows FR-08 on each transport.
- No new default dependency, and no serialization stack beyond what the library already uses. `vim_rs::perf` itself is the `perf` feature, off by default, so callers who do not query performance do not compile the catalog. Optional civil-time (`perf-chrono`) and columnar (`perf-polars`) features enable `perf` and stay off the default build.
- Session credentials are never logged. Performance queries do not carry session secrets in result types.

## Out of Scope

- Editing vCenter collection intervals, statistics levels, or the max-query-metrics advanced option.
- vSAN performance manager and any non-VIM telemetry API.
- Saving the counter catalog to disk between processes.
- A library-owned poll loop, timer, or infinite stream.
- Composite queries rooted at a resource pool, folder, or cluster.
- Bisecting a batch to find a deleted object. Omission stays “no samples”.
- Querying the option manager for a chunk-size hint.
- Hand-editing generated counter constants. Scope and naming changes go through the generator and the committed catalog.
- Porting the external inventory application in this change.
- New required dependencies on the default build.
