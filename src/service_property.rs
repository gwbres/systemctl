use strum_macros::{EnumString, IntoStaticStr};

#[derive(Copy, Clone, PartialEq, Eq, EnumString, IntoStaticStr, Debug)]
pub enum ServiceProperty {
    #[strum(serialize = "Type")]
    Type,
    #[strum(serialize = "ExitType")]
    ExitType,
    #[strum(serialize = "Restart")]
    Restart,
    #[strum(serialize = "RestartMode")]
    RestartMode,
    #[strum(serialize = "NotifyAccess")]
    NotifyAccess,
    #[strum(serialize = "RestartUSec")]
    RestartUSec,
    #[strum(serialize = "RestartSteps")]
    RestartSteps,
    #[strum(serialize = "RestartMaxDelayUSec")]
    RestartMaxDelayUSec,
    #[strum(serialize = "RestartUSecNext")]
    RestartUSecNext,
    #[strum(serialize = "TimeoutStartUSec")]
    TimeoutStartUSec,
    #[strum(serialize = "TimeoutStopUSec")]
    TimeoutStopUSec,
    #[strum(serialize = "TimeoutAbortUSec")]
    TimeoutAbortUSec,
    #[strum(serialize = "TimeoutStartFailureMode")]
    TimeoutStartFailureMode,
    #[strum(serialize = "TimeoutStopFailureMode")]
    TimeoutStopFailureMode,
    #[strum(serialize = "RuntimeMaxUSec")]
    RuntimeMaxUSec,
    #[strum(serialize = "RuntimeRandomizedExtraUSec")]
    RuntimeRandomizedExtraUSec,
    #[strum(serialize = "WatchdogUSec")]
    WatchdogUSec,
    #[strum(serialize = "WatchdogTimestampMonotonic")]
    WatchdogTimestampMonotonic,
    #[strum(serialize = "RootDirectoryStartOnly")]
    RootDirectoryStartOnly,
    #[strum(serialize = "RemainAfterExit")]
    RemainAfterExit,
    #[strum(serialize = "GuessMainPID")]
    GuessMainPID,
    #[strum(serialize = "MainPID")]
    MainPID,
    #[strum(serialize = "ControlPID")]
    ControlPID,
    #[strum(serialize = "BusName")]
    BusName,
    #[strum(serialize = "FileDescriptorStoreMax")]
    FileDescriptorStoreMax,
    #[strum(serialize = "NFileDescriptorStore")]
    NFileDescriptorStore,
    #[strum(serialize = "FileDescriptorStorePreserve")]
    FileDescriptorStorePreserve,
    #[strum(serialize = "StatusErrno")]
    StatusErrno,
    #[strum(serialize = "Result")]
    Result,
    #[strum(serialize = "ReloadResult")]
    ReloadResult,
    #[strum(serialize = "CleanResult")]
    CleanResult,
    #[strum(serialize = "UID")]
    UID,
    #[strum(serialize = "GID")]
    GID,
    #[strum(serialize = "NRestarts")]
    NRestarts,
    #[strum(serialize = "OOMPolicy")]
    OOMPolicy,
    #[strum(serialize = "ReloadSignal")]
    ReloadSignal,
    #[strum(serialize = "ExecMainStartTimestamp")]
    ExecMainStartTimestamp,
    #[strum(serialize = "ExecMainStartTimestampMonotonic")]
    ExecMainStartTimestampMonotonic,
    #[strum(serialize = "ExecMainExitTimestampMonotonic")]
    ExecMainExitTimestampMonotonic,
    #[strum(serialize = "ExecMainHandoffTimestamp")]
    ExecMainHandoffTimestamp,
    #[strum(serialize = "ExecMainHandoffTimestampMonotonic")]
    ExecMainHandoffTimestampMonotonic,
    #[strum(serialize = "ExecMainPID")]
    ExecMainPID,
    #[strum(serialize = "ExecMainCode")]
    ExecMainCode,
    #[strum(serialize = "ExecMainStatus")]
    ExecMainStatus,
    #[strum(serialize = "ExecStart")]
    ExecStart,
    #[strum(serialize = "ExecStartEx")]
    ExecStartEx,
    #[strum(serialize = "ExecReload")]
    ExecReload,
    #[strum(serialize = "ExecReloadEx")]
    ExecReloadEx,
    #[strum(serialize = "Slice")]
    Slice,
    #[strum(serialize = "ControlGroup")]
    ControlGroup,
    #[strum(serialize = "ControlGroupId")]
    ControlGroupId,
    #[strum(serialize = "MemoryCurrent")]
    MemoryCurrent,
    #[strum(serialize = "MemoryPeak")]
    MemoryPeak,
    #[strum(serialize = "MemorySwapCurrent")]
    MemorySwapCurrent,
    #[strum(serialize = "MemorySwapPeak")]
    MemorySwapPeak,
    #[strum(serialize = "MemoryZSwapCurrent")]
    MemoryZSwapCurrent,
    #[strum(serialize = "MemoryAvailable")]
    MemoryAvailable,
    #[strum(serialize = "EffectiveMemoryMax")]
    EffectiveMemoryMax,
    #[strum(serialize = "EffectiveMemoryHigh")]
    EffectiveMemoryHigh,
    #[strum(serialize = "CPUUsageNSec")]
    CPUUsageNSec,
    #[strum(serialize = "TasksCurrent")]
    TasksCurrent,
    #[strum(serialize = "EffectiveTasksMax")]
    EffectiveTasksMax,
    #[strum(serialize = "IPIngressBytes")]
    IPIngressBytes,
    #[strum(serialize = "IPIngressPackets")]
    IPIngressPackets,
    #[strum(serialize = "IPEgressBytes")]
    IPEgressBytes,
    #[strum(serialize = "IPEgressPackets")]
    IPEgressPackets,
    #[strum(serialize = "IOReadBytes")]
    IOReadBytes,
    #[strum(serialize = "IOReadOperations")]
    IOReadOperations,
    #[strum(serialize = "IOWriteBytes")]
    IOWriteBytes,
    #[strum(serialize = "IOWriteOperations")]
    IOWriteOperations,
    #[strum(serialize = "Delegate")]
    Delegate,
    #[strum(serialize = "CPUAccounting")]
    CPUAccounting,
    #[strum(serialize = "CPUWeight")]
    CPUWeight,
    #[strum(serialize = "StartupCPUWeight")]
    StartupCPUWeight,
    #[strum(serialize = "CPUShares")]
    CPUShares,
    #[strum(serialize = "StartupCPUShares")]
    StartupCPUShares,
    #[strum(serialize = "CPUQuotaPerSecUSec")]
    CPUQuotaPerSecUSec,
    #[strum(serialize = "CPUQuotaPeriodUSec")]
    CPUQuotaPeriodUSec,
    #[strum(serialize = "IOAccounting")]
    IOAccounting,
    #[strum(serialize = "IOWeight")]
    IOWeight,
    #[strum(serialize = "StartupIOWeight")]
    StartupIOWeight,
    #[strum(serialize = "BlockIOAccounting")]
    BlockIOAccounting,
    #[strum(serialize = "BlockIOWeight")]
    BlockIOWeight,
    #[strum(serialize = "StartupBlockIOWeight")]
    StartupBlockIOWeight,
    #[strum(serialize = "MemoryAccounting")]
    MemoryAccounting,
    #[strum(serialize = "DefaultMemoryLow")]
    DefaultMemoryLow,
    #[strum(serialize = "DefaultStartupMemoryLow")]
    DefaultStartupMemoryLow,
    #[strum(serialize = "DefaultMemoryMin")]
    DefaultMemoryMin,
    #[strum(serialize = "MemoryMin")]
    MemoryMin,
    #[strum(serialize = "MemoryLow")]
    MemoryLow,
    #[strum(serialize = "StartupMemoryLow")]
    StartupMemoryLow,
    #[strum(serialize = "MemoryHigh")]
    MemoryHigh,
    #[strum(serialize = "StartupMemoryHigh")]
    StartupMemoryHigh,
    #[strum(serialize = "MemoryMax")]
    MemoryMax,
    #[strum(serialize = "StartupMemoryMax")]
    StartupMemoryMax,
    #[strum(serialize = "MemorySwapMax")]
    MemorySwapMax,
    #[strum(serialize = "StartupMemorySwapMax")]
    StartupMemorySwapMax,
    #[strum(serialize = "MemoryZSwapMax")]
    MemoryZSwapMax,
    #[strum(serialize = "StartupMemoryZSwapMax")]
    StartupMemoryZSwapMax,
    #[strum(serialize = "MemoryZSwapWriteback")]
    MemoryZSwapWriteback,
    #[strum(serialize = "MemoryLimit")]
    MemoryLimit,
    #[strum(serialize = "DevicePolicy")]
    DevicePolicy,
    #[strum(serialize = "TasksAccounting")]
    TasksAccounting,
    #[strum(serialize = "TasksMax")]
    TasksMax,
    #[strum(serialize = "IPAccounting")]
    IPAccounting,
    #[strum(serialize = "ManagedOOMSwap")]
    ManagedOOMSwap,
    #[strum(serialize = "ManagedOOMMemoryPressure")]
    ManagedOOMMemoryPressure,
    #[strum(serialize = "ManagedOOMMemoryPressureLimit")]
    ManagedOOMMemoryPressureLimit,
    #[strum(serialize = "ManagedOOMPreference")]
    ManagedOOMPreference,
    #[strum(serialize = "MemoryPressureWatch")]
    MemoryPressureWatch,
    #[strum(serialize = "MemoryPressureThresholdUSec")]
    MemoryPressureThresholdUSec,
    #[strum(serialize = "CoredumpReceive")]
    CoredumpReceive,
    #[strum(serialize = "UMask")]
    UMask,
    #[strum(serialize = "LimitCPU")]
    LimitCPU,
    #[strum(serialize = "LimitCPUSoft")]
    LimitCPUSoft,
    #[strum(serialize = "LimitFSIZE")]
    LimitFSIZE,
    #[strum(serialize = "LimitFSIZESoft")]
    LimitFSIZESoft,
    #[strum(serialize = "LimitDATA")]
    LimitDATA,
    #[strum(serialize = "LimitDATASoft")]
    LimitDATASoft,
    #[strum(serialize = "LimitSTACK")]
    LimitSTACK,
    #[strum(serialize = "LimitSTACKSoft")]
    LimitSTACKSoft,
    #[strum(serialize = "LimitCORE")]
    LimitCORE,
    #[strum(serialize = "LimitCORESoft")]
    LimitCORESoft,
    #[strum(serialize = "LimitRSS")]
    LimitRSS,
    #[strum(serialize = "LimitRSSSoft")]
    LimitRSSSoft,
    #[strum(serialize = "LimitNOFILE")]
    LimitNOFILE,
    #[strum(serialize = "LimitNOFILESoft")]
    LimitNOFILESoft,
    #[strum(serialize = "LimitAS")]
    LimitAS,
    #[strum(serialize = "LimitASSoft")]
    LimitASSoft,
    #[strum(serialize = "LimitNPROC")]
    LimitNPROC,
    #[strum(serialize = "LimitNPROCSoft")]
    LimitNPROCSoft,
    #[strum(serialize = "LimitMEMLOCK")]
    LimitMEMLOCK,
    #[strum(serialize = "LimitMEMLOCKSoft")]
    LimitMEMLOCKSoft,
    #[strum(serialize = "LimitLOCKS")]
    LimitLOCKS,
    #[strum(serialize = "LimitLOCKSSoft")]
    LimitLOCKSSoft,
    #[strum(serialize = "LimitSIGPENDING")]
    LimitSIGPENDING,
    #[strum(serialize = "LimitSIGPENDINGSoft")]
    LimitSIGPENDINGSoft,
    #[strum(serialize = "LimitMSGQUEUE")]
    LimitMSGQUEUE,
    #[strum(serialize = "LimitMSGQUEUESoft")]
    LimitMSGQUEUESoft,
    #[strum(serialize = "LimitNICE")]
    LimitNICE,
    #[strum(serialize = "LimitNICESoft")]
    LimitNICESoft,
    #[strum(serialize = "LimitRTPRIO")]
    LimitRTPRIO,
    #[strum(serialize = "LimitRTPRIOSoft")]
    LimitRTPRIOSoft,
    #[strum(serialize = "LimitRTTIME")]
    LimitRTTIME,
    #[strum(serialize = "LimitRTTIMESoft")]
    LimitRTTIMESoft,
    #[strum(serialize = "RootEphemeral")]
    RootEphemeral,
    #[strum(serialize = "OOMScoreAdjust")]
    OOMScoreAdjust,
    #[strum(serialize = "CoredumpFilter")]
    CoredumpFilter,
    #[strum(serialize = "Nice")]
    Nice,
    #[strum(serialize = "IOSchedulingClass")]
    IOSchedulingClass,
    #[strum(serialize = "IOSchedulingPriority")]
    IOSchedulingPriority,
    #[strum(serialize = "CPUSchedulingPolicy")]
    CPUSchedulingPolicy,
    #[strum(serialize = "CPUSchedulingPriority")]
    CPUSchedulingPriority,
    #[strum(serialize = "CPUAffinityFromNUMA")]
    CPUAffinityFromNUMA,
    #[strum(serialize = "NUMAPolicy")]
    NUMAPolicy,
    #[strum(serialize = "TimerSlackNSec")]
    TimerSlackNSec,
    #[strum(serialize = "CPUSchedulingResetOnFork")]
    CPUSchedulingResetOnFork,
    #[strum(serialize = "NonBlocking")]
    NonBlocking,
    #[strum(serialize = "StandardInput")]
    StandardInput,
    #[strum(serialize = "StandardOutput")]
    StandardOutput,
    #[strum(serialize = "StandardError")]
    StandardError,
    #[strum(serialize = "TTYReset")]
    TTYReset,
    #[strum(serialize = "TTYVHangup")]
    TTYVHangup,
    #[strum(serialize = "TTYVTDisallocate")]
    TTYVTDisallocate,
    #[strum(serialize = "SyslogPriority")]
    SyslogPriority,
    #[strum(serialize = "SyslogLevelPrefix")]
    SyslogLevelPrefix,
    #[strum(serialize = "SyslogLevel")]
    SyslogLevel,
    #[strum(serialize = "SyslogFacility")]
    SyslogFacility,
    #[strum(serialize = "LogLevelMax")]
    LogLevelMax,
    #[strum(serialize = "LogRateLimitIntervalUSec")]
    LogRateLimitIntervalUSec,
    #[strum(serialize = "LogRateLimitBurst")]
    LogRateLimitBurst,
    #[strum(serialize = "SecureBits")]
    SecureBits,
    #[strum(serialize = "CapabilityBoundingSet")]
    CapabilityBoundingSet,
    #[strum(serialize = "DynamicUser")]
    DynamicUser,
    #[strum(serialize = "SetLoginEnvironment")]
    SetLoginEnvironment,
    #[strum(serialize = "RemoveIPC")]
    RemoveIPC,
    #[strum(serialize = "PrivateTmp")]
    PrivateTmp,
    #[strum(serialize = "PrivateDevices")]
    PrivateDevices,
    #[strum(serialize = "ProtectClock")]
    ProtectClock,
    #[strum(serialize = "ProtectKernelTunables")]
    ProtectKernelTunables,
    #[strum(serialize = "ProtectKernelModules")]
    ProtectKernelModules,
    #[strum(serialize = "ProtectKernelLogs")]
    ProtectKernelLogs,
    #[strum(serialize = "ProtectControlGroups")]
    ProtectControlGroups,
    #[strum(serialize = "PrivateNetwork")]
    PrivateNetwork,
    #[strum(serialize = "PrivateUsers")]
    PrivateUsers,
    #[strum(serialize = "PrivateMounts")]
    PrivateMounts,
    #[strum(serialize = "PrivateIPC")]
    PrivateIPC,
    #[strum(serialize = "ProtectHome")]
    ProtectHome,
    #[strum(serialize = "ProtectSystem")]
    ProtectSystem,
    #[strum(serialize = "SameProcessGroup")]
    SameProcessGroup,
    #[strum(serialize = "UtmpMode")]
    UtmpMode,
    #[strum(serialize = "IgnoreSIGPIPE")]
    IgnoreSIGPIPE,
    #[strum(serialize = "NoNewPrivileges")]
    NoNewPrivileges,
    #[strum(serialize = "SystemCallErrorNumber")]
    SystemCallErrorNumber,
    #[strum(serialize = "LockPersonality")]
    LockPersonality,
    #[strum(serialize = "RuntimeDirectoryPreserve")]
    RuntimeDirectoryPreserve,
    #[strum(serialize = "RuntimeDirectoryMode")]
    RuntimeDirectoryMode,
    #[strum(serialize = "StateDirectoryMode")]
    StateDirectoryMode,
    #[strum(serialize = "CacheDirectoryMode")]
    CacheDirectoryMode,
    #[strum(serialize = "LogsDirectoryMode")]
    LogsDirectoryMode,
    #[strum(serialize = "ConfigurationDirectoryMode")]
    ConfigurationDirectoryMode,
    #[strum(serialize = "TimeoutCleanUSec")]
    TimeoutCleanUSec,
    #[strum(serialize = "MemoryDenyWriteExecute")]
    MemoryDenyWriteExecute,
    #[strum(serialize = "RestrictRealtime")]
    RestrictRealtime,
    #[strum(serialize = "RestrictSUIDSGID")]
    RestrictSUIDSGID,
    #[strum(serialize = "RestrictNamespaces")]
    RestrictNamespaces,
    #[strum(serialize = "MountAPIVFS")]
    MountAPIVFS,
    #[strum(serialize = "KeyringMode")]
    KeyringMode,
    #[strum(serialize = "ProtectProc")]
    ProtectProc,
    #[strum(serialize = "ProcSubset")]
    ProcSubset,
    #[strum(serialize = "ProtectHostname")]
    ProtectHostname,
    #[strum(serialize = "MemoryKSM")]
    MemoryKSM,
    #[strum(serialize = "RootImagePolicy")]
    RootImagePolicy,
    #[strum(serialize = "MountImagePolicy")]
    MountImagePolicy,
    #[strum(serialize = "ExtensionImagePolicy")]
    ExtensionImagePolicy,
    #[strum(serialize = "KillMode")]
    KillMode,
    #[strum(serialize = "KillSignal")]
    KillSignal,
    #[strum(serialize = "RestartKillSignal")]
    RestartKillSignal,
    #[strum(serialize = "FinalKillSignal")]
    FinalKillSignal,
    #[strum(serialize = "SendSIGKILL")]
    SendSIGKILL,
    #[strum(serialize = "SendSIGHUP")]
    SendSIGHUP,
    #[strum(serialize = "WatchdogSignal")]
    WatchdogSignal,
    #[strum(serialize = "Id")]
    Id,
    #[strum(serialize = "Names")]
    Names,
    #[strum(serialize = "Requires")]
    Requires,
    #[strum(serialize = "Wants")]
    Wants,
    #[strum(serialize = "BindsTo")]
    BindsTo,
    #[strum(serialize = "RequiredBy")]
    RequiredBy,
    #[strum(serialize = "WantedBy")]
    WantedBy,
    #[strum(serialize = "Conflicts")]
    Conflicts,
    #[strum(serialize = "Before")]
    Before,
    #[strum(serialize = "After")]
    After,
    #[strum(serialize = "Documentation")]
    Documentation,
    #[strum(serialize = "Description")]
    Description,
    #[strum(serialize = "LoadState")]
    LoadState,
    #[strum(serialize = "ActiveState")]
    ActiveState,
    #[strum(serialize = "FreezerState")]
    FreezerState,
    #[strum(serialize = "SubState")]
    SubState,
    #[strum(serialize = "FragmentPath")]
    FragmentPath,
    #[strum(serialize = "UnitFileState")]
    UnitFileState,
    #[strum(serialize = "UnitFilePreset")]
    UnitFilePreset,
    #[strum(serialize = "StateChangeTimestamp")]
    StateChangeTimestamp,
    #[strum(serialize = "StateChangeTimestampMonotonic")]
    StateChangeTimestampMonotonic,
    #[strum(serialize = "InactiveExitTimestamp")]
    InactiveExitTimestamp,
    #[strum(serialize = "InactiveExitTimestampMonotonic")]
    InactiveExitTimestampMonotonic,
    #[strum(serialize = "ActiveEnterTimestamp")]
    ActiveEnterTimestamp,
    #[strum(serialize = "ActiveEnterTimestampMonotonic")]
    ActiveEnterTimestampMonotonic,
    #[strum(serialize = "ActiveExitTimestamp")]
    ActiveExitTimestamp,
    #[strum(serialize = "ActiveExitTimestampMonotonic")]
    ActiveExitTimestampMonotonic,
    #[strum(serialize = "InactiveEnterTimestamp")]
    InactiveEnterTimestamp,
    #[strum(serialize = "InactiveEnterTimestampMonotonic")]
    InactiveEnterTimestampMonotonic,
    #[strum(serialize = "CanStart")]
    CanStart,
    #[strum(serialize = "CanStop")]
    CanStop,
    #[strum(serialize = "CanReload")]
    CanReload,
    #[strum(serialize = "CanIsolate")]
    CanIsolate,
    #[strum(serialize = "CanFreeze")]
    CanFreeze,
    #[strum(serialize = "StopWhenUnneeded")]
    StopWhenUnneeded,
    #[strum(serialize = "RefuseManualStart")]
    RefuseManualStart,
    #[strum(serialize = "RefuseManualStop")]
    RefuseManualStop,
    #[strum(serialize = "AllowIsolate")]
    AllowIsolate,
    #[strum(serialize = "DefaultDependencies")]
    DefaultDependencies,
    #[strum(serialize = "SurviveFinalKillSignal")]
    SurviveFinalKillSignal,
    #[strum(serialize = "OnSuccessJobMode")]
    OnSuccessJobMode,
    #[strum(serialize = "OnFailureJobMode")]
    OnFailureJobMode,
    #[strum(serialize = "IgnoreOnIsolate")]
    IgnoreOnIsolate,
    #[strum(serialize = "NeedDaemonReload")]
    NeedDaemonReload,
    #[strum(serialize = "JobTimeoutUSec")]
    JobTimeoutUSec,
    #[strum(serialize = "JobRunningTimeoutUSec")]
    JobRunningTimeoutUSec,
    #[strum(serialize = "JobTimeoutAction")]
    JobTimeoutAction,
    #[strum(serialize = "ConditionResult")]
    ConditionResult,
    #[strum(serialize = "AssertResult")]
    AssertResult,
    #[strum(serialize = "ConditionTimestamp")]
    ConditionTimestamp,
    #[strum(serialize = "ConditionTimestampMonotonic")]
    ConditionTimestampMonotonic,
    #[strum(serialize = "AssertTimestamp")]
    AssertTimestamp,
    #[strum(serialize = "AssertTimestampMonotonic")]
    AssertTimestampMonotonic,
    #[strum(serialize = "Transient")]
    Transient,
    #[strum(serialize = "Perpetual")]
    Perpetual,
    #[strum(serialize = "StartLimitIntervalUSec")]
    StartLimitIntervalUSec,
    #[strum(serialize = "StartLimitBurst")]
    StartLimitBurst,
    #[strum(serialize = "StartLimitAction")]
    StartLimitAction,
    #[strum(serialize = "FailureAction")]
    FailureAction,
    #[strum(serialize = "SuccessAction")]
    SuccessAction,
    #[strum(serialize = "InvocationID")]
    InvocationID,
    #[strum(serialize = "CollectMode")]
    CollectMode,
}
