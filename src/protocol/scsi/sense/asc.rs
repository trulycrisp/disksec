//! Additional Sense Code (ASC/ASCQ) parsing.

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdditionalSenseCode {
    NoAdditionalSenseInformation,
    FilemarkDetected,
    EndOfPartitionOrMediumDetected,
    SetmarkDetected,
    BeginningOfPartitionOrMediumDetected,
    EndOfDataDetected,
    IoProcessTerminated,
    ProgrammableEarlyWarningDetected,
    AudioPlayOperationInProgress,
    AudioPlayOperationPaused,
    AudioPlayOperationSuccessfullyCompleted,
    AudioPlayOperationStoppedDueToError,
    NoCurrentAudioStatusToReturn,
    OperationInProgress,
    CleaningRequested,
    EraseOperationInProgress,
    LocateOperationInProgress,
    RewindOperationInProgress,
    SetCapacityOperationInProgress,
    VerifyOperationInProgress,
    AtaPassThroughInformationAvailable,
    ConflictingSaCreationRequest,
    LogicalUnitTransitioningToAnotherPowerCondition,
    ExtendedCopyInformationAvailable,
    AtomicCommandAbortedDueToAca,
    DeferredMicrocodeIsPending,
    OverlappingAtomicCommandInProgress,
    NoIndexOrSectorSignal,
    NoSeekComplete,
    PeripheralDeviceWriteFault,
    NoWriteCurrent,
    ExcessiveWriteErrors,
    LogicalUnitNotReadyCauseNotReportable,
    LogicalUnitIsInProcessOfBecomingReady,
    LogicalUnitNotReadyInitializingCommandRequired,
    LogicalUnitNotReadyManualInterventionRequired,
    LogicalUnitNotReadyFormatInProgress,
    LogicalUnitNotReadyRebuildInProgress,
    LogicalUnitNotReadyRecalculationInProgress,
    LogicalUnitNotReadyOperationInProgress,
    LogicalUnitNotReadyLongWriteInProgress,
    LogicalUnitNotReadySelfTestInProgress,
    LogicalUnitNotAccessibleAsymmetricAccessStateTransition,
    LogicalUnitNotAccessibleTargetPortInStandbyState,
    LogicalUnitNotAccessibleTargetPortInUnavailableState,
    LogicalUnitNotReadyStructureCheckRequired,
    LogicalUnitNotReadySecuritySessionInProgress,
    LogicalUnitNotReadyAuxiliaryMemoryNotAccessible,
    LogicalUnitNotReadyNotifyEnableSpinupRequired,
    LogicalUnitNotReadyOffline,
    LogicalUnitNotReadySaCreationInProgress,
    LogicalUnitNotReadySpaceAllocationInProgress,
    LogicalUnitNotReadyRoboticsDisabled,
    LogicalUnitNotReadyConfigurationRequired,
    LogicalUnitNotReadyCalibrationRequired,
    LogicalUnitNotReadyADoorIsOpen,
    LogicalUnitNotReadyOperatingInSequentialMode,
    LogicalUnitNotReadyStartStopUnitCommandInProgress,
    LogicalUnitNotReadySanitizeInProgress,
    LogicalUnitNotReadyAdditionalPowerUseNotYetGranted,
    LogicalUnitNotReadyConfigurationInProgress,
    LogicalUnitNotReadyMicrocodeActivationRequired,
    LogicalUnitNotReadyMicrocodeDownloadRequired,
    LogicalUnitNotReadyLogicalUnitResetRequired,
    LogicalUnitNotReadyHardResetRequired,
    LogicalUnitNotReadyPowerCycleRequired,
    LogicalUnitNotReadyAffiliationRequired,
    DepopulationInProgress,
    DepopulationRestorationInProgress,
    LogicalUnitDoesNotRespondToSelection,
    NoReferencePositionFound,
    MultiplePeripheralDevicesSelected,
    LogicalUnitCommunicationFailure,
    LogicalUnitCommunicationTimeOut,
    LogicalUnitCommunicationParityError,
    LogicalUnitCommunicationCrcErrorUltraDma32,
    UnreachableCopyTarget,
    TrackFollowingError,
    TrackingServoFailure,
    FocusServoFailure,
    SpindleServoFailure,
    HeadSelectFault,
    VibrationInducedTrackingError,
    ErrorLogOverflow,
    Warning,
    WarningSpecifiedTemperatureExceeded,
    WarningEnclosureDegraded,
    WarningBackgroundSelfTestFailed,
    WarningBackgroundPreScanDetectedMediumError,
    WarningBackgroundMediumScanDetectedMediumError,
    WarningNonVolatileCacheNowVolatile,
    WarningDegradedPowerToNonVolatileCache,
    WarningPowerLossExpected,
    WarningDeviceStatisticsNotificationActive,
    WarningHighCriticalTemperatureLimitExceeded,
    WarningLowCriticalTemperatureLimitExceeded,
    WarningHighOperatingTemperatureLimitExceeded,
    WarningLowOperatingTemperatureLimitExceeded,
    WarningHighCriticalHumidityLimitExceeded,
    WarningLowCriticalHumidityLimitExceeded,
    WarningHighOperatingHumidityLimitExceeded,
    WarningLowOperatingHumidityLimitExceeded,
    WarningMicrocodeSecurityAtRisk,
    WarningMicrocodeDigitalSignatureValidationFailure,
    WarningPhysicalElementStatusChange,
    WriteError,
    WriteErrorRecoveredWithAutoReallocation,
    WriteErrorAutoReallocationFailed,
    WriteErrorRecommendReassignment,
    CompressionCheckMiscompareError,
    DataExpansionOccurredDuringCompression,
    BlockNotCompressible,
    WriteErrorRecoveryNeeded,
    WriteErrorRecoveryFailed,
    WriteErrorLossOfStreaming,
    WriteErrorPaddingBlocksAdded,
    AuxiliaryMemoryWriteError,
    WriteErrorUnexpectedUnsolicitedData,
    WriteErrorNotEnoughUnsolicitedData,
    MultipleWriteErrors,
    DefectsInErrorWindow,
    IncompleteMultipleAtomicWriteOperations,
    WriteErrorRecoveryScanNeeded,
    WriteErrorInsufficientZoneResources,
    ErrorDetectedByThirdPartyTemporaryInitiator,
    ThirdPartyDeviceFailure,
    CopyTargetDeviceNotReachable,
    IncorrectCopyTargetDeviceType,
    CopyTargetDeviceDataUnderrun,
    CopyTargetDeviceDataOverrun,
    InvalidInformationUnit,
    InformationUnitTooShort,
    InformationUnitTooLong,
    InvalidFieldInCommandInformationUnit,
    IdCrcOrEccError,
    LogicalBlockGuardCheckFailed,
    LogicalBlockApplicationTagCheckFailed,
    LogicalBlockReferenceTagCheckFailed,
    LogicalBlockProtectionErrorOnRecoverBufferedData,
    LogicalBlockProtectionMethodError,
    UnrecoveredReadError,
    ReadRetriesExhausted,
    ErrorTooLongToCorrect,
    MultipleReadErrors,
    UnrecoveredReadErrorAutoReallocateFailed,
    LEcUncorrectableError,
    CircUnrecoveredError,
    DataReSynchronizationError,
    IncompleteBlockRead,
    NoGapFound,
    MiscorrectedError,
    UnrecoveredReadErrorRecommendReassignment,
    UnrecoveredReadErrorRecommendRewriteTheData,
    DeCompressionCrcError,
    CannotDecompressUsingDeclaredAlgorithm,
    ErrorReadingUpcOrEanNumber,
    ErrorReadingIsrcNumber,
    ReadErrorLossOfStreaming,
    AuxiliaryMemoryReadError,
    ReadErrorFailedRetransmissionRequest,
    ReadErrorLbaMarkedBadByApplicationClient,
    WriteAfterSanitizeRequired,
    AddressMarkNotFoundForIdField,
    AddressMarkNotFoundForDataField,
    RecordedEntityNotFound,
    RecordNotFound,
    FilemarkOrSetmarkNotFound,
    EndOfDataNotFound,
    BlockSequenceError,
    RecordNotFoundRecommendReassignment,
    RecordNotFoundDataAutoReallocated,
    LocateOperationFailure,
    RandomPositioningError,
    MechanicalPositioningError,
    PositioningErrorDetectedByReadOfMedium,
    DataSynchronizationMarkError,
    DataSyncErrorDataRewritten,
    DataSyncErrorRecommendRewrite,
    DataSyncErrorDataAutoReallocated,
    DataSyncErrorRecommendReassignment,
    RecoveredDataWithNoErrorCorrectionApplied,
    RecoveredDataWithRetries,
    RecoveredDataWithPositiveHeadOffset,
    RecoveredDataWithNegativeHeadOffset,
    RecoveredDataWithRetriesAndOrCircApplied,
    RecoveredDataUsingPreviousSectorId,
    RecoveredDataWithoutEccDataAutoReallocated,
    RecoveredDataWithoutEccRecommendReassignment,
    RecoveredDataWithoutEccRecommendRewrite,
    RecoveredDataWithoutEccDataRewritten,
    RecoveredDataWithErrorCorrectionApplied,
    RecoveredDataWithErrorCorrAndRetriesApplied,
    RecoveredDataDataAutoReallocated,
    RecoveredDataWithCirc,
    RecoveredDataWithLEc,
    RecoveredDataRecommendReassignment,
    RecoveredDataRecommendRewrite,
    RecoveredDataWithEccDataRewritten,
    RecoveredDataWithLinking,
    DefectListError,
    DefectListNotAvailable,
    DefectListErrorInPrimaryList,
    DefectListErrorInGrownList,
    ParameterListLengthError,
    SynchronousDataTransferError,
    DefectListNotFound,
    PrimaryDefectListNotFound,
    GrownDefectListNotFound,
    MiscompareDuringVerifyOperation,
    MiscompareVerifyOfUnmappedLba,
    RecoveredIdWithEccCorrection,
    PartialDefectListTransfer,
    InvalidCommandOperationCode,
    AccessDeniedInitiatorPendingEnrolled,
    AccessDeniedNoAccessRights,
    AccessDeniedInvalidMgmtIdKey,
    IllegalCommandWhileInWriteCapableState,
    IllegalCommandWhileInExplicitAddressMode,
    IllegalCommandWhileInImplicitAddressMode,
    AccessDeniedEnrollmentConflict,
    AccessDeniedInvalidLuIdentifier,
    AccessDeniedInvalidProxyToken,
    AccessDeniedAclLunConflict,
    IllegalCommandWhenNotInAppendOnlyMode,
    NotAnAdministrativeLogicalUnit,
    NotASubsidiaryLogicalUnit,
    NotAConglomerateLogicalUnit,
    LogicalBlockAddressOutOfRange,
    InvalidElementAddress,
    InvalidAddressForWrite,
    InvalidWriteCrossingLayerJump,
    UnalignedWriteCommand,
    WriteBoundaryViolation,
    AttemptToReadInvalidData,
    ReadBoundaryViolation,
    MisalignedWriteCommand,
    AttemptToAccessGapZone,
    IllegalFunction,
    InvalidTokenOperationCauseNotReportable,
    InvalidTokenOperationUnsupportedTokenType,
    InvalidTokenOperationRemoteTokenUsageNotSupported,
    InvalidTokenOperationRemoteRodTokenCreationNotSupported,
    InvalidTokenOperationTokenUnknown,
    InvalidTokenOperationTokenCorrupt,
    InvalidTokenOperationTokenRevoked,
    InvalidTokenOperationTokenExpired,
    InvalidTokenOperationTokenCancelled,
    InvalidTokenOperationTokenDeleted,
    InvalidTokenOperationInvalidTokenLength,
    InvalidFieldInCdb,
    CdbDecryptionError,
    SecurityAuditValueFrozen,
    SecurityWorkingKeyFrozen,
    NonceNotUnique,
    NonceTimestampOutOfRange,
    InvalidXcdb,
    InvalidFastFormat,
    LogicalUnitNotSupported,
    InvalidFieldInParameterList,
    ParameterNotSupported,
    ParameterValueInvalid,
    ThresholdParametersNotSupported,
    InvalidReleaseOfPersistentReservation,
    DataDecryptionError,
    TooManyTargetDescriptors,
    UnsupportedTargetDescriptorTypeCode,
    TooManySegmentDescriptors,
    UnsupportedSegmentDescriptorTypeCode,
    UnexpectedInexactSegment,
    InlineDataLengthExceeded,
    InvalidOperationForCopySourceOrDestination,
    CopySegmentGranularityViolation,
    InvalidParameterWhilePortIsEnabled,
    InvalidDataOutBufferIntegrityCheckValue,
    DataDecryptionKeyFailLimitReached,
    IncompleteKeyAssociatedDataSet,
    VendorSpecificKeyReferenceNotFound,
    ApplicationTagModePageIsInvalid,
    TapeStreamMirroringPrevented,
    CopySourceOrCopyDestinationNotAuthorized,
    FastCopyNotPossible,
    WriteProtected,
    HardwareWriteProtected,
    LogicalUnitSoftwareWriteProtected,
    AssociatedWriteProtect,
    PersistentWriteProtect,
    PermanentWriteProtect,
    ConditionalWriteProtect,
    SpaceAllocationFailedWriteProtect,
    ZoneIsReadOnly,
    NotReadyToReadyChangeMediumMayHaveChanged,
    ImportOrExportElementAccessed,
    FormatLayerMayHaveChanged,
    ImportOrExportElementAccessedMediumChanged,
    PowerOnResetOrBusDeviceResetOccurred,
    PowerOnOccurred,
    ScsiBusResetOccurred,
    BusDeviceResetFunctionOccurred,
    DeviceInternalReset,
    TransceiverModeChangedToSingleEnded,
    TransceiverModeChangedToLvd,
    ITNexusLossOccurred,
    ParametersChanged,
    ModeParametersChanged,
    LogParametersChanged,
    ReservationsPreempted,
    ReservationsReleased,
    RegistrationsPreempted,
    AsymmetricAccessStateChanged,
    ImplicitAsymmetricAccessStateTransitionFailed,
    PriorityChanged,
    CapacityDataHasChanged,
    ErrorHistoryITNexusCleared,
    ErrorHistorySnapshotReleased,
    ErrorRecoveryAttributesHaveChanged,
    DataEncryptionCapabilitiesChanged,
    TimestampChanged,
    DataEncryptionParametersChangedByAnotherITNexus,
    DataEncryptionParametersChangedByVendorSpecificEvent,
    DataEncryptionKeyInstanceCounterHasChanged,
    SaCreationCapabilitiesDataHasChanged,
    MediumRemovalPreventionPreempted,
    ZoneResetWritePointerRecommended,
    CopyCannotExecuteSinceHostCannotDisconnect,
    CommandSequenceError,
    TooManyWindowsSpecified,
    InvalidCombinationOfWindowsSpecified,
    CurrentProgramAreaIsNotEmpty,
    CurrentProgramAreaIsEmpty,
    IllegalPowerConditionRequest,
    PersistentPreventConflict,
    PreviousBusyStatus,
    PreviousTaskSetFullStatus,
    PreviousReservationConflictStatus,
    PartitionOrCollectionContainsUserObjects,
    NotReserved,
    OrwriteGenerationDoesNotMatch,
    ResetWritePointerNotAllowed,
    ZoneIsOffline,
    StreamNotOpen,
    UnwrittenDataInZone,
    DescriptorFormatSenseDataRequired,
    ZoneIsInactive,
    WellKnownLogicalUnitAccessRequired,
    OverwriteErrorOnUpdateInPlace,
    InsufficientTimeForOperation,
    CommandTimeoutBeforeProcessing,
    CommandTimeoutDuringProcessing,
    CommandTimeoutDuringProcessingDueToErrorRecovery,
    CommandsClearedByAnotherInitiator,
    CommandsClearedByPowerLossNotification,
    CommandsClearedByDeviceServer,
    SomeCommandsClearedByQueuingLayerEvent,
    IncompatibleMediumInstalled,
    CannotReadMediumUnknownFormat,
    CannotReadMediumIncompatibleFormat,
    CleaningCartridgeInstalled,
    CannotWriteMediumUnknownFormat,
    CannotWriteMediumIncompatibleFormat,
    CannotFormatMediumIncompatibleMedium,
    CleaningFailure,
    CannotWriteApplicationCodeMismatch,
    CurrentSessionNotFixatedForAppend,
    CleaningRequestRejected,
    WormMediumOverwriteAttempted,
    WormMediumIntegrityCheck,
    MediumNotFormatted,
    IncompatibleVolumeType,
    IncompatibleVolumeQualifier,
    CleaningVolumeExpired,
    MediumFormatCorrupted,
    FormatCommandFailed,
    ZonedFormattingFailedDueToSpareLinking,
    SanitizeCommandFailed,
    DepopulationFailed,
    DepopulationRestorationFailed,
    NoDefectSpareLocationAvailable,
    DefectListUpdateFailure,
    TapeLengthError,
    EnclosureFailure,
    EnclosureServicesFailure,
    UnsupportedEnclosureFunction,
    EnclosureServicesUnavailable,
    EnclosureServicesTransferFailure,
    EnclosureServicesTransferRefused,
    EnclosureServicesChecksumError,
    RibbonInkOrTonerFailure,
    RoundedParameter,
    EventStatusNotification,
    EsnPowerManagementClassEvent,
    EsnMediaClassEvent,
    EsnDeviceBusyClassEvent,
    ThinProvisioningSoftThresholdReached,
    DepopulationInterrupted,
    DepopulationRestorationInterrupted,
    SavingParametersNotSupported,
    MediumNotPresent,
    MediumNotPresentTrayClosed,
    MediumNotPresentTrayOpen,
    MediumNotPresentLoadable,
    MediumNotPresentMediumAuxiliaryMemoryAccessible,
    SequentialPositioningError,
    TapePositionErrorAtBeginningOfMedium,
    TapePositionErrorAtEndOfMedium,
    TapeOrElectronicVerticalFormsUnitNotReady,
    SlewFailure,
    PaperJam,
    FailedToSenseTopOfForm,
    FailedToSenseBottomOfForm,
    RepositionError,
    ReadPastEndOfMedium,
    ReadPastBeginningOfMedium,
    PositionPastEndOfMedium,
    PositionPastBeginningOfMedium,
    MediumDestinationElementFull,
    MediumSourceElementEmpty,
    EndOfMediumReached,
    MediumMagazineNotAccessible,
    MediumMagazineRemoved,
    MediumMagazineInserted,
    MediumMagazineLocked,
    MediumMagazineUnlocked,
    MechanicalPositioningOrChangerError,
    ReadPastEndOfUserObject,
    ElementDisabled,
    ElementEnabled,
    DataTransferDeviceRemoved,
    DataTransferDeviceInserted,
    TooManyLogicalObjectsOnPartitionToSupportOperation,
    ElementStaticInformationChanged,
    InvalidBitsInIdentifyMessage,
    LogicalUnitHasNotSelfConfiguredYet,
    LogicalUnitFailure,
    TimeoutOnLogicalUnit,
    LogicalUnitFailedSelfTest,
    LogicalUnitUnableToUpdateSelfTestLog,
    TargetOperatingConditionsHaveChanged,
    MicrocodeHasBeenChanged,
    ChangedOperatingDefinition,
    InquiryDataHasChanged,
    ComponentDeviceAttached,
    DeviceIdentifierChanged,
    RedundancyGroupCreatedOrModified,
    RedundancyGroupDeleted,
    SpareCreatedOrModified,
    SpareDeleted,
    VolumeSetCreatedOrModified,
    VolumeSetDeleted,
    VolumeSetDeassigned,
    VolumeSetReassigned,
    ReportedLunsDataHasChanged,
    EchoBufferOverwritten,
    MediumLoadable,
    MediumAuxiliaryMemoryAccessible,
    IscsiIpAddressAdded,
    IscsiIpAddressRemoved,
    IscsiIpAddressChanged,
    InspectReferralsSenseDescriptors,
    MicrocodeHasBeenChangedWithoutReset,
    ZoneTransitionToFull,
    BindCompleted,
    BindRedirected,
    SubsidiaryBindingChanged,
    RamFailure,
    DataPathFailure,
    PowerOnOrSelfTestFailure,
    MessageError,
    InternalTargetFailure,
    PersistentReservationInformationLost,
    AtaDeviceFailedSetFeatures,
    SelectOrReselectFailure,
    UnsuccessfulSoftReset,
    ScsiParityError,
    DataPhaseCrcErrorDetected,
    ScsiParityErrorDetectedDuringStDataPhase,
    InformationUnitIucrcErrorDetected,
    AsynchronousInformationProtectionErrorDetected,
    ProtocolServiceCrcError,
    PhyTestFunctionInProgress,
    SomeCommandsClearedByIscsiProtocolEvent,
    InitiatorDetectedErrorMessageReceived,
    InvalidMessageError,
    CommandPhaseError,
    DataPhaseError,
    InvalidTargetPortTransferTagReceived,
    TooMuchWriteData,
    AckNakTimeout,
    NakReceived,
    DataOffsetError,
    InitiatorResponseTimeout,
    ConnectionLost,
    DataInBufferOverflowDataBufferSize,
    DataInBufferOverflowDataBufferDescriptorArea,
    DataInBufferError,
    DataOutBufferOverflowDataBufferSize,
    DataOutBufferOverflowDataBufferDescriptorArea,
    DataOutBufferError,
    PcieFabricError,
    PcieCompletionTimeout,
    PcieCompleterAbort,
    PciePoisonedTlpReceived,
    PcieEcrcCheckFailed,
    PcieUnsupportedRequest,
    PcieAcsViolation,
    PcieTlpPrefixBlocked,
    LogicalUnitFailedSelfConfiguration,
    OverlappedCommandsAttempted,
    WriteAppendError,
    WriteAppendPositionError,
    PositionErrorRelatedToTiming,
    EraseFailure,
    EraseFailureIncompleteEraseOperationDetected,
    CartridgeFault,
    MediaLoadOrEjectFailed,
    UnloadTapeFailure,
    MediumRemovalPrevented,
    MediumRemovalPreventedByDataTransferElement,
    MediumThreadOrUnthreadFailure,
    VolumeIdentifierInvalid,
    VolumeIdentifierMissing,
    DuplicateVolumeIdentifier,
    ElementStatusUnknown,
    DataTransferDeviceErrorLoadFailed,
    DataTransferDeviceErrorUnloadFailed,
    DataTransferDeviceErrorUnloadMissing,
    DataTransferDeviceErrorEjectFailed,
    DataTransferDeviceErrorLibraryCommunicationFailed,
    ScsiToHostSystemInterfaceFailure,
    SystemResourceFailure,
    SystemBufferFull,
    InsufficientReservationResources,
    InsufficientResources,
    InsufficientRegistrationResources,
    InsufficientAccessControlResources,
    AuxiliaryMemoryOutOfSpace,
    QuotaError,
    MaximumNumberOfSupplementalDecryptionKeysExceeded,
    MediumAuxiliaryMemoryNotAccessible,
    DataCurrentlyUnavailable,
    InsufficientPowerForOperation,
    InsufficientResourcesToCreateRod,
    InsufficientResourcesToCreateRodToken,
    InsufficientZoneResources,
    InsufficientZoneResourcesToCompleteWrite,
    MaximumNumberOfStreamsOpen,
    InsufficientResourcesToBind,
    UnableToRecoverTableOfContents,
    GenerationDoesNotExist,
    UpdatedBlockRead,
    OperatorRequestOrStateChangeInput,
    OperatorMediumRemovalRequest,
    OperatorSelectedWriteProtect,
    OperatorSelectedWritePermit,
    LogException,
    ThresholdConditionMet,
    LogCounterAtMaximum,
    LogListCodesExhausted,
    RplStatusChange,
    SpindlesSynchronized,
    SpindlesNotSynchronized,
    FailurePredictionThresholdExceeded,
    MediaFailurePredictionThresholdExceeded,
    LogicalUnitFailurePredictionThresholdExceeded,
    SpareAreaExhaustionPredictionThresholdExceeded,
    HardwareImpendingFailureGeneralHardDriveFailure,
    HardwareImpendingFailureDriveErrorRateTooHigh,
    HardwareImpendingFailureDataErrorRateTooHigh,
    HardwareImpendingFailureSeekErrorRateTooHigh,
    HardwareImpendingFailureTooManyBlockReassigns,
    HardwareImpendingFailureAccessTimesTooHigh,
    HardwareImpendingFailureStartUnitTimesTooHigh,
    HardwareImpendingFailureChannelParametrics,
    HardwareImpendingFailureControllerDetected,
    HardwareImpendingFailureThroughputPerformance,
    HardwareImpendingFailureSeekTimePerformance,
    HardwareImpendingFailureSpinUpRetryCount,
    HardwareImpendingFailureDriveCalibrationRetryCount,
    HardwareImpendingFailurePowerLossProtectionCircuit,
    ControllerImpendingFailureGeneralHardDriveFailure,
    ControllerImpendingFailureDriveErrorRateTooHigh,
    ControllerImpendingFailureDataErrorRateTooHigh,
    ControllerImpendingFailureSeekErrorRateTooHigh,
    ControllerImpendingFailureTooManyBlockReassigns,
    ControllerImpendingFailureAccessTimesTooHigh,
    ControllerImpendingFailureStartUnitTimesTooHigh,
    ControllerImpendingFailureChannelParametrics,
    ControllerImpendingFailureControllerDetected,
    ControllerImpendingFailureThroughputPerformance,
    ControllerImpendingFailureSeekTimePerformance,
    ControllerImpendingFailureSpinUpRetryCount,
    ControllerImpendingFailureDriveCalibrationRetryCount,
    DataChannelImpendingFailureGeneralHardDriveFailure,
    DataChannelImpendingFailureDriveErrorRateTooHigh,
    DataChannelImpendingFailureDataErrorRateTooHigh,
    DataChannelImpendingFailureSeekErrorRateTooHigh,
    DataChannelImpendingFailureTooManyBlockReassigns,
    DataChannelImpendingFailureAccessTimesTooHigh,
    DataChannelImpendingFailureStartUnitTimesTooHigh,
    DataChannelImpendingFailureChannelParametrics,
    DataChannelImpendingFailureControllerDetected,
    DataChannelImpendingFailureThroughputPerformance,
    DataChannelImpendingFailureSeekTimePerformance,
    DataChannelImpendingFailureSpinUpRetryCount,
    DataChannelImpendingFailureDriveCalibrationRetryCount,
    ServoImpendingFailureGeneralHardDriveFailure,
    ServoImpendingFailureDriveErrorRateTooHigh,
    ServoImpendingFailureDataErrorRateTooHigh,
    ServoImpendingFailureSeekErrorRateTooHigh,
    ServoImpendingFailureTooManyBlockReassigns,
    ServoImpendingFailureAccessTimesTooHigh,
    ServoImpendingFailureStartUnitTimesTooHigh,
    ServoImpendingFailureChannelParametrics,
    ServoImpendingFailureControllerDetected,
    ServoImpendingFailureThroughputPerformance,
    ServoImpendingFailureSeekTimePerformance,
    ServoImpendingFailureSpinUpRetryCount,
    ServoImpendingFailureDriveCalibrationRetryCount,
    SpindleImpendingFailureGeneralHardDriveFailure,
    SpindleImpendingFailureDriveErrorRateTooHigh,
    SpindleImpendingFailureDataErrorRateTooHigh,
    SpindleImpendingFailureSeekErrorRateTooHigh,
    SpindleImpendingFailureTooManyBlockReassigns,
    SpindleImpendingFailureAccessTimesTooHigh,
    SpindleImpendingFailureStartUnitTimesTooHigh,
    SpindleImpendingFailureChannelParametrics,
    SpindleImpendingFailureControllerDetected,
    SpindleImpendingFailureThroughputPerformance,
    SpindleImpendingFailureSeekTimePerformance,
    SpindleImpendingFailureSpinUpRetryCount,
    SpindleImpendingFailureDriveCalibrationRetryCount,
    FirmwareImpendingFailureGeneralHardDriveFailure,
    FirmwareImpendingFailureDriveErrorRateTooHigh,
    FirmwareImpendingFailureDataErrorRateTooHigh,
    FirmwareImpendingFailureSeekErrorRateTooHigh,
    FirmwareImpendingFailureTooManyBlockReassigns,
    FirmwareImpendingFailureAccessTimesTooHigh,
    FirmwareImpendingFailureStartUnitTimesTooHigh,
    FirmwareImpendingFailureChannelParametrics,
    FirmwareImpendingFailureControllerDetected,
    FirmwareImpendingFailureThroughputPerformance,
    FirmwareImpendingFailureSeekTimePerformance,
    FirmwareImpendingFailureSpinUpRetryCount,
    FirmwareImpendingFailureDriveCalibrationRetryCount,
    MediaImpendingFailureEnduranceLimitMet,
    FailurePredictionThresholdExceededFalse,
    LowPowerConditionOn,
    IdleConditionActivatedByTimer,
    StandbyConditionActivatedByTimer,
    IdleConditionActivatedByCommand,
    StandbyConditionActivatedByCommand,
    IdleBConditionActivatedByTimer,
    IdleBConditionActivatedByCommand,
    IdleCConditionActivatedByTimer,
    IdleCConditionActivatedByCommand,
    StandbyYConditionActivatedByTimer,
    StandbyYConditionActivatedByCommand,
    PowerStateChangeToActive,
    PowerStateChangeToIdle,
    PowerStateChangeToStandby,
    PowerStateChangeToSleep,
    PowerStateChangeToDeviceControl,
    LampFailure,
    VideoAcquisitionError,
    UnableToAcquireVideo,
    OutOfFocus,
    ScanHeadPositioningError,
    EndOfUserAreaEncounteredOnThisTrack,
    PacketDoesNotFitInAvailableSpace,
    IllegalModeForThisTrack,
    InvalidPacketSize,
    VoltageFault,
    AutomaticDocumentFeederCoverUp,
    AutomaticDocumentFeederLiftUp,
    DocumentJamInAutomaticDocumentFeeder,
    DocumentMissFeedAutomaticInDocumentFeeder,
    ConfigurationFailure,
    ConfigurationOfIncapableLogicalUnitsFailed,
    AddLogicalUnitFailed,
    ModificationOfLogicalUnitFailed,
    ExchangeOfLogicalUnitFailed,
    RemoveOfLogicalUnitFailed,
    AttachmentOfLogicalUnitFailed,
    CreationOfLogicalUnitFailed,
    AssignFailureOccurred,
    MultiplyAssignedLogicalUnit,
    SetTargetPortGroupsCommandFailed,
    AtaDeviceFeatureNotEnabled,
    CommandRejected,
    ExplicitBindNotAllowed,
    FeatureNotEnabled,
    LogicalUnitNotConfigured,
    SubsidiaryLogicalUnitNotConfigured,
    DataLossOnLogicalUnit,
    MultipleLogicalUnitFailures,
    ParityOrDataMismatch,
    InformationalReferToLog,
    StateChangeHasOccurred,
    RedundancyLevelGotBetter,
    RedundancyLevelGotWorse,
    RebuildFailureOccurred,
    RecalculateFailureOccurred,
    CommandToLogicalUnitFailed,
    CopyProtectionKeyExchangeFailureAuthenticationFailure,
    CopyProtectionKeyExchangeFailureKeyNotPresent,
    CopyProtectionKeyExchangeFailureKeyNotEstablished,
    ReadOfScrambledSectorWithoutAuthentication,
    MediaRegionCodeIsMismatchedToLogicalUnitRegion,
    DriveRegionMustBePermanentOrRegionResetCountError,
    InsufficientBlockCountForBindingNonceRecording,
    ConflictInBindingNonceRecording,
    InsufficientPermission,
    InvalidDriveHostPairingServer,
    DriveHostPairingSuspended,
    DecompressionExceptionLongAlgorithmId,
    SessionFixationError,
    SessionFixationErrorWritingLeadIn,
    SessionFixationErrorWritingLeadOut,
    SessionFixationErrorIncompleteTrackInSession,
    EmptyOrPartiallyWrittenReservedTrack,
    NoMoreTrackReservationsAllowed,
    RmzExtensionIsNotAllowed,
    NoMoreTestZoneExtensionsAreAllowed,
    CdControlError,
    PowerCalibrationAreaAlmostFull,
    PowerCalibrationAreaIsFull,
    PowerCalibrationAreaError,
    ProgramMemoryAreaUpdateFailure,
    ProgramMemoryAreaIsFull,
    RmaOrPmaIsAlmostFull,
    CurrentPowerCalibrationAreaAlmostFull,
    CurrentPowerCalibrationAreaIsFull,
    RdzIsFull,
    SecurityError,
    UnableToDecryptData,
    UnencryptedDataEncounteredWhileDecrypting,
    IncorrectDataEncryptionKey,
    CryptographicIntegrityValidationFailed,
    ErrorDecryptingData,
    UnknownSignatureVerificationKey,
    EncryptionParametersNotUseable,
    DigitalSignatureValidationFailure,
    EncryptionModeMismatchOnRead,
    EncryptedBlockNotRawReadEnabled,
    IncorrectEncryptionParameters,
    UnableToDecryptParameterList,
    EncryptionAlgorithmDisabled,
    SaCreationParameterValueInvalid,
    SaCreationParameterValueRejected,
    InvalidSaUsage,
    DataEncryptionConfigurationPrevented,
    SaCreationParameterNotSupported,
    AuthenticationFailed,
    ExternalDataEncryptionKeyManagerAccessError,
    ExternalDataEncryptionKeyManagerError,
    ExternalDataEncryptionKeyNotFound,
    ExternalDataEncryptionRequestNotAuthorized,
    ExternalDataEncryptionControlTimeout,
    ExternalDataEncryptionControlError,
    LogicalUnitAccessNotAuthorized,
    SecurityConflictInTranslatedDevice,
    DiagnosticFailureOnComponent(u8),
    TaggedOverlappedCommands(u8),
    DecompressionExceptionShortAlgorithmId(u8),
    Obsolete(u8, u8),
    UnassignedDeviceTypeCode(u8, u8),
    VendorSpecific(u8, u8),
    VendorSpecificQualification(u8, u8),
    Reserved(u8, u8),
}

impl AdditionalSenseCode {
    /// Decodes the ASC and ASCQ sense bytes into a variant; unrecognized pairs
    /// fall through to `Reserved(asc, ascq)`.
    #[allow(clippy::too_many_lines)]
    pub fn parse(asc: u8, ascq: u8) -> Self {
        match (asc, ascq) {
            (0x00, 0x00) => Self::NoAdditionalSenseInformation,
            (0x00, 0x01) => Self::FilemarkDetected,
            (0x00, 0x02) => Self::EndOfPartitionOrMediumDetected,
            (0x00, 0x03) => Self::SetmarkDetected,
            (0x00, 0x04) => Self::BeginningOfPartitionOrMediumDetected,
            (0x00, 0x05) => Self::EndOfDataDetected,
            (0x00, 0x06) => Self::IoProcessTerminated,
            (0x00, 0x07) => Self::ProgrammableEarlyWarningDetected,
            (0x00, 0x11) => Self::AudioPlayOperationInProgress,
            (0x00, 0x12) => Self::AudioPlayOperationPaused,
            (0x00, 0x13) => Self::AudioPlayOperationSuccessfullyCompleted,
            (0x00, 0x14) => Self::AudioPlayOperationStoppedDueToError,
            (0x00, 0x15) => Self::NoCurrentAudioStatusToReturn,
            (0x00, 0x16) => Self::OperationInProgress,
            (0x00, 0x17) => Self::CleaningRequested,
            (0x00, 0x18) => Self::EraseOperationInProgress,
            (0x00, 0x19) => Self::LocateOperationInProgress,
            (0x00, 0x1A) => Self::RewindOperationInProgress,
            (0x00, 0x1B) => Self::SetCapacityOperationInProgress,
            (0x00, 0x1C) => Self::VerifyOperationInProgress,
            (0x00, 0x1D) => Self::AtaPassThroughInformationAvailable,
            (0x00, 0x1E) => Self::ConflictingSaCreationRequest,
            (0x00, 0x1F) => Self::LogicalUnitTransitioningToAnotherPowerCondition,
            (0x00, 0x20) => Self::ExtendedCopyInformationAvailable,
            (0x00, 0x21) => Self::AtomicCommandAbortedDueToAca,
            (0x00, 0x22) => Self::DeferredMicrocodeIsPending,
            (0x00, 0x23) => Self::OverlappingAtomicCommandInProgress,
            (0x01, 0x00) => Self::NoIndexOrSectorSignal,
            (0x02, 0x00) => Self::NoSeekComplete,
            (0x03, 0x00) => Self::PeripheralDeviceWriteFault,
            (0x03, 0x01) => Self::NoWriteCurrent,
            (0x03, 0x02) => Self::ExcessiveWriteErrors,
            (0x04, 0x00) => Self::LogicalUnitNotReadyCauseNotReportable,
            (0x04, 0x01) => Self::LogicalUnitIsInProcessOfBecomingReady,
            (0x04, 0x02) => Self::LogicalUnitNotReadyInitializingCommandRequired,
            (0x04, 0x03) => Self::LogicalUnitNotReadyManualInterventionRequired,
            (0x04, 0x04) => Self::LogicalUnitNotReadyFormatInProgress,
            (0x04, 0x05) => Self::LogicalUnitNotReadyRebuildInProgress,
            (0x04, 0x06) => Self::LogicalUnitNotReadyRecalculationInProgress,
            (0x04, 0x07) => Self::LogicalUnitNotReadyOperationInProgress,
            (0x04, 0x08) => Self::LogicalUnitNotReadyLongWriteInProgress,
            (0x04, 0x09) => Self::LogicalUnitNotReadySelfTestInProgress,
            (0x04, 0x0A) => Self::LogicalUnitNotAccessibleAsymmetricAccessStateTransition,
            (0x04, 0x0B) => Self::LogicalUnitNotAccessibleTargetPortInStandbyState,
            (0x04, 0x0C) => Self::LogicalUnitNotAccessibleTargetPortInUnavailableState,
            (0x04, 0x0D) => Self::LogicalUnitNotReadyStructureCheckRequired,
            (0x04, 0x0E) => Self::LogicalUnitNotReadySecuritySessionInProgress,
            (0x04, 0x10) => Self::LogicalUnitNotReadyAuxiliaryMemoryNotAccessible,
            (0x04, 0x11) => Self::LogicalUnitNotReadyNotifyEnableSpinupRequired,
            (0x04, 0x12) => Self::LogicalUnitNotReadyOffline,
            (0x04, 0x13) => Self::LogicalUnitNotReadySaCreationInProgress,
            (0x04, 0x14) => Self::LogicalUnitNotReadySpaceAllocationInProgress,
            (0x04, 0x15) => Self::LogicalUnitNotReadyRoboticsDisabled,
            (0x04, 0x16) => Self::LogicalUnitNotReadyConfigurationRequired,
            (0x04, 0x17) => Self::LogicalUnitNotReadyCalibrationRequired,
            (0x04, 0x18) => Self::LogicalUnitNotReadyADoorIsOpen,
            (0x04, 0x19) => Self::LogicalUnitNotReadyOperatingInSequentialMode,
            (0x04, 0x1A) => Self::LogicalUnitNotReadyStartStopUnitCommandInProgress,
            (0x04, 0x1B) => Self::LogicalUnitNotReadySanitizeInProgress,
            (0x04, 0x1C) => Self::LogicalUnitNotReadyAdditionalPowerUseNotYetGranted,
            (0x04, 0x1D) => Self::LogicalUnitNotReadyConfigurationInProgress,
            (0x04, 0x1E) => Self::LogicalUnitNotReadyMicrocodeActivationRequired,
            (0x04, 0x1F) => Self::LogicalUnitNotReadyMicrocodeDownloadRequired,
            (0x04, 0x20) => Self::LogicalUnitNotReadyLogicalUnitResetRequired,
            (0x04, 0x21) => Self::LogicalUnitNotReadyHardResetRequired,
            (0x04, 0x22) => Self::LogicalUnitNotReadyPowerCycleRequired,
            (0x04, 0x23) => Self::LogicalUnitNotReadyAffiliationRequired,
            (0x04, 0x24) => Self::DepopulationInProgress,
            (0x04, 0x25) => Self::DepopulationRestorationInProgress,
            (0x05, 0x00) => Self::LogicalUnitDoesNotRespondToSelection,
            (0x06, 0x00) => Self::NoReferencePositionFound,
            (0x07, 0x00) => Self::MultiplePeripheralDevicesSelected,
            (0x08, 0x00) => Self::LogicalUnitCommunicationFailure,
            (0x08, 0x01) => Self::LogicalUnitCommunicationTimeOut,
            (0x08, 0x02) => Self::LogicalUnitCommunicationParityError,
            (0x08, 0x03) => Self::LogicalUnitCommunicationCrcErrorUltraDma32,
            (0x08, 0x04) => Self::UnreachableCopyTarget,
            (0x09, 0x00) => Self::TrackFollowingError,
            (0x09, 0x01) => Self::TrackingServoFailure,
            (0x09, 0x02) => Self::FocusServoFailure,
            (0x09, 0x03) => Self::SpindleServoFailure,
            (0x09, 0x04) => Self::HeadSelectFault,
            (0x09, 0x05) => Self::VibrationInducedTrackingError,
            (0x0A, 0x00) => Self::ErrorLogOverflow,
            (0x0B, 0x00) => Self::Warning,
            (0x0B, 0x01) => Self::WarningSpecifiedTemperatureExceeded,
            (0x0B, 0x02) => Self::WarningEnclosureDegraded,
            (0x0B, 0x03) => Self::WarningBackgroundSelfTestFailed,
            (0x0B, 0x04) => Self::WarningBackgroundPreScanDetectedMediumError,
            (0x0B, 0x05) => Self::WarningBackgroundMediumScanDetectedMediumError,
            (0x0B, 0x06) => Self::WarningNonVolatileCacheNowVolatile,
            (0x0B, 0x07) => Self::WarningDegradedPowerToNonVolatileCache,
            (0x0B, 0x08) => Self::WarningPowerLossExpected,
            (0x0B, 0x09) => Self::WarningDeviceStatisticsNotificationActive,
            (0x0B, 0x0A) => Self::WarningHighCriticalTemperatureLimitExceeded,
            (0x0B, 0x0B) => Self::WarningLowCriticalTemperatureLimitExceeded,
            (0x0B, 0x0C) => Self::WarningHighOperatingTemperatureLimitExceeded,
            (0x0B, 0x0D) => Self::WarningLowOperatingTemperatureLimitExceeded,
            (0x0B, 0x0E) => Self::WarningHighCriticalHumidityLimitExceeded,
            (0x0B, 0x0F) => Self::WarningLowCriticalHumidityLimitExceeded,
            (0x0B, 0x10) => Self::WarningHighOperatingHumidityLimitExceeded,
            (0x0B, 0x11) => Self::WarningLowOperatingHumidityLimitExceeded,
            (0x0B, 0x12) => Self::WarningMicrocodeSecurityAtRisk,
            (0x0B, 0x13) => Self::WarningMicrocodeDigitalSignatureValidationFailure,
            (0x0B, 0x14) => Self::WarningPhysicalElementStatusChange,
            (0x0C, 0x00) => Self::WriteError,
            (0x0C, 0x01) => Self::WriteErrorRecoveredWithAutoReallocation,
            (0x0C, 0x02) => Self::WriteErrorAutoReallocationFailed,
            (0x0C, 0x03) => Self::WriteErrorRecommendReassignment,
            (0x0C, 0x04) => Self::CompressionCheckMiscompareError,
            (0x0C, 0x05) => Self::DataExpansionOccurredDuringCompression,
            (0x0C, 0x06) => Self::BlockNotCompressible,
            (0x0C, 0x07) => Self::WriteErrorRecoveryNeeded,
            (0x0C, 0x08) => Self::WriteErrorRecoveryFailed,
            (0x0C, 0x09) => Self::WriteErrorLossOfStreaming,
            (0x0C, 0x0A) => Self::WriteErrorPaddingBlocksAdded,
            (0x0C, 0x0B) => Self::AuxiliaryMemoryWriteError,
            (0x0C, 0x0C) => Self::WriteErrorUnexpectedUnsolicitedData,
            (0x0C, 0x0D) => Self::WriteErrorNotEnoughUnsolicitedData,
            (0x0C, 0x0E) => Self::MultipleWriteErrors,
            (0x0C, 0x0F) => Self::DefectsInErrorWindow,
            (0x0C, 0x10) => Self::IncompleteMultipleAtomicWriteOperations,
            (0x0C, 0x11) => Self::WriteErrorRecoveryScanNeeded,
            (0x0C, 0x12) => Self::WriteErrorInsufficientZoneResources,
            (0x0D, 0x00) => Self::ErrorDetectedByThirdPartyTemporaryInitiator,
            (0x0D, 0x01) => Self::ThirdPartyDeviceFailure,
            (0x0D, 0x02) => Self::CopyTargetDeviceNotReachable,
            (0x0D, 0x03) => Self::IncorrectCopyTargetDeviceType,
            (0x0D, 0x04) => Self::CopyTargetDeviceDataUnderrun,
            (0x0D, 0x05) => Self::CopyTargetDeviceDataOverrun,
            (0x0E, 0x00) => Self::InvalidInformationUnit,
            (0x0E, 0x01) => Self::InformationUnitTooShort,
            (0x0E, 0x02) => Self::InformationUnitTooLong,
            (0x0E, 0x03) => Self::InvalidFieldInCommandInformationUnit,
            (0x10, 0x00) => Self::IdCrcOrEccError,
            (0x10, 0x01) => Self::LogicalBlockGuardCheckFailed,
            (0x10, 0x02) => Self::LogicalBlockApplicationTagCheckFailed,
            (0x10, 0x03) => Self::LogicalBlockReferenceTagCheckFailed,
            (0x10, 0x04) => Self::LogicalBlockProtectionErrorOnRecoverBufferedData,
            (0x10, 0x05) => Self::LogicalBlockProtectionMethodError,
            (0x11, 0x00) => Self::UnrecoveredReadError,
            (0x11, 0x01) => Self::ReadRetriesExhausted,
            (0x11, 0x02) => Self::ErrorTooLongToCorrect,
            (0x11, 0x03) => Self::MultipleReadErrors,
            (0x11, 0x04) => Self::UnrecoveredReadErrorAutoReallocateFailed,
            (0x11, 0x05) => Self::LEcUncorrectableError,
            (0x11, 0x06) => Self::CircUnrecoveredError,
            (0x11, 0x07) => Self::DataReSynchronizationError,
            (0x11, 0x08) => Self::IncompleteBlockRead,
            (0x11, 0x09) => Self::NoGapFound,
            (0x11, 0x0A) => Self::MiscorrectedError,
            (0x11, 0x0B) => Self::UnrecoveredReadErrorRecommendReassignment,
            (0x11, 0x0C) => Self::UnrecoveredReadErrorRecommendRewriteTheData,
            (0x11, 0x0D) => Self::DeCompressionCrcError,
            (0x11, 0x0E) => Self::CannotDecompressUsingDeclaredAlgorithm,
            (0x11, 0x0F) => Self::ErrorReadingUpcOrEanNumber,
            (0x11, 0x10) => Self::ErrorReadingIsrcNumber,
            (0x11, 0x11) => Self::ReadErrorLossOfStreaming,
            (0x11, 0x12) => Self::AuxiliaryMemoryReadError,
            (0x11, 0x13) => Self::ReadErrorFailedRetransmissionRequest,
            (0x11, 0x14) => Self::ReadErrorLbaMarkedBadByApplicationClient,
            (0x11, 0x15) => Self::WriteAfterSanitizeRequired,
            (0x12, 0x00) => Self::AddressMarkNotFoundForIdField,
            (0x13, 0x00) => Self::AddressMarkNotFoundForDataField,
            (0x14, 0x00) => Self::RecordedEntityNotFound,
            (0x14, 0x01) => Self::RecordNotFound,
            (0x14, 0x02) => Self::FilemarkOrSetmarkNotFound,
            (0x14, 0x03) => Self::EndOfDataNotFound,
            (0x14, 0x04) => Self::BlockSequenceError,
            (0x14, 0x05) => Self::RecordNotFoundRecommendReassignment,
            (0x14, 0x06) => Self::RecordNotFoundDataAutoReallocated,
            (0x14, 0x07) => Self::LocateOperationFailure,
            (0x15, 0x00) => Self::RandomPositioningError,
            (0x15, 0x01) => Self::MechanicalPositioningError,
            (0x15, 0x02) => Self::PositioningErrorDetectedByReadOfMedium,
            (0x16, 0x00) => Self::DataSynchronizationMarkError,
            (0x16, 0x01) => Self::DataSyncErrorDataRewritten,
            (0x16, 0x02) => Self::DataSyncErrorRecommendRewrite,
            (0x16, 0x03) => Self::DataSyncErrorDataAutoReallocated,
            (0x16, 0x04) => Self::DataSyncErrorRecommendReassignment,
            (0x17, 0x00) => Self::RecoveredDataWithNoErrorCorrectionApplied,
            (0x17, 0x01) => Self::RecoveredDataWithRetries,
            (0x17, 0x02) => Self::RecoveredDataWithPositiveHeadOffset,
            (0x17, 0x03) => Self::RecoveredDataWithNegativeHeadOffset,
            (0x17, 0x04) => Self::RecoveredDataWithRetriesAndOrCircApplied,
            (0x17, 0x05) => Self::RecoveredDataUsingPreviousSectorId,
            (0x17, 0x06) => Self::RecoveredDataWithoutEccDataAutoReallocated,
            (0x17, 0x07) => Self::RecoveredDataWithoutEccRecommendReassignment,
            (0x17, 0x08) => Self::RecoveredDataWithoutEccRecommendRewrite,
            (0x17, 0x09) => Self::RecoveredDataWithoutEccDataRewritten,
            (0x18, 0x00) => Self::RecoveredDataWithErrorCorrectionApplied,
            (0x18, 0x01) => Self::RecoveredDataWithErrorCorrAndRetriesApplied,
            (0x18, 0x02) => Self::RecoveredDataDataAutoReallocated,
            (0x18, 0x03) => Self::RecoveredDataWithCirc,
            (0x18, 0x04) => Self::RecoveredDataWithLEc,
            (0x18, 0x05) => Self::RecoveredDataRecommendReassignment,
            (0x18, 0x06) => Self::RecoveredDataRecommendRewrite,
            (0x18, 0x07) => Self::RecoveredDataWithEccDataRewritten,
            (0x18, 0x08) => Self::RecoveredDataWithLinking,
            (0x19, 0x00) => Self::DefectListError,
            (0x19, 0x01) => Self::DefectListNotAvailable,
            (0x19, 0x02) => Self::DefectListErrorInPrimaryList,
            (0x19, 0x03) => Self::DefectListErrorInGrownList,
            (0x1A, 0x00) => Self::ParameterListLengthError,
            (0x1B, 0x00) => Self::SynchronousDataTransferError,
            (0x1C, 0x00) => Self::DefectListNotFound,
            (0x1C, 0x01) => Self::PrimaryDefectListNotFound,
            (0x1C, 0x02) => Self::GrownDefectListNotFound,
            (0x1D, 0x00) => Self::MiscompareDuringVerifyOperation,
            (0x1D, 0x01) => Self::MiscompareVerifyOfUnmappedLba,
            (0x1E, 0x00) => Self::RecoveredIdWithEccCorrection,
            (0x1F, 0x00) => Self::PartialDefectListTransfer,
            (0x20, 0x00) => Self::InvalidCommandOperationCode,
            (0x20, 0x01) => Self::AccessDeniedInitiatorPendingEnrolled,
            (0x20, 0x02) => Self::AccessDeniedNoAccessRights,
            (0x20, 0x03) => Self::AccessDeniedInvalidMgmtIdKey,
            (0x20, 0x04) => Self::IllegalCommandWhileInWriteCapableState,
            (0x20, 0x06) => Self::IllegalCommandWhileInExplicitAddressMode,
            (0x20, 0x07) => Self::IllegalCommandWhileInImplicitAddressMode,
            (0x20, 0x08) => Self::AccessDeniedEnrollmentConflict,
            (0x20, 0x09) => Self::AccessDeniedInvalidLuIdentifier,
            (0x20, 0x0A) => Self::AccessDeniedInvalidProxyToken,
            (0x20, 0x0B) => Self::AccessDeniedAclLunConflict,
            (0x20, 0x0C) => Self::IllegalCommandWhenNotInAppendOnlyMode,
            (0x20, 0x0D) => Self::NotAnAdministrativeLogicalUnit,
            (0x20, 0x0E) => Self::NotASubsidiaryLogicalUnit,
            (0x20, 0x0F) => Self::NotAConglomerateLogicalUnit,
            (0x21, 0x00) => Self::LogicalBlockAddressOutOfRange,
            (0x21, 0x01) => Self::InvalidElementAddress,
            (0x21, 0x02) => Self::InvalidAddressForWrite,
            (0x21, 0x03) => Self::InvalidWriteCrossingLayerJump,
            (0x21, 0x04) => Self::UnalignedWriteCommand,
            (0x21, 0x05) => Self::WriteBoundaryViolation,
            (0x21, 0x06) => Self::AttemptToReadInvalidData,
            (0x21, 0x07) => Self::ReadBoundaryViolation,
            (0x21, 0x08) => Self::MisalignedWriteCommand,
            (0x21, 0x09) => Self::AttemptToAccessGapZone,
            (0x22, 0x00) => Self::IllegalFunction,
            (0x23, 0x00) => Self::InvalidTokenOperationCauseNotReportable,
            (0x23, 0x01) => Self::InvalidTokenOperationUnsupportedTokenType,
            (0x23, 0x02) => Self::InvalidTokenOperationRemoteTokenUsageNotSupported,
            (0x23, 0x03) => Self::InvalidTokenOperationRemoteRodTokenCreationNotSupported,
            (0x23, 0x04) => Self::InvalidTokenOperationTokenUnknown,
            (0x23, 0x05) => Self::InvalidTokenOperationTokenCorrupt,
            (0x23, 0x06) => Self::InvalidTokenOperationTokenRevoked,
            (0x23, 0x07) => Self::InvalidTokenOperationTokenExpired,
            (0x23, 0x08) => Self::InvalidTokenOperationTokenCancelled,
            (0x23, 0x09) => Self::InvalidTokenOperationTokenDeleted,
            (0x23, 0x0A) => Self::InvalidTokenOperationInvalidTokenLength,
            (0x24, 0x00) => Self::InvalidFieldInCdb,
            (0x24, 0x01) => Self::CdbDecryptionError,
            (0x24, 0x04) => Self::SecurityAuditValueFrozen,
            (0x24, 0x05) => Self::SecurityWorkingKeyFrozen,
            (0x24, 0x06) => Self::NonceNotUnique,
            (0x24, 0x07) => Self::NonceTimestampOutOfRange,
            (0x24, 0x08) => Self::InvalidXcdb,
            (0x24, 0x09) => Self::InvalidFastFormat,
            (0x25, 0x00) => Self::LogicalUnitNotSupported,
            (0x26, 0x00) => Self::InvalidFieldInParameterList,
            (0x26, 0x01) => Self::ParameterNotSupported,
            (0x26, 0x02) => Self::ParameterValueInvalid,
            (0x26, 0x03) => Self::ThresholdParametersNotSupported,
            (0x26, 0x04) => Self::InvalidReleaseOfPersistentReservation,
            (0x26, 0x05) => Self::DataDecryptionError,
            (0x26, 0x06) => Self::TooManyTargetDescriptors,
            (0x26, 0x07) => Self::UnsupportedTargetDescriptorTypeCode,
            (0x26, 0x08) => Self::TooManySegmentDescriptors,
            (0x26, 0x09) => Self::UnsupportedSegmentDescriptorTypeCode,
            (0x26, 0x0A) => Self::UnexpectedInexactSegment,
            (0x26, 0x0B) => Self::InlineDataLengthExceeded,
            (0x26, 0x0C) => Self::InvalidOperationForCopySourceOrDestination,
            (0x26, 0x0D) => Self::CopySegmentGranularityViolation,
            (0x26, 0x0E) => Self::InvalidParameterWhilePortIsEnabled,
            (0x26, 0x0F) => Self::InvalidDataOutBufferIntegrityCheckValue,
            (0x26, 0x10) => Self::DataDecryptionKeyFailLimitReached,
            (0x26, 0x11) => Self::IncompleteKeyAssociatedDataSet,
            (0x26, 0x12) => Self::VendorSpecificKeyReferenceNotFound,
            (0x26, 0x13) => Self::ApplicationTagModePageIsInvalid,
            (0x26, 0x14) => Self::TapeStreamMirroringPrevented,
            (0x26, 0x15) => Self::CopySourceOrCopyDestinationNotAuthorized,
            (0x26, 0x16) => Self::FastCopyNotPossible,
            (0x27, 0x00) => Self::WriteProtected,
            (0x27, 0x01) => Self::HardwareWriteProtected,
            (0x27, 0x02) => Self::LogicalUnitSoftwareWriteProtected,
            (0x27, 0x03) => Self::AssociatedWriteProtect,
            (0x27, 0x04) => Self::PersistentWriteProtect,
            (0x27, 0x05) => Self::PermanentWriteProtect,
            (0x27, 0x06) => Self::ConditionalWriteProtect,
            (0x27, 0x07) => Self::SpaceAllocationFailedWriteProtect,
            (0x27, 0x08) => Self::ZoneIsReadOnly,
            (0x28, 0x00) => Self::NotReadyToReadyChangeMediumMayHaveChanged,
            (0x28, 0x01) => Self::ImportOrExportElementAccessed,
            (0x28, 0x02) => Self::FormatLayerMayHaveChanged,
            (0x28, 0x03) => Self::ImportOrExportElementAccessedMediumChanged,
            (0x29, 0x00) => Self::PowerOnResetOrBusDeviceResetOccurred,
            (0x29, 0x01) => Self::PowerOnOccurred,
            (0x29, 0x02) => Self::ScsiBusResetOccurred,
            (0x29, 0x03) => Self::BusDeviceResetFunctionOccurred,
            (0x29, 0x04) => Self::DeviceInternalReset,
            (0x29, 0x05) => Self::TransceiverModeChangedToSingleEnded,
            (0x29, 0x06) => Self::TransceiverModeChangedToLvd,
            (0x29, 0x07) => Self::ITNexusLossOccurred,
            (0x2A, 0x00) => Self::ParametersChanged,
            (0x2A, 0x01) => Self::ModeParametersChanged,
            (0x2A, 0x02) => Self::LogParametersChanged,
            (0x2A, 0x03) => Self::ReservationsPreempted,
            (0x2A, 0x04) => Self::ReservationsReleased,
            (0x2A, 0x05) => Self::RegistrationsPreempted,
            (0x2A, 0x06) => Self::AsymmetricAccessStateChanged,
            (0x2A, 0x07) => Self::ImplicitAsymmetricAccessStateTransitionFailed,
            (0x2A, 0x08) => Self::PriorityChanged,
            (0x2A, 0x09) => Self::CapacityDataHasChanged,
            (0x2A, 0x0A) => Self::ErrorHistoryITNexusCleared,
            (0x2A, 0x0B) => Self::ErrorHistorySnapshotReleased,
            (0x2A, 0x0C) => Self::ErrorRecoveryAttributesHaveChanged,
            (0x2A, 0x0D) => Self::DataEncryptionCapabilitiesChanged,
            (0x2A, 0x10) => Self::TimestampChanged,
            (0x2A, 0x11) => Self::DataEncryptionParametersChangedByAnotherITNexus,
            (0x2A, 0x12) => Self::DataEncryptionParametersChangedByVendorSpecificEvent,
            (0x2A, 0x13) => Self::DataEncryptionKeyInstanceCounterHasChanged,
            (0x2A, 0x14) => Self::SaCreationCapabilitiesDataHasChanged,
            (0x2A, 0x15) => Self::MediumRemovalPreventionPreempted,
            (0x2A, 0x16) => Self::ZoneResetWritePointerRecommended,
            (0x2B, 0x00) => Self::CopyCannotExecuteSinceHostCannotDisconnect,
            (0x2C, 0x00) => Self::CommandSequenceError,
            (0x2C, 0x01) => Self::TooManyWindowsSpecified,
            (0x2C, 0x02) => Self::InvalidCombinationOfWindowsSpecified,
            (0x2C, 0x03) => Self::CurrentProgramAreaIsNotEmpty,
            (0x2C, 0x04) => Self::CurrentProgramAreaIsEmpty,
            (0x2C, 0x05) => Self::IllegalPowerConditionRequest,
            (0x2C, 0x06) => Self::PersistentPreventConflict,
            (0x2C, 0x07) => Self::PreviousBusyStatus,
            (0x2C, 0x08) => Self::PreviousTaskSetFullStatus,
            (0x2C, 0x09) => Self::PreviousReservationConflictStatus,
            (0x2C, 0x0A) => Self::PartitionOrCollectionContainsUserObjects,
            (0x2C, 0x0B) => Self::NotReserved,
            (0x2C, 0x0C) => Self::OrwriteGenerationDoesNotMatch,
            (0x2C, 0x0D) => Self::ResetWritePointerNotAllowed,
            (0x2C, 0x0E) => Self::ZoneIsOffline,
            (0x2C, 0x0F) => Self::StreamNotOpen,
            (0x2C, 0x10) => Self::UnwrittenDataInZone,
            (0x2C, 0x11) => Self::DescriptorFormatSenseDataRequired,
            (0x2C, 0x12) => Self::ZoneIsInactive,
            (0x2C, 0x13) => Self::WellKnownLogicalUnitAccessRequired,
            (0x2D, 0x00) => Self::OverwriteErrorOnUpdateInPlace,
            (0x2E, 0x00) => Self::InsufficientTimeForOperation,
            (0x2E, 0x01) => Self::CommandTimeoutBeforeProcessing,
            (0x2E, 0x02) => Self::CommandTimeoutDuringProcessing,
            (0x2E, 0x03) => Self::CommandTimeoutDuringProcessingDueToErrorRecovery,
            (0x2F, 0x00) => Self::CommandsClearedByAnotherInitiator,
            (0x2F, 0x01) => Self::CommandsClearedByPowerLossNotification,
            (0x2F, 0x02) => Self::CommandsClearedByDeviceServer,
            (0x2F, 0x03) => Self::SomeCommandsClearedByQueuingLayerEvent,
            (0x30, 0x00) => Self::IncompatibleMediumInstalled,
            (0x30, 0x01) => Self::CannotReadMediumUnknownFormat,
            (0x30, 0x02) => Self::CannotReadMediumIncompatibleFormat,
            (0x30, 0x03) => Self::CleaningCartridgeInstalled,
            (0x30, 0x04) => Self::CannotWriteMediumUnknownFormat,
            (0x30, 0x05) => Self::CannotWriteMediumIncompatibleFormat,
            (0x30, 0x06) => Self::CannotFormatMediumIncompatibleMedium,
            (0x30, 0x07) => Self::CleaningFailure,
            (0x30, 0x08) => Self::CannotWriteApplicationCodeMismatch,
            (0x30, 0x09) => Self::CurrentSessionNotFixatedForAppend,
            (0x30, 0x0A) => Self::CleaningRequestRejected,
            (0x30, 0x0C) => Self::WormMediumOverwriteAttempted,
            (0x30, 0x0D) => Self::WormMediumIntegrityCheck,
            (0x30, 0x10) => Self::MediumNotFormatted,
            (0x30, 0x11) => Self::IncompatibleVolumeType,
            (0x30, 0x12) => Self::IncompatibleVolumeQualifier,
            (0x30, 0x13) => Self::CleaningVolumeExpired,
            (0x31, 0x00) => Self::MediumFormatCorrupted,
            (0x31, 0x01) => Self::FormatCommandFailed,
            (0x31, 0x02) => Self::ZonedFormattingFailedDueToSpareLinking,
            (0x31, 0x03) => Self::SanitizeCommandFailed,
            (0x31, 0x04) => Self::DepopulationFailed,
            (0x31, 0x05) => Self::DepopulationRestorationFailed,
            (0x32, 0x00) => Self::NoDefectSpareLocationAvailable,
            (0x32, 0x01) => Self::DefectListUpdateFailure,
            (0x33, 0x00) => Self::TapeLengthError,
            (0x34, 0x00) => Self::EnclosureFailure,
            (0x35, 0x00) => Self::EnclosureServicesFailure,
            (0x35, 0x01) => Self::UnsupportedEnclosureFunction,
            (0x35, 0x02) => Self::EnclosureServicesUnavailable,
            (0x35, 0x03) => Self::EnclosureServicesTransferFailure,
            (0x35, 0x04) => Self::EnclosureServicesTransferRefused,
            (0x35, 0x05) => Self::EnclosureServicesChecksumError,
            (0x36, 0x00) => Self::RibbonInkOrTonerFailure,
            (0x37, 0x00) => Self::RoundedParameter,
            (0x38, 0x00) => Self::EventStatusNotification,
            (0x38, 0x02) => Self::EsnPowerManagementClassEvent,
            (0x38, 0x04) => Self::EsnMediaClassEvent,
            (0x38, 0x06) => Self::EsnDeviceBusyClassEvent,
            (0x38, 0x07) => Self::ThinProvisioningSoftThresholdReached,
            (0x38, 0x08) => Self::DepopulationInterrupted,
            (0x38, 0x09) => Self::DepopulationRestorationInterrupted,
            (0x39, 0x00) => Self::SavingParametersNotSupported,
            (0x3A, 0x00) => Self::MediumNotPresent,
            (0x3A, 0x01) => Self::MediumNotPresentTrayClosed,
            (0x3A, 0x02) => Self::MediumNotPresentTrayOpen,
            (0x3A, 0x03) => Self::MediumNotPresentLoadable,
            (0x3A, 0x04) => Self::MediumNotPresentMediumAuxiliaryMemoryAccessible,
            (0x3B, 0x00) => Self::SequentialPositioningError,
            (0x3B, 0x01) => Self::TapePositionErrorAtBeginningOfMedium,
            (0x3B, 0x02) => Self::TapePositionErrorAtEndOfMedium,
            (0x3B, 0x03) => Self::TapeOrElectronicVerticalFormsUnitNotReady,
            (0x3B, 0x04) => Self::SlewFailure,
            (0x3B, 0x05) => Self::PaperJam,
            (0x3B, 0x06) => Self::FailedToSenseTopOfForm,
            (0x3B, 0x07) => Self::FailedToSenseBottomOfForm,
            (0x3B, 0x08) => Self::RepositionError,
            (0x3B, 0x09) => Self::ReadPastEndOfMedium,
            (0x3B, 0x0A) => Self::ReadPastBeginningOfMedium,
            (0x3B, 0x0B) => Self::PositionPastEndOfMedium,
            (0x3B, 0x0C) => Self::PositionPastBeginningOfMedium,
            (0x3B, 0x0D) => Self::MediumDestinationElementFull,
            (0x3B, 0x0E) => Self::MediumSourceElementEmpty,
            (0x3B, 0x0F) => Self::EndOfMediumReached,
            (0x3B, 0x11) => Self::MediumMagazineNotAccessible,
            (0x3B, 0x12) => Self::MediumMagazineRemoved,
            (0x3B, 0x13) => Self::MediumMagazineInserted,
            (0x3B, 0x14) => Self::MediumMagazineLocked,
            (0x3B, 0x15) => Self::MediumMagazineUnlocked,
            (0x3B, 0x16) => Self::MechanicalPositioningOrChangerError,
            (0x3B, 0x17) => Self::ReadPastEndOfUserObject,
            (0x3B, 0x18) => Self::ElementDisabled,
            (0x3B, 0x19) => Self::ElementEnabled,
            (0x3B, 0x1A) => Self::DataTransferDeviceRemoved,
            (0x3B, 0x1B) => Self::DataTransferDeviceInserted,
            (0x3B, 0x1C) => Self::TooManyLogicalObjectsOnPartitionToSupportOperation,
            (0x3B, 0x20) => Self::ElementStaticInformationChanged,
            (0x3D, 0x00) => Self::InvalidBitsInIdentifyMessage,
            (0x3E, 0x00) => Self::LogicalUnitHasNotSelfConfiguredYet,
            (0x3E, 0x01) => Self::LogicalUnitFailure,
            (0x3E, 0x02) => Self::TimeoutOnLogicalUnit,
            (0x3E, 0x03) => Self::LogicalUnitFailedSelfTest,
            (0x3E, 0x04) => Self::LogicalUnitUnableToUpdateSelfTestLog,
            (0x3F, 0x00) => Self::TargetOperatingConditionsHaveChanged,
            (0x3F, 0x01) => Self::MicrocodeHasBeenChanged,
            (0x3F, 0x02) => Self::ChangedOperatingDefinition,
            (0x3F, 0x03) => Self::InquiryDataHasChanged,
            (0x3F, 0x04) => Self::ComponentDeviceAttached,
            (0x3F, 0x05) => Self::DeviceIdentifierChanged,
            (0x3F, 0x06) => Self::RedundancyGroupCreatedOrModified,
            (0x3F, 0x07) => Self::RedundancyGroupDeleted,
            (0x3F, 0x08) => Self::SpareCreatedOrModified,
            (0x3F, 0x09) => Self::SpareDeleted,
            (0x3F, 0x0A) => Self::VolumeSetCreatedOrModified,
            (0x3F, 0x0B) => Self::VolumeSetDeleted,
            (0x3F, 0x0C) => Self::VolumeSetDeassigned,
            (0x3F, 0x0D) => Self::VolumeSetReassigned,
            (0x3F, 0x0E) => Self::ReportedLunsDataHasChanged,
            (0x3F, 0x0F) => Self::EchoBufferOverwritten,
            (0x3F, 0x10) => Self::MediumLoadable,
            (0x3F, 0x11) => Self::MediumAuxiliaryMemoryAccessible,
            (0x3F, 0x12) => Self::IscsiIpAddressAdded,
            (0x3F, 0x13) => Self::IscsiIpAddressRemoved,
            (0x3F, 0x14) => Self::IscsiIpAddressChanged,
            (0x3F, 0x15) => Self::InspectReferralsSenseDescriptors,
            (0x3F, 0x16) => Self::MicrocodeHasBeenChangedWithoutReset,
            (0x3F, 0x17) => Self::ZoneTransitionToFull,
            (0x3F, 0x18) => Self::BindCompleted,
            (0x3F, 0x19) => Self::BindRedirected,
            (0x3F, 0x1A) => Self::SubsidiaryBindingChanged,
            (0x40, 0x00) => Self::RamFailure,
            (0x41, 0x00) => Self::DataPathFailure,
            (0x42, 0x00) => Self::PowerOnOrSelfTestFailure,
            (0x43, 0x00) => Self::MessageError,
            (0x44, 0x00) => Self::InternalTargetFailure,
            (0x44, 0x01) => Self::PersistentReservationInformationLost,
            (0x44, 0x71) => Self::AtaDeviceFailedSetFeatures,
            (0x45, 0x00) => Self::SelectOrReselectFailure,
            (0x46, 0x00) => Self::UnsuccessfulSoftReset,
            (0x47, 0x00) => Self::ScsiParityError,
            (0x47, 0x01) => Self::DataPhaseCrcErrorDetected,
            (0x47, 0x02) => Self::ScsiParityErrorDetectedDuringStDataPhase,
            (0x47, 0x03) => Self::InformationUnitIucrcErrorDetected,
            (0x47, 0x04) => Self::AsynchronousInformationProtectionErrorDetected,
            (0x47, 0x05) => Self::ProtocolServiceCrcError,
            (0x47, 0x06) => Self::PhyTestFunctionInProgress,
            (0x47, 0x7F) => Self::SomeCommandsClearedByIscsiProtocolEvent,
            (0x48, 0x00) => Self::InitiatorDetectedErrorMessageReceived,
            (0x49, 0x00) => Self::InvalidMessageError,
            (0x4A, 0x00) => Self::CommandPhaseError,
            (0x4B, 0x00) => Self::DataPhaseError,
            (0x4B, 0x01) => Self::InvalidTargetPortTransferTagReceived,
            (0x4B, 0x02) => Self::TooMuchWriteData,
            (0x4B, 0x03) => Self::AckNakTimeout,
            (0x4B, 0x04) => Self::NakReceived,
            (0x4B, 0x05) => Self::DataOffsetError,
            (0x4B, 0x06) => Self::InitiatorResponseTimeout,
            (0x4B, 0x07) => Self::ConnectionLost,
            (0x4B, 0x08) => Self::DataInBufferOverflowDataBufferSize,
            (0x4B, 0x09) => Self::DataInBufferOverflowDataBufferDescriptorArea,
            (0x4B, 0x0A) => Self::DataInBufferError,
            (0x4B, 0x0B) => Self::DataOutBufferOverflowDataBufferSize,
            (0x4B, 0x0C) => Self::DataOutBufferOverflowDataBufferDescriptorArea,
            (0x4B, 0x0D) => Self::DataOutBufferError,
            (0x4B, 0x0E) => Self::PcieFabricError,
            (0x4B, 0x0F) => Self::PcieCompletionTimeout,
            (0x4B, 0x10) => Self::PcieCompleterAbort,
            (0x4B, 0x11) => Self::PciePoisonedTlpReceived,
            (0x4B, 0x12) => Self::PcieEcrcCheckFailed,
            (0x4B, 0x13) => Self::PcieUnsupportedRequest,
            (0x4B, 0x14) => Self::PcieAcsViolation,
            (0x4B, 0x15) => Self::PcieTlpPrefixBlocked,
            (0x4C, 0x00) => Self::LogicalUnitFailedSelfConfiguration,
            (0x4E, 0x00) => Self::OverlappedCommandsAttempted,
            (0x50, 0x00) => Self::WriteAppendError,
            (0x50, 0x01) => Self::WriteAppendPositionError,
            (0x50, 0x02) => Self::PositionErrorRelatedToTiming,
            (0x51, 0x00) => Self::EraseFailure,
            (0x51, 0x01) => Self::EraseFailureIncompleteEraseOperationDetected,
            (0x52, 0x00) => Self::CartridgeFault,
            (0x53, 0x00) => Self::MediaLoadOrEjectFailed,
            (0x53, 0x01) => Self::UnloadTapeFailure,
            (0x53, 0x02) => Self::MediumRemovalPrevented,
            (0x53, 0x03) => Self::MediumRemovalPreventedByDataTransferElement,
            (0x53, 0x04) => Self::MediumThreadOrUnthreadFailure,
            (0x53, 0x05) => Self::VolumeIdentifierInvalid,
            (0x53, 0x06) => Self::VolumeIdentifierMissing,
            (0x53, 0x07) => Self::DuplicateVolumeIdentifier,
            (0x53, 0x08) => Self::ElementStatusUnknown,
            (0x53, 0x09) => Self::DataTransferDeviceErrorLoadFailed,
            (0x53, 0x0A) => Self::DataTransferDeviceErrorUnloadFailed,
            (0x53, 0x0B) => Self::DataTransferDeviceErrorUnloadMissing,
            (0x53, 0x0C) => Self::DataTransferDeviceErrorEjectFailed,
            (0x53, 0x0D) => Self::DataTransferDeviceErrorLibraryCommunicationFailed,
            (0x54, 0x00) => Self::ScsiToHostSystemInterfaceFailure,
            (0x55, 0x00) => Self::SystemResourceFailure,
            (0x55, 0x01) => Self::SystemBufferFull,
            (0x55, 0x02) => Self::InsufficientReservationResources,
            (0x55, 0x03) => Self::InsufficientResources,
            (0x55, 0x04) => Self::InsufficientRegistrationResources,
            (0x55, 0x05) => Self::InsufficientAccessControlResources,
            (0x55, 0x06) => Self::AuxiliaryMemoryOutOfSpace,
            (0x55, 0x07) => Self::QuotaError,
            (0x55, 0x08) => Self::MaximumNumberOfSupplementalDecryptionKeysExceeded,
            (0x55, 0x09) => Self::MediumAuxiliaryMemoryNotAccessible,
            (0x55, 0x0A) => Self::DataCurrentlyUnavailable,
            (0x55, 0x0B) => Self::InsufficientPowerForOperation,
            (0x55, 0x0C) => Self::InsufficientResourcesToCreateRod,
            (0x55, 0x0D) => Self::InsufficientResourcesToCreateRodToken,
            (0x55, 0x0E) => Self::InsufficientZoneResources,
            (0x55, 0x0F) => Self::InsufficientZoneResourcesToCompleteWrite,
            (0x55, 0x10) => Self::MaximumNumberOfStreamsOpen,
            (0x55, 0x11) => Self::InsufficientResourcesToBind,
            (0x57, 0x00) => Self::UnableToRecoverTableOfContents,
            (0x58, 0x00) => Self::GenerationDoesNotExist,
            (0x59, 0x00) => Self::UpdatedBlockRead,
            (0x5A, 0x00) => Self::OperatorRequestOrStateChangeInput,
            (0x5A, 0x01) => Self::OperatorMediumRemovalRequest,
            (0x5A, 0x02) => Self::OperatorSelectedWriteProtect,
            (0x5A, 0x03) => Self::OperatorSelectedWritePermit,
            (0x5B, 0x00) => Self::LogException,
            (0x5B, 0x01) => Self::ThresholdConditionMet,
            (0x5B, 0x02) => Self::LogCounterAtMaximum,
            (0x5B, 0x03) => Self::LogListCodesExhausted,
            (0x5C, 0x00) => Self::RplStatusChange,
            (0x5C, 0x01) => Self::SpindlesSynchronized,
            (0x5C, 0x02) => Self::SpindlesNotSynchronized,
            (0x5D, 0x00) => Self::FailurePredictionThresholdExceeded,
            (0x5D, 0x01) => Self::MediaFailurePredictionThresholdExceeded,
            (0x5D, 0x02) => Self::LogicalUnitFailurePredictionThresholdExceeded,
            (0x5D, 0x03) => Self::SpareAreaExhaustionPredictionThresholdExceeded,
            (0x5D, 0x10) => Self::HardwareImpendingFailureGeneralHardDriveFailure,
            (0x5D, 0x11) => Self::HardwareImpendingFailureDriveErrorRateTooHigh,
            (0x5D, 0x12) => Self::HardwareImpendingFailureDataErrorRateTooHigh,
            (0x5D, 0x13) => Self::HardwareImpendingFailureSeekErrorRateTooHigh,
            (0x5D, 0x14) => Self::HardwareImpendingFailureTooManyBlockReassigns,
            (0x5D, 0x15) => Self::HardwareImpendingFailureAccessTimesTooHigh,
            (0x5D, 0x16) => Self::HardwareImpendingFailureStartUnitTimesTooHigh,
            (0x5D, 0x17) => Self::HardwareImpendingFailureChannelParametrics,
            (0x5D, 0x18) => Self::HardwareImpendingFailureControllerDetected,
            (0x5D, 0x19) => Self::HardwareImpendingFailureThroughputPerformance,
            (0x5D, 0x1A) => Self::HardwareImpendingFailureSeekTimePerformance,
            (0x5D, 0x1B) => Self::HardwareImpendingFailureSpinUpRetryCount,
            (0x5D, 0x1C) => Self::HardwareImpendingFailureDriveCalibrationRetryCount,
            (0x5D, 0x1D) => Self::HardwareImpendingFailurePowerLossProtectionCircuit,
            (0x5D, 0x20) => Self::ControllerImpendingFailureGeneralHardDriveFailure,
            (0x5D, 0x21) => Self::ControllerImpendingFailureDriveErrorRateTooHigh,
            (0x5D, 0x22) => Self::ControllerImpendingFailureDataErrorRateTooHigh,
            (0x5D, 0x23) => Self::ControllerImpendingFailureSeekErrorRateTooHigh,
            (0x5D, 0x24) => Self::ControllerImpendingFailureTooManyBlockReassigns,
            (0x5D, 0x25) => Self::ControllerImpendingFailureAccessTimesTooHigh,
            (0x5D, 0x26) => Self::ControllerImpendingFailureStartUnitTimesTooHigh,
            (0x5D, 0x27) => Self::ControllerImpendingFailureChannelParametrics,
            (0x5D, 0x28) => Self::ControllerImpendingFailureControllerDetected,
            (0x5D, 0x29) => Self::ControllerImpendingFailureThroughputPerformance,
            (0x5D, 0x2A) => Self::ControllerImpendingFailureSeekTimePerformance,
            (0x5D, 0x2B) => Self::ControllerImpendingFailureSpinUpRetryCount,
            (0x5D, 0x2C) => Self::ControllerImpendingFailureDriveCalibrationRetryCount,
            (0x5D, 0x30) => Self::DataChannelImpendingFailureGeneralHardDriveFailure,
            (0x5D, 0x31) => Self::DataChannelImpendingFailureDriveErrorRateTooHigh,
            (0x5D, 0x32) => Self::DataChannelImpendingFailureDataErrorRateTooHigh,
            (0x5D, 0x33) => Self::DataChannelImpendingFailureSeekErrorRateTooHigh,
            (0x5D, 0x34) => Self::DataChannelImpendingFailureTooManyBlockReassigns,
            (0x5D, 0x35) => Self::DataChannelImpendingFailureAccessTimesTooHigh,
            (0x5D, 0x36) => Self::DataChannelImpendingFailureStartUnitTimesTooHigh,
            (0x5D, 0x37) => Self::DataChannelImpendingFailureChannelParametrics,
            (0x5D, 0x38) => Self::DataChannelImpendingFailureControllerDetected,
            (0x5D, 0x39) => Self::DataChannelImpendingFailureThroughputPerformance,
            (0x5D, 0x3A) => Self::DataChannelImpendingFailureSeekTimePerformance,
            (0x5D, 0x3B) => Self::DataChannelImpendingFailureSpinUpRetryCount,
            (0x5D, 0x3C) => Self::DataChannelImpendingFailureDriveCalibrationRetryCount,
            (0x5D, 0x40) => Self::ServoImpendingFailureGeneralHardDriveFailure,
            (0x5D, 0x41) => Self::ServoImpendingFailureDriveErrorRateTooHigh,
            (0x5D, 0x42) => Self::ServoImpendingFailureDataErrorRateTooHigh,
            (0x5D, 0x43) => Self::ServoImpendingFailureSeekErrorRateTooHigh,
            (0x5D, 0x44) => Self::ServoImpendingFailureTooManyBlockReassigns,
            (0x5D, 0x45) => Self::ServoImpendingFailureAccessTimesTooHigh,
            (0x5D, 0x46) => Self::ServoImpendingFailureStartUnitTimesTooHigh,
            (0x5D, 0x47) => Self::ServoImpendingFailureChannelParametrics,
            (0x5D, 0x48) => Self::ServoImpendingFailureControllerDetected,
            (0x5D, 0x49) => Self::ServoImpendingFailureThroughputPerformance,
            (0x5D, 0x4A) => Self::ServoImpendingFailureSeekTimePerformance,
            (0x5D, 0x4B) => Self::ServoImpendingFailureSpinUpRetryCount,
            (0x5D, 0x4C) => Self::ServoImpendingFailureDriveCalibrationRetryCount,
            (0x5D, 0x50) => Self::SpindleImpendingFailureGeneralHardDriveFailure,
            (0x5D, 0x51) => Self::SpindleImpendingFailureDriveErrorRateTooHigh,
            (0x5D, 0x52) => Self::SpindleImpendingFailureDataErrorRateTooHigh,
            (0x5D, 0x53) => Self::SpindleImpendingFailureSeekErrorRateTooHigh,
            (0x5D, 0x54) => Self::SpindleImpendingFailureTooManyBlockReassigns,
            (0x5D, 0x55) => Self::SpindleImpendingFailureAccessTimesTooHigh,
            (0x5D, 0x56) => Self::SpindleImpendingFailureStartUnitTimesTooHigh,
            (0x5D, 0x57) => Self::SpindleImpendingFailureChannelParametrics,
            (0x5D, 0x58) => Self::SpindleImpendingFailureControllerDetected,
            (0x5D, 0x59) => Self::SpindleImpendingFailureThroughputPerformance,
            (0x5D, 0x5A) => Self::SpindleImpendingFailureSeekTimePerformance,
            (0x5D, 0x5B) => Self::SpindleImpendingFailureSpinUpRetryCount,
            (0x5D, 0x5C) => Self::SpindleImpendingFailureDriveCalibrationRetryCount,
            (0x5D, 0x60) => Self::FirmwareImpendingFailureGeneralHardDriveFailure,
            (0x5D, 0x61) => Self::FirmwareImpendingFailureDriveErrorRateTooHigh,
            (0x5D, 0x62) => Self::FirmwareImpendingFailureDataErrorRateTooHigh,
            (0x5D, 0x63) => Self::FirmwareImpendingFailureSeekErrorRateTooHigh,
            (0x5D, 0x64) => Self::FirmwareImpendingFailureTooManyBlockReassigns,
            (0x5D, 0x65) => Self::FirmwareImpendingFailureAccessTimesTooHigh,
            (0x5D, 0x66) => Self::FirmwareImpendingFailureStartUnitTimesTooHigh,
            (0x5D, 0x67) => Self::FirmwareImpendingFailureChannelParametrics,
            (0x5D, 0x68) => Self::FirmwareImpendingFailureControllerDetected,
            (0x5D, 0x69) => Self::FirmwareImpendingFailureThroughputPerformance,
            (0x5D, 0x6A) => Self::FirmwareImpendingFailureSeekTimePerformance,
            (0x5D, 0x6B) => Self::FirmwareImpendingFailureSpinUpRetryCount,
            (0x5D, 0x6C) => Self::FirmwareImpendingFailureDriveCalibrationRetryCount,
            (0x5D, 0x73) => Self::MediaImpendingFailureEnduranceLimitMet,
            (0x5D, 0xFF) => Self::FailurePredictionThresholdExceededFalse,
            (0x5E, 0x00) => Self::LowPowerConditionOn,
            (0x5E, 0x01) => Self::IdleConditionActivatedByTimer,
            (0x5E, 0x02) => Self::StandbyConditionActivatedByTimer,
            (0x5E, 0x03) => Self::IdleConditionActivatedByCommand,
            (0x5E, 0x04) => Self::StandbyConditionActivatedByCommand,
            (0x5E, 0x05) => Self::IdleBConditionActivatedByTimer,
            (0x5E, 0x06) => Self::IdleBConditionActivatedByCommand,
            (0x5E, 0x07) => Self::IdleCConditionActivatedByTimer,
            (0x5E, 0x08) => Self::IdleCConditionActivatedByCommand,
            (0x5E, 0x09) => Self::StandbyYConditionActivatedByTimer,
            (0x5E, 0x0A) => Self::StandbyYConditionActivatedByCommand,
            (0x5E, 0x41) => Self::PowerStateChangeToActive,
            (0x5E, 0x42) => Self::PowerStateChangeToIdle,
            (0x5E, 0x43) => Self::PowerStateChangeToStandby,
            (0x5E, 0x45) => Self::PowerStateChangeToSleep,
            (0x5E, 0x47) => Self::PowerStateChangeToDeviceControl,
            (0x60, 0x00) => Self::LampFailure,
            (0x61, 0x00) => Self::VideoAcquisitionError,
            (0x61, 0x01) => Self::UnableToAcquireVideo,
            (0x61, 0x02) => Self::OutOfFocus,
            (0x62, 0x00) => Self::ScanHeadPositioningError,
            (0x63, 0x00) => Self::EndOfUserAreaEncounteredOnThisTrack,
            (0x63, 0x01) => Self::PacketDoesNotFitInAvailableSpace,
            (0x64, 0x00) => Self::IllegalModeForThisTrack,
            (0x64, 0x01) => Self::InvalidPacketSize,
            (0x65, 0x00) => Self::VoltageFault,
            (0x66, 0x00) => Self::AutomaticDocumentFeederCoverUp,
            (0x66, 0x01) => Self::AutomaticDocumentFeederLiftUp,
            (0x66, 0x02) => Self::DocumentJamInAutomaticDocumentFeeder,
            (0x66, 0x03) => Self::DocumentMissFeedAutomaticInDocumentFeeder,
            (0x67, 0x00) => Self::ConfigurationFailure,
            (0x67, 0x01) => Self::ConfigurationOfIncapableLogicalUnitsFailed,
            (0x67, 0x02) => Self::AddLogicalUnitFailed,
            (0x67, 0x03) => Self::ModificationOfLogicalUnitFailed,
            (0x67, 0x04) => Self::ExchangeOfLogicalUnitFailed,
            (0x67, 0x05) => Self::RemoveOfLogicalUnitFailed,
            (0x67, 0x06) => Self::AttachmentOfLogicalUnitFailed,
            (0x67, 0x07) => Self::CreationOfLogicalUnitFailed,
            (0x67, 0x08) => Self::AssignFailureOccurred,
            (0x67, 0x09) => Self::MultiplyAssignedLogicalUnit,
            (0x67, 0x0A) => Self::SetTargetPortGroupsCommandFailed,
            (0x67, 0x0B) => Self::AtaDeviceFeatureNotEnabled,
            (0x67, 0x0C) => Self::CommandRejected,
            (0x67, 0x0D) => Self::ExplicitBindNotAllowed,
            (0x67, 0x0E) => Self::FeatureNotEnabled,
            (0x68, 0x00) => Self::LogicalUnitNotConfigured,
            (0x68, 0x01) => Self::SubsidiaryLogicalUnitNotConfigured,
            (0x69, 0x00) => Self::DataLossOnLogicalUnit,
            (0x69, 0x01) => Self::MultipleLogicalUnitFailures,
            (0x69, 0x02) => Self::ParityOrDataMismatch,
            (0x6A, 0x00) => Self::InformationalReferToLog,
            (0x6B, 0x00) => Self::StateChangeHasOccurred,
            (0x6B, 0x01) => Self::RedundancyLevelGotBetter,
            (0x6B, 0x02) => Self::RedundancyLevelGotWorse,
            (0x6C, 0x00) => Self::RebuildFailureOccurred,
            (0x6D, 0x00) => Self::RecalculateFailureOccurred,
            (0x6E, 0x00) => Self::CommandToLogicalUnitFailed,
            (0x6F, 0x00) => Self::CopyProtectionKeyExchangeFailureAuthenticationFailure,
            (0x6F, 0x01) => Self::CopyProtectionKeyExchangeFailureKeyNotPresent,
            (0x6F, 0x02) => Self::CopyProtectionKeyExchangeFailureKeyNotEstablished,
            (0x6F, 0x03) => Self::ReadOfScrambledSectorWithoutAuthentication,
            (0x6F, 0x04) => Self::MediaRegionCodeIsMismatchedToLogicalUnitRegion,
            (0x6F, 0x05) => Self::DriveRegionMustBePermanentOrRegionResetCountError,
            (0x6F, 0x06) => Self::InsufficientBlockCountForBindingNonceRecording,
            (0x6F, 0x07) => Self::ConflictInBindingNonceRecording,
            (0x6F, 0x08) => Self::InsufficientPermission,
            (0x6F, 0x09) => Self::InvalidDriveHostPairingServer,
            (0x6F, 0x0A) => Self::DriveHostPairingSuspended,
            (0x71, 0x00) => Self::DecompressionExceptionLongAlgorithmId,
            (0x72, 0x00) => Self::SessionFixationError,
            (0x72, 0x01) => Self::SessionFixationErrorWritingLeadIn,
            (0x72, 0x02) => Self::SessionFixationErrorWritingLeadOut,
            (0x72, 0x03) => Self::SessionFixationErrorIncompleteTrackInSession,
            (0x72, 0x04) => Self::EmptyOrPartiallyWrittenReservedTrack,
            (0x72, 0x05) => Self::NoMoreTrackReservationsAllowed,
            (0x72, 0x06) => Self::RmzExtensionIsNotAllowed,
            (0x72, 0x07) => Self::NoMoreTestZoneExtensionsAreAllowed,
            (0x73, 0x00) => Self::CdControlError,
            (0x73, 0x01) => Self::PowerCalibrationAreaAlmostFull,
            (0x73, 0x02) => Self::PowerCalibrationAreaIsFull,
            (0x73, 0x03) => Self::PowerCalibrationAreaError,
            (0x73, 0x04) => Self::ProgramMemoryAreaUpdateFailure,
            (0x73, 0x05) => Self::ProgramMemoryAreaIsFull,
            (0x73, 0x06) => Self::RmaOrPmaIsAlmostFull,
            (0x73, 0x10) => Self::CurrentPowerCalibrationAreaAlmostFull,
            (0x73, 0x11) => Self::CurrentPowerCalibrationAreaIsFull,
            (0x73, 0x17) => Self::RdzIsFull,
            (0x74, 0x00) => Self::SecurityError,
            (0x74, 0x01) => Self::UnableToDecryptData,
            (0x74, 0x02) => Self::UnencryptedDataEncounteredWhileDecrypting,
            (0x74, 0x03) => Self::IncorrectDataEncryptionKey,
            (0x74, 0x04) => Self::CryptographicIntegrityValidationFailed,
            (0x74, 0x05) => Self::ErrorDecryptingData,
            (0x74, 0x06) => Self::UnknownSignatureVerificationKey,
            (0x74, 0x07) => Self::EncryptionParametersNotUseable,
            (0x74, 0x08) => Self::DigitalSignatureValidationFailure,
            (0x74, 0x09) => Self::EncryptionModeMismatchOnRead,
            (0x74, 0x0A) => Self::EncryptedBlockNotRawReadEnabled,
            (0x74, 0x0B) => Self::IncorrectEncryptionParameters,
            (0x74, 0x0C) => Self::UnableToDecryptParameterList,
            (0x74, 0x0D) => Self::EncryptionAlgorithmDisabled,
            (0x74, 0x10) => Self::SaCreationParameterValueInvalid,
            (0x74, 0x11) => Self::SaCreationParameterValueRejected,
            (0x74, 0x12) => Self::InvalidSaUsage,
            (0x74, 0x21) => Self::DataEncryptionConfigurationPrevented,
            (0x74, 0x30) => Self::SaCreationParameterNotSupported,
            (0x74, 0x40) => Self::AuthenticationFailed,
            (0x74, 0x61) => Self::ExternalDataEncryptionKeyManagerAccessError,
            (0x74, 0x62) => Self::ExternalDataEncryptionKeyManagerError,
            (0x74, 0x63) => Self::ExternalDataEncryptionKeyNotFound,
            (0x74, 0x64) => Self::ExternalDataEncryptionRequestNotAuthorized,
            (0x74, 0x6E) => Self::ExternalDataEncryptionControlTimeout,
            (0x74, 0x6F) => Self::ExternalDataEncryptionControlError,
            (0x74, 0x71) => Self::LogicalUnitAccessNotAuthorized,
            (0x74, 0x79) => Self::SecurityConflictInTranslatedDevice,
            (0x20, 0x05) | (0x24, 0x02 | 0x03) => Self::Obsolete(asc, ascq),
            (
                0x0F | 0x3C | 0x4F | 0x56 | 0x5F | 0x75 | 0x76 | 0x77 | 0x78 | 0x79 | 0x7A | 0x7B
                | 0x7C | 0x7D | 0x7E | 0x7F,
                0x00,
            ) => Self::UnassignedDeviceTypeCode(asc, ascq),
            (0x40, ascq) => Self::DiagnosticFailureOnComponent(ascq),
            (0x4D, ascq) => Self::TaggedOverlappedCommands(ascq),
            (0x70, ascq) => Self::DecompressionExceptionShortAlgorithmId(ascq),
            (0x80..=0xFF, _) => Self::VendorSpecific(asc, ascq),
            (_, 0x80..=0xFF) => Self::VendorSpecificQualification(asc, ascq),
            _ => Self::Reserved(asc, ascq),
        }
    }
}

impl std::fmt::Display for AdditionalSenseCode {
    /// Writes the spec's human-readable description for the condition (e.g.
    /// "LOGICAL UNIT NOT READY, FORMAT IN PROGRESS").
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoAdditionalSenseInformation => write!(f, "NO ADDITIONAL SENSE INFORMATION"),
            Self::FilemarkDetected => write!(f, "FILEMARK DETECTED"),
            Self::EndOfPartitionOrMediumDetected => write!(f, "END-OF-PARTITION/MEDIUM DETECTED"),
            Self::SetmarkDetected => write!(f, "SETMARK DETECTED"),
            Self::BeginningOfPartitionOrMediumDetected => {
                write!(f, "BEGINNING-OF-PARTITION/MEDIUM DETECTED")
            },
            Self::EndOfDataDetected => write!(f, "END-OF-DATA DETECTED"),
            Self::IoProcessTerminated => write!(f, "I/O PROCESS TERMINATED"),
            Self::ProgrammableEarlyWarningDetected => {
                write!(f, "PROGRAMMABLE EARLY WARNING DETECTED")
            },
            Self::AudioPlayOperationInProgress => write!(f, "AUDIO PLAY OPERATION IN PROGRESS"),
            Self::AudioPlayOperationPaused => write!(f, "AUDIO PLAY OPERATION PAUSED"),
            Self::AudioPlayOperationSuccessfullyCompleted => {
                write!(f, "AUDIO PLAY OPERATION SUCCESSFULLY COMPLETED")
            },
            Self::AudioPlayOperationStoppedDueToError => {
                write!(f, "AUDIO PLAY OPERATION STOPPED DUE TO ERROR")
            },
            Self::NoCurrentAudioStatusToReturn => write!(f, "NO CURRENT AUDIO STATUS TO RETURN"),
            Self::OperationInProgress => write!(f, "OPERATION IN PROGRESS"),
            Self::CleaningRequested => write!(f, "CLEANING REQUESTED"),
            Self::EraseOperationInProgress => write!(f, "ERASE OPERATION IN PROGRESS"),
            Self::LocateOperationInProgress => write!(f, "LOCATE OPERATION IN PROGRESS"),
            Self::RewindOperationInProgress => write!(f, "REWIND OPERATION IN PROGRESS"),
            Self::SetCapacityOperationInProgress => {
                write!(f, "SET CAPACITY OPERATION IN PROGRESS")
            },
            Self::VerifyOperationInProgress => write!(f, "VERIFY OPERATION IN PROGRESS"),
            Self::AtaPassThroughInformationAvailable => {
                write!(f, "ATA PASS THROUGH INFORMATION AVAILABLE")
            },
            Self::ConflictingSaCreationRequest => write!(f, "CONFLICTING SA CREATION REQUEST"),
            Self::LogicalUnitTransitioningToAnotherPowerCondition => {
                write!(f, "LOGICAL UNIT TRANSITIONING TO ANOTHER POWER CONDITION")
            },
            Self::ExtendedCopyInformationAvailable => {
                write!(f, "EXTENDED COPY INFORMATION AVAILABLE")
            },
            Self::AtomicCommandAbortedDueToAca => write!(f, "ATOMIC COMMAND ABORTED DUE TO ACA"),
            Self::DeferredMicrocodeIsPending => write!(f, "DEFERRED MICROCODE IS PENDING"),
            Self::OverlappingAtomicCommandInProgress => {
                write!(f, "OVERLAPPING ATOMIC COMMAND IN PROGRESS")
            },
            Self::NoIndexOrSectorSignal => write!(f, "NO INDEX/SECTOR SIGNAL"),
            Self::NoSeekComplete => write!(f, "NO SEEK COMPLETE"),
            Self::PeripheralDeviceWriteFault => write!(f, "PERIPHERAL DEVICE WRITE FAULT"),
            Self::NoWriteCurrent => write!(f, "NO WRITE CURRENT"),
            Self::ExcessiveWriteErrors => write!(f, "EXCESSIVE WRITE ERRORS"),
            Self::LogicalUnitNotReadyCauseNotReportable => {
                write!(f, "LOGICAL UNIT NOT READY, CAUSE NOT REPORTABLE")
            },
            Self::LogicalUnitIsInProcessOfBecomingReady => {
                write!(f, "LOGICAL UNIT IS IN PROCESS OF BECOMING READY")
            },
            Self::LogicalUnitNotReadyInitializingCommandRequired => {
                write!(f, "LOGICAL UNIT NOT READY, INITIALIZING COMMAND REQUIRED")
            },
            Self::LogicalUnitNotReadyManualInterventionRequired => {
                write!(f, "LOGICAL UNIT NOT READY, MANUAL INTERVENTION REQUIRED")
            },
            Self::LogicalUnitNotReadyFormatInProgress => {
                write!(f, "LOGICAL UNIT NOT READY, FORMAT IN PROGRESS")
            },
            Self::LogicalUnitNotReadyRebuildInProgress => {
                write!(f, "LOGICAL UNIT NOT READY, REBUILD IN PROGRESS")
            },
            Self::LogicalUnitNotReadyRecalculationInProgress => {
                write!(f, "LOGICAL UNIT NOT READY, RECALCULATION IN PROGRESS")
            },
            Self::LogicalUnitNotReadyOperationInProgress => {
                write!(f, "LOGICAL UNIT NOT READY, OPERATION IN PROGRESS")
            },
            Self::LogicalUnitNotReadyLongWriteInProgress => {
                write!(f, "LOGICAL UNIT NOT READY, LONG WRITE IN PROGRESS")
            },
            Self::LogicalUnitNotReadySelfTestInProgress => {
                write!(f, "LOGICAL UNIT NOT READY, SELF-TEST IN PROGRESS")
            },
            Self::LogicalUnitNotAccessibleAsymmetricAccessStateTransition => {
                write!(
                    f,
                    "LOGICAL UNIT NOT ACCESSIBLE, ASYMMETRIC ACCESS STATE TRANSITION"
                )
            },
            Self::LogicalUnitNotAccessibleTargetPortInStandbyState => {
                write!(
                    f,
                    "LOGICAL UNIT NOT ACCESSIBLE, TARGET PORT IN STANDBY STATE"
                )
            },
            Self::LogicalUnitNotAccessibleTargetPortInUnavailableState => {
                write!(
                    f,
                    "LOGICAL UNIT NOT ACCESSIBLE, TARGET PORT IN UNAVAILABLE STATE"
                )
            },
            Self::LogicalUnitNotReadyStructureCheckRequired => {
                write!(f, "LOGICAL UNIT NOT READY, STRUCTURE CHECK REQUIRED")
            },
            Self::LogicalUnitNotReadySecuritySessionInProgress => {
                write!(f, "LOGICAL UNIT NOT READY, SECURITY SESSION IN PROGRESS")
            },
            Self::LogicalUnitNotReadyAuxiliaryMemoryNotAccessible => {
                write!(f, "LOGICAL UNIT NOT READY, AUXILIARY MEMORY NOT ACCESSIBLE")
            },
            Self::LogicalUnitNotReadyNotifyEnableSpinupRequired => {
                write!(f, "LOGICAL UNIT NOT READY, NOTIFY (ENABLE SPINUP) REQUIRED")
            },
            Self::LogicalUnitNotReadyOffline => write!(f, "LOGICAL UNIT NOT READY, OFFLINE"),
            Self::LogicalUnitNotReadySaCreationInProgress => {
                write!(f, "LOGICAL UNIT NOT READY, SA CREATION IN PROGRESS")
            },
            Self::LogicalUnitNotReadySpaceAllocationInProgress => {
                write!(f, "LOGICAL UNIT NOT READY, SPACE ALLOCATION IN PROGRESS")
            },
            Self::LogicalUnitNotReadyRoboticsDisabled => {
                write!(f, "LOGICAL UNIT NOT READY, ROBOTICS DISABLED")
            },
            Self::LogicalUnitNotReadyConfigurationRequired => {
                write!(f, "LOGICAL UNIT NOT READY, CONFIGURATION REQUIRED")
            },
            Self::LogicalUnitNotReadyCalibrationRequired => {
                write!(f, "LOGICAL UNIT NOT READY, CALIBRATION REQUIRED")
            },
            Self::LogicalUnitNotReadyADoorIsOpen => {
                write!(f, "LOGICAL UNIT NOT READY, A DOOR IS OPEN")
            },
            Self::LogicalUnitNotReadyOperatingInSequentialMode => {
                write!(f, "LOGICAL UNIT NOT READY, OPERATING IN SEQUENTIAL MODE")
            },
            Self::LogicalUnitNotReadyStartStopUnitCommandInProgress => {
                write!(
                    f,
                    "LOGICAL UNIT NOT READY, START STOP UNIT COMMAND IN PROGRESS"
                )
            },
            Self::LogicalUnitNotReadySanitizeInProgress => {
                write!(f, "LOGICAL UNIT NOT READY, SANITIZE IN PROGRESS")
            },
            Self::LogicalUnitNotReadyAdditionalPowerUseNotYetGranted => {
                write!(
                    f,
                    "LOGICAL UNIT NOT READY, ADDITIONAL POWER USE NOT YET GRANTED"
                )
            },
            Self::LogicalUnitNotReadyConfigurationInProgress => {
                write!(f, "LOGICAL UNIT NOT READY, CONFIGURATION IN PROGRESS")
            },
            Self::LogicalUnitNotReadyMicrocodeActivationRequired => {
                write!(f, "LOGICAL UNIT NOT READY, MICROCODE ACTIVATION REQUIRED")
            },
            Self::LogicalUnitNotReadyMicrocodeDownloadRequired => {
                write!(f, "LOGICAL UNIT NOT READY, MICROCODE DOWNLOAD REQUIRED")
            },
            Self::LogicalUnitNotReadyLogicalUnitResetRequired => {
                write!(f, "LOGICAL UNIT NOT READY, LOGICAL UNIT RESET REQUIRED")
            },
            Self::LogicalUnitNotReadyHardResetRequired => {
                write!(f, "LOGICAL UNIT NOT READY, HARD RESET REQUIRED")
            },
            Self::LogicalUnitNotReadyPowerCycleRequired => {
                write!(f, "LOGICAL UNIT NOT READY, POWER CYCLE REQUIRED")
            },
            Self::LogicalUnitNotReadyAffiliationRequired => {
                write!(f, "LOGICAL UNIT NOT READY, AFFILIATION REQUIRED")
            },
            Self::DepopulationInProgress => write!(f, "DEPOPULATION IN PROGRESS"),
            Self::DepopulationRestorationInProgress => {
                write!(f, "DEPOPULATION RESTORATION IN PROGRESS")
            },
            Self::LogicalUnitDoesNotRespondToSelection => {
                write!(f, "LOGICAL UNIT DOES NOT RESPOND TO SELECTION")
            },
            Self::NoReferencePositionFound => write!(f, "NO REFERENCE POSITION FOUND"),
            Self::MultiplePeripheralDevicesSelected => {
                write!(f, "MULTIPLE PERIPHERAL DEVICES SELECTED")
            },
            Self::LogicalUnitCommunicationFailure => {
                write!(f, "LOGICAL UNIT COMMUNICATION FAILURE")
            },
            Self::LogicalUnitCommunicationTimeOut => {
                write!(f, "LOGICAL UNIT COMMUNICATION TIME-OUT")
            },
            Self::LogicalUnitCommunicationParityError => {
                write!(f, "LOGICAL UNIT COMMUNICATION PARITY ERROR")
            },
            Self::LogicalUnitCommunicationCrcErrorUltraDma32 => {
                write!(f, "LOGICAL UNIT COMMUNICATION CRC ERROR (ULTRA-DMA/32)")
            },
            Self::UnreachableCopyTarget => write!(f, "UNREACHABLE COPY TARGET"),
            Self::TrackFollowingError => write!(f, "TRACK FOLLOWING ERROR"),
            Self::TrackingServoFailure => write!(f, "TRACKING SERVO FAILURE"),
            Self::FocusServoFailure => write!(f, "FOCUS SERVO FAILURE"),
            Self::SpindleServoFailure => write!(f, "SPINDLE SERVO FAILURE"),
            Self::HeadSelectFault => write!(f, "HEAD SELECT FAULT"),
            Self::VibrationInducedTrackingError => write!(f, "VIBRATION INDUCED TRACKING ERROR"),
            Self::ErrorLogOverflow => write!(f, "ERROR LOG OVERFLOW"),
            Self::Warning => write!(f, "WARNING"),
            Self::WarningSpecifiedTemperatureExceeded => {
                write!(f, "WARNING - SPECIFIED TEMPERATURE EXCEEDED")
            },
            Self::WarningEnclosureDegraded => write!(f, "WARNING - ENCLOSURE DEGRADED"),
            Self::WarningBackgroundSelfTestFailed => {
                write!(f, "WARNING - BACKGROUND SELF-TEST FAILED")
            },
            Self::WarningBackgroundPreScanDetectedMediumError => {
                write!(f, "WARNING - BACKGROUND PRE-SCAN DETECTED MEDIUM ERROR")
            },
            Self::WarningBackgroundMediumScanDetectedMediumError => {
                write!(f, "WARNING - BACKGROUND MEDIUM SCAN DETECTED MEDIUM ERROR")
            },
            Self::WarningNonVolatileCacheNowVolatile => {
                write!(f, "WARNING - NON-VOLATILE CACHE NOW VOLATILE")
            },
            Self::WarningDegradedPowerToNonVolatileCache => {
                write!(f, "WARNING - DEGRADED POWER TO NON-VOLATILE CACHE")
            },
            Self::WarningPowerLossExpected => write!(f, "WARNING - POWER LOSS EXPECTED"),
            Self::WarningDeviceStatisticsNotificationActive => {
                write!(f, "WARNING - DEVICE STATISTICS NOTIFICATION ACTIVE")
            },
            Self::WarningHighCriticalTemperatureLimitExceeded => {
                write!(f, "WARNING - HIGH CRITICAL TEMPERATURE LIMIT EXCEEDED")
            },
            Self::WarningLowCriticalTemperatureLimitExceeded => {
                write!(f, "WARNING - LOW CRITICAL TEMPERATURE LIMIT EXCEEDED")
            },
            Self::WarningHighOperatingTemperatureLimitExceeded => {
                write!(f, "WARNING - HIGH OPERATING TEMPERATURE LIMIT EXCEEDED")
            },
            Self::WarningLowOperatingTemperatureLimitExceeded => {
                write!(f, "WARNING - LOW OPERATING TEMPERATURE LIMIT EXCEEDED")
            },
            Self::WarningHighCriticalHumidityLimitExceeded => {
                write!(f, "WARNING - HIGH CRITICAL HUMIDITY LIMIT EXCEEDED")
            },
            Self::WarningLowCriticalHumidityLimitExceeded => {
                write!(f, "WARNING - LOW CRITICAL HUMIDITY LIMIT EXCEEDED")
            },
            Self::WarningHighOperatingHumidityLimitExceeded => {
                write!(f, "WARNING - HIGH OPERATING HUMIDITY LIMIT EXCEEDED")
            },
            Self::WarningLowOperatingHumidityLimitExceeded => {
                write!(f, "WARNING - LOW OPERATING HUMIDITY LIMIT EXCEEDED")
            },
            Self::WarningMicrocodeSecurityAtRisk => {
                write!(f, "WARNING - MICROCODE SECURITY AT RISK")
            },
            Self::WarningMicrocodeDigitalSignatureValidationFailure => {
                write!(
                    f,
                    "WARNING - MICROCODE DIGITAL SIGNATURE VALIDATION FAILURE"
                )
            },
            Self::WarningPhysicalElementStatusChange => {
                write!(f, "WARNING - PHYSICAL ELEMENT STATUS CHANGE")
            },
            Self::WriteError => write!(f, "WRITE ERROR"),
            Self::WriteErrorRecoveredWithAutoReallocation => {
                write!(f, "WRITE ERROR - RECOVERED WITH AUTO REALLOCATION")
            },
            Self::WriteErrorAutoReallocationFailed => {
                write!(f, "WRITE ERROR - AUTO REALLOCATION FAILED")
            },
            Self::WriteErrorRecommendReassignment => {
                write!(f, "WRITE ERROR - RECOMMEND REASSIGNMENT")
            },
            Self::CompressionCheckMiscompareError => {
                write!(f, "COMPRESSION CHECK MISCOMPARE ERROR")
            },
            Self::DataExpansionOccurredDuringCompression => {
                write!(f, "DATA EXPANSION OCCURRED DURING COMPRESSION")
            },
            Self::BlockNotCompressible => write!(f, "BLOCK NOT COMPRESSIBLE"),
            Self::WriteErrorRecoveryNeeded => write!(f, "WRITE ERROR - RECOVERY NEEDED"),
            Self::WriteErrorRecoveryFailed => write!(f, "WRITE ERROR - RECOVERY FAILED"),
            Self::WriteErrorLossOfStreaming => write!(f, "WRITE ERROR - LOSS OF STREAMING"),
            Self::WriteErrorPaddingBlocksAdded => write!(f, "WRITE ERROR - PADDING BLOCKS ADDED"),
            Self::AuxiliaryMemoryWriteError => write!(f, "AUXILIARY MEMORY WRITE ERROR"),
            Self::WriteErrorUnexpectedUnsolicitedData => {
                write!(f, "WRITE ERROR - UNEXPECTED UNSOLICITED DATA")
            },
            Self::WriteErrorNotEnoughUnsolicitedData => {
                write!(f, "WRITE ERROR - NOT ENOUGH UNSOLICITED DATA")
            },
            Self::MultipleWriteErrors => write!(f, "MULTIPLE WRITE ERRORS"),
            Self::DefectsInErrorWindow => write!(f, "DEFECTS IN ERROR WINDOW"),
            Self::IncompleteMultipleAtomicWriteOperations => {
                write!(f, "INCOMPLETE MULTIPLE ATOMIC WRITE OPERATIONS")
            },
            Self::WriteErrorRecoveryScanNeeded => write!(f, "WRITE ERROR - RECOVERY SCAN NEEDED"),
            Self::WriteErrorInsufficientZoneResources => {
                write!(f, "WRITE ERROR - INSUFFICIENT ZONE RESOURCES")
            },
            Self::ErrorDetectedByThirdPartyTemporaryInitiator => {
                write!(f, "ERROR DETECTED BY THIRD PARTY TEMPORARY INITIATOR")
            },
            Self::ThirdPartyDeviceFailure => write!(f, "THIRD PARTY DEVICE FAILURE"),
            Self::CopyTargetDeviceNotReachable => write!(f, "COPY TARGET DEVICE NOT REACHABLE"),
            Self::IncorrectCopyTargetDeviceType => write!(f, "INCORRECT COPY TARGET DEVICE TYPE"),
            Self::CopyTargetDeviceDataUnderrun => write!(f, "COPY TARGET DEVICE DATA UNDERRUN"),
            Self::CopyTargetDeviceDataOverrun => write!(f, "COPY TARGET DEVICE DATA OVERRUN"),
            Self::InvalidInformationUnit => write!(f, "INVALID INFORMATION UNIT"),
            Self::InformationUnitTooShort => write!(f, "INFORMATION UNIT TOO SHORT"),
            Self::InformationUnitTooLong => write!(f, "INFORMATION UNIT TOO LONG"),
            Self::InvalidFieldInCommandInformationUnit => {
                write!(f, "INVALID FIELD IN COMMAND INFORMATION UNIT")
            },
            Self::IdCrcOrEccError => write!(f, "ID CRC OR ECC ERROR"),
            Self::LogicalBlockGuardCheckFailed => write!(f, "LOGICAL BLOCK GUARD CHECK FAILED"),
            Self::LogicalBlockApplicationTagCheckFailed => {
                write!(f, "LOGICAL BLOCK APPLICATION TAG CHECK FAILED")
            },
            Self::LogicalBlockReferenceTagCheckFailed => {
                write!(f, "LOGICAL BLOCK REFERENCE TAG CHECK FAILED")
            },
            Self::LogicalBlockProtectionErrorOnRecoverBufferedData => {
                write!(f, "LOGICAL BLOCK PROTECTION ERROR ON RECOVER BUFFERED DATA")
            },
            Self::LogicalBlockProtectionMethodError => {
                write!(f, "LOGICAL BLOCK PROTECTION METHOD ERROR")
            },
            Self::UnrecoveredReadError => write!(f, "UNRECOVERED READ ERROR"),
            Self::ReadRetriesExhausted => write!(f, "READ RETRIES EXHAUSTED"),
            Self::ErrorTooLongToCorrect => write!(f, "ERROR TOO LONG TO CORRECT"),
            Self::MultipleReadErrors => write!(f, "MULTIPLE READ ERRORS"),
            Self::UnrecoveredReadErrorAutoReallocateFailed => {
                write!(f, "UNRECOVERED READ ERROR - AUTO REALLOCATE FAILED")
            },
            Self::LEcUncorrectableError => write!(f, "L-EC UNCORRECTABLE ERROR"),
            Self::CircUnrecoveredError => write!(f, "CIRC UNRECOVERED ERROR"),
            Self::DataReSynchronizationError => write!(f, "DATA RE-SYNCHRONIZATION ERROR"),
            Self::IncompleteBlockRead => write!(f, "INCOMPLETE BLOCK READ"),
            Self::NoGapFound => write!(f, "NO GAP FOUND"),
            Self::MiscorrectedError => write!(f, "MISCORRECTED ERROR"),
            Self::UnrecoveredReadErrorRecommendReassignment => {
                write!(f, "UNRECOVERED READ ERROR - RECOMMEND REASSIGNMENT")
            },
            Self::UnrecoveredReadErrorRecommendRewriteTheData => {
                write!(f, "UNRECOVERED READ ERROR - RECOMMEND REWRITE THE DATA")
            },
            Self::DeCompressionCrcError => write!(f, "DE-COMPRESSION CRC ERROR"),
            Self::CannotDecompressUsingDeclaredAlgorithm => {
                write!(f, "CANNOT DECOMPRESS USING DECLARED ALGORITHM")
            },
            Self::ErrorReadingUpcOrEanNumber => write!(f, "ERROR READING UPC/EAN NUMBER"),
            Self::ErrorReadingIsrcNumber => write!(f, "ERROR READING ISRC NUMBER"),
            Self::ReadErrorLossOfStreaming => write!(f, "READ ERROR - LOSS OF STREAMING"),
            Self::AuxiliaryMemoryReadError => write!(f, "AUXILIARY MEMORY READ ERROR"),
            Self::ReadErrorFailedRetransmissionRequest => {
                write!(f, "READ ERROR - FAILED RETRANSMISSION REQUEST")
            },
            Self::ReadErrorLbaMarkedBadByApplicationClient => {
                write!(f, "READ ERROR - LBA MARKED BAD BY APPLICATION CLIENT")
            },
            Self::WriteAfterSanitizeRequired => write!(f, "WRITE AFTER SANITIZE REQUIRED"),
            Self::AddressMarkNotFoundForIdField => {
                write!(f, "ADDRESS MARK NOT FOUND FOR ID FIELD")
            },
            Self::AddressMarkNotFoundForDataField => {
                write!(f, "ADDRESS MARK NOT FOUND FOR DATA FIELD")
            },
            Self::RecordedEntityNotFound => write!(f, "RECORDED ENTITY NOT FOUND"),
            Self::RecordNotFound => write!(f, "RECORD NOT FOUND"),
            Self::FilemarkOrSetmarkNotFound => write!(f, "FILEMARK OR SETMARK NOT FOUND"),
            Self::EndOfDataNotFound => write!(f, "END-OF-DATA NOT FOUND"),
            Self::BlockSequenceError => write!(f, "BLOCK SEQUENCE ERROR"),
            Self::RecordNotFoundRecommendReassignment => {
                write!(f, "RECORD NOT FOUND - RECOMMEND REASSIGNMENT")
            },
            Self::RecordNotFoundDataAutoReallocated => {
                write!(f, "RECORD NOT FOUND - DATA AUTO-REALLOCATED")
            },
            Self::LocateOperationFailure => write!(f, "LOCATE OPERATION FAILURE"),
            Self::RandomPositioningError => write!(f, "RANDOM POSITIONING ERROR"),
            Self::MechanicalPositioningError => write!(f, "MECHANICAL POSITIONING ERROR"),
            Self::PositioningErrorDetectedByReadOfMedium => {
                write!(f, "POSITIONING ERROR DETECTED BY READ OF MEDIUM")
            },
            Self::DataSynchronizationMarkError => write!(f, "DATA SYNCHRONIZATION MARK ERROR"),
            Self::DataSyncErrorDataRewritten => write!(f, "DATA SYNC ERROR - DATA REWRITTEN"),
            Self::DataSyncErrorRecommendRewrite => {
                write!(f, "DATA SYNC ERROR - RECOMMEND REWRITE")
            },
            Self::DataSyncErrorDataAutoReallocated => {
                write!(f, "DATA SYNC ERROR - DATA AUTO-REALLOCATED")
            },
            Self::DataSyncErrorRecommendReassignment => {
                write!(f, "DATA SYNC ERROR - RECOMMEND REASSIGNMENT")
            },
            Self::RecoveredDataWithNoErrorCorrectionApplied => {
                write!(f, "RECOVERED DATA WITH NO ERROR CORRECTION APPLIED")
            },
            Self::RecoveredDataWithRetries => write!(f, "RECOVERED DATA WITH RETRIES"),
            Self::RecoveredDataWithPositiveHeadOffset => {
                write!(f, "RECOVERED DATA WITH POSITIVE HEAD OFFSET")
            },
            Self::RecoveredDataWithNegativeHeadOffset => {
                write!(f, "RECOVERED DATA WITH NEGATIVE HEAD OFFSET")
            },
            Self::RecoveredDataWithRetriesAndOrCircApplied => {
                write!(f, "RECOVERED DATA WITH RETRIES AND/OR CIRC APPLIED")
            },
            Self::RecoveredDataUsingPreviousSectorId => {
                write!(f, "RECOVERED DATA USING PREVIOUS SECTOR ID")
            },
            Self::RecoveredDataWithoutEccDataAutoReallocated => {
                write!(f, "RECOVERED DATA WITHOUT ECC - DATA AUTO-REALLOCATED")
            },
            Self::RecoveredDataWithoutEccRecommendReassignment => {
                write!(f, "RECOVERED DATA WITHOUT ECC - RECOMMEND REASSIGNMENT")
            },
            Self::RecoveredDataWithoutEccRecommendRewrite => {
                write!(f, "RECOVERED DATA WITHOUT ECC - RECOMMEND REWRITE")
            },
            Self::RecoveredDataWithoutEccDataRewritten => {
                write!(f, "RECOVERED DATA WITHOUT ECC - DATA REWRITTEN")
            },
            Self::RecoveredDataWithErrorCorrectionApplied => {
                write!(f, "RECOVERED DATA WITH ERROR CORRECTION APPLIED")
            },
            Self::RecoveredDataWithErrorCorrAndRetriesApplied => {
                write!(f, "RECOVERED DATA WITH ERROR CORR. & RETRIES APPLIED")
            },
            Self::RecoveredDataDataAutoReallocated => {
                write!(f, "RECOVERED DATA - DATA AUTO-REALLOCATED")
            },
            Self::RecoveredDataWithCirc => write!(f, "RECOVERED DATA WITH CIRC"),
            Self::RecoveredDataWithLEc => write!(f, "RECOVERED DATA WITH L-EC"),
            Self::RecoveredDataRecommendReassignment => {
                write!(f, "RECOVERED DATA - RECOMMEND REASSIGNMENT")
            },
            Self::RecoveredDataRecommendRewrite => {
                write!(f, "RECOVERED DATA - RECOMMEND REWRITE")
            },
            Self::RecoveredDataWithEccDataRewritten => {
                write!(f, "RECOVERED DATA WITH ECC - DATA REWRITTEN")
            },
            Self::RecoveredDataWithLinking => write!(f, "RECOVERED DATA WITH LINKING"),
            Self::DefectListError => write!(f, "DEFECT LIST ERROR"),
            Self::DefectListNotAvailable => write!(f, "DEFECT LIST NOT AVAILABLE"),
            Self::DefectListErrorInPrimaryList => write!(f, "DEFECT LIST ERROR IN PRIMARY LIST"),
            Self::DefectListErrorInGrownList => write!(f, "DEFECT LIST ERROR IN GROWN LIST"),
            Self::ParameterListLengthError => write!(f, "PARAMETER LIST LENGTH ERROR"),
            Self::SynchronousDataTransferError => write!(f, "SYNCHRONOUS DATA TRANSFER ERROR"),
            Self::DefectListNotFound => write!(f, "DEFECT LIST NOT FOUND"),
            Self::PrimaryDefectListNotFound => write!(f, "PRIMARY DEFECT LIST NOT FOUND"),
            Self::GrownDefectListNotFound => write!(f, "GROWN DEFECT LIST NOT FOUND"),
            Self::MiscompareDuringVerifyOperation => {
                write!(f, "MISCOMPARE DURING VERIFY OPERATION")
            },
            Self::MiscompareVerifyOfUnmappedLba => write!(f, "MISCOMPARE VERIFY OF UNMAPPED LBA"),
            Self::RecoveredIdWithEccCorrection => write!(f, "RECOVERED ID WITH ECC CORRECTION"),
            Self::PartialDefectListTransfer => write!(f, "PARTIAL DEFECT LIST TRANSFER"),
            Self::InvalidCommandOperationCode => write!(f, "INVALID COMMAND OPERATION CODE"),
            Self::AccessDeniedInitiatorPendingEnrolled => {
                write!(f, "ACCESS DENIED - INITIATOR PENDING-ENROLLED")
            },
            Self::AccessDeniedNoAccessRights => write!(f, "ACCESS DENIED - NO ACCESS RIGHTS"),
            Self::AccessDeniedInvalidMgmtIdKey => {
                write!(f, "ACCESS DENIED - INVALID MGMT ID KEY")
            },
            Self::IllegalCommandWhileInWriteCapableState => {
                write!(f, "ILLEGAL COMMAND WHILE IN WRITE CAPABLE STATE")
            },
            Self::IllegalCommandWhileInExplicitAddressMode => {
                write!(f, "ILLEGAL COMMAND WHILE IN EXPLICIT ADDRESS MODE")
            },
            Self::IllegalCommandWhileInImplicitAddressMode => {
                write!(f, "ILLEGAL COMMAND WHILE IN IMPLICIT ADDRESS MODE")
            },
            Self::AccessDeniedEnrollmentConflict => {
                write!(f, "ACCESS DENIED - ENROLLMENT CONFLICT")
            },
            Self::AccessDeniedInvalidLuIdentifier => {
                write!(f, "ACCESS DENIED - INVALID LU IDENTIFIER")
            },
            Self::AccessDeniedInvalidProxyToken => {
                write!(f, "ACCESS DENIED - INVALID PROXY TOKEN")
            },
            Self::AccessDeniedAclLunConflict => write!(f, "ACCESS DENIED - ACL LUN CONFLICT"),
            Self::IllegalCommandWhenNotInAppendOnlyMode => {
                write!(f, "ILLEGAL COMMAND WHEN NOT IN APPEND-ONLY MODE")
            },
            Self::NotAnAdministrativeLogicalUnit => {
                write!(f, "NOT AN ADMINISTRATIVE LOGICAL UNIT")
            },
            Self::NotASubsidiaryLogicalUnit => write!(f, "NOT A SUBSIDIARY LOGICAL UNIT"),
            Self::NotAConglomerateLogicalUnit => write!(f, "NOT A CONGLOMERATE LOGICAL UNIT"),
            Self::LogicalBlockAddressOutOfRange => {
                write!(f, "LOGICAL BLOCK ADDRESS OUT OF RANGE")
            },
            Self::InvalidElementAddress => write!(f, "INVALID ELEMENT ADDRESS"),
            Self::InvalidAddressForWrite => write!(f, "INVALID ADDRESS FOR WRITE"),
            Self::InvalidWriteCrossingLayerJump => write!(f, "INVALID WRITE CROSSING LAYER JUMP"),
            Self::UnalignedWriteCommand => write!(f, "UNALIGNED WRITE COMMAND"),
            Self::WriteBoundaryViolation => write!(f, "WRITE BOUNDARY VIOLATION"),
            Self::AttemptToReadInvalidData => write!(f, "ATTEMPT TO READ INVALID DATA"),
            Self::ReadBoundaryViolation => write!(f, "READ BOUNDARY VIOLATION"),
            Self::MisalignedWriteCommand => write!(f, "MISALIGNED WRITE COMMAND"),
            Self::AttemptToAccessGapZone => write!(f, "ATTEMPT TO ACCESS GAP ZONE"),
            Self::IllegalFunction => write!(f, "ILLEGAL FUNCTION (USE 20 00, 24 00, OR 26 00)"),
            Self::InvalidTokenOperationCauseNotReportable => {
                write!(f, "INVALID TOKEN OPERATION, CAUSE NOT REPORTABLE")
            },
            Self::InvalidTokenOperationUnsupportedTokenType => {
                write!(f, "INVALID TOKEN OPERATION, UNSUPPORTED TOKEN TYPE")
            },
            Self::InvalidTokenOperationRemoteTokenUsageNotSupported => {
                write!(
                    f,
                    "INVALID TOKEN OPERATION, REMOTE TOKEN USAGE NOT SUPPORTED"
                )
            },
            Self::InvalidTokenOperationRemoteRodTokenCreationNotSupported => {
                write!(
                    f,
                    "INVALID TOKEN OPERATION, REMOTE ROD TOKEN CREATION NOT SUPPORTED"
                )
            },
            Self::InvalidTokenOperationTokenUnknown => {
                write!(f, "INVALID TOKEN OPERATION, TOKEN UNKNOWN")
            },
            Self::InvalidTokenOperationTokenCorrupt => {
                write!(f, "INVALID TOKEN OPERATION, TOKEN CORRUPT")
            },
            Self::InvalidTokenOperationTokenRevoked => {
                write!(f, "INVALID TOKEN OPERATION, TOKEN REVOKED")
            },
            Self::InvalidTokenOperationTokenExpired => {
                write!(f, "INVALID TOKEN OPERATION, TOKEN EXPIRED")
            },
            Self::InvalidTokenOperationTokenCancelled => {
                write!(f, "INVALID TOKEN OPERATION, TOKEN CANCELLED")
            },
            Self::InvalidTokenOperationTokenDeleted => {
                write!(f, "INVALID TOKEN OPERATION, TOKEN DELETED")
            },
            Self::InvalidTokenOperationInvalidTokenLength => {
                write!(f, "INVALID TOKEN OPERATION, INVALID TOKEN LENGTH")
            },
            Self::InvalidFieldInCdb => write!(f, "INVALID FIELD IN CDB"),
            Self::CdbDecryptionError => write!(f, "CDB DECRYPTION ERROR"),
            Self::SecurityAuditValueFrozen => write!(f, "SECURITY AUDIT VALUE FROZEN"),
            Self::SecurityWorkingKeyFrozen => write!(f, "SECURITY WORKING KEY FROZEN"),
            Self::NonceNotUnique => write!(f, "NONCE NOT UNIQUE"),
            Self::NonceTimestampOutOfRange => write!(f, "NONCE TIMESTAMP OUT OF RANGE"),
            Self::InvalidXcdb => write!(f, "INVALID XCDB"),
            Self::InvalidFastFormat => write!(f, "INVALID FAST FORMAT"),
            Self::LogicalUnitNotSupported => write!(f, "LOGICAL UNIT NOT SUPPORTED"),
            Self::InvalidFieldInParameterList => write!(f, "INVALID FIELD IN PARAMETER LIST"),
            Self::ParameterNotSupported => write!(f, "PARAMETER NOT SUPPORTED"),
            Self::ParameterValueInvalid => write!(f, "PARAMETER VALUE INVALID"),
            Self::ThresholdParametersNotSupported => {
                write!(f, "THRESHOLD PARAMETERS NOT SUPPORTED")
            },
            Self::InvalidReleaseOfPersistentReservation => {
                write!(f, "INVALID RELEASE OF PERSISTENT RESERVATION")
            },
            Self::DataDecryptionError => write!(f, "DATA DECRYPTION ERROR"),
            Self::TooManyTargetDescriptors => write!(f, "TOO MANY TARGET DESCRIPTORS"),
            Self::UnsupportedTargetDescriptorTypeCode => {
                write!(f, "UNSUPPORTED TARGET DESCRIPTOR TYPE CODE")
            },
            Self::TooManySegmentDescriptors => write!(f, "TOO MANY SEGMENT DESCRIPTORS"),
            Self::UnsupportedSegmentDescriptorTypeCode => {
                write!(f, "UNSUPPORTED SEGMENT DESCRIPTOR TYPE CODE")
            },
            Self::UnexpectedInexactSegment => write!(f, "UNEXPECTED INEXACT SEGMENT"),
            Self::InlineDataLengthExceeded => write!(f, "INLINE DATA LENGTH EXCEEDED"),
            Self::InvalidOperationForCopySourceOrDestination => {
                write!(f, "INVALID OPERATION FOR COPY SOURCE OR DESTINATION")
            },
            Self::CopySegmentGranularityViolation => {
                write!(f, "COPY SEGMENT GRANULARITY VIOLATION")
            },
            Self::InvalidParameterWhilePortIsEnabled => {
                write!(f, "INVALID PARAMETER WHILE PORT IS ENABLED")
            },
            Self::InvalidDataOutBufferIntegrityCheckValue => {
                write!(f, "INVALID DATA-OUT BUFFER INTEGRITY CHECK VALUE")
            },
            Self::DataDecryptionKeyFailLimitReached => {
                write!(f, "DATA DECRYPTION KEY FAIL LIMIT REACHED")
            },
            Self::IncompleteKeyAssociatedDataSet => {
                write!(f, "INCOMPLETE KEY-ASSOCIATED DATA SET")
            },
            Self::VendorSpecificKeyReferenceNotFound => {
                write!(f, "VENDOR SPECIFIC KEY REFERENCE NOT FOUND")
            },
            Self::ApplicationTagModePageIsInvalid => {
                write!(f, "APPLICATION TAG MODE PAGE IS INVALID")
            },
            Self::TapeStreamMirroringPrevented => write!(f, "TAPE STREAM MIRRORING PREVENTED"),
            Self::CopySourceOrCopyDestinationNotAuthorized => {
                write!(f, "COPY SOURCE OR COPY DESTINATION NOT AUTHORIZED")
            },
            Self::FastCopyNotPossible => write!(f, "FAST COPY NOT POSSIBLE"),
            Self::WriteProtected => write!(f, "WRITE PROTECTED"),
            Self::HardwareWriteProtected => write!(f, "HARDWARE WRITE PROTECTED"),
            Self::LogicalUnitSoftwareWriteProtected => {
                write!(f, "LOGICAL UNIT SOFTWARE WRITE PROTECTED")
            },
            Self::AssociatedWriteProtect => write!(f, "ASSOCIATED WRITE PROTECT"),
            Self::PersistentWriteProtect => write!(f, "PERSISTENT WRITE PROTECT"),
            Self::PermanentWriteProtect => write!(f, "PERMANENT WRITE PROTECT"),
            Self::ConditionalWriteProtect => write!(f, "CONDITIONAL WRITE PROTECT"),
            Self::SpaceAllocationFailedWriteProtect => {
                write!(f, "SPACE ALLOCATION FAILED WRITE PROTECT")
            },
            Self::ZoneIsReadOnly => write!(f, "ZONE IS READ ONLY"),
            Self::NotReadyToReadyChangeMediumMayHaveChanged => {
                write!(f, "NOT READY TO READY CHANGE, MEDIUM MAY HAVE CHANGED")
            },
            Self::ImportOrExportElementAccessed => write!(f, "IMPORT OR EXPORT ELEMENT ACCESSED"),
            Self::FormatLayerMayHaveChanged => write!(f, "FORMAT-LAYER MAY HAVE CHANGED"),
            Self::ImportOrExportElementAccessedMediumChanged => {
                write!(f, "IMPORT/EXPORT ELEMENT ACCESSED, MEDIUM CHANGED")
            },
            Self::PowerOnResetOrBusDeviceResetOccurred => {
                write!(f, "POWER ON, RESET, OR BUS DEVICE RESET OCCURRED")
            },
            Self::PowerOnOccurred => write!(f, "POWER ON OCCURRED"),
            Self::ScsiBusResetOccurred => write!(f, "SCSI BUS RESET OCCURRED"),
            Self::BusDeviceResetFunctionOccurred => {
                write!(f, "BUS DEVICE RESET FUNCTION OCCURRED")
            },
            Self::DeviceInternalReset => write!(f, "DEVICE INTERNAL RESET"),
            Self::TransceiverModeChangedToSingleEnded => {
                write!(f, "TRANSCEIVER MODE CHANGED TO SINGLE-ENDED")
            },
            Self::TransceiverModeChangedToLvd => write!(f, "TRANSCEIVER MODE CHANGED TO LVD"),
            Self::ITNexusLossOccurred => write!(f, "I_T NEXUS LOSS OCCURRED"),
            Self::ParametersChanged => write!(f, "PARAMETERS CHANGED"),
            Self::ModeParametersChanged => write!(f, "MODE PARAMETERS CHANGED"),
            Self::LogParametersChanged => write!(f, "LOG PARAMETERS CHANGED"),
            Self::ReservationsPreempted => write!(f, "RESERVATIONS PREEMPTED"),
            Self::ReservationsReleased => write!(f, "RESERVATIONS RELEASED"),
            Self::RegistrationsPreempted => write!(f, "REGISTRATIONS PREEMPTED"),
            Self::AsymmetricAccessStateChanged => write!(f, "ASYMMETRIC ACCESS STATE CHANGED"),
            Self::ImplicitAsymmetricAccessStateTransitionFailed => {
                write!(f, "IMPLICIT ASYMMETRIC ACCESS STATE TRANSITION FAILED")
            },
            Self::PriorityChanged => write!(f, "PRIORITY CHANGED"),
            Self::CapacityDataHasChanged => write!(f, "CAPACITY DATA HAS CHANGED"),
            Self::ErrorHistoryITNexusCleared => write!(f, "ERROR HISTORY I_T NEXUS CLEARED"),
            Self::ErrorHistorySnapshotReleased => write!(f, "ERROR HISTORY SNAPSHOT RELEASED"),
            Self::ErrorRecoveryAttributesHaveChanged => {
                write!(f, "ERROR RECOVERY ATTRIBUTES HAVE CHANGED")
            },
            Self::DataEncryptionCapabilitiesChanged => {
                write!(f, "DATA ENCRYPTION CAPABILITIES CHANGED")
            },
            Self::TimestampChanged => write!(f, "TIMESTAMP CHANGED"),
            Self::DataEncryptionParametersChangedByAnotherITNexus => {
                write!(f, "DATA ENCRYPTION PARAMETERS CHANGED BY ANOTHER I_T NEXUS")
            },
            Self::DataEncryptionParametersChangedByVendorSpecificEvent => {
                write!(
                    f,
                    "DATA ENCRYPTION PARAMETERS CHANGED BY VENDOR SPECIFIC EVENT"
                )
            },
            Self::DataEncryptionKeyInstanceCounterHasChanged => {
                write!(f, "DATA ENCRYPTION KEY INSTANCE COUNTER HAS CHANGED")
            },
            Self::SaCreationCapabilitiesDataHasChanged => {
                write!(f, "SA CREATION CAPABILITIES DATA HAS CHANGED")
            },
            Self::MediumRemovalPreventionPreempted => {
                write!(f, "MEDIUM REMOVAL PREVENTION PREEMPTED")
            },
            Self::ZoneResetWritePointerRecommended => {
                write!(f, "ZONE RESET WRITE POINTER RECOMMENDED")
            },
            Self::CopyCannotExecuteSinceHostCannotDisconnect => {
                write!(f, "COPY CANNOT EXECUTE SINCE HOST CANNOT DISCONNECT")
            },
            Self::CommandSequenceError => write!(f, "COMMAND SEQUENCE ERROR"),
            Self::TooManyWindowsSpecified => write!(f, "TOO MANY WINDOWS SPECIFIED"),
            Self::InvalidCombinationOfWindowsSpecified => {
                write!(f, "INVALID COMBINATION OF WINDOWS SPECIFIED")
            },
            Self::CurrentProgramAreaIsNotEmpty => write!(f, "CURRENT PROGRAM AREA IS NOT EMPTY"),
            Self::CurrentProgramAreaIsEmpty => write!(f, "CURRENT PROGRAM AREA IS EMPTY"),
            Self::IllegalPowerConditionRequest => write!(f, "ILLEGAL POWER CONDITION REQUEST"),
            Self::PersistentPreventConflict => write!(f, "PERSISTENT PREVENT CONFLICT"),
            Self::PreviousBusyStatus => write!(f, "PREVIOUS BUSY STATUS"),
            Self::PreviousTaskSetFullStatus => write!(f, "PREVIOUS TASK SET FULL STATUS"),
            Self::PreviousReservationConflictStatus => {
                write!(f, "PREVIOUS RESERVATION CONFLICT STATUS")
            },
            Self::PartitionOrCollectionContainsUserObjects => {
                write!(f, "PARTITION OR COLLECTION CONTAINS USER OBJECTS")
            },
            Self::NotReserved => write!(f, "NOT RESERVED"),
            Self::OrwriteGenerationDoesNotMatch => write!(f, "ORWRITE GENERATION DOES NOT MATCH"),
            Self::ResetWritePointerNotAllowed => write!(f, "RESET WRITE POINTER NOT ALLOWED"),
            Self::ZoneIsOffline => write!(f, "ZONE IS OFFLINE"),
            Self::StreamNotOpen => write!(f, "STREAM NOT OPEN"),
            Self::UnwrittenDataInZone => write!(f, "UNWRITTEN DATA IN ZONE"),
            Self::DescriptorFormatSenseDataRequired => {
                write!(f, "DESCRIPTOR FORMAT SENSE DATA REQUIRED")
            },
            Self::ZoneIsInactive => write!(f, "ZONE IS INACTIVE"),
            Self::WellKnownLogicalUnitAccessRequired => {
                write!(f, "WELL KNOWN LOGICAL UNIT ACCESS REQUIRED")
            },
            Self::OverwriteErrorOnUpdateInPlace => {
                write!(f, "OVERWRITE ERROR ON UPDATE IN PLACE")
            },
            Self::InsufficientTimeForOperation => write!(f, "INSUFFICIENT TIME FOR OPERATION"),
            Self::CommandTimeoutBeforeProcessing => {
                write!(f, "COMMAND TIMEOUT BEFORE PROCESSING")
            },
            Self::CommandTimeoutDuringProcessing => {
                write!(f, "COMMAND TIMEOUT DURING PROCESSING")
            },
            Self::CommandTimeoutDuringProcessingDueToErrorRecovery => {
                write!(f, "COMMAND TIMEOUT DURING PROCESSING DUE TO ERROR RECOVERY")
            },
            Self::CommandsClearedByAnotherInitiator => {
                write!(f, "COMMANDS CLEARED BY ANOTHER INITIATOR")
            },
            Self::CommandsClearedByPowerLossNotification => {
                write!(f, "COMMANDS CLEARED BY POWER LOSS NOTIFICATION")
            },
            Self::CommandsClearedByDeviceServer => write!(f, "COMMANDS CLEARED BY DEVICE SERVER"),
            Self::SomeCommandsClearedByQueuingLayerEvent => {
                write!(f, "SOME COMMANDS CLEARED BY QUEUING LAYER EVENT")
            },
            Self::IncompatibleMediumInstalled => write!(f, "INCOMPATIBLE MEDIUM INSTALLED"),
            Self::CannotReadMediumUnknownFormat => {
                write!(f, "CANNOT READ MEDIUM - UNKNOWN FORMAT")
            },
            Self::CannotReadMediumIncompatibleFormat => {
                write!(f, "CANNOT READ MEDIUM - INCOMPATIBLE FORMAT")
            },
            Self::CleaningCartridgeInstalled => write!(f, "CLEANING CARTRIDGE INSTALLED"),
            Self::CannotWriteMediumUnknownFormat => {
                write!(f, "CANNOT WRITE MEDIUM - UNKNOWN FORMAT")
            },
            Self::CannotWriteMediumIncompatibleFormat => {
                write!(f, "CANNOT WRITE MEDIUM - INCOMPATIBLE FORMAT")
            },
            Self::CannotFormatMediumIncompatibleMedium => {
                write!(f, "CANNOT FORMAT MEDIUM - INCOMPATIBLE MEDIUM")
            },
            Self::CleaningFailure => write!(f, "CLEANING FAILURE"),
            Self::CannotWriteApplicationCodeMismatch => {
                write!(f, "CANNOT WRITE - APPLICATION CODE MISMATCH")
            },
            Self::CurrentSessionNotFixatedForAppend => {
                write!(f, "CURRENT SESSION NOT FIXATED FOR APPEND")
            },
            Self::CleaningRequestRejected => write!(f, "CLEANING REQUEST REJECTED"),
            Self::WormMediumOverwriteAttempted => write!(f, "WORM MEDIUM - OVERWRITE ATTEMPTED"),
            Self::WormMediumIntegrityCheck => write!(f, "WORM MEDIUM - INTEGRITY CHECK"),
            Self::MediumNotFormatted => write!(f, "MEDIUM NOT FORMATTED"),
            Self::IncompatibleVolumeType => write!(f, "INCOMPATIBLE VOLUME TYPE"),
            Self::IncompatibleVolumeQualifier => write!(f, "INCOMPATIBLE VOLUME QUALIFIER"),
            Self::CleaningVolumeExpired => write!(f, "CLEANING VOLUME EXPIRED"),
            Self::MediumFormatCorrupted => write!(f, "MEDIUM FORMAT CORRUPTED"),
            Self::FormatCommandFailed => write!(f, "FORMAT COMMAND FAILED"),
            Self::ZonedFormattingFailedDueToSpareLinking => {
                write!(f, "ZONED FORMATTING FAILED DUE TO SPARE LINKING")
            },
            Self::SanitizeCommandFailed => write!(f, "SANITIZE COMMAND FAILED"),
            Self::DepopulationFailed => write!(f, "DEPOPULATION FAILED"),
            Self::DepopulationRestorationFailed => write!(f, "DEPOPULATION RESTORATION FAILED"),
            Self::NoDefectSpareLocationAvailable => {
                write!(f, "NO DEFECT SPARE LOCATION AVAILABLE")
            },
            Self::DefectListUpdateFailure => write!(f, "DEFECT LIST UPDATE FAILURE"),
            Self::TapeLengthError => write!(f, "TAPE LENGTH ERROR"),
            Self::EnclosureFailure => write!(f, "ENCLOSURE FAILURE"),
            Self::EnclosureServicesFailure => write!(f, "ENCLOSURE SERVICES FAILURE"),
            Self::UnsupportedEnclosureFunction => write!(f, "UNSUPPORTED ENCLOSURE FUNCTION"),
            Self::EnclosureServicesUnavailable => write!(f, "ENCLOSURE SERVICES UNAVAILABLE"),
            Self::EnclosureServicesTransferFailure => {
                write!(f, "ENCLOSURE SERVICES TRANSFER FAILURE")
            },
            Self::EnclosureServicesTransferRefused => {
                write!(f, "ENCLOSURE SERVICES TRANSFER REFUSED")
            },
            Self::EnclosureServicesChecksumError => {
                write!(f, "ENCLOSURE SERVICES CHECKSUM ERROR")
            },
            Self::RibbonInkOrTonerFailure => write!(f, "RIBBON, INK, OR TONER FAILURE"),
            Self::RoundedParameter => write!(f, "ROUNDED PARAMETER"),
            Self::EventStatusNotification => write!(f, "EVENT STATUS NOTIFICATION"),
            Self::EsnPowerManagementClassEvent => write!(f, "ESN - POWER MANAGEMENT CLASS EVENT"),
            Self::EsnMediaClassEvent => write!(f, "ESN - MEDIA CLASS EVENT"),
            Self::EsnDeviceBusyClassEvent => write!(f, "ESN - DEVICE BUSY CLASS EVENT"),
            Self::ThinProvisioningSoftThresholdReached => {
                write!(f, "THIN PROVISIONING SOFT THRESHOLD REACHED")
            },
            Self::DepopulationInterrupted => write!(f, "DEPOPULATION INTERRUPTED"),
            Self::DepopulationRestorationInterrupted => {
                write!(f, "DEPOPULATION RESTORATION INTERRUPTED")
            },
            Self::SavingParametersNotSupported => write!(f, "SAVING PARAMETERS NOT SUPPORTED"),
            Self::MediumNotPresent => write!(f, "MEDIUM NOT PRESENT"),
            Self::MediumNotPresentTrayClosed => write!(f, "MEDIUM NOT PRESENT - TRAY CLOSED"),
            Self::MediumNotPresentTrayOpen => write!(f, "MEDIUM NOT PRESENT - TRAY OPEN"),
            Self::MediumNotPresentLoadable => write!(f, "MEDIUM NOT PRESENT - LOADABLE"),
            Self::MediumNotPresentMediumAuxiliaryMemoryAccessible => {
                write!(f, "MEDIUM NOT PRESENT - MEDIUM AUXILIARY MEMORY ACCESSIBLE")
            },
            Self::SequentialPositioningError => write!(f, "SEQUENTIAL POSITIONING ERROR"),
            Self::TapePositionErrorAtBeginningOfMedium => {
                write!(f, "TAPE POSITION ERROR AT BEGINNING-OF-MEDIUM")
            },
            Self::TapePositionErrorAtEndOfMedium => {
                write!(f, "TAPE POSITION ERROR AT END-OF-MEDIUM")
            },
            Self::TapeOrElectronicVerticalFormsUnitNotReady => {
                write!(f, "TAPE OR ELECTRONIC VERTICAL FORMS UNIT NOT READY")
            },
            Self::SlewFailure => write!(f, "SLEW FAILURE"),
            Self::PaperJam => write!(f, "PAPER JAM"),
            Self::FailedToSenseTopOfForm => write!(f, "FAILED TO SENSE TOP-OF-FORM"),
            Self::FailedToSenseBottomOfForm => write!(f, "FAILED TO SENSE BOTTOM-OF-FORM"),
            Self::RepositionError => write!(f, "REPOSITION ERROR"),
            Self::ReadPastEndOfMedium => write!(f, "READ PAST END OF MEDIUM"),
            Self::ReadPastBeginningOfMedium => write!(f, "READ PAST BEGINNING OF MEDIUM"),
            Self::PositionPastEndOfMedium => write!(f, "POSITION PAST END OF MEDIUM"),
            Self::PositionPastBeginningOfMedium => write!(f, "POSITION PAST BEGINNING OF MEDIUM"),
            Self::MediumDestinationElementFull => write!(f, "MEDIUM DESTINATION ELEMENT FULL"),
            Self::MediumSourceElementEmpty => write!(f, "MEDIUM SOURCE ELEMENT EMPTY"),
            Self::EndOfMediumReached => write!(f, "END OF MEDIUM REACHED"),
            Self::MediumMagazineNotAccessible => write!(f, "MEDIUM MAGAZINE NOT ACCESSIBLE"),
            Self::MediumMagazineRemoved => write!(f, "MEDIUM MAGAZINE REMOVED"),
            Self::MediumMagazineInserted => write!(f, "MEDIUM MAGAZINE INSERTED"),
            Self::MediumMagazineLocked => write!(f, "MEDIUM MAGAZINE LOCKED"),
            Self::MediumMagazineUnlocked => write!(f, "MEDIUM MAGAZINE UNLOCKED"),
            Self::MechanicalPositioningOrChangerError => {
                write!(f, "MECHANICAL POSITIONING OR CHANGER ERROR")
            },
            Self::ReadPastEndOfUserObject => write!(f, "READ PAST END OF USER OBJECT"),
            Self::ElementDisabled => write!(f, "ELEMENT DISABLED"),
            Self::ElementEnabled => write!(f, "ELEMENT ENABLED"),
            Self::DataTransferDeviceRemoved => write!(f, "DATA TRANSFER DEVICE REMOVED"),
            Self::DataTransferDeviceInserted => write!(f, "DATA TRANSFER DEVICE INSERTED"),
            Self::TooManyLogicalObjectsOnPartitionToSupportOperation => {
                write!(
                    f,
                    "TOO MANY LOGICAL OBJECTS ON PARTITION TO SUPPORT OPERATION"
                )
            },
            Self::ElementStaticInformationChanged => {
                write!(f, "ELEMENT STATIC INFORMATION CHANGED")
            },
            Self::InvalidBitsInIdentifyMessage => write!(f, "INVALID BITS IN IDENTIFY MESSAGE"),
            Self::LogicalUnitHasNotSelfConfiguredYet => {
                write!(f, "LOGICAL UNIT HAS NOT SELF-CONFIGURED YET")
            },
            Self::LogicalUnitFailure => write!(f, "LOGICAL UNIT FAILURE"),
            Self::TimeoutOnLogicalUnit => write!(f, "TIMEOUT ON LOGICAL UNIT"),
            Self::LogicalUnitFailedSelfTest => write!(f, "LOGICAL UNIT FAILED SELF-TEST"),
            Self::LogicalUnitUnableToUpdateSelfTestLog => {
                write!(f, "LOGICAL UNIT UNABLE TO UPDATE SELF-TEST LOG")
            },
            Self::TargetOperatingConditionsHaveChanged => {
                write!(f, "TARGET OPERATING CONDITIONS HAVE CHANGED")
            },
            Self::MicrocodeHasBeenChanged => write!(f, "MICROCODE HAS BEEN CHANGED"),
            Self::ChangedOperatingDefinition => write!(f, "CHANGED OPERATING DEFINITION"),
            Self::InquiryDataHasChanged => write!(f, "INQUIRY DATA HAS CHANGED"),
            Self::ComponentDeviceAttached => write!(f, "COMPONENT DEVICE ATTACHED"),
            Self::DeviceIdentifierChanged => write!(f, "DEVICE IDENTIFIER CHANGED"),
            Self::RedundancyGroupCreatedOrModified => {
                write!(f, "REDUNDANCY GROUP CREATED OR MODIFIED")
            },
            Self::RedundancyGroupDeleted => write!(f, "REDUNDANCY GROUP DELETED"),
            Self::SpareCreatedOrModified => write!(f, "SPARE CREATED OR MODIFIED"),
            Self::SpareDeleted => write!(f, "SPARE DELETED"),
            Self::VolumeSetCreatedOrModified => write!(f, "VOLUME SET CREATED OR MODIFIED"),
            Self::VolumeSetDeleted => write!(f, "VOLUME SET DELETED"),
            Self::VolumeSetDeassigned => write!(f, "VOLUME SET DEASSIGNED"),
            Self::VolumeSetReassigned => write!(f, "VOLUME SET REASSIGNED"),
            Self::ReportedLunsDataHasChanged => write!(f, "REPORTED LUNS DATA HAS CHANGED"),
            Self::EchoBufferOverwritten => write!(f, "ECHO BUFFER OVERWRITTEN"),
            Self::MediumLoadable => write!(f, "MEDIUM LOADABLE"),
            Self::MediumAuxiliaryMemoryAccessible => {
                write!(f, "MEDIUM AUXILIARY MEMORY ACCESSIBLE")
            },
            Self::IscsiIpAddressAdded => write!(f, "iSCSI IP ADDRESS ADDED"),
            Self::IscsiIpAddressRemoved => write!(f, "iSCSI IP ADDRESS REMOVED"),
            Self::IscsiIpAddressChanged => write!(f, "iSCSI IP ADDRESS CHANGED"),
            Self::InspectReferralsSenseDescriptors => {
                write!(f, "INSPECT REFERRALS SENSE DESCRIPTORS")
            },
            Self::MicrocodeHasBeenChangedWithoutReset => {
                write!(f, "MICROCODE HAS BEEN CHANGED WITHOUT RESET")
            },
            Self::ZoneTransitionToFull => write!(f, "ZONE TRANSITION TO FULL"),
            Self::BindCompleted => write!(f, "BIND COMPLETED"),
            Self::BindRedirected => write!(f, "BIND REDIRECTED"),
            Self::SubsidiaryBindingChanged => write!(f, "SUBSIDIARY BINDING CHANGED"),
            Self::RamFailure => write!(f, "RAM FAILURE (SHOULD USE 40 NN)"),
            Self::DataPathFailure => write!(f, "DATA PATH FAILURE (SHOULD USE 40 NN)"),
            Self::PowerOnOrSelfTestFailure => {
                write!(f, "POWER-ON OR SELF-TEST FAILURE (SHOULD USE 40 NN)")
            },
            Self::MessageError => write!(f, "MESSAGE ERROR"),
            Self::InternalTargetFailure => write!(f, "INTERNAL TARGET FAILURE"),
            Self::PersistentReservationInformationLost => {
                write!(f, "PERSISTENT RESERVATION INFORMATION LOST")
            },
            Self::AtaDeviceFailedSetFeatures => write!(f, "ATA DEVICE FAILED SET FEATURES"),
            Self::SelectOrReselectFailure => write!(f, "SELECT OR RESELECT FAILURE"),
            Self::UnsuccessfulSoftReset => write!(f, "UNSUCCESSFUL SOFT RESET"),
            Self::ScsiParityError => write!(f, "SCSI PARITY ERROR"),
            Self::DataPhaseCrcErrorDetected => write!(f, "DATA PHASE CRC ERROR DETECTED"),
            Self::ScsiParityErrorDetectedDuringStDataPhase => {
                write!(f, "SCSI PARITY ERROR DETECTED DURING ST DATA PHASE")
            },
            Self::InformationUnitIucrcErrorDetected => {
                write!(f, "INFORMATION UNIT iuCRC ERROR DETECTED")
            },
            Self::AsynchronousInformationProtectionErrorDetected => {
                write!(f, "ASYNCHRONOUS INFORMATION PROTECTION ERROR DETECTED")
            },
            Self::ProtocolServiceCrcError => write!(f, "PROTOCOL SERVICE CRC ERROR"),
            Self::PhyTestFunctionInProgress => write!(f, "PHY TEST FUNCTION IN PROGRESS"),
            Self::SomeCommandsClearedByIscsiProtocolEvent => {
                write!(f, "SOME COMMANDS CLEARED BY ISCSI PROTOCOL EVENT")
            },
            Self::InitiatorDetectedErrorMessageReceived => {
                write!(f, "INITIATOR DETECTED ERROR MESSAGE RECEIVED")
            },
            Self::InvalidMessageError => write!(f, "INVALID MESSAGE ERROR"),
            Self::CommandPhaseError => write!(f, "COMMAND PHASE ERROR"),
            Self::DataPhaseError => write!(f, "DATA PHASE ERROR"),
            Self::InvalidTargetPortTransferTagReceived => {
                write!(f, "INVALID TARGET PORT TRANSFER TAG RECEIVED")
            },
            Self::TooMuchWriteData => write!(f, "TOO MUCH WRITE DATA"),
            Self::AckNakTimeout => write!(f, "ACK/NAK TIMEOUT"),
            Self::NakReceived => write!(f, "NAK RECEIVED"),
            Self::DataOffsetError => write!(f, "DATA OFFSET ERROR"),
            Self::InitiatorResponseTimeout => write!(f, "INITIATOR RESPONSE TIMEOUT"),
            Self::ConnectionLost => write!(f, "CONNECTION LOST"),
            Self::DataInBufferOverflowDataBufferSize => {
                write!(f, "DATA-IN BUFFER OVERFLOW - DATA BUFFER SIZE")
            },
            Self::DataInBufferOverflowDataBufferDescriptorArea => {
                write!(f, "DATA-IN BUFFER OVERFLOW - DATA BUFFER DESCRIPTOR AREA")
            },
            Self::DataInBufferError => write!(f, "DATA-IN BUFFER ERROR"),
            Self::DataOutBufferOverflowDataBufferSize => {
                write!(f, "DATA-OUT BUFFER OVERFLOW - DATA BUFFER SIZE")
            },
            Self::DataOutBufferOverflowDataBufferDescriptorArea => {
                write!(f, "DATA-OUT BUFFER OVERFLOW - DATA BUFFER DESCRIPTOR AREA")
            },
            Self::DataOutBufferError => write!(f, "DATA-OUT BUFFER ERROR"),
            Self::PcieFabricError => write!(f, "PCIE FABRIC ERROR"),
            Self::PcieCompletionTimeout => write!(f, "PCIE COMPLETION TIMEOUT"),
            Self::PcieCompleterAbort => write!(f, "PCIE COMPLETER ABORT"),
            Self::PciePoisonedTlpReceived => write!(f, "PCIE POISONED TLP RECEIVED"),
            Self::PcieEcrcCheckFailed => write!(f, "PCIE ECRC CHECK FAILED"),
            Self::PcieUnsupportedRequest => write!(f, "PCIE UNSUPPORTED REQUEST"),
            Self::PcieAcsViolation => write!(f, "PCIE ACS VIOLATION"),
            Self::PcieTlpPrefixBlocked => write!(f, "PCIE TLP PREFIX BLOCKED"),
            Self::LogicalUnitFailedSelfConfiguration => {
                write!(f, "LOGICAL UNIT FAILED SELF-CONFIGURATION")
            },
            Self::OverlappedCommandsAttempted => write!(f, "OVERLAPPED COMMANDS ATTEMPTED"),
            Self::WriteAppendError => write!(f, "WRITE APPEND ERROR"),
            Self::WriteAppendPositionError => write!(f, "WRITE APPEND POSITION ERROR"),
            Self::PositionErrorRelatedToTiming => write!(f, "POSITION ERROR RELATED TO TIMING"),
            Self::EraseFailure => write!(f, "ERASE FAILURE"),
            Self::EraseFailureIncompleteEraseOperationDetected => {
                write!(f, "ERASE FAILURE - INCOMPLETE ERASE OPERATION DETECTED")
            },
            Self::CartridgeFault => write!(f, "CARTRIDGE FAULT"),
            Self::MediaLoadOrEjectFailed => write!(f, "MEDIA LOAD OR EJECT FAILED"),
            Self::UnloadTapeFailure => write!(f, "UNLOAD TAPE FAILURE"),
            Self::MediumRemovalPrevented => write!(f, "MEDIUM REMOVAL PREVENTED"),
            Self::MediumRemovalPreventedByDataTransferElement => {
                write!(f, "MEDIUM REMOVAL PREVENTED BY DATA TRANSFER ELEMENT")
            },
            Self::MediumThreadOrUnthreadFailure => write!(f, "MEDIUM THREAD OR UNTHREAD FAILURE"),
            Self::VolumeIdentifierInvalid => write!(f, "VOLUME IDENTIFIER INVALID"),
            Self::VolumeIdentifierMissing => write!(f, "VOLUME IDENTIFIER MISSING"),
            Self::DuplicateVolumeIdentifier => write!(f, "DUPLICATE VOLUME IDENTIFIER"),
            Self::ElementStatusUnknown => write!(f, "ELEMENT STATUS UNKNOWN"),
            Self::DataTransferDeviceErrorLoadFailed => {
                write!(f, "DATA TRANSFER DEVICE ERROR - LOAD FAILED")
            },
            Self::DataTransferDeviceErrorUnloadFailed => {
                write!(f, "DATA TRANSFER DEVICE ERROR - UNLOAD FAILED")
            },
            Self::DataTransferDeviceErrorUnloadMissing => {
                write!(f, "DATA TRANSFER DEVICE ERROR - UNLOAD MISSING")
            },
            Self::DataTransferDeviceErrorEjectFailed => {
                write!(f, "DATA TRANSFER DEVICE ERROR - EJECT FAILED")
            },
            Self::DataTransferDeviceErrorLibraryCommunicationFailed => {
                write!(
                    f,
                    "DATA TRANSFER DEVICE ERROR - LIBRARY COMMUNICATION FAILED"
                )
            },
            Self::ScsiToHostSystemInterfaceFailure => {
                write!(f, "SCSI TO HOST SYSTEM INTERFACE FAILURE")
            },
            Self::SystemResourceFailure => write!(f, "SYSTEM RESOURCE FAILURE"),
            Self::SystemBufferFull => write!(f, "SYSTEM BUFFER FULL"),
            Self::InsufficientReservationResources => {
                write!(f, "INSUFFICIENT RESERVATION RESOURCES")
            },
            Self::InsufficientResources => write!(f, "INSUFFICIENT RESOURCES"),
            Self::InsufficientRegistrationResources => {
                write!(f, "INSUFFICIENT REGISTRATION RESOURCES")
            },
            Self::InsufficientAccessControlResources => {
                write!(f, "INSUFFICIENT ACCESS CONTROL RESOURCES")
            },
            Self::AuxiliaryMemoryOutOfSpace => write!(f, "AUXILIARY MEMORY OUT OF SPACE"),
            Self::QuotaError => write!(f, "QUOTA ERROR"),
            Self::MaximumNumberOfSupplementalDecryptionKeysExceeded => {
                write!(f, "MAXIMUM NUMBER OF SUPPLEMENTAL DECRYPTION KEYS EXCEEDED")
            },
            Self::MediumAuxiliaryMemoryNotAccessible => {
                write!(f, "MEDIUM AUXILIARY MEMORY NOT ACCESSIBLE")
            },
            Self::DataCurrentlyUnavailable => write!(f, "DATA CURRENTLY UNAVAILABLE"),
            Self::InsufficientPowerForOperation => write!(f, "INSUFFICIENT POWER FOR OPERATION"),
            Self::InsufficientResourcesToCreateRod => {
                write!(f, "INSUFFICIENT RESOURCES TO CREATE ROD")
            },
            Self::InsufficientResourcesToCreateRodToken => {
                write!(f, "INSUFFICIENT RESOURCES TO CREATE ROD TOKEN")
            },
            Self::InsufficientZoneResources => write!(f, "INSUFFICIENT ZONE RESOURCES"),
            Self::InsufficientZoneResourcesToCompleteWrite => {
                write!(f, "INSUFFICIENT ZONE RESOURCES TO COMPLETE WRITE")
            },
            Self::MaximumNumberOfStreamsOpen => write!(f, "MAXIMUM NUMBER OF STREAMS OPEN"),
            Self::InsufficientResourcesToBind => write!(f, "INSUFFICIENT RESOURCES TO BIND"),
            Self::UnableToRecoverTableOfContents => {
                write!(f, "UNABLE TO RECOVER TABLE-OF-CONTENTS")
            },
            Self::GenerationDoesNotExist => write!(f, "GENERATION DOES NOT EXIST"),
            Self::UpdatedBlockRead => write!(f, "UPDATED BLOCK READ"),
            Self::OperatorRequestOrStateChangeInput => {
                write!(f, "OPERATOR REQUEST OR STATE CHANGE INPUT")
            },
            Self::OperatorMediumRemovalRequest => write!(f, "OPERATOR MEDIUM REMOVAL REQUEST"),
            Self::OperatorSelectedWriteProtect => write!(f, "OPERATOR SELECTED WRITE PROTECT"),
            Self::OperatorSelectedWritePermit => write!(f, "OPERATOR SELECTED WRITE PERMIT"),
            Self::LogException => write!(f, "LOG EXCEPTION"),
            Self::ThresholdConditionMet => write!(f, "THRESHOLD CONDITION MET"),
            Self::LogCounterAtMaximum => write!(f, "LOG COUNTER AT MAXIMUM"),
            Self::LogListCodesExhausted => write!(f, "LOG LIST CODES EXHAUSTED"),
            Self::RplStatusChange => write!(f, "RPL STATUS CHANGE"),
            Self::SpindlesSynchronized => write!(f, "SPINDLES SYNCHRONIZED"),
            Self::SpindlesNotSynchronized => write!(f, "SPINDLES NOT SYNCHRONIZED"),
            Self::FailurePredictionThresholdExceeded => {
                write!(f, "FAILURE PREDICTION THRESHOLD EXCEEDED")
            },
            Self::MediaFailurePredictionThresholdExceeded => {
                write!(f, "MEDIA FAILURE PREDICTION THRESHOLD EXCEEDED")
            },
            Self::LogicalUnitFailurePredictionThresholdExceeded => {
                write!(f, "LOGICAL UNIT FAILURE PREDICTION THRESHOLD EXCEEDED")
            },
            Self::SpareAreaExhaustionPredictionThresholdExceeded => {
                write!(f, "SPARE AREA EXHAUSTION PREDICTION THRESHOLD EXCEEDED")
            },
            Self::HardwareImpendingFailureGeneralHardDriveFailure => {
                write!(f, "HARDWARE IMPENDING FAILURE GENERAL HARD DRIVE FAILURE")
            },
            Self::HardwareImpendingFailureDriveErrorRateTooHigh => {
                write!(f, "HARDWARE IMPENDING FAILURE DRIVE ERROR RATE TOO HIGH")
            },
            Self::HardwareImpendingFailureDataErrorRateTooHigh => {
                write!(f, "HARDWARE IMPENDING FAILURE DATA ERROR RATE TOO HIGH")
            },
            Self::HardwareImpendingFailureSeekErrorRateTooHigh => {
                write!(f, "HARDWARE IMPENDING FAILURE SEEK ERROR RATE TOO HIGH")
            },
            Self::HardwareImpendingFailureTooManyBlockReassigns => {
                write!(f, "HARDWARE IMPENDING FAILURE TOO MANY BLOCK REASSIGNS")
            },
            Self::HardwareImpendingFailureAccessTimesTooHigh => {
                write!(f, "HARDWARE IMPENDING FAILURE ACCESS TIMES TOO HIGH")
            },
            Self::HardwareImpendingFailureStartUnitTimesTooHigh => {
                write!(f, "HARDWARE IMPENDING FAILURE START UNIT TIMES TOO HIGH")
            },
            Self::HardwareImpendingFailureChannelParametrics => {
                write!(f, "HARDWARE IMPENDING FAILURE CHANNEL PARAMETRICS")
            },
            Self::HardwareImpendingFailureControllerDetected => {
                write!(f, "HARDWARE IMPENDING FAILURE CONTROLLER DETECTED")
            },
            Self::HardwareImpendingFailureThroughputPerformance => {
                write!(f, "HARDWARE IMPENDING FAILURE THROUGHPUT PERFORMANCE")
            },
            Self::HardwareImpendingFailureSeekTimePerformance => {
                write!(f, "HARDWARE IMPENDING FAILURE SEEK TIME PERFORMANCE")
            },
            Self::HardwareImpendingFailureSpinUpRetryCount => {
                write!(f, "HARDWARE IMPENDING FAILURE SPIN-UP RETRY COUNT")
            },
            Self::HardwareImpendingFailureDriveCalibrationRetryCount => {
                write!(
                    f,
                    "HARDWARE IMPENDING FAILURE DRIVE CALIBRATION RETRY COUNT"
                )
            },
            Self::HardwareImpendingFailurePowerLossProtectionCircuit => {
                write!(
                    f,
                    "HARDWARE IMPENDING FAILURE POWER LOSS PROTECTION CIRCUIT"
                )
            },
            Self::ControllerImpendingFailureGeneralHardDriveFailure => {
                write!(f, "CONTROLLER IMPENDING FAILURE GENERAL HARD DRIVE FAILURE")
            },
            Self::ControllerImpendingFailureDriveErrorRateTooHigh => {
                write!(f, "CONTROLLER IMPENDING FAILURE DRIVE ERROR RATE TOO HIGH")
            },
            Self::ControllerImpendingFailureDataErrorRateTooHigh => {
                write!(f, "CONTROLLER IMPENDING FAILURE DATA ERROR RATE TOO HIGH")
            },
            Self::ControllerImpendingFailureSeekErrorRateTooHigh => {
                write!(f, "CONTROLLER IMPENDING FAILURE SEEK ERROR RATE TOO HIGH")
            },
            Self::ControllerImpendingFailureTooManyBlockReassigns => {
                write!(f, "CONTROLLER IMPENDING FAILURE TOO MANY BLOCK REASSIGNS")
            },
            Self::ControllerImpendingFailureAccessTimesTooHigh => {
                write!(f, "CONTROLLER IMPENDING FAILURE ACCESS TIMES TOO HIGH")
            },
            Self::ControllerImpendingFailureStartUnitTimesTooHigh => {
                write!(f, "CONTROLLER IMPENDING FAILURE START UNIT TIMES TOO HIGH")
            },
            Self::ControllerImpendingFailureChannelParametrics => {
                write!(f, "CONTROLLER IMPENDING FAILURE CHANNEL PARAMETRICS")
            },
            Self::ControllerImpendingFailureControllerDetected => {
                write!(f, "CONTROLLER IMPENDING FAILURE CONTROLLER DETECTED")
            },
            Self::ControllerImpendingFailureThroughputPerformance => {
                write!(f, "CONTROLLER IMPENDING FAILURE THROUGHPUT PERFORMANCE")
            },
            Self::ControllerImpendingFailureSeekTimePerformance => {
                write!(f, "CONTROLLER IMPENDING FAILURE SEEK TIME PERFORMANCE")
            },
            Self::ControllerImpendingFailureSpinUpRetryCount => {
                write!(f, "CONTROLLER IMPENDING FAILURE SPIN-UP RETRY COUNT")
            },
            Self::ControllerImpendingFailureDriveCalibrationRetryCount => {
                write!(
                    f,
                    "CONTROLLER IMPENDING FAILURE DRIVE CALIBRATION RETRY COUNT"
                )
            },
            Self::DataChannelImpendingFailureGeneralHardDriveFailure => {
                write!(
                    f,
                    "DATA CHANNEL IMPENDING FAILURE GENERAL HARD DRIVE FAILURE"
                )
            },
            Self::DataChannelImpendingFailureDriveErrorRateTooHigh => {
                write!(
                    f,
                    "DATA CHANNEL IMPENDING FAILURE DRIVE ERROR RATE TOO HIGH"
                )
            },
            Self::DataChannelImpendingFailureDataErrorRateTooHigh => {
                write!(f, "DATA CHANNEL IMPENDING FAILURE DATA ERROR RATE TOO HIGH")
            },
            Self::DataChannelImpendingFailureSeekErrorRateTooHigh => {
                write!(f, "DATA CHANNEL IMPENDING FAILURE SEEK ERROR RATE TOO HIGH")
            },
            Self::DataChannelImpendingFailureTooManyBlockReassigns => {
                write!(f, "DATA CHANNEL IMPENDING FAILURE TOO MANY BLOCK REASSIGNS")
            },
            Self::DataChannelImpendingFailureAccessTimesTooHigh => {
                write!(f, "DATA CHANNEL IMPENDING FAILURE ACCESS TIMES TOO HIGH")
            },
            Self::DataChannelImpendingFailureStartUnitTimesTooHigh => {
                write!(
                    f,
                    "DATA CHANNEL IMPENDING FAILURE START UNIT TIMES TOO HIGH"
                )
            },
            Self::DataChannelImpendingFailureChannelParametrics => {
                write!(f, "DATA CHANNEL IMPENDING FAILURE CHANNEL PARAMETRICS")
            },
            Self::DataChannelImpendingFailureControllerDetected => {
                write!(f, "DATA CHANNEL IMPENDING FAILURE CONTROLLER DETECTED")
            },
            Self::DataChannelImpendingFailureThroughputPerformance => {
                write!(f, "DATA CHANNEL IMPENDING FAILURE THROUGHPUT PERFORMANCE")
            },
            Self::DataChannelImpendingFailureSeekTimePerformance => {
                write!(f, "DATA CHANNEL IMPENDING FAILURE SEEK TIME PERFORMANCE")
            },
            Self::DataChannelImpendingFailureSpinUpRetryCount => {
                write!(f, "DATA CHANNEL IMPENDING FAILURE SPIN-UP RETRY COUNT")
            },
            Self::DataChannelImpendingFailureDriveCalibrationRetryCount => {
                write!(
                    f,
                    "DATA CHANNEL IMPENDING FAILURE DRIVE CALIBRATION RETRY COUNT"
                )
            },
            Self::ServoImpendingFailureGeneralHardDriveFailure => {
                write!(f, "SERVO IMPENDING FAILURE GENERAL HARD DRIVE FAILURE")
            },
            Self::ServoImpendingFailureDriveErrorRateTooHigh => {
                write!(f, "SERVO IMPENDING FAILURE DRIVE ERROR RATE TOO HIGH")
            },
            Self::ServoImpendingFailureDataErrorRateTooHigh => {
                write!(f, "SERVO IMPENDING FAILURE DATA ERROR RATE TOO HIGH")
            },
            Self::ServoImpendingFailureSeekErrorRateTooHigh => {
                write!(f, "SERVO IMPENDING FAILURE SEEK ERROR RATE TOO HIGH")
            },
            Self::ServoImpendingFailureTooManyBlockReassigns => {
                write!(f, "SERVO IMPENDING FAILURE TOO MANY BLOCK REASSIGNS")
            },
            Self::ServoImpendingFailureAccessTimesTooHigh => {
                write!(f, "SERVO IMPENDING FAILURE ACCESS TIMES TOO HIGH")
            },
            Self::ServoImpendingFailureStartUnitTimesTooHigh => {
                write!(f, "SERVO IMPENDING FAILURE START UNIT TIMES TOO HIGH")
            },
            Self::ServoImpendingFailureChannelParametrics => {
                write!(f, "SERVO IMPENDING FAILURE CHANNEL PARAMETRICS")
            },
            Self::ServoImpendingFailureControllerDetected => {
                write!(f, "SERVO IMPENDING FAILURE CONTROLLER DETECTED")
            },
            Self::ServoImpendingFailureThroughputPerformance => {
                write!(f, "SERVO IMPENDING FAILURE THROUGHPUT PERFORMANCE")
            },
            Self::ServoImpendingFailureSeekTimePerformance => {
                write!(f, "SERVO IMPENDING FAILURE SEEK TIME PERFORMANCE")
            },
            Self::ServoImpendingFailureSpinUpRetryCount => {
                write!(f, "SERVO IMPENDING FAILURE SPIN-UP RETRY COUNT")
            },
            Self::ServoImpendingFailureDriveCalibrationRetryCount => {
                write!(f, "SERVO IMPENDING FAILURE DRIVE CALIBRATION RETRY COUNT")
            },
            Self::SpindleImpendingFailureGeneralHardDriveFailure => {
                write!(f, "SPINDLE IMPENDING FAILURE GENERAL HARD DRIVE FAILURE")
            },
            Self::SpindleImpendingFailureDriveErrorRateTooHigh => {
                write!(f, "SPINDLE IMPENDING FAILURE DRIVE ERROR RATE TOO HIGH")
            },
            Self::SpindleImpendingFailureDataErrorRateTooHigh => {
                write!(f, "SPINDLE IMPENDING FAILURE DATA ERROR RATE TOO HIGH")
            },
            Self::SpindleImpendingFailureSeekErrorRateTooHigh => {
                write!(f, "SPINDLE IMPENDING FAILURE SEEK ERROR RATE TOO HIGH")
            },
            Self::SpindleImpendingFailureTooManyBlockReassigns => {
                write!(f, "SPINDLE IMPENDING FAILURE TOO MANY BLOCK REASSIGNS")
            },
            Self::SpindleImpendingFailureAccessTimesTooHigh => {
                write!(f, "SPINDLE IMPENDING FAILURE ACCESS TIMES TOO HIGH")
            },
            Self::SpindleImpendingFailureStartUnitTimesTooHigh => {
                write!(f, "SPINDLE IMPENDING FAILURE START UNIT TIMES TOO HIGH")
            },
            Self::SpindleImpendingFailureChannelParametrics => {
                write!(f, "SPINDLE IMPENDING FAILURE CHANNEL PARAMETRICS")
            },
            Self::SpindleImpendingFailureControllerDetected => {
                write!(f, "SPINDLE IMPENDING FAILURE CONTROLLER DETECTED")
            },
            Self::SpindleImpendingFailureThroughputPerformance => {
                write!(f, "SPINDLE IMPENDING FAILURE THROUGHPUT PERFORMANCE")
            },
            Self::SpindleImpendingFailureSeekTimePerformance => {
                write!(f, "SPINDLE IMPENDING FAILURE SEEK TIME PERFORMANCE")
            },
            Self::SpindleImpendingFailureSpinUpRetryCount => {
                write!(f, "SPINDLE IMPENDING FAILURE SPIN-UP RETRY COUNT")
            },
            Self::SpindleImpendingFailureDriveCalibrationRetryCount => {
                write!(f, "SPINDLE IMPENDING FAILURE DRIVE CALIBRATION RETRY COUNT")
            },
            Self::FirmwareImpendingFailureGeneralHardDriveFailure => {
                write!(f, "FIRMWARE IMPENDING FAILURE GENERAL HARD DRIVE FAILURE")
            },
            Self::FirmwareImpendingFailureDriveErrorRateTooHigh => {
                write!(f, "FIRMWARE IMPENDING FAILURE DRIVE ERROR RATE TOO HIGH")
            },
            Self::FirmwareImpendingFailureDataErrorRateTooHigh => {
                write!(f, "FIRMWARE IMPENDING FAILURE DATA ERROR RATE TOO HIGH")
            },
            Self::FirmwareImpendingFailureSeekErrorRateTooHigh => {
                write!(f, "FIRMWARE IMPENDING FAILURE SEEK ERROR RATE TOO HIGH")
            },
            Self::FirmwareImpendingFailureTooManyBlockReassigns => {
                write!(f, "FIRMWARE IMPENDING FAILURE TOO MANY BLOCK REASSIGNS")
            },
            Self::FirmwareImpendingFailureAccessTimesTooHigh => {
                write!(f, "FIRMWARE IMPENDING FAILURE ACCESS TIMES TOO HIGH")
            },
            Self::FirmwareImpendingFailureStartUnitTimesTooHigh => {
                write!(f, "FIRMWARE IMPENDING FAILURE START UNIT TIMES TOO HIGH")
            },
            Self::FirmwareImpendingFailureChannelParametrics => {
                write!(f, "FIRMWARE IMPENDING FAILURE CHANNEL PARAMETRICS")
            },
            Self::FirmwareImpendingFailureControllerDetected => {
                write!(f, "FIRMWARE IMPENDING FAILURE CONTROLLER DETECTED")
            },
            Self::FirmwareImpendingFailureThroughputPerformance => {
                write!(f, "FIRMWARE IMPENDING FAILURE THROUGHPUT PERFORMANCE")
            },
            Self::FirmwareImpendingFailureSeekTimePerformance => {
                write!(f, "FIRMWARE IMPENDING FAILURE SEEK TIME PERFORMANCE")
            },
            Self::FirmwareImpendingFailureSpinUpRetryCount => {
                write!(f, "FIRMWARE IMPENDING FAILURE SPIN-UP RETRY COUNT")
            },
            Self::FirmwareImpendingFailureDriveCalibrationRetryCount => {
                write!(
                    f,
                    "FIRMWARE IMPENDING FAILURE DRIVE CALIBRATION RETRY COUNT"
                )
            },
            Self::MediaImpendingFailureEnduranceLimitMet => {
                write!(f, "MEDIA IMPENDING FAILURE ENDURANCE LIMIT MET")
            },
            Self::FailurePredictionThresholdExceededFalse => {
                write!(f, "FAILURE PREDICTION THRESHOLD EXCEEDED (FALSE)")
            },
            Self::LowPowerConditionOn => write!(f, "LOW POWER CONDITION ON"),
            Self::IdleConditionActivatedByTimer => write!(f, "IDLE CONDITION ACTIVATED BY TIMER"),
            Self::StandbyConditionActivatedByTimer => {
                write!(f, "STANDBY CONDITION ACTIVATED BY TIMER")
            },
            Self::IdleConditionActivatedByCommand => {
                write!(f, "IDLE CONDITION ACTIVATED BY COMMAND")
            },
            Self::StandbyConditionActivatedByCommand => {
                write!(f, "STANDBY CONDITION ACTIVATED BY COMMAND")
            },
            Self::IdleBConditionActivatedByTimer => {
                write!(f, "IDLE_B CONDITION ACTIVATED BY TIMER")
            },
            Self::IdleBConditionActivatedByCommand => {
                write!(f, "IDLE_B CONDITION ACTIVATED BY COMMAND")
            },
            Self::IdleCConditionActivatedByTimer => {
                write!(f, "IDLE_C CONDITION ACTIVATED BY TIMER")
            },
            Self::IdleCConditionActivatedByCommand => {
                write!(f, "IDLE_C CONDITION ACTIVATED BY COMMAND")
            },
            Self::StandbyYConditionActivatedByTimer => {
                write!(f, "STANDBY_Y CONDITION ACTIVATED BY TIMER")
            },
            Self::StandbyYConditionActivatedByCommand => {
                write!(f, "STANDBY_Y CONDITION ACTIVATED BY COMMAND")
            },
            Self::PowerStateChangeToActive => write!(f, "POWER STATE CHANGE TO ACTIVE"),
            Self::PowerStateChangeToIdle => write!(f, "POWER STATE CHANGE TO IDLE"),
            Self::PowerStateChangeToStandby => write!(f, "POWER STATE CHANGE TO STANDBY"),
            Self::PowerStateChangeToSleep => write!(f, "POWER STATE CHANGE TO SLEEP"),
            Self::PowerStateChangeToDeviceControl => {
                write!(f, "POWER STATE CHANGE TO DEVICE CONTROL")
            },
            Self::LampFailure => write!(f, "LAMP FAILURE"),
            Self::VideoAcquisitionError => write!(f, "VIDEO ACQUISITION ERROR"),
            Self::UnableToAcquireVideo => write!(f, "UNABLE TO ACQUIRE VIDEO"),
            Self::OutOfFocus => write!(f, "OUT OF FOCUS"),
            Self::ScanHeadPositioningError => write!(f, "SCAN HEAD POSITIONING ERROR"),
            Self::EndOfUserAreaEncounteredOnThisTrack => {
                write!(f, "END OF USER AREA ENCOUNTERED ON THIS TRACK")
            },
            Self::PacketDoesNotFitInAvailableSpace => {
                write!(f, "PACKET DOES NOT FIT IN AVAILABLE SPACE")
            },
            Self::IllegalModeForThisTrack => write!(f, "ILLEGAL MODE FOR THIS TRACK"),
            Self::InvalidPacketSize => write!(f, "INVALID PACKET SIZE"),
            Self::VoltageFault => write!(f, "VOLTAGE FAULT"),
            Self::AutomaticDocumentFeederCoverUp => {
                write!(f, "AUTOMATIC DOCUMENT FEEDER COVER UP")
            },
            Self::AutomaticDocumentFeederLiftUp => write!(f, "AUTOMATIC DOCUMENT FEEDER LIFT UP"),
            Self::DocumentJamInAutomaticDocumentFeeder => {
                write!(f, "DOCUMENT JAM IN AUTOMATIC DOCUMENT FEEDER")
            },
            Self::DocumentMissFeedAutomaticInDocumentFeeder => {
                write!(f, "DOCUMENT MISS FEED AUTOMATIC IN DOCUMENT FEEDER")
            },
            Self::ConfigurationFailure => write!(f, "CONFIGURATION FAILURE"),
            Self::ConfigurationOfIncapableLogicalUnitsFailed => {
                write!(f, "CONFIGURATION OF INCAPABLE LOGICAL UNITS FAILED")
            },
            Self::AddLogicalUnitFailed => write!(f, "ADD LOGICAL UNIT FAILED"),
            Self::ModificationOfLogicalUnitFailed => {
                write!(f, "MODIFICATION OF LOGICAL UNIT FAILED")
            },
            Self::ExchangeOfLogicalUnitFailed => write!(f, "EXCHANGE OF LOGICAL UNIT FAILED"),
            Self::RemoveOfLogicalUnitFailed => write!(f, "REMOVE OF LOGICAL UNIT FAILED"),
            Self::AttachmentOfLogicalUnitFailed => write!(f, "ATTACHMENT OF LOGICAL UNIT FAILED"),
            Self::CreationOfLogicalUnitFailed => write!(f, "CREATION OF LOGICAL UNIT FAILED"),
            Self::AssignFailureOccurred => write!(f, "ASSIGN FAILURE OCCURRED"),
            Self::MultiplyAssignedLogicalUnit => write!(f, "MULTIPLY ASSIGNED LOGICAL UNIT"),
            Self::SetTargetPortGroupsCommandFailed => {
                write!(f, "SET TARGET PORT GROUPS COMMAND FAILED")
            },
            Self::AtaDeviceFeatureNotEnabled => write!(f, "ATA DEVICE FEATURE NOT ENABLED"),
            Self::CommandRejected => write!(f, "COMMAND REJECTED"),
            Self::ExplicitBindNotAllowed => write!(f, "EXPLICIT BIND NOT ALLOWED"),
            Self::FeatureNotEnabled => write!(f, "FEATURE NOT ENABLED"),
            Self::LogicalUnitNotConfigured => write!(f, "LOGICAL UNIT NOT CONFIGURED"),
            Self::SubsidiaryLogicalUnitNotConfigured => {
                write!(f, "SUBSIDIARY LOGICAL UNIT NOT CONFIGURED")
            },
            Self::DataLossOnLogicalUnit => write!(f, "DATA LOSS ON LOGICAL UNIT"),
            Self::MultipleLogicalUnitFailures => write!(f, "MULTIPLE LOGICAL UNIT FAILURES"),
            Self::ParityOrDataMismatch => write!(f, "PARITY/DATA MISMATCH"),
            Self::InformationalReferToLog => write!(f, "INFORMATIONAL, REFER TO LOG"),
            Self::StateChangeHasOccurred => write!(f, "STATE CHANGE HAS OCCURRED"),
            Self::RedundancyLevelGotBetter => write!(f, "REDUNDANCY LEVEL GOT BETTER"),
            Self::RedundancyLevelGotWorse => write!(f, "REDUNDANCY LEVEL GOT WORSE"),
            Self::RebuildFailureOccurred => write!(f, "REBUILD FAILURE OCCURRED"),
            Self::RecalculateFailureOccurred => write!(f, "RECALCULATE FAILURE OCCURRED"),
            Self::CommandToLogicalUnitFailed => write!(f, "COMMAND TO LOGICAL UNIT FAILED"),
            Self::CopyProtectionKeyExchangeFailureAuthenticationFailure => {
                write!(
                    f,
                    "COPY PROTECTION KEY EXCHANGE FAILURE - AUTHENTICATION FAILURE"
                )
            },
            Self::CopyProtectionKeyExchangeFailureKeyNotPresent => {
                write!(f, "COPY PROTECTION KEY EXCHANGE FAILURE - KEY NOT PRESENT")
            },
            Self::CopyProtectionKeyExchangeFailureKeyNotEstablished => {
                write!(
                    f,
                    "COPY PROTECTION KEY EXCHANGE FAILURE - KEY NOT ESTABLISHED"
                )
            },
            Self::ReadOfScrambledSectorWithoutAuthentication => {
                write!(f, "READ OF SCRAMBLED SECTOR WITHOUT AUTHENTICATION")
            },
            Self::MediaRegionCodeIsMismatchedToLogicalUnitRegion => {
                write!(f, "MEDIA REGION CODE IS MISMATCHED TO LOGICAL UNIT REGION")
            },
            Self::DriveRegionMustBePermanentOrRegionResetCountError => {
                write!(f, "DRIVE REGION MUST BE PERMANENT/REGION RESET COUNT ERROR")
            },
            Self::InsufficientBlockCountForBindingNonceRecording => {
                write!(f, "INSUFFICIENT BLOCK COUNT FOR BINDING NONCE RECORDING")
            },
            Self::ConflictInBindingNonceRecording => {
                write!(f, "CONFLICT IN BINDING NONCE RECORDING")
            },
            Self::InsufficientPermission => write!(f, "INSUFFICIENT PERMISSION"),
            Self::InvalidDriveHostPairingServer => write!(f, "INVALID DRIVE-HOST PAIRING SERVER"),
            Self::DriveHostPairingSuspended => write!(f, "DRIVE-HOST PAIRING SUSPENDED"),
            Self::DecompressionExceptionLongAlgorithmId => {
                write!(f, "DECOMPRESSION EXCEPTION LONG ALGORITHM ID")
            },
            Self::SessionFixationError => write!(f, "SESSION FIXATION ERROR"),
            Self::SessionFixationErrorWritingLeadIn => {
                write!(f, "SESSION FIXATION ERROR WRITING LEAD-IN")
            },
            Self::SessionFixationErrorWritingLeadOut => {
                write!(f, "SESSION FIXATION ERROR WRITING LEAD-OUT")
            },
            Self::SessionFixationErrorIncompleteTrackInSession => {
                write!(f, "SESSION FIXATION ERROR - INCOMPLETE TRACK IN SESSION")
            },
            Self::EmptyOrPartiallyWrittenReservedTrack => {
                write!(f, "EMPTY OR PARTIALLY WRITTEN RESERVED TRACK")
            },
            Self::NoMoreTrackReservationsAllowed => {
                write!(f, "NO MORE TRACK RESERVATIONS ALLOWED")
            },
            Self::RmzExtensionIsNotAllowed => write!(f, "RMZ EXTENSION IS NOT ALLOWED"),
            Self::NoMoreTestZoneExtensionsAreAllowed => {
                write!(f, "NO MORE TEST ZONE EXTENSIONS ARE ALLOWED")
            },
            Self::CdControlError => write!(f, "CD CONTROL ERROR"),
            Self::PowerCalibrationAreaAlmostFull => {
                write!(f, "POWER CALIBRATION AREA ALMOST FULL")
            },
            Self::PowerCalibrationAreaIsFull => write!(f, "POWER CALIBRATION AREA IS FULL"),
            Self::PowerCalibrationAreaError => write!(f, "POWER CALIBRATION AREA ERROR"),
            Self::ProgramMemoryAreaUpdateFailure => {
                write!(f, "PROGRAM MEMORY AREA UPDATE FAILURE")
            },
            Self::ProgramMemoryAreaIsFull => write!(f, "PROGRAM MEMORY AREA IS FULL"),
            Self::RmaOrPmaIsAlmostFull => write!(f, "RMA/PMA IS ALMOST FULL"),
            Self::CurrentPowerCalibrationAreaAlmostFull => {
                write!(f, "CURRENT POWER CALIBRATION AREA ALMOST FULL")
            },
            Self::CurrentPowerCalibrationAreaIsFull => {
                write!(f, "CURRENT POWER CALIBRATION AREA IS FULL")
            },
            Self::RdzIsFull => write!(f, "RDZ IS FULL"),
            Self::SecurityError => write!(f, "SECURITY ERROR"),
            Self::UnableToDecryptData => write!(f, "UNABLE TO DECRYPT DATA"),
            Self::UnencryptedDataEncounteredWhileDecrypting => {
                write!(f, "UNENCRYPTED DATA ENCOUNTERED WHILE DECRYPTING")
            },
            Self::IncorrectDataEncryptionKey => write!(f, "INCORRECT DATA ENCRYPTION KEY"),
            Self::CryptographicIntegrityValidationFailed => {
                write!(f, "CRYPTOGRAPHIC INTEGRITY VALIDATION FAILED")
            },
            Self::ErrorDecryptingData => write!(f, "ERROR DECRYPTING DATA"),
            Self::UnknownSignatureVerificationKey => {
                write!(f, "UNKNOWN SIGNATURE VERIFICATION KEY")
            },
            Self::EncryptionParametersNotUseable => {
                write!(f, "ENCRYPTION PARAMETERS NOT USEABLE")
            },
            Self::DigitalSignatureValidationFailure => {
                write!(f, "DIGITAL SIGNATURE VALIDATION FAILURE")
            },
            Self::EncryptionModeMismatchOnRead => write!(f, "ENCRYPTION MODE MISMATCH ON READ"),
            Self::EncryptedBlockNotRawReadEnabled => {
                write!(f, "ENCRYPTED BLOCK NOT RAW READ ENABLED")
            },
            Self::IncorrectEncryptionParameters => write!(f, "INCORRECT ENCRYPTION PARAMETERS"),
            Self::UnableToDecryptParameterList => write!(f, "UNABLE TO DECRYPT PARAMETER LIST"),
            Self::EncryptionAlgorithmDisabled => write!(f, "ENCRYPTION ALGORITHM DISABLED"),
            Self::SaCreationParameterValueInvalid => {
                write!(f, "SA CREATION PARAMETER VALUE INVALID")
            },
            Self::SaCreationParameterValueRejected => {
                write!(f, "SA CREATION PARAMETER VALUE REJECTED")
            },
            Self::InvalidSaUsage => write!(f, "INVALID SA USAGE"),
            Self::DataEncryptionConfigurationPrevented => {
                write!(f, "DATA ENCRYPTION CONFIGURATION PREVENTED")
            },
            Self::SaCreationParameterNotSupported => {
                write!(f, "SA CREATION PARAMETER NOT SUPPORTED")
            },
            Self::AuthenticationFailed => write!(f, "AUTHENTICATION FAILED"),
            Self::ExternalDataEncryptionKeyManagerAccessError => {
                write!(f, "EXTERNAL DATA ENCRYPTION KEY MANAGER ACCESS ERROR")
            },
            Self::ExternalDataEncryptionKeyManagerError => {
                write!(f, "EXTERNAL DATA ENCRYPTION KEY MANAGER ERROR")
            },
            Self::ExternalDataEncryptionKeyNotFound => {
                write!(f, "EXTERNAL DATA ENCRYPTION KEY NOT FOUND")
            },
            Self::ExternalDataEncryptionRequestNotAuthorized => {
                write!(f, "EXTERNAL DATA ENCRYPTION REQUEST NOT AUTHORIZED")
            },
            Self::ExternalDataEncryptionControlTimeout => {
                write!(f, "EXTERNAL DATA ENCRYPTION CONTROL TIMEOUT")
            },
            Self::ExternalDataEncryptionControlError => {
                write!(f, "EXTERNAL DATA ENCRYPTION CONTROL ERROR")
            },
            Self::LogicalUnitAccessNotAuthorized => {
                write!(f, "LOGICAL UNIT ACCESS NOT AUTHORIZED")
            },
            Self::SecurityConflictInTranslatedDevice => {
                write!(f, "SECURITY CONFLICT IN TRANSLATED DEVICE")
            },
            Self::DiagnosticFailureOnComponent(nn) => {
                write!(f, "DIAGNOSTIC FAILURE ON COMPONENT {nn:02X}h")
            },
            Self::TaggedOverlappedCommands(nn) => {
                write!(f, "TAGGED OVERLAPPED COMMANDS ({nn:02X}h = TASK TAG)")
            },
            Self::DecompressionExceptionShortAlgorithmId(nn) => {
                write!(f, "DECOMPRESSION EXCEPTION SHORT ALGORITHM ID OF {nn:02X}h")
            },
            Self::Obsolete(asc, ascq) => write!(f, "OBSOLETE (ASC 0x{asc:02X}, ASCQ 0x{ascq:02X})"),
            Self::UnassignedDeviceTypeCode(asc, ascq) => write!(
                f,
                "UNASSIGNED DEVICE TYPE CODE (ASC 0x{asc:02X}, ASCQ 0x{ascq:02X})"
            ),
            Self::VendorSpecific(asc, ascq) => {
                write!(f, "VENDOR SPECIFIC (ASC 0x{asc:02X}, ASCQ 0x{ascq:02X})")
            },
            Self::VendorSpecificQualification(asc, ascq) => write!(
                f,
                "VENDOR SPECIFIC QUALIFICATION OF STANDARD ASC (ASC 0x{asc:02X}, ASCQ \
                 0x{ascq:02X})"
            ),
            Self::Reserved(asc, ascq) => write!(f, "RESERVED (ASC 0x{asc:02X}, ASCQ 0x{ascq:02X})"),
        }
    }
}
