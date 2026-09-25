//! VUC error codes.

/// VUC error code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ErrorCode {
    /// Unknown.
    Unknown(u32),
    /// SMART Command Transport invalid function code.
    SctInvalidFunctionCode = 0x1,
    /// SMART Command Transport invalid LBA.
    SctInvalidLba = 0x2,
    /// SMART Command Transport request sector count overflow.
    SctRequestSectorCountOverflow = 0x3,
    /// SMART Command Transport invalid error recovery function code.
    SctInvalidErrorRecoveryFunctionCode = 0x4,
    /// SMART Command Transport invalid error recovery select code.
    SctInvalidErrorRecoverySelectCode = 0x5,
    /// SMART Command Transport host read timer less than minimum.
    SctHostReadTimerLessThanMinimum = 0x6,
    /// SMART Command Transport host write timer less than minimum.
    SctHostWriteTimerLessThanMinimum = 0x7,
    /// SMART Command Transport background command abort by INT host command.
    SctBackgroundCommandAbortByIntHost = 0x8,
    /// SMART Command Transport background command terminated unrecoverable
    /// servo.
    SctBackgroundTerminatedUnrecoverableServo = 0x9,
    /// SMART Command Transport invalid function code in long sector access
    /// command.
    SctInvalidFunctionCodeInLongSectorAccess = 0xA,
    /// SMART Command Transport data transfer command issued without a key
    /// sector command.
    SctDataTransferWithoutKeySector = 0xB,
    /// SMART Command Transport invalid function code in feature control
    /// command.
    SctInvalidFunctionCodeInFeatureControl = 0xC,
    /// SMART Command Transport invalid feature code in feature control command.
    SctInvalidFeatureCodeInFeatureControl = 0xD,
    /// SMART Command Transport invalid new state feature control command.
    SctInvalidNewStateFeatureControlCommand = 0xE,
    /// SMART Command Transport invalid option flags value in feature control
    /// command.
    SctInvalidOptionFlagsInFeatureControl = 0xF,
    /// SMART Command Transport invalid action code.
    SctInvalidActionCode = 0x10,
    /// SMART Command Transport invalid table ID.
    SctInvalidTableId = 0x11,
    /// SMART Command Transport command abort drive security lock.
    SctCommandAbortDriveSecurityLock = 0x12,
    /// SMART Command Transport invalid revision code.
    SctInvalidRevisionCode = 0x13,
    /// SMART Command Transport foreground command terminated unrecoverable.
    SctForegroundTerminatedUnrecoverable = 0x14,
    /// SMART Command Transport Time-Limited Error Recovery timeout status.
    SctTlerTimeoutStatus = 0x15,
    /// Controller FW request command abort.
    CtlrFwRequestCommandAbort = 0x801,
    /// Illegal drive model.
    IllegalDriveModel = 0x802,
    /// Illegal parameter 0.
    IllegalParameter0 = 0x803,
    /// Illegal parameter 1.
    IllegalParameter1 = 0x804,
    /// Illegal parameter 2.
    IllegalParameter2 = 0x805,
    /// Illegal parameter 3.
    IllegalParameter3 = 0x806,
    /// Illegal command.
    IllegalCommand = 0x807,
    /// Fatal servo.
    FatalServo = 0x808,
    /// Timeout.
    Timeout = 0x809,
    /// Index not found.
    IndexNotFound = 0x80A,
    /// Sector count mismatch.
    SectorCountMismatch = 0x80B,
    /// SAM.
    Sam = 0x80C,
    /// Unable to spin-up.
    UnableToSpinUp = 0x80D,
    /// Unable to spin-down.
    UnableToSpinDown = 0x80E,
    /// Unable to speed.
    UnableToSpeed = 0x80F,
    /// Burst sync fail.
    BurstSyncFail = 0x810,
    /// Unlatch fail.
    UnlatchFail = 0x811,
    /// CTR PD seek fail.
    CtrPdSeekFail = 0x812,
    /// Burst slope calibration fail.
    BurstSlopeCalibrationFail = 0x813,
    /// Flex bias calibration fail.
    FlexBiasCalibrationFail = 0x814,
    /// Motor torque calibration fail.
    MotorTorqueCalibrationFail = 0x815,
    /// Repeatable Run Out calibration fail.
    RroCalibrationFail = 0x816,
    /// Far gain calibration fail.
    FarGainCalibrationFail = 0x817,
    /// Track 0 seek fail.
    Track0SeekFail = 0x818,
    /// RC servo gain calibration fail.
    RcServoGainCalibrationFail = 0x819,
    /// RC Lock to Reference Open Loop calibration fail.
    RcLtrOlCalibrationFail = 0x81A,
    /// Snapshot gain calibration fail.
    SnapshotGainCalibrationFail = 0x81B,
    /// Tangential head offset calibration fail.
    TangentialHeadOffsetCalibrationFail = 0x81C,
    /// Bandwidth calibration fail.
    BandwidthCalibrationFail = 0x81D,
    /// Repeatable Run Out learning overflow.
    RroLearningOverflow = 0x81E,
    /// Repeatable Run Out learning.
    RroLearning = 0x81F,
    /// Servo Interface Returned unknown B6.
    SvirUnknownB6 = 0x8B6,
    /// Servo Interface Returned unknown D2.
    SvirUnknownD2 = 0x8D2,
    /// Servo Interface Returned unknown D4.
    SvirUnknownD4 = 0x8D4,
    /// Drive Protect disk error, Drive Protect Overlay.
    DriveProtectDiskOverlay = 0x1004,
    /// Drive Protect disk error, Drive Protect 1.
    DriveProtectDisk1 = 0x1005,
    /// Drive Protect disk config sector.
    DriveProtectDiskConfigSector = 0x1007,
    /// Drive Protect command invalid CRC checksum.
    DriveProtectCommandInvalidCrcChecksum = 0x2024,
    /// Drive Protect command invalid opcode.
    DriveProtectCommandInvalidOpcode = 0x2025,
    /// Drive Protect command keys already loaded.
    DriveProtectCommandKeysAlreadyLoaded = 0x2026,
    /// Drive Protect command load zero key.
    DriveProtectCommandLoadZeroKey = 0x2027,
    /// Drive Protect command invalid conditions.
    DriveProtectCommandInvalidConditions = 0x2028,
    /// Drive Protect command invalid sub opcode.
    DriveProtectCommandInvalidSubOpcode = 0x2029,
    /// Drive Protect command invalid customer config.
    DriveProtectCommandInvalidCustomerConfig = 0x202A,
    /// Drive Protect drive is unlocked.
    DriveProtectDriveIsUnlocked = 0x202B,
    /// Drive Protect drive is locked.
    DriveProtectDriveIsLocked = 0x202C,
    /// Drive Protect encryption keys not loaded.
    DriveProtectEncryptionKeysNotLoaded = 0x202D,
    /// Drive Protect data should not be encrypted.
    DriveProtectDataShouldNotBeEncrypted = 0x202E,
    /// Drive Protect data should be encrypted.
    DriveProtectDataShouldBeEncrypted = 0x202F,
    /// Drive Protect command set zero password.
    DriveProtectCommandSetZeroPassword = 0x2030,
    /// Drive Protect lock prepare not set.
    DriveProtectLockPrepareNotSet = 0x2031,
    /// Drive Protect change key in lock countdown.
    DriveProtectChangeKeyInLockCountdown = 0x2032,
    /// Drive Protect mismatch Host Random Number.
    DriveProtectMismatchHrn = 0x2033,
    /// Drive Protect mismatch Drive Random Number.
    DriveProtectMismatchDrn = 0x2034,
    /// Drive Protect mismatch password.
    DriveProtectMismatchPassword = 0x2035,
    /// Drive Protect no Host Random Number or Drive Random Number was issued.
    DriveProtectNoHrnOrDrnWasIssued = 0x2036,
    /// Drive Protect no password set in drive.
    DriveProtectNoPasswordSetInDrive = 0x2037,
    /// Drive Protect set password in lock countdown.
    DriveProtectSetPasswordInLockCountdown = 0x2038,
    /// Drive Protect lock fail update config or flush.
    DriveProtectLockFailUpdateConfigOrFlush = 0x2039,
    /// VSC mode disabled.
    VscModeDisabled = 0x3016,
    /// Config write failed.
    ConfigWriteFailed = 0x3037,
    /// Clear Drive Reliability Monitor section failed.
    ClearDrmSectionFailed = 0x3039,
    /// Set config LBA failed.
    SetConfigLbaFailed = 0x303A,
    /// Format set failed.
    FormatSetFailed = 0x303B,
    /// Error rate table not present.
    ErrorRateTableNotPresent = 0x303D,
    /// Intelligent Burn In mode bit not set.
    IbiModeBitNotSet = 0x303E,
    /// Permanent overlay already loaded.
    PermanentOverlayAlreadyLoaded = 0x3041,
    /// Permanent overlay not loaded.
    PermanentOverlayNotLoaded = 0x3042,
    /// Invalid Drive Reliability Monitor group section.
    InvalidDrmGroupSection = 0x3044,
    /// Invalid Drive Reliability Monitor group Q-subsection.
    InvalidDrmGroupQSubsection = 0x3045,
    /// Drive Reliability Monitor group queue empty.
    DrmGroupQueueEmpty = 0x3046,
    /// Invalid Drive Reliability Monitor group L-subsection.
    InvalidDrmGroupLSubsection = 0x3047,
    /// Invalid Drive Reliability Monitor subsection.
    InvalidDrmSubsection = 0x3048,
    /// Invalid life queue flag.
    InvalidLifeQueueFlag = 0x3049,
    /// Protocol violation.
    ProtocolViolation = 0x304A,
    /// Depop disabled.
    DepopDisabled = 0x304B,
    /// Depop invalid AP Block.
    DepopInvalidApb = 0x304C,
    /// Last background VSC command failed; see secondary error code for the
    /// extended error.
    LastBackgroundVscCommandFailed = 0x3059,
    /// ODTAA not initialized.
    OdtaaNotInitialized = 0x3208,
    /// Track list data not available.
    TrackListDataNotAvailable = 0x3209,
    /// ODTAA read watchdog.
    OdtaaReadWatchdog = 0x320C,
    /// Repeatable Run Out not initialized.
    RroNotInitialized = 0x320E,
    /// Drive Reliability Monitor spin counters read fail.
    DrmSpinCountersReadFail = 0x3219,
    /// Drive Reliability Monitor actuator counters read fail.
    DrmActuatorCountersReadFail = 0x321A,
    /// Static file invalid.
    StaticFileInvalid = 0x321B,
    /// MRM request failed.
    MrmRequestFailed = 0x321C,
    /// MRM request timed out.
    MrmRequestTimedOut = 0x321D,
    /// MRM request rejected.
    MrmRequestRejected = 0x321E,
    /// MRM request canceled.
    MrmRequestCanceled = 0x321F,
    /// MRM start canceled.
    MrmStartCanceled = 0x3220,
    /// MRM wait canceled.
    MrmWaitCanceled = 0x3221,
    /// Resource allocation failed.
    ResourceAllocationFailed = 0x3222,
    /// Drive Reliability Monitor log not loaded.
    DrmLogNotLoaded = 0x3223,
    /// Dynamic Fly Height calibration failed.
    DfhCalibrationFailed = 0x3224,
    /// Drive Reliability Monitor log sections bad checksum.
    DrmLogSectionsBadChecksum = 0x3225,
    /// VSC Data Lifeguard 2 not active.
    VscDlg2NotActive = 0x3226,
    /// Drive Reliability Monitor log period log bad checksum.
    DrmPeriodLogBadChecksum = 0x3232,
    /// SMART read self-test log.
    SmartReadSelfTestLog = 0x3300,
    /// SMART Drive Reliability Monitor log has not been loaded.
    SmartDrmLogHasNotBeenLoaded = 0x3301,
    /// SMART Drive Reliability Monitor load disabled.
    SmartDrmLoadDisabled = 0x3307,
    /// SMART log page A3 not loaded.
    SmartLogPageA3NotLoaded = 0x3308,
    /// Cache flush cached relocation got disk.
    CacheFlushCachedRelocationDisk = 0x3502,
    /// Cache cannot invalidate Segment Descriptor in dynamic state.
    CacheCannotInvalidateSdInDynamicState = 0x3503,
    /// Resource allocation sectors beyond range.
    ResourceAllocationSectorsBeyondRange = 0x3600,
    /// Resource allocation got disk.
    ResourceAllocationDisk = 0x3602,
    /// Resource allocation no contiguous buffer.
    ResourceAllocationNoContiguousBuffer = 0x3603,
    /// Resource allocate transient with cache valid.
    ResourceAllocateTransientWithCacheValid = 0x3604,
    /// Resource allocate transient buffer used.
    ResourceAllocateTransientBufferUsed = 0x3605,
    /// Resource allocate Segment Descriptor, none available.
    ResourceAllocateSdNoneAvailable = 0x3606,
    /// Resource allocation no buffers available.
    ResourceAllocationNoBuffersAvailable = 0x3607,
    /// Resource allocation got disk error Debug Stop.
    ResourceAllocationDiskDbs = 0x3608,
    /// Resource allocate Transfer Descriptor, none available.
    ResourceAllocateTdNoneAvailable = 0x3609,
    /// File Manager file info.
    FmFileInfo = 0x3700,
    /// File Manager directory.
    FmDirectory = 0x3701,
    /// File Manager file ID.
    FmFileId = 0x3702,
    /// File Manager checksum.
    FmChecksum = 0x3703,
    /// File Manager compatibility.
    FmCompatibility = 0x3704,
    /// File Manager timeout.
    FmTimeout = 0x3705,
    /// File Manager not static file.
    FmNotStaticFile = 0x3706,
    /// File Manager no buffer.
    FmNoBuffer = 0x3707,
    /// File Manager drive not ready.
    FmDriveNotReady = 0x3708,
    /// File Manager file header size zero.
    FmFileHeaderSizeZero = 0x3709,
    /// File Manager incompatible version.
    FmIncompatibleVersion = 0x370A,
    /// File Manager not flash file.
    FmNotFlashFile = 0x370C,
    /// File Manager cannot rename to existing file ID.
    FmCannotRenameToExistingFileId = 0x370D,
    /// File Manager files good but checksums different.
    FmFilesGoodButChecksumsDifferent = 0x370E,
    /// File Manager header II signature invalid.
    FmHeaderIiSignatureInvalid = 0x370F,
    /// File Manager header II too small.
    FmHeaderIiTooSmall = 0x3710,
    /// File Manager partial file request invalid.
    FmPartialFileRequestInvalid = 0x3711,
    /// File Manager sector count exceeds maximum allocated buffer.
    FmSectorCountExceedsMaxAllocatedBuffer = 0x3712,
    /// File Manager file size exceeds static buffer size.
    FmFileSizeExceedsStaticBuffer = 0x3713,
    /// File Manager partial file buffer offset exceeds EOF.
    FmPartialFileBufferOffsetExceedsEof = 0x3714,
    /// File Manager partial file sector count exceeds EOF.
    FmPartialFileSectorCountExceedsEof = 0x3715,
    /// File Manager no Segment Descriptor created for the requested file ID.
    FmNoSdCreatedForTheRequestedFileId = 0x3716,
    /// File Manager no more space in directory.
    FmNoMoreSpaceInDirectory = 0x3717,
    /// File Manager number copy more than maximum copy.
    FmNumberCopyMoreThanMaximum = 0x3718,
    /// File Manager no more space in region.
    FmNoMoreSpaceInRegion = 0x3719,
    /// File Manager file cannot create on existing file.
    FmFileCannotCreateOnExistingFile = 0x3720,
    /// File Manager file entry number not found in directory sector.
    FmFileEntryNotFoundInDirectorySector = 0x3721,
    /// File Manager trying to copy to same region.
    FmTryingToCopyToSameRegion = 0x3722,
    /// File Manager file initialization placeholder bit not set.
    FmFileInitializationPlaceholderBitNotSet = 0x3723,
    /// File Manager file target RLBA overlap.
    FmFileTargetRlbaOverlap = 0x3724,
    /// File Manager no contiguous space in region.
    FmNoContiguousSpaceInRegion = 0x3725,
    /// File Manager cannot resolve overlap.
    FmCannotResolveOverlap = 0x3726,
    /// File Manager cannot free enough space.
    FmCannotFreeEnoughSpace = 0x3727,
    /// File Manager cannot create contiguous space in region.
    FmCannotCreateContiguousSpaceInRegion = 0x3728,
    /// File Manager request RLBA exceed region boundary.
    FmRequestRlbaExceedRegionBoundary = 0x3729,
    /// File Manager directory entry not same.
    FmDirectoryEntryNotSame = 0x3730,
    /// File Manager defrag detect packet file in reserved.
    FmDefragDetectPacketFileInReserved = 0x3731,
    /// File Manager gather field file.
    FmGatherFieldFile = 0x3732,
    /// File Manager sort directory sector.
    FmSortDirectorySector = 0x3733,
    /// File Manager defrag reserved.
    FmDefragReserved = 0x3734,
    /// File Manager directory error while delete.
    FmDirectoryWhileDelete = 0x3737,
    /// File Manager file not found in any directory.
    FmFileNotFoundInAnyDirectory3742 = 0x3742,
    /// File Manager file not found in any directory.
    FmFileNotFoundInAnyDirectory3743 = 0x3743,
    /// File Manager file not found in any directory.
    FmFileNotFoundInAnyDirectory3746 = 0x3746,
    /// File Manager file 6F structure incorrect.
    FmFile6FStructureIncorrect = 0x3753,
    /// Background Process Self-Test aborted by reset.
    BackgroundPstAbortedByReset = 0x3801,
    /// Background Process Self-Test aborted by deadman.
    BackgroundPstAbortedByDeadman = 0x3802,
    /// Background invalid ASCAN config parameters.
    BackgroundInvalidAscanConfigParameters = 0x3803,
    /// Background track list data not available.
    BackgroundTrackListDataNotAvailable = 0x3804,
    /// Background Process Self-Test unable to load Process Test Module.
    BackgroundPstUnableToLoadPtm = 0x3805,
    /// Background Process Self-Test resource allocation failed.
    BackgroundPstResourceAllocationFailed = 0x3806,
    /// Background Process Self-Test disabled via Debug Stop.
    BackgroundPstDisabledViaDbs = 0x3807,
    /// Background processing disabled.
    BackgroundProcessingDisabled = 0x3809,
    /// Background Process Self-Test invalid Process Test Module load address.
    BackgroundPstInvalidPtmLoadAddress = 0x380A,
    /// Background Process Self-Test invalid Process Test Module start address.
    BackgroundPstInvalidPtmStartAddress = 0x380B,
    /// Background non-captive memory test not allowed.
    BackgroundNonCaptiveMemoryTestNotAllowed = 0x380C,
    /// Background Process Self-Test unable to flash Process Test Module.
    BackgroundPstUnableToFlashPtm = 0x380D,
    /// Background self-test aborted timed out.
    BackgroundSelfTestAbortedTimedOut = 0x380E,
    /// Overlay Manager permanent already loaded.
    OvmPermanentAlreadyLoaded = 0x3900,
    /// Overlay Manager transient already loaded.
    OvmTransientAlreadyLoaded = 0x3901,
    /// Overlay Manager permanent not loaded.
    OvmPermanentNotLoaded = 0x3902,
    /// Overlay Manager transient not loaded.
    OvmTransientNotLoaded = 0x3903,
    /// Overlay Manager not compatible.
    OvmNotCompatible = 0x3904,
    /// Overlay Manager checksum.
    OvmChecksum = 0x3905,
    /// Overlay Manager undefined function.
    OvmUndefinedFunction = 0x3906,
    /// Overlay Manager build ID mismatch.
    OvmBuildIdMismatch = 0x3907,
    /// Flash unknown.
    FlashUnknown = 0x3A00,
    /// Flash invalid flash sector address.
    FlashInvalidSectorAddress = 0x3A01,
    /// Flash write latch enable.
    FlashWriteLatchEnable = 0x3A02,
    /// Flash write page send byte.
    FlashWritePageSendByte = 0x3A03,
    /// Flash write page timeout.
    FlashWritePageTimeout = 0x3A04,
    /// Flash write page to Static Memory.
    FlashWritePageToStaticMemory = 0x3A05,
    /// Flash read block get byte.
    FlashReadBlockGetByte = 0x3A06,
    /// Flash byte count exceeds device limit.
    FlashByteCountExceedsDeviceLimit = 0x3A07,
    /// Flash invalid flash address.
    FlashInvalidAddress = 0x3A08,
    /// Flash initial boot header missing.
    FlashInitialBootHeaderMissing = 0x3A09,
    /// Flash send byte timeout.
    FlashSendByteTimeout = 0x3A0A,
    /// Flash read command.
    FlashReadCommand = 0x3A0B,
    /// Flash invalid flash data.
    FlashInvalidData = 0x3A0C,
    /// Flash data compare.
    FlashDataCompare = 0x3A0D,
    /// Flash device ID.
    FlashDeviceId = 0x3A0E,
    /// Flash read info start timeout.
    FlashReadInfoStartTimeout = 0x3A0F,
    /// Flash status timeout.
    FlashStatusTimeout = 0x3A10,
    /// Flash command timeout.
    FlashCommandTimeout = 0x3A11,
    /// Download Process Test Module malloc failure.
    DptmMallocFailure = 0x3A20,
    /// Download Process Test Module free failure.
    DptmFreeFailure = 0x3A21,
    /// Download Process Test Module initialization failure.
    DptmInitializationFailure = 0x3A22,
    /// Download Process Test Module failed to process downloaded packet.
    DptmFailedToProcessDownloadedPacket = 0x3A23,
    /// Download Process Test Module failed to back up flash files.
    DptmFailedToBackUpFlashFiles = 0x3A24,
    /// Download Process Test Module preserved file has different version or
    /// size.
    DptmPreservedFileHasDiffVersionOrSize = 0x3A25,
    /// Download Process Test Module unhandled file list exception.
    DptmUnhandledFileListException = 0x3A26,
    /// Download Process Test Module file ID not in file list.
    DptmFileIdNotInList = 0x3A27,
    /// Download Process Test Module flash write buffer invalid.
    DptmFlashWriteBufferInvalid = 0x3A28,
    /// Download Process Test Module flash image too big.
    DptmFlashImageTooBig = 0x3A29,
    /// Download Process Test Module flash program failure.
    DptmFlashProgramFailure = 0x3A2A,
    /// Download Process Test Module failed to verify programmed flash.
    DptmFailedToVerifyProgrammedFlash = 0x3A2B,
    /// Download Process Test Module failed to apply mod bytes.
    DptmFailedToApplyModBytes = 0x3A2C,
    /// Download Process Test Module cleanup failure.
    DptmCleanupFailure = 0x3A2D,
    /// Download Process Test Module failed to write config.
    DptmFailedToWriteConfig = 0x3A2E,
    /// Download Process Test Module packet does not contain file list.
    DptmPacketDoesNotContainFileList = 0x3A2F,
    /// Download Process Test Module try to rename to a file that existed.
    DptmTryToRenameToAFileThatExisted = 0x3A30,
    /// Download Process Test Module exceeds maximum undo list.
    DptmExceedsMaximumUndoList = 0x3A31,
    /// Download Process Test Module replace original file not exist.
    DptmReplaceOriginalFileNotExist = 0x3A32,
    /// Download Process Test Module replace original file with different file
    /// ID.
    DptmReplaceOriginalFileDifferentFileId = 0x3A33,
    /// Download Process Test Module flash directory not found in flash file.
    DptmFlashDirectoryNotFoundInFlashFile = 0x3A34,
    /// Download Process Test Module AC55 UCCM total bytes expected mismatch.
    DptmAc55UccmTotalBytesExpectedMismatch = 0x3A35,
    /// Download Process Test Module AC55 new file 118 not identical to old one.
    DptmAc55NewFile118NotIdenticalToOldOne = 0x3A36,
    /// Download Process Test Module AC55 input no key sectors defined.
    DptmAc55InputNoKeySectorsDefined = 0x3A37,
    /// Download Process Test Module AC55 input invalid action code.
    DptmAc55InputInvalidActionCode = 0x3A38,
    /// Download Process Test Module AC55 input invalid function code request.
    DptmAc55InputInvalidFunctionCodeRequest = 0x3A39,
    /// Download Process Test Module AC55 input invalid config code.
    DptmAc55InputInvalidConfigCode = 0x3A3A,
    /// Download Process Test Module AC55 input cache family mismatch.
    DptmAc55InputCacheFamilyMismatch = 0x3A3B,
    /// Download Process Test Module AC55 major revision mismatch.
    DptmAc55MajorRevisionMismatch = 0x3A3C,
    /// Download Process Test Module AC55 FW structure revision mismatch.
    DptmAc55FwStructureRevisionMismatch = 0x3A3D,
    /// RSEEK malloc failure.
    RseekMallocFailure = 0x3A60,
    /// RSEEK free failure.
    RseekFreeFailure = 0x3A61,
    /// Self-test check resident file.
    SelfTestCheckResFile = 0x3B00,
    /// Self-test scan.
    SelfTestScan = 0x3B01,
    /// Self-test SRAM hard.
    SelfTestSramHard = 0x3B02,
    /// Self-test SRAM soft.
    SelfTestSramSoft = 0x3B03,
    /// Self-test SRAM multi-soft.
    SelfTestSramMultiSoft = 0x3B04,
    /// Self-test DRAM hard.
    SelfTestDramHard = 0x3B05,
    /// Self-test DRAM soft.
    SelfTestDramSoft = 0x3B06,
    /// Self-test DRAM multi-soft.
    SelfTestDramMultiSoft = 0x3B07,
    /// Self-test transient load fault.
    SelfTestTransientLoadFault = 0x3B08,
    /// Format P-list not found.
    FmtPlistNotFound = 0x3C00,
    /// Format invalid P-list.
    FmtInvalidPlist = 0x3C01,
    /// Format G-list not found.
    FmtGlistNotFound = 0x3C02,
    /// Format invalid G-list.
    FmtInvalidGlist = 0x3C03,
    /// Format exceeded push downs.
    FmtExceededPushDowns = 0x3C04,
    /// Format Push Down List write fail.
    FmtPushDownListWriteFail = 0x3C05,
    /// Format new bad tracks.
    FmtNewBadTracks = 0x3C06,
    /// Format failure.
    FmtFailure = 0x3C07,
    /// Format exceeded G-list.
    FmtExceededGlist = 0x3C08,
    /// Format G-list write fail.
    FmtGlistWriteFail = 0x3C09,
    /// Format capacity.
    FmtCapacity = 0x3C0A,
    /// Format Zone Segment Descriptor not loaded.
    FmtZsdNotLoaded = 0x3C0B,
    /// Format buffer allocation.
    FmtBufferAllocation = 0x3C0C,
    /// Format relocation list write fail.
    FmtRelocationListWriteFail = 0x3C0D,
    /// Format Push Down List not found.
    FmtPushDownListNotFound = 0x3C0E,
    /// Format relocation list not found.
    FmtRelocationListNotFound = 0x3C0F,
    /// Format exceeded relocation list.
    FmtExceededRelocationList = 0x3C10,
    /// Format write fail.
    FmtWriteFail = 0x3C11,
    /// Format exceeded Reserved Push Down List.
    FmtExceededReservedPushDownList = 0x3C12,
    /// Format Reserved Push Down List write fail.
    FmtReservedPushDownListWriteFail = 0x3C13,
    /// Format Reserved Push Down List not loaded.
    FmtReservedPushDownListNotLoaded = 0x3C14,
    /// Format P-list Physical Sector Number out of range.
    FmtPlistPsnOutOfRange = 0x3C15,
    /// Format push count overflow.
    FmtPushCountOverflow = 0x3C16,
    /// Format hash table overflow.
    FmtHashTableOverflow = 0x3C17,
    /// Format invalid zone table.
    FmtInvalidZoneTable = 0x3C18,
    /// Format cannot merge P and G list.
    FmtCannotMergePAndGList = 0x3C19,
    /// Format P-list write fail.
    FmtPlistWriteFail = 0x3C1A,
    /// Format slips exceed limit.
    FmtSlipsExceedLimit = 0x3C1B,
    /// Format track pushed down.
    FmtTrackPushedDown = 0x3C1C,
    /// Format field list write fail.
    FmtFieldListWriteFail = 0x3C1D,
    /// Format P-list cylinder out of range.
    FmtPlistCylinderOutOfRange = 0x3C1E,
    /// Format P-list head out of range.
    FmtPlistHeadOutOfRange = 0x3C1F,
    /// Format remerge required.
    FmtRemergeRequired = 0x3C20,
    /// Format M-list write fail.
    FmtMlistWriteFail = 0x3C21,
    /// Defect List defect found.
    DflDefectFound = 0x3D80,
    /// Defect List no defect.
    DflNoDefect = 0x3D81,
    /// Defect List track defect found.
    DflTrackDefectFound = 0x3D82,
    /// Defect List empty.
    DflEmpty = 0x3D83,
    /// Defect List no memory.
    DflNoMemory = 0x3D84,
    /// Defect List P-list write.
    DflPlistWrite = 0x3D85,
    /// Defect List G-list write.
    DflGlistWrite = 0x3D86,
    /// Defect List push down list write.
    DflPushDownListWrite = 0x3D87,
    /// Defect List relocation list write.
    DflRelocationListWrite = 0x3D89,
    /// Defect List G-list full.
    DflGlistFull = 0x3D8A,
    /// Defect List P-list full.
    DflPlistFull = 0x3D8B,
    /// Defect List C-list full.
    DflClistFull = 0x3D8C,
    /// Defect List P-list defect.
    DflPlistDefect = 0x3D8D,
    /// Defect List G-list defect.
    DflGlistDefect = 0x3D8E,
    /// Defect List no list.
    DflNoList = 0x3D8F,
    /// Defect List DEF1 less than.
    DflDef1LessThan = 0x3D90,
    /// Defect List DEF1 greater than.
    DflDef1GreaterThan = 0x3D91,
    /// Defect List DEF1 equal.
    DflDef1Equal = 0x3D92,
    /// Defect List invalid LBA.
    DflInvalidLba = 0x3D93,
    /// Defect List invalid LBA range.
    DflInvalidLbaRange = 0x3D94,
    /// Defect List duplicate defect.
    DflDuplicateDefect = 0x3D95,
    /// Defect List D-list full.
    DflDlistFull = 0x3D96,
    /// Defect List M-list full.
    DflMlistFull = 0x3D97,
    /// Defect List VFS buffer conversion.
    DflVfsBufferConversion = 0x3D98,
    /// Cache relocation spare RW timeout.
    CacheRelocationSpareRwTimeout = 0x3E00,
    /// Cache relocation insufficient cache space.
    CacheRelocationInsufficientSpace = 0x3E01,
    /// Cache relocation read relocation does not exist.
    CacheRelocationReadDoesNotExist = 0x3E02,
    /// Cache relocation write relocation does not exist.
    CacheRelocationWriteDoesNotExist = 0x3E03,
    /// Cache relocation read new relocation failure.
    CacheRelocationReadNewFailure = 0x3E04,
    /// Cache relocation load track cache failure.
    CacheRelocationLoadTrackFailure = 0x3E05,
    /// Cache relocation RW in progress.
    CacheRelocationRwInProgress = 0x3E06,
    /// Defect Manager translation out of range.
    DmTranslationOutOfRange = 0x3F00,
    /// Defect Manager sector out of range.
    DmSectorOutOfRange = 0x3F01,
    /// Defect Manager RM not a spare relocation.
    DmRmNotASpareRelocation = 0x3F02,
    /// Defect Manager RM not a spare LBA.
    DmRmNotASpareLba = 0x3F03,
    /// Defect Manager RM not a relocation list entry.
    DmRmNotARelocationListEntry = 0x3F04,
    /// Defect Manager RM not a user LBA.
    DmRmNotAUserLba = 0x3F05,
    /// Defect Manager RM inserting in full relocation list.
    DmRmInsertingInFullRelocationList = 0x3F06,
    /// Defect Manager RM inserting an existing LBA.
    DmRmInsertingAnExistingLba = 0x3F07,
    /// Defect Manager head check wrong cylinder for reserved area.
    DmHeadCheckWrongCylinderForReservedArea = 0x3F08,
    /// Defect Manager head check wrong head for user area.
    DmHeadCheckWrongHeadForUserArea = 0x3F09,
    /// Defect Manager RM no more spares for cache relocation.
    DmRmNoMoreSparesForCacheRelocation = 0x3F0A,
    /// Defect Manager RM inserting WU in full relocation list.
    DmRmInsertingWuInFullRelocationList = 0x3F0B,
    /// Format reserved area Reserved Push Down List overflow.
    FmtReservedAreaPushDownListOverflow = 0x4080,
    /// Format reserved area P-list Physical Sector Number out of range.
    FmtReservedAreaPlistPsnOutOfRange = 0x4081,
    /// Format reserved area P-list file ID invalid.
    FmtReservedAreaPlistFileIdInvalid = 0x4082,
    /// Format reserved area hash block empty.
    FmtReservedAreaHashBlockEmpty = 0x4083,
    /// Format reserved area head count zero.
    FmtReservedAreaHeadCountZero = 0x4084,
    /// Format reserved area zone table Sectors Per Track zero.
    FmtReservedAreaZoneTableSptZero = 0x4085,
    /// Format reserved area no spares available.
    FmtReservedAreaNoSparesAvailable = 0x4086,
    /// Format reserved area region crosses head boundary.
    FmtReservedAreaRegionCrossesHeadBoundary = 0x4087,
    /// Format reserved area region too large.
    FmtReservedAreaRegionTooLarge = 0x4088,
    /// Format reserved area two region on same virtual head.
    FmtReservedAreaTwoRegionOnSameVirtualHead = 0x4089,
    /// Format reserved area incorrect RASP control parameter.
    FmtReservedAreaIncorrectRaspControl = 0x408A,
    /// Format reserved area invalid files encountered.
    FmtReservedAreaInvalidFilesEncountered = 0x408B,
    /// Format reserved area alt Reserved Push Down List size mismatch.
    FmtReservedAreaAltRpdListSizeMismatch = 0x408C,
    /// Format reserved area invalid target size.
    FmtReservedAreaInvalidTargetSize = 0x408D,
    /// Format reserved area invalid RASP region table.
    FmtReservedAreaInvalidRaspRegionTable = 0x408E,
    /// Format reserved area invalid target table.
    FmtReservedAreaInvalidTargetTable = 0x408F,
    /// Format reserved area RASP in middle of surface.
    FmtReservedAreaRaspInMiddleOfSurface = 0x4091,
    /// Memory test data bus.
    MemoryTestDataBus = 0x4100,
    /// Memory test address bus.
    MemoryTestAddressBus = 0x4101,
    /// Memory test device bus.
    MemoryTestDeviceBus = 0x4102,
    /// Host download microcode bad COMP char.
    HostDlmcBadCompChar = 0x4506,
    /// Host download microcode bad checksum.
    HostDlmcBadChecksum = 0x4507,
    /// Host download microcode invalid packet.
    HostDlmcInvalidPacket = 0x4509,
    /// Host download microcode invalid PROD family.
    HostDlmcInvalidProdFamily = 0x450C,
    /// Host download microcode invalid tracks per inch code.
    HostDlmcInvalidTpiCode = 0x451E,
    /// Host download microcode invalid section offset.
    HostDlmcInvalidSectionOffset = 0x4520,
    /// Host download microcode invalid customer ID.
    HostDlmcInvalidCustomerId = 0x4524,
    /// Host download microcode total transferred too large.
    HostDlmcTransferTooLarge = 0x4525,
    /// Host download microcode failed saving drive state.
    HostDlmcSavingDriveState = 0x4526,
    /// Host download microcode transfer size out of range.
    HostDlmcSizeOutOfRange = 0x4527,
    /// Host download microcode invalid Process Test Module start address.
    HostDlmcInvalidPtmStartAddress = 0x4528,
    /// Host download microcode need module 19E and 19D.
    HostDlmcNeedModule19EAnd19D = 0x4535,
    /// Host latched fatal write fault.
    HostLatchedFatalWriteFault = 0x4580,
    /// Host resident files not loaded.
    HostResidentFilesNotLoaded = 0x4582,
    /// Host Debug Stop occurred.
    HostDbsOccurred = 0x4583,
    /// Host download microcode.
    HostDlmc = 0x4584,
    /// Host download microcode no Segment Descriptor.
    HostDlmcNoSd = 0x4585,
    /// Host cache overlay not loaded.
    HostCacheOverlayNotLoaded = 0x4587,
    /// Host download microcode no Process Test Module code.
    HostDlmcNoPtmCode = 0x4588,
    /// Host aborted command.
    HostAbortedCommand = 0x4612,
    /// Host transfer cancel.
    HostTransferCancel = 0x4630,
    /// Cache flush failure 1.
    CacheFlushFailure1 = 0x4700,
    /// Cache flush failure 2.
    CacheFlushFailure2 = 0x4701,
    /// Command timeout.
    CommandTimeout = 0x4900,
    /// Command timeout Audio Video Command Completion Time Out.
    CommandTimeoutAvCcto = 0x4901,
    /// Command timeout System Area streaming.
    CommandTimeoutSaStreaming = 0x4902,
    /// Command timeout Time-Limited Error Recovery.
    CommandTimeoutTler = 0x4903,
    /// Command timeout read Time-Limited Error Recovery.
    CommandTimeoutReadTler = 0x4904,
    /// Command timeout write Time-Limited Error Recovery.
    CommandTimeoutWriteTler = 0x4905,
    /// Command timeout flush Time-Limited Error Recovery.
    CommandTimeoutFlushTler = 0x4906,
    /// Command timeout read Time-Limited Error Recovery NetApp.
    CommandTimeoutReadTlerNetApp = 0x4907,
    /// Command timeout write Time-Limited Error Recovery NetApp.
    CommandTimeoutWriteTlerNetApp = 0x4908,
    /// Command timeout flush Time-Limited Error Recovery NetApp.
    CommandTimeoutFlushTlerNetApp = 0x4909,
    /// Data Lifeguard invalid test track LBA.
    DlgInvalidTestTrackLba = 0x4A01,
    /// Data Lifeguard no warehouse tracks exist.
    DlgNoWarehouseTracksExist = 0x4A02,
    /// Data Lifeguard not enough buffer for checkpoint recovery.
    DlgNotEnoughBufferForCheckpointRecovery = 0x4A03,
    /// Data Lifeguard no valid header found.
    DlgNoValidHeaderFound = 0x4A04,
    /// Data Lifeguard invalid check point found.
    DlgInvalidCheckPointFound = 0x4A05,
    /// Data Lifeguard checkpoint header read.
    DlgCheckpointHeaderRead = 0x4A06,
    /// Data Lifeguard checkpoint track read.
    DlgCheckpointTrackRead = 0x4A07,
    /// Data Lifeguard not enough resources allocation.
    DlgNotEnoughResourcesAllocation = 0x4A08,
    /// Data Lifeguard disk request timed out.
    DlgDiskRequestTimedOut = 0x4A09,
    /// Access denied, no access rights.
    AccessDenied = 0x4F10,
    /// Disk ECC corrected.
    DiskEccCorrected = 0x5101,
    /// Disk Thermal Asperity detect status.
    DiskTaDetectStatus = 0x5102,
    /// Disk Thermal Asperity 2nd sync mark.
    DiskTa2ndSyncMark = 0x5103,
    /// Disk unsafe 2nd sync mark.
    DiskUnsafe2ndSyncMark = 0x5104,
    /// Disk FIFO overrun or underrun.
    DiskFifoOverUnder = 0x5122,
    /// Disk FIFO overrun.
    DiskFifoOverrun = 0x5123,
    /// Disk FIFO underrun.
    DiskFifoUnderrun = 0x5124,
    /// Disk sector pulse Read Gate.
    DiskSectorPulseRg = 0x5125,
    /// Disk sector pulse Write Gate.
    DiskSectorPulseWg = 0x5126,
    /// Disk ECC data size.
    DiskEccDataSize = 0x5127,
    /// Disk Read Gate over servo.
    DiskRgOverServo = 0x5128,
    /// Disk Data Address Mark.
    DiskDam = 0x5181,
    /// Disk Data Address Mark Thermal Asperity.
    DiskDamTa = 0x5182,
    /// Disk sector pulse Read Gate error recovered.
    DiskSectorPulseRgRecovered = 0x5183,
    /// Disk ECC data size error recovered.
    DiskEccDataSizeRecovered = 0x5184,
    /// Disk Read Gate over servo error recovered.
    DiskRgOverServoRecovered = 0x5185,
    /// Disk SPBA.
    DiskSpba = 0x5191,
    /// Disk timeout sector not found.
    DiskTimeoutSectorNotFound = 0x5192,
    /// Disk CRC.
    DiskCrc = 0x51A1,
    /// Disk Error Correction Uncorrectable unsafe.
    DiskEcuUnsafe = 0x51A2,
    /// Disk Error Correction Uncorrectable unsafe Thermal Asperity.
    DiskEcuUnsafeTa = 0x51A3,
    /// Disk firmware ECC failure.
    DiskFwEccFailure51A4 = 0x51A4,
    /// Disk Error Correction Uncorrectable WU pseudo log.
    DiskEcuWuPseudoLog = 0x51A5,
    /// Disk Error Correction Uncorrectable WU pseudo not logged.
    DiskEcuWuPseudoNotLogged = 0x51A6,
    /// Disk Error Correction Uncorrectable WU flagged log.
    DiskEcuWuFlaggedLog = 0x51A7,
    /// Disk Error Correction Uncorrectable WU flagged not logged.
    DiskEcuWuFlaggedNotLogged = 0x51A8,
    /// Disk Error Correction Uncorrectable transfer hardware assist recovered.
    DiskEcuTransferHwAssistRecovered = 0x51A9,
    /// Disk write.
    DiskWrite = 0x51C1,
    /// Disk read CRC.
    DiskReadCrc = 0x51D0,
    /// Disk write CRC.
    DiskWriteCrc = 0x51D1,
    /// Disk logged CRC.
    DiskLoggedCrc = 0x51D2,
    /// Disk write RLL CRC.
    DiskWriteRllCrc = 0x51D3,
    /// Disk timeout Defect Manager not active.
    DiskTimeoutDmNotActive = 0x51E2,
    /// Disk timeout Defect Manager not active read.
    DiskTimeoutDmNotActiveRead = 0x51E3,
    /// Disk timeout Defect Manager not active write.
    DiskTimeoutDmNotActiveWrite = 0x51E4,
    /// Disk timeout buffer not ready.
    DiskTimeoutBufferNotReady = 0x51E5,
    /// Disk timeout DF.
    DiskTimeoutDf = 0x51E6,
    /// Disk general.
    DiskGeneral = 0x51E7,
    /// Disk timeout Time-Limited Error Recovery.
    DiskTimeoutTler = 0x51E8,
    /// Disk event timeout DF.
    DiskEventTimeoutDf = 0x51E9,
    /// Disk buffer full.
    DiskBufferFull = 0x51EA,
    /// Disk timeout seek not started.
    DiskTimeoutSeekNotStarted = 0x51EB,
    /// Disk firmware ECC failure.
    DiskFwEccFailure5204 = 0x5204,
    /// Disk recalibration failure.
    DiskRecalibrationFailure = 0x521A,
    /// Tone scan defect buffer overflow.
    ToneScanDefectBufferOverflow = 0x521C,
    /// Disk wedge command in progress.
    DiskWedgeCommandInProgress = 0x521D,
    /// Disk spin-up timeout 1.
    DiskSpinUpTimeout1 = 0x521E,
    /// Disk spin-up timeout 2.
    DiskSpinUpTimeout2 = 0x521F,
    /// Disk wedge invalid wedge count.
    DiskWedgeInvalidCount = 0x5220,
    /// Disk relocation permanent overlay not loaded.
    DiskRelocationPermanentOverlayNotLoaded = 0x5221,
    /// Relocation Selective Self-Test failed.
    RelocationSstFailed = 0x5230,
    /// Relocation not possible on spare.
    RelocationNotPossibleOnSpare = 0x5231,
    /// Relocation invalid request count.
    RelocationInvalidRequestCount = 0x5232,
    /// Relocation not possible on reserved.
    RelocationNotPossibleOnReserved = 0x5233,
    /// Disk stop on create relocation.
    DiskStopOnCreateRelocation = 0x5234,
    /// Disk stop on create Transparent Auto Relocation.
    DiskStopOnCreateTare = 0x5235,
    /// Relocation disabled.
    RelocationDisabled = 0x5236,
    /// Relocation disabled, format unit was not run.
    RelocationDisabledFormatUnitNotRun = 0x5237,
    /// Relocation in post read canceled.
    RelocationInPostReadCanceled = 0x5238,
    /// Relocation last LBA in user area.
    RelocationLastLbaInUserArea = 0x5239,
    /// Relocation disabled not initialized.
    RelocationDisabledNotInitialized = 0x523A,
    /// Relocation EC spare LBA with no user LBA.
    RelocationEcSpareLbaWithNoUserLba = 0x523B,
    /// PM illegal mode transition.
    PmIllegalModeTransition = 0x5240,
    /// PM transition not needed ok.
    PmTransitionNotNeededOk = 0x5241,
    /// Disk channel head size initialization.
    DiskChannelHeadSizeInitialization = 0x5251,
    /// Disk servo.
    DiskServo = 0x5380,
    /// Disk servo not ready.
    DiskServoNotReady = 0x5381,
    /// Disk servo head not found.
    DiskServoHeadNotFound = 0x5393,
    /// Disk servo spindle.
    DiskServoSpindle = 0x53A0,
    /// Disk servo spindle spin-up.
    DiskServoSpindleSpinUp = 0x53A1,
    /// Disk servo spindle spin-down.
    DiskServoSpindleSpinDown = 0x53A2,
    /// Disk servo spindle off speed.
    DiskServoSpindleOffSpeed = 0x53A3,
    /// Disk servo spindle spin-up fatal.
    DiskServoSpindleSpinUpFatal = 0x53A4,
    /// Disk servo spindle get spin-up time.
    DiskServoSpindleGetSpinUpTime = 0x53A5,
    /// Disk servo actuator.
    DiskServoActuator = 0x53C0,
    /// Disk servo actuator drive fault.
    DiskServoActuatorDriveFault = 0x53C1,
    /// Disk servo actuator abort.
    DiskServoActuatorAbort = 0x53C2,
    /// Disk servo actuator write inhibit.
    DiskServoActuatorWriteInhibit = 0x53C3,
    /// Disk servo actuator control fault.
    DiskServoActuatorControlFault = 0x53C4,
    /// Disk servo actuator shock fault.
    DiskServoActuatorShockFault = 0x53C5,
    /// Disk servo actuator write unsafe.
    DiskServoActuatorWriteUnsafe = 0x53C6,
    /// Disk servo actuator servo fault.
    DiskServoActuatorFault = 0x53C7,
    /// Disk servo actuator Write Gate mask.
    DiskServoActuatorWgMask = 0x53C8,
    /// Disk servo actuator fatal.
    DiskServoActuatorFatal = 0x53C9,
    /// Disk servo actuator timeout.
    DiskServoActuatorTimeout = 0x53CA,
    /// Disk servo actuator SAIL.
    DiskServoActuatorSail = 0x53CB,
    /// Disk servo actuator bad wedge.
    DiskServoActuatorBadWedge = 0x53CC,
    /// Disk servo actuator no Servo Channel target wedge.
    DiskServoActuatorNoScTargetWedge = 0x53CD,
    /// Disk servo actuator split EN timeout.
    DiskServoActuatorSplitEnTimeout = 0x53CE,
    /// Disk servo actuator Timing Base Generator unlock detect.
    DiskServoActuatorTbgUnlockDetect = 0x53CF,
    /// Disk servo write fault write unsafe.
    DiskServoWriteFaultUnsafe = 0x53D1,
    /// Disk servo write fault spindle at speed.
    DiskServoWriteFaultSpindleAtSpeed = 0x53D2,
    /// Disk servo write fault SSM timeout.
    DiskServoWriteFaultSsmTimeout = 0x53D3,
    /// Disk servo write fault illegal gray code.
    DiskServoWriteFaultIllegalGrayCode = 0x53D4,
    /// Disk servo write fault illegal cylinder.
    DiskServoWriteFaultIllegalCylinder = 0x53D5,
    /// Disk servo write fault off-track.
    DiskServoWriteFaultOffTrack = 0x53D6,
    /// Disk servo write fault fatal error path servo dead.
    DiskServoWriteFaultFatalPathServoDead = 0x53D7,
    /// Disk servo write fault read off-track.
    DiskServoWriteFaultReadOffTrack = 0x53D8,
    /// Disk servo write fault tone scan SSM timeout.
    DiskServoWriteFaultToneScanSsmTimeout = 0x53D9,
    /// Disk servo write fault low gray code quality.
    DiskServoWriteFaultLowGrayCodeQuality = 0x53DA,
    /// Disk servo write fault predictive off-track.
    DiskServoWriteFaultPredictiveOffTrack = 0x53DB,
    /// Disk servo write fault bad sign detected.
    DiskServoWriteFaultBadSignDetected = 0x53DC,
    /// Disk servo write fault bad parity detected.
    DiskServoWriteFaultBadParityDetected = 0x53DD,
    /// Disk servo write fault tone scan SSM timeout.
    DiskServoWriteFaultToneScanSsmTimeout53DE = 0x53DE,
    /// Disk servo requires actuator initialization.
    DiskServoRequiresActuatorInitialization = 0x53E0,
    /// Disk servo Actuator Init no Servo Channel target wedge.
    DiskServoActuatorAiNoScTargetWedge = 0x53E1,
    /// Disk servo Actuator Init timeout.
    DiskServoActuatorAiTimeout = 0x53E2,
    /// Disk servo Actuator Init bad wedge.
    DiskServoActuatorAiBadWedge = 0x53E3,
    /// Disk servo Actuator Init fatal.
    DiskServoActuatorAiFatal = 0x53E4,
    /// Disk servo Actuator Init servo state not active.
    DiskServoActuatorAiServoStateNotActive = 0x53E5,
    /// Disk servo write fault Actuator Init write unsafe.
    DiskServoWriteFaultAiUnsafe = 0x53F1,
    /// Disk servo write fault Actuator Init spindle at speed.
    DiskServoWriteFaultAiSpindleAtSpeed = 0x53F2,
    /// Disk servo write fault Actuator Init SSM timeout.
    DiskServoWriteFaultAiSsmTimeout = 0x53F3,
    /// Disk servo write fault Actuator Init illegal gray code.
    DiskServoWriteFaultAiIllegalGrayCode = 0x53F4,
    /// Disk servo write fault Actuator Init illegal cylinder.
    DiskServoWriteFaultAiIllegalCylinder = 0x53F5,
    /// Disk servo write fault Actuator Init off-track.
    DiskServoWriteFaultAiOffTrack = 0x53F6,
    /// Disk servo write fault Actuator Init fatal error path servo dead.
    DiskServoWriteFaultAiFatalPathServoDead = 0x53F7,
    /// Disk servo write fault Actuator Init read off-track.
    DiskServoWriteFaultAiReadOffTrack = 0x53F8,
    /// Disk servo write fault Actuator Init shock sensor.
    DiskServoWriteFaultAiShockSensor = 0x53F9,
    /// Disk servo write fault Actuator Init low gray code quality.
    DiskServoWriteFaultAiLowGrayCodeQuality = 0x53FA,
    /// Disk servo write fault Actuator Init off-track occurred.
    DiskServoWriteFaultAiOffTrackOccurred = 0x53FB,
    /// Disk servo write fault Actuator Init bad sign detected.
    DiskServoWriteFaultAiBadSignDetected = 0x53FC,
    /// Disk servo write fault Actuator Init bad parity detected.
    DiskServoWriteFaultAiBadParityDetected = 0x53FD,
    /// Servo Interface Returned ok.
    SvirOk = 0x5400,
    /// Servo Interface Returned abort.
    SvirAbort = 0x5401,
    /// Servo Interface Returned model.
    SvirModel = 0x5402,
    /// Servo Interface Returned head.
    SvirHead = 0x5410,
    /// Servo Interface Returned cylinder.
    SvirCylinder = 0x5411,
    /// Servo Interface Returned parameter 1.
    SvirParameter1 = 0x5412,
    /// Servo Interface Returned parameter 2.
    SvirParameter2 = 0x5413,
    /// Servo Interface Returned parameter 3.
    SvirParameter3 = 0x5414,
    /// Servo Interface Returned sub-command.
    SvirSubcommand = 0x5416,
    /// Servo Interface Returned length.
    SvirLength = 0x5417,
    /// Servo Interface Returned command.
    SvirCommand = 0x5418,
    /// Servo Interface Returned.
    Svir = 0x5420,
    /// Servo Interface Returned timeout.
    SvirTimeout = 0x5421,
    /// Servo Interface Returned index.
    SvirIndex = 0x5422,
    /// Servo Interface Returned sector.
    SvirSector = 0x5423,
    /// Servo Interface Returned SAM.
    SvirSam = 0x5424,
    /// Servo Interface Returned Written Repeatable Run Out on track 2 learn.
    SvirWrroOnTrack2Learn = 0x5426,
    /// Servo Interface Returned Written Repeatable Run Out on track 2 write.
    SvirWrroOnTrack2Write = 0x5427,
    /// Servo Interface Returned Written Repeatable Run Out calibration learn.
    SvirWrroCalibrationLearn = 0x5428,
    /// Servo Interface Returned Written Repeatable Run Out burst.
    SvirWrroBurst = 0x5429,
    /// Servo Interface Returned Written Repeatable Run Out read limit learn.
    SvirWrroReadLimitLearn = 0x542A,
    /// Servo Interface Returned Written Repeatable Run Out jog learn.
    SvirWrroJogLearn = 0x542B,
    /// Servo Interface Returned Written Repeatable Run Out write.
    SvirWrroWrite = 0x542C,
    /// Servo Interface Returned Written Repeatable Run Out setup.
    SvirWrroSetup = 0x542D,
    /// Servo Interface Returned Written Repeatable Run Out TRO limit learn.
    SvirWrroTroLimitLearn = 0x542E,
    /// Servo Interface Returned Written Repeatable Run Out timeout.
    SvirWrroTimeout = 0x542F,
    /// Servo Interface Returned spin up.
    SvirSpinUp = 0x5430,
    /// Servo Interface Returned spin down.
    SvirSpinDown = 0x5431,
    /// Servo Interface Returned actuator speed.
    SvirActuatorSpeed = 0x5440,
    /// Servo Interface Returned actuator sync.
    SvirActuatorSync = 0x5441,
    /// Servo Interface Returned actuator unlatch.
    SvirActuatorUnlatch = 0x5442,
    /// Servo Interface Returned actuator PD fail.
    SvirActuatorPdFail = 0x5443,
    /// Servo Interface Returned drive not calibrated.
    SvirDriveNotCalibrated = 0x5444,
    /// Servo Interface Returned Written Repeatable Run Out load.
    SvirWrroLoad = 0x5447,
    /// Servo Interface Returned Written Repeatable Run Out compare.
    SvirWrroCompare = 0x5449,
    /// Servo Interface Returned Position Error Signal miss sample.
    SvirPesMissSample = 0x544A,
    /// Servo Interface Returned Written Repeatable Run Out data interrupt
    /// service routine timeout.
    SvirWrroDataIsrTimeout544B = 0x544B,
    /// Servo Interface Returned Written Repeatable Run Out data interrupt
    /// service routine sync.
    SvirWrroDataIsrSync = 0x544D,
    /// Servo Interface Returned calibration normal.
    SvirCalibrationNormal = 0x5450,
    /// Servo Interface Returned calibration flex.
    SvirCalibrationFlex = 0x5458,
    /// Servo Interface Returned calibration motor.
    SvirCalibrationMotor = 0x5459,
    /// Servo Interface Returned calibration Repeatable Run Out.
    SvirCalibrationRro = 0x545A,
    /// Servo Interface Returned calibration F gain.
    SvirCalibrationFGain = 0x545B,
    /// Servo Interface Returned calibration seek.
    SvirCalibrationSeek = 0x545C,
    /// Servo Interface Returned calibration AP gain.
    SvirCalibrationApGain = 0x545D,
    /// Servo Interface Returned calibration Lock to Reference.
    SvirCalibrationLtr = 0x545E,
    /// Servo Interface Returned calibration gain S.
    SvirCalibrationGainS = 0x545F,
    /// Servo Interface Returned calibration tangential head offset.
    SvirCalibrationTangentialHeadOffset = 0x5460,
    /// Servo Interface Returned calibration bandwidth.
    SvirCalibrationBandwidth = 0x5461,
    /// Servo Interface Returned F temperature invalid.
    SvirFTemperatureInvalid = 0x5462,
    /// Servo Interface Returned AFC calibration F.
    SvirAfcCalibrationF = 0x5463,
    /// Servo Interface Returned Repeatable Run Out overflow.
    SvirRroOverflow = 0x5470,
    /// Servo Interface Returned Repeatable Run Out algorithm.
    SvirRroAlgorithm = 0x5471,
    /// Servo Interface Returned ramp load unload.
    SvirRampLoadUnload = 0x5472,
    /// Servo Interface Returned latch hang.
    SvirLatchHang = 0x5473,
    /// Servo Interface Returned load 2 fast.
    SvirLoad2Fast = 0x5474,
    /// Servo Interface Returned load 2 slow.
    SvirLoad2Slow = 0x5475,
    /// Servo Interface Returned IR calibration.
    SvirIrCalibration = 0x5476,
    /// Servo Interface Returned AD change.
    SvirAdChange = 0x5477,
    /// Servo Interface Returned ramp calibration range.
    SvirRampCalibrationRange = 0x5478,
    /// Servo Interface Returned calibration Repeatable Run Out head 0.
    SvirCalibrationRroHead0 = 0x5480,
    /// Servo Interface Returned calibration Repeatable Run Out head 1.
    SvirCalibrationRroHead1 = 0x5481,
    /// Servo Interface Returned calibration Repeatable Run Out head 2.
    SvirCalibrationRroHead2 = 0x5482,
    /// Servo Interface Returned calibration Repeatable Run Out head 3.
    SvirCalibrationRroHead3 = 0x5483,
    /// Servo Interface Returned calibration Repeatable Run Out head 4.
    SvirCalibrationRroHead4 = 0x5484,
    /// Servo Interface Returned calibration Repeatable Run Out head 5.
    SvirCalibrationRroHead5 = 0x5485,
    /// Servo Interface Returned calibration Repeatable Run Out head 6.
    SvirCalibrationRroHead6 = 0x5486,
    /// Servo Interface Returned calibration Repeatable Run Out head 7.
    SvirCalibrationRroHead7 = 0x5487,
    /// Servo Interface Returned S trace file.
    SvirSTraceFile = 0x5490,
    /// Serial IO invalid Segment Descriptor index.
    SioInvalidSdIndex = 0x5500,
    /// Serial IO invalid buffer pointer.
    SioInvalidBufferPointer = 0x5501,
    /// Serial IO timeout.
    SioTimeout = 0x5502,
    /// SPP device initialization.
    SppDeviceInitialization = 0x5503,
    /// Serial IO invalid parameter.
    SioInvalidParameter = 0x5504,
    /// Serial IO invalid transfer command header checksum.
    SioInvalidTransferHeaderChecksum = 0x5505,
    /// Serial IO invalid command function.
    SioInvalidCommandFunction = 0x5506,
    /// Serial IO invalid CRC.
    SioInvalidCrc = 0x5507,
    /// Serial IO unknown.
    SioUnknown = 0x5508,
    /// Serial IO transfer request exceeds available data.
    SioTransferRequestExceedsAvailableData = 0x5509,
    /// Serial IO acknowledge size request exceeds transfer length request.
    SioAckSizeExceedsTransferLength = 0x550A,
    /// Serial IO transfer abort request.
    SioTransferAbortRequest = 0x550B,
    /// Serial IO timeout transmit interrupt service routine.
    SioTimeoutTransmitIsr = 0x550C,
    /// Serial IO timeout transmit data.
    SioTimeoutTransmitData = 0x550D,
    /// Serial IO timeout receive interrupt service routine.
    SioTimeoutReceiveIsr = 0x550E,
    /// Serial IO timeout transfer request.
    SioTimeoutTransferRequest = 0x550F,
    /// Serial IO protocol error, command not expected.
    SioProtocolCommandNotExpected = 0x5510,
    /// Serial IO invalid transfer payload length.
    SioInvalidTransferPayloadLength = 0x5511,
    /// Serial IO invalid transfer length.
    SioInvalidTransferLength = 0x5512,
    /// Serial IO no data transfer in progress.
    SioNoDataTransferInProgress = 0x5513,
    /// Serial IO transfer command acknowledge size.
    SioTransferCommandAckSize = 0x5514,
    /// Serial IO invalid Absolute Block Number.
    SioInvalidAbn = 0x5515,
    /// Serial IO maximum CRC errors receive data.
    SioMaxCrcsReceiveData = 0x5516,
    /// Serial IO CRC error received retry sent.
    SioCrcReceivedRetrySent = 0x5517,
    /// Serial IO maximum retries.
    SioMaxRetries = 0x5518,
    /// Serial IO invalid transfer command payload length.
    SioInvalidTransferCommandPayloadLength = 0x5519,
    /// Serial IO command aborted.
    SioCommandAborted = 0x5520,
    /// Serial IO timeout waiting for acknowledge command.
    SioTimeoutWaitingForAckCommand = 0x5521,
    /// Serial IO maximum CRC errors non sequence Absolute Block Number receive
    /// data.
    SioMaxCrcsNonSequenceAbn = 0x5522,
    /// Serial IO waiting for transfer complete.
    SioWaitingForTransferComplete = 0x5523,
    /// Serial IO invalid command header reserve field.
    SioInvalidCommandHeaderReserveField = 0x5524,
    /// Serial IO invalid transfer command header parameter field.
    SioInvalidTransferHeaderParameter = 0x5525,
    /// Serial IO invalid transfer command payload CRC.
    SioInvalidTransferCommandPayloadCrc = 0x5526,
    /// Serial IO invalid transfer command direction field.
    SioInvalidTransferCommandDirectionField = 0x5527,
    /// Serial IO invalid acknowledge command header reserved field.
    SioInvalidAckHeaderReserved = 0x5528,
    /// Serial IO invalid transfer complete command reserved field.
    SioInvalidTransferCompleteReserved = 0x5529,
    /// Serial IO invalid retry command reserved field.
    SioInvalidRetryCommandReservedField = 0x5530,
    /// Serial IO invalid acknowledge command header length field.
    SioInvalidAckHeaderLength = 0x5531,
    /// Serial IO invalid transfer complete command length field.
    SioInvalidTransferCompleteLength = 0x5532,
    /// Serial IO invalid retry command length field.
    SioInvalidRetryCommandLengthField = 0x5533,
    /// Serial IO invalid acknowledge command header parameter field.
    SioInvalidAckHeaderParameter = 0x5534,
    /// Serial IO invalid transfer complete command parameter field.
    SioInvalidTransferCompleteParameter = 0x5535,
    /// Serial IO invalid retry command parameter field.
    SioInvalidRetryCommandParameterField = 0x5536,
    /// Serial IO invalid acknowledge command checksum.
    SioInvalidAckCommandChecksum = 0x5537,
    /// Serial IO invalid transfer complete command checksum.
    SioInvalidTransferCompleteChecksum = 0x5538,
    /// Serial IO invalid retry command checksum.
    SioInvalidRetryCommandChecksum = 0x5539,
    /// UART other.
    UartOther = 0x5540,
    /// UART overrun.
    UartOverrun = 0x5541,
    /// UART framing.
    UartFraming = 0x5542,
    /// Disk servo PZT fault.
    DiskServoPztFault = 0x5607,
    /// Cache flush all got canceled.
    CacheFlushAllGotCanceled = 0x6401,
    /// Cache flush cached relocation got canceled.
    CacheFlushCachedRelocationGotCanceled = 0x6402,
    /// Disk cancel.
    DiskCancel = 0x6403,
    /// Execution operation canceled.
    ExecutionOperationCanceled = 0x6404,
    /// Resource allocation got canceled.
    ResourceAllocationGotCanceled = 0x6405,
    /// Background canceled.
    BackgroundCanceled = 0x6406,
    /// Cache relocation operation canceled.
    CacheRelocationOperationCanceled = 0x6407,
    /// Format unit canceled.
    FormatUnitCanceled = 0x6408,
    /// File Manager canceled.
    FmCanceled = 0x6409,
    /// Disk remove by request.
    DiskRemoveByRequest = 0x640A,
    /// Host operation canceled.
    HostOperationCanceled = 0x640B,
    /// Aggressive OL Drive Reliability Monitor flush canceled.
    AggressiveOlDrmFlushCanceled = 0x640C,
    /// Data Lifeguard 2 got canceled.
    Dlg2GotCanceled = 0x640D,
    /// Process Test Module Process Self-Test invalid parameter.
    PtmPstInvalidParameter = 0x7001,
    /// Process Test Module Process Self-Test incompatible version.
    PtmPstIncompatibleVersion = 0x7002,
    /// Process Test Module Process Self-Test mem 49 read fail.
    PtmPstMem49ReadFail = 0x7003,
    /// Process Test Module Process Self-Test mem 4A read fail.
    PtmPstMem4AReadFail = 0x7004,
    /// Process Test Module Process Self-Test file 49 read fail.
    PtmPstFile49ReadFail = 0x7005,
    /// Process Test Module Process Self-Test file 49 write fail.
    PtmPstFile49WriteFail = 0x7006,
    /// Process Test Module Process Self-Test file 4A read fail.
    PtmPstFile4AReadFail = 0x7007,
    /// Process Test Module Process Self-Test file 4A write fail.
    PtmPstFile4AWriteFail = 0x7008,
    /// Process Test Module Process Self-Test flex bias calibration fail.
    PtmPstFlexBiasCalibrationFail = 0x7009,
    /// Process Test Module Process Self-Test motor torque calibration fail.
    PtmPstMotorTorqueCalibrationFail = 0x700A,
    /// Process Test Module Process Self-Test gain calibration fail.
    PtmPstGainCalibrationFail = 0x700B,
    /// Process Test Module Process Self-Test bias lin calibration fail.
    PtmPstBiasLinCalibrationFail = 0x700C,
    /// Process Test Module Process Self-Test load fail.
    PtmPstLoadFail = 0x700D,
    /// Process Test Module Process Self-Test unload fail.
    PtmPstUnloadFail = 0x700E,
    /// Process Test Module Process Self-Test read memory table fail.
    PtmPstReadMemoryTableFail = 0x700F,
    /// Process Test Module Process Self-Test write read verify fail.
    PtmPstWriteReadVerifyFail = 0x7010,
    /// Process Test Module Process Self-Test resident file create fail.
    PtmPstResidentFileCreateFail = 0x7011,
    /// Process Test Module Process Self-Test resident file read fail.
    PtmPstResidentFileReadFail = 0x7012,
    /// Process Test Module Process Self-Test resident file write fail.
    PtmPstResidentFileWriteFail = 0x7013,
    /// Process Test Module Process Self-Test too many measure points.
    PtmPstTooManyMeasurePoints = 0x7014,
    /// Process Test Module Process Self-Test set Bode offset fail.
    PtmPstSetBodeOffsetFail = 0x7015,
    /// Process Test Module Process Self-Test AC Bode fail.
    PtmPstAcBodeFail = 0x7016,
    /// Process Test Module Process Self-Test full stroke servo hang.
    PtmPstFullStrokeServoHang = 0x7017,
    /// Process Test Module Process Self-Test full stroke seek limit fail.
    PtmPstFullStrokeSeekLimitFail = 0x7018,
    /// Process Test Module Process Self-Test full stroke servo limit fail.
    PtmPstFullStrokeServoLimitFail = 0x7019,
    /// Process Test Module Process Self-Test running average seek invalid
    /// result.
    PtmPstRunningAverageSeekInvalidResult = 0x701A,
    /// Process Test Module Process Self-Test data size larger than allocate
    /// memory.
    PtmPstDataSizeLargerThanAllocateMemory = 0x701B,
    /// Process Test Module Process Self-Test NX Bode fail.
    PtmPstNxBodeFail = 0x701C,
    /// Process Test Module Process Self-Test seam calibration fail.
    PtmPstSeamCalibrationFail = 0x701D,
    /// Process Test Module Process Self-Test file 4D read fail.
    PtmPstFile4DReadFail = 0x701E,
    /// Process Test Module Process Self-Test file 4F read fail.
    PtmPstFile4FReadFail = 0x701F,
    /// Process Test Module Process Self-Test file 4D write fail.
    PtmPstFile4DWriteFail = 0x7020,
    /// Process Test Module Process Self-Test file 4F write fail.
    PtmPstFile4FWriteFail = 0x7021,
    /// Process Test Module Process Self-Test power up bandwidth calibration
    /// fail.
    PtmPstPowerUpBandwidthCalibrationFail = 0x7022,
    /// Process Test Module Process Self-Test power up bandwidth calibration
    /// head exceeded.
    PtmPstPowerUpBandwidthHeadExceeded = 0x7023,
    /// Process Test Module Process Self-Test power up bandwidth calibration not
    /// valid.
    PtmPstPowerUpBandwidthCalibrationNotValid = 0x7024,
    /// Process Test Module Process Self-Test file 49 create fail.
    PtmPstFile49CreateFail = 0x7025,
    /// Process Test Module Process Self-Test file 4A create fail.
    PtmPstFile4ACreateFail = 0x7026,
    /// Process Test Module Process Self-Test mini calibration in Drive
    /// Validation Test fail.
    PtmPstMiniCalibrationInDvtFail = 0x7027,
    /// Process Test Module Process Self-Test mini calibration in not valid.
    PtmPstMiniCalibrationInNotValid = 0x7028,
    /// Process Test Module Process Self-Test mini calibration in head exceeded.
    PtmPstMiniCalibrationInHeadExceeded = 0x7029,
    /// Process Test Module Process Self-Test fatal Written Repeatable Run Out
    /// B5 log full.
    PtmPstFatalWrroB5LogFull = 0x702A,
    /// Process Test Module Process Self-Test need clear 4F.
    PtmPstNeedClear4F = 0x7064,
    /// Enable DSA fail.
    EnableDsaFail = 0x7087,
    /// Advanced Read Channel Optimization CHS write.
    ArcoChsWrite = 0x7215,
    /// Advanced Read Channel Optimization CHS read.
    ArcoChsRead = 0x7216,
    /// Advanced Read Channel Optimization invalid Drive Configuration Matrix
    /// codes.
    ArcoInvalidDcmCodes721A = 0x721A,
    /// Advanced Read Channel Optimization invalid config file or format.
    ArcoInvalidConfigFileOrFormat = 0x722D,
    /// Advanced Read Channel Optimization directory sector read.
    ArcoDirectorySectorRead = 0x7280,
    /// Advanced Read Channel Optimization file 46h checksum.
    ArcoFile46hChecksum = 0x7281,
    /// Advanced Read Channel Optimization invalid command in CO buffer.
    ArcoInvalidCommandInCoBuffer = 0x7282,
    /// Advanced Read Channel Optimization checksum.
    ArcoChecksum = 0x7283,
    /// Advanced Read Channel Optimization incompatible Process Self-Test
    /// version.
    ArcoIncompatiblePstVersion = 0x7284,
    /// Advanced Read Channel Optimization incompatible channel firmware
    /// version.
    ArcoIncompatibleChannelFirmwareVersion = 0x7285,
    /// Advanced Read Channel Optimization incompatible VSC firmware version.
    ArcoIncompatibleVscFirmwareVersion = 0x7286,
    /// Advanced Read Channel Optimization VSC pressure sensor.
    ArcoVscPressureSensor = 0x728A,
    /// Advanced Read Channel Optimization pressure sensor driver
    /// initialization.
    ArcoPressureSensorDriverInitialization = 0x728B,
    /// Advanced Read Channel Optimization pressure sensor ref limit exceeded.
    ArcoPressureSensorRefLimitExceeded = 0x728C,
    /// Advanced Read Channel Optimization pressure sensor ref threshold
    /// exceeded.
    ArcoPressureSensorRefThresholdExceeded = 0x728D,
    /// Advanced Read Channel Optimization setup.
    ArcoSetup = 0x7290,
    /// Advanced Read Channel Optimization file ID.
    ArcoFileId = 0x7291,
    /// Advanced Read Channel Optimization file read.
    ArcoFileRead = 0x7292,
    /// Advanced Read Channel Optimization module read.
    ArcoModuleRead = 0x7294,
    /// Advanced Read Channel Optimization file write.
    ArcoFileWrite = 0x7295,
    /// Advanced Read Channel Optimization invalid header.
    ArcoInvalidHeader = 0x7296,
    /// Advanced Read Channel Optimization too many zones.
    ArcoTooManyZones = 0x7297,
    /// Advanced Read Channel Optimization too many heads.
    ArcoTooManyHeads = 0x7298,
    /// Advanced Read Channel Optimization test time exceed limit.
    ArcoTestTimeExceedLimit = 0x7299,
    /// Advanced Read Channel Optimization invalid input.
    ArcoInvalidInput = 0x729A,
    /// Advanced Read Channel Optimization failed to read codata file.
    ArcoFailedToReadCodataFile = 0x729B,
    /// Advanced Read Channel Optimization failed to create file.
    ArcoFailedToCreateFile = 0x729C,
    /// Advanced Read Channel Optimization failed to switch WCS.
    ArcoFailedToSwitchWcs = 0x729D,
    /// Advanced Read Channel Optimization standby command failed.
    ArcoStandbyCommandFailed = 0x72B0,
    /// Advanced Read Channel Optimization recal command failed.
    ArcoRecalCommandFailed = 0x72B1,
    /// Advanced Read Channel Optimization logical to BPI zone translate.
    ArcoLogicalToBpiZoneTranslate = 0x72B5,
    /// Advanced Read Channel Optimization BPI to logical zone translate.
    ArcoBpiToLogicalZoneTranslate = 0x72B6,
    /// Advanced Read Channel Optimization curve fit order out of bound.
    ArcoCurveFitOrderOutOfBound = 0x72B7,
    /// Advanced Read Channel Optimization not enough data point for curve fit.
    ArcoNotEnoughDataPointForCurveFit = 0x72B8,
    /// Advanced Read Channel Optimization unexpected data in log.
    ArcoUnexpectedDataInLog = 0x72BE,
    /// Advanced Read Channel Optimization failed to read module.
    ArcoFailedToReadModule = 0x72BF,
    /// Advanced Read Channel Optimization TD calibration same flex min max.
    ArcoTdCalibrationSameFlexMinMax = 0x72C0,
    /// Advanced Read Channel Optimization recovery register list too small.
    ArcoRecoveryRegisterListTooSmall = 0x72C4,
    /// Advanced Read Channel Optimization invalid Drive Configuration Matrix
    /// codes.
    ArcoInvalidDcmCodes72C8 = 0x72C8,
    /// Advanced Read Channel Optimization VSC translation.
    ArcoVscTranslation = 0x72D0,
    /// Advanced Read Channel Optimization VSC get drive data.
    ArcoVscGetDriveData = 0x72D1,
    /// Advanced Read Channel Optimization VSC read write memory file 46.
    ArcoVscReadWriteMemoryFile46 = 0x72D2,
    /// Advanced Read Channel Optimization VSC exception control during on/off
    /// MRR cycling.
    ArcoVscExceptionControlMrrCycling = 0x72D3,
    /// Advanced Read Channel Optimization VSC exception control during on/off
    /// jog interpolation.
    ArcoVscExceptionControlJogInterpolation = 0x72D4,
    /// Advanced Read Channel Optimization VSC exception control during select
    /// channel update mode.
    ArcoVscExceptionControlChannelUpdate = 0x72D5,
    /// Advanced Read Channel Optimization VSC exception handling command.
    ArcoVscExceptionHandlingCommand = 0x72D6,
    /// Advanced Read Channel Optimization VSC read error rate table command.
    ArcoVscReadErrorRateTableCommand = 0x72D7,
    /// Advanced Read Channel Optimization VSC MNP access command.
    ArcoVscMnpAccessCommand = 0x72D8,
    /// Advanced Read Channel Optimization VSC spin down command.
    ArcoVscSpinDownCommand = 0x72D9,
    /// Advanced Read Channel Optimization VSC spin up command.
    ArcoVscSpinUpCommand = 0x72DA,
    /// Advanced Read Channel Optimization read write field command.
    ArcoReadWriteFieldCommand = 0x72DB,
    /// Advanced Read Channel Optimization format select command.
    ArcoFormatSelectCommand = 0x72DC,
    /// Advanced Read Channel Optimization VSC command event pending.
    ArcoVscCommandEventPending = 0x72DF,
    /// Advanced Read Channel Optimization VSC switch format command.
    ArcoVscSwitchFormatCommand = 0x72E0,
    /// Advanced Read Channel Optimization initialize defect list.
    ArcoInitializeDefectList = 0x72E1,
    /// Advanced Read Channel Optimization initialize good cylinder list.
    ArcoInitializeGoodCylinderList = 0x72E2,
    /// Advanced Read Channel Optimization cannot find a good cylinder.
    ArcoCannotFindGoodCylinder = 0x72E3,
    /// Advanced Read Channel Optimization invalid zone.
    ArcoInvalidZone = 0x72E4,
    /// Advanced Read Channel Optimization delta greater than threshold.
    ArcoDeltaGreaterThanThreshold = 0x72E5,
    /// Advanced Read Channel Optimization prep test track.
    ArcoPrepTestTrack = 0x72E6,
    /// Advanced Read Channel Optimization drive temperature calibration.
    ArcoDriveTemperatureCalibration = 0x72E7,
    /// Advanced Read Channel Optimization invalid preamp gain value.
    ArcoInvalidPreampGainValue = 0x72E8,
    /// Advanced Read Channel Optimization preamp gain calibration.
    ArcoPreampGainCalibration = 0x72E9,
    /// Advanced Read Channel Optimization codata address invalid.
    ArcoCodataAddressInvalid = 0x72EB,
    /// Advanced Read Channel Optimization temperature above target temperature.
    ArcoTemperatureAboveTargetTemperature = 0x72ED,
    /// Advanced Read Channel Optimization PBERT Drive Validation Test write.
    ArcoPbertDvtWrite = 0x72EF,
    /// Advanced Read Channel Optimization illegal optimization number
    /// requested.
    ArcoIllegalOptimizationNumberRequested = 0x72F0,
    /// Advanced Read Channel Optimization illegal Sectors Per Track requested.
    ArcoIllegalSptRequested = 0x72F1,
    /// Advanced Read Channel Optimization invalid Drive Configuration Matrix
    /// code.
    ArcoInvalidDcmCode = 0x72F2,
    /// Advanced Read Channel Optimization bad checksum in data file.
    ArcoBadChecksumInDataFile = 0x72F3,
    /// Advanced Read Channel Optimization invalid buffer.
    ArcoInvalidBuffer = 0x72F4,
    /// Advanced Read Channel Optimization unsupported preamp ID.
    ArcoUnsupportedPreampId = 0x72F5,
    /// Advanced Read Channel Optimization bad or invalid log info.
    ArcoBadOrInvalidLogInfo = 0x72F6,
    /// Advanced Read Channel Optimization invalid entry.
    ArcoInvalidEntry = 0x72F7,
    /// Advanced Read Channel Optimization too many format code.
    ArcoTooManyFormatCode = 0x72F8,
    /// Advanced Read Channel Optimization failed to access full stroke.
    ArcoFailedToAccessFullStroke = 0x72F9,
    /// Advanced Read Channel Optimization invalid model list.
    ArcoInvalidModelList = 0x72FB,
    /// Intelligent Burn In default abort code.
    IbiDefaultAbortCode = 0x7601,
    /// Intelligent Burn In full head surface log.
    IbiFullHeadSurfaceLog7603 = 0x7603,
    /// Intelligent Burn In full head surface log.
    IbiFullHeadSurfaceLog7604 = 0x7604,
    /// Intelligent Burn In full head surface log.
    IbiFullHeadSurfaceLog7605 = 0x7605,
    /// Intelligent Burn In full head surface log.
    IbiFullHeadSurfaceLog7608 = 0x7608,
    /// Intelligent Burn In timeout.
    IbiTimeout = 0x760A,
    /// Intelligent Burn In servo log test.
    IbiServoLogTest = 0x760B,
    /// Intelligent Burn In exceeded head defects limit.
    IbiExceededHeadDefectsLimit760D = 0x760D,
    /// Intelligent Burn In format capacity.
    IbiFmtCapacity760E = 0x760E,
    /// Intelligent Burn In specified capacity not reached.
    IbiSpecifiedCapacityNotReached760F = 0x760F,
    /// Intelligent Burn In format capacity.
    IbiFmtCapacity7611 = 0x7611,
    /// Intelligent Burn In specified capacity not reached.
    IbiSpecifiedCapacityNotReached7612 = 0x7612,
    /// Intelligent Burn In format capacity.
    IbiFmtCapacity7613 = 0x7613,
    /// Intelligent Burn In full head surface log.
    IbiFullHeadSurfaceLog7614 = 0x7614,
    /// Intelligent Burn In exceeded defects limit.
    IbiExceededDefectsLimit7616 = 0x7616,
    /// Intelligent Burn In T-list.
    IbiTlist = 0x7619,
    /// Intelligent Burn In exceeded head defects limit.
    IbiExceededHeadDefectsLimit761C = 0x761C,
    /// Intelligent Burn In illegal parameters.
    IbiIllegalParameters = 0x763F,
    /// Intelligent Burn In log read.
    IbiLogRead = 0x7646,
    /// Intelligent Burn In exceeded head defects limit.
    IbiExceededHeadDefectsLimit764A = 0x764A,
    /// Intelligent Burn In wrong parameters.
    IbiWrongParameters = 0x764D,
    /// Intelligent Burn In test mini.
    IbiTestMini7652 = 0x7652,
    /// Intelligent Burn In reserved Push Down List overflow.
    IbiReservedPushDownListOverflow = 0x7654,
    /// Intelligent Burn In test mini.
    IbiTestMini7657 = 0x7657,
    /// Intelligent Burn In test B9 exceeded defects limit.
    IbiTestB9ExceededDefectsLimit = 0x766E,
    /// Intelligent Burn In too many soft errors.
    IbiTooManySoftErrors = 0x7677,
    /// Intelligent Burn In test BA too many soft errors.
    IbiTestBaTooManySoftErrors = 0x7678,
    /// Intelligent Burn In exceeded defects limit.
    IbiExceededDefectsLimit767C = 0x767C,
    /// Intelligent Burn In exceeded defects limit.
    IbiExceededDefectsLimit767E = 0x767E,
    /// Intelligent Burn In test D1 full head surface log.
    IbiTestD1FullHeadSurfaceLog = 0x768C,
    /// Intelligent Burn In test B9.
    IbiTestB9 = 0x76CE,
    /// Intelligent Burn In format write fail exceeded defects limit.
    IbiFmtWriteFailExceededDefectsLimit = 0x76D9,
    /// Intelligent Burn In too many tracks in P-list.
    IbiTooManyTracksInPlist76F6 = 0x76F6,
    /// Intelligent Burn In too many tracks in P-list.
    IbiTooManyTracksInPlist76F7 = 0x76F7,
    /// Intelligent Burn In native maximum LBA too big.
    IbiNativeMaximumLbaTooBig = 0x76F8,
    /// Intelligent Burn In exceeded defects limit.
    IbiExceededDefectsLimit76FF = 0x76FF,
    /// Process Test Module invalid vector table version.
    PtmInvalidVectorTableVersion = 0x7F01,
    /// HAL invalid parameter.
    HalInvalidParameter = 0x8000,
    /// HAL flash unknown.
    HalFlashUnknown = 0x8020,
    /// HAL flash invalid flash sector address.
    HalFlashInvalidSectorAddress = 0x8021,
    /// HAL flash write latch enable.
    HalFlashWriteLatchEnable = 0x8022,
    /// HAL flash write page send byte.
    HalFlashWritePageSendByte = 0x8023,
    /// HAL flash write page timeout.
    HalFlashWritePageTimeout = 0x8024,
    /// HAL flash write page to Static Memory.
    HalFlashWritePageToStaticMemory = 0x8025,
    /// HAL flash read block get byte.
    HalFlashReadBlockGetByte = 0x8026,
    /// HAL flash byte count exceeds device limit.
    HalFlashByteCountExceedsDeviceLimit = 0x8027,
    /// HAL flash invalid flash address.
    HalFlashInvalidAddress = 0x8028,
    /// HAL flash initial boot header missing.
    HalFlashInitialBootHeaderMissing = 0x8029,
    /// HAL flash send byte timeout.
    HalFlashSendByteTimeout = 0x802A,
    /// HAL flash read command.
    HalFlashReadCommand = 0x802B,
    /// HAL flash invalid flash data.
    HalFlashInvalidData = 0x802C,
    /// HAL flash data compare.
    HalFlashDataCompare = 0x802D,
    /// HAL flash device ID.
    HalFlashDeviceId = 0x802E,
    /// HAL flash read info start timeout.
    HalFlashReadInfoStartTimeout = 0x802F,
    /// HAL flash status timeout.
    HalFlashStatusTimeout = 0x8030,
    /// HAL flash command timeout.
    HalFlashCommandTimeout = 0x8031,
    /// HAL flash bad checksum.
    HalFlashBadChecksum = 0x8032,
    /// HAL system phase-locked loop lock failure.
    HalSystemPllLockFailure = 0x8100,
    /// HAL system SPP check fail.
    HalSystemSppCheckFail = 0x8101,
    /// HAL system UART FIFO full.
    HalSystemUartFifoFull = 0x8102,
    /// HAL system UART FIFO empty.
    HalSystemUartFifoEmpty = 0x8103,
    /// HAL system UART overrun.
    HalSystemUartOverrun = 0x8104,
    /// HAL system UART others.
    HalSystemUartOthers = 0x8105,
    /// HAL system UART transmit FIFO full.
    HalSystemUartTransmitFifoFull = 0x8106,
    /// VSC not support read Written Repeatable Run Out.
    VscNotSupportReadWrro = 0x8814,
    /// Servo Interface Returned Written Repeatable Run Out data interrupt
    /// service routine timeout.
    SvirWrroDataIsrTimeout884B = 0x884B,
    /// Servo Interface Returned seek busy.
    SvirSeekBusy = 0x88B1,
    /// Invalid mod act code request.
    InvalidModActCodeRequest = 0xB001,
    /// VSC command set not enabled.
    VscCommandSetNotEnabled = 0xB002,
    /// Invalid mod byte in modify config sector command.
    InvalidModByteInModifyConfigSector = 0xB003,
    /// Invalid SMART enable code.
    InvalidSmartEnableCode = 0xB004,
    /// Invalid operation request.
    InvalidOperationRequest = 0xB005,
    /// Offset too large.
    OffsetTooLarge = 0xB006,
    /// Invalid head number.
    InvalidHeadNumber = 0xB007,
    /// Cylinder above limit.
    CylinderAboveLimit = 0xB008,
    /// Invalid wedge offset.
    InvalidWedgeOffset = 0xB009,
    /// Invalid wedge size.
    InvalidWedgeSize = 0xB00A,
    /// Start address too large.
    StartAddressTooLarge = 0xB00B,
    /// Length too large.
    LengthTooLarge = 0xB00C,
    /// Invalid table ID.
    InvalidTableId = 0xB00D,
    /// Unsupported action code.
    UnsupportedActionCode = 0xB00E,
    /// Unsupported function.
    UnsupportedFunction = 0xB00F,
    /// Value action code unsupported feature.
    ValueActionCodeUnsupportedFeature = 0xB010,
    /// Value action code unsupported operation.
    ValueActionCodeUnsupportedOperation = 0xB011,
    /// Invalid function code request.
    InvalidFunctionCodeRequest = 0xB012,
    /// Table offset too large.
    TableOffsetTooLarge = 0xB013,
    /// Invalid exception feature.
    InvalidExceptionFeature = 0xB014,
    /// Invalid offset.
    InvalidOffset = 0xB015,
    /// Invalid key sector size.
    InvalidKeySectorSize = 0xB017,
    /// Transfer request exceeds available data.
    TransferRequestExceedAvailableData = 0xB018,
    /// Invalid VSC source.
    InvalidVscSource = 0xB019,
    /// Action code out of range.
    ActionCodeOutOfRange = 0xB01A,
    /// Key sector must precede data transfer request.
    KeySectorMustPrecedeDataTransferRequest = 0xB01B,
    /// Invalid settle mode.
    InvalidSettleMode = 0xB01C,
    /// Invalid enable disable key in feature registers.
    InvalidEnableDisableKeyInFeatureRegisters = 0xB01D,
    /// Function not supported on system on chip platform.
    FunctionNotSupportedOnSocPlatform = 0xB01E,
    /// Invalid sector request.
    InvalidSectorRequest = 0xB01F,
    /// Flash length too small.
    FlashLengthTooSmall = 0xB020,
    /// Flash start address too small.
    FlashStartAddressTooSmall = 0xB021,
    /// Flash start sector too small.
    FlashStartSectorTooSmall = 0xB022,
    /// Flash access range request too large.
    FlashAccessRangeRequestTooLarge = 0xB023,
    /// Invalid LBA request.
    InvalidLbaRequest = 0xB024,
    /// Process Self-Test buffer not allocated.
    PstBufferNotAllocated = 0xB025,
    /// Command not allowed from Process Self-Test.
    CommandNotAllowedFromPst = 0xB026,
    /// VSC invalid Process Self-Test test ID.
    VscInvalidPstTestId = 0xB027,
    /// VSC invalid Process Self-Test vector address.
    VscInvalidPstVectorAddress = 0xB028,
    /// Process Self-Test VSCD buffer too small.
    PstVscdBufferTooSmall = 0xB029,
    /// Sector offset not from zero.
    SectorOffsetNotFromZero = 0xB02A,
    /// Invalid resource memory request.
    InvalidResourceMemoryRequest = 0xB02B,
    /// Invalid Process Self-Test test mode request.
    InvalidPstTestModeRequest = 0xB02C,
    /// Clear Drive Reliability Monitor log failed.
    ClearDrmLogFailed = 0xB02D,
    /// Clear factory file failed.
    ClearFactoryFileFailed = 0xB02E,
    /// Warning wear level with background disabled.
    WarningWearLevelWithBackgroundDisabled = 0xB02F,
    /// Invalid wear level argument.
    InvalidWearLevelArgument = 0xB030,
    /// Invalid period shift controller argument.
    InvalidPeriodShiftCtlrArgument = 0xB031,
    /// Warning Drive Reliability Monitor flush with background disabled.
    WarningDrmFlushWithBackgroundDisabled = 0xB032,
    /// Invalid Drive Reliability Monitor flush control argument.
    InvalidDrmFlushControlArgument = 0xB033,
    /// Invalid SMART backdoor argument.
    InvalidSmartBackdoorArgument = 0xB034,
    /// Invalid background activity argument.
    InvalidBackgroundActivityArgument = 0xB035,
    /// Invalid drive temperature sampling argument.
    InvalidDriveTemperatureSamplingArgument = 0xB036,
    /// Invalid clear Drive Reliability Monitor section.
    InvalidClearDrmSection = 0xB038,
    /// Depop invalid head ID.
    DepopInvalidHeadId = 0xB03C,
    /// Request end LBA less than start LBA.
    RequestEndLbaLessThanStart = 0xB03F,
    /// Depop only one head.
    DepopOnlyOneHead = 0xB040,
    /// Invalid Process Self-Test mode argument.
    InvalidPstModeArgument = 0xB043,
    /// Push downs on track.
    PushDownsOnTrack = 0xB04D,
    /// Invalid period sum parameter.
    InvalidPeriodSumParameter = 0xB04E,
    /// Host data transfer did not occur.
    HostDataTransferDidNotOccur = 0xB04F,
    /// Invalid clear Drive Reliability Monitor agent code.
    InvalidClearDrmAgentCode = 0xB050,
    /// Feature control invalid argument.
    FeatureControlInvalidArgument = 0xB051,
    /// Feature control read feature unsupported.
    FeatureControlReadUnsupported = 0xB052,
    /// Memory table is read only.
    MemoryTableIsReadOnly = 0xB053,
    /// Debug Stop occurred.
    DebugStopOccurred = 0xB054,
    /// Read write field invalid length.
    ReadWriteFieldInvalidLength = 0xB055,
    /// Requested relocations greater than available.
    RequestedRelocationsGreaterThanAvailable = 0xB056,
    /// Invalid length.
    InvalidLength = 0xB057,
    /// Invalid count value.
    InvalidCountValue = 0xB058,
    /// Invalid address mode.
    InvalidAddressMode = 0xB05A,
    /// Servo trace disabled.
    ServoTraceDisabled = 0xB05B,
    /// Config servo trace already enabled.
    ConfigServoTraceAlreadyEnabled = 0xB05C,
    /// Invalid start wedge.
    InvalidStartWedge = 0xB05D,
    /// Invalid zone number.
    InvalidZoneNumber = 0xB05E,
    /// Cylinder not in gain calibration zone.
    CylinderNotInGainCalibrationZone = 0xB05F,
    /// Disable gain calibration to run this command.
    DisableGainCalibrationToRunThisCommand = 0xB060,
    /// Invalid config section.
    InvalidConfigSection = 0xB061,
    /// Servo Interface Returned invalid table size.
    SvirInvalidTableSize = 0xB062,
    /// Gain calibration table not initialized.
    GainCalibrationTableNotInitialized = 0xB063,
    /// Gain calibration training not started.
    GainCalibrationTrainingNotStarted = 0xB064,
    /// Gain calibration value not trained.
    GainCalibrationValueNotTrained = 0xB065,
    /// Gain calibration feature not implemented.
    GainCalibrationFeatureNotImplemented = 0xB066,
    /// Partial file request past EOF.
    PartialFileRequestPastEof = 0xB067,
    /// Invalid Drive Validation Test opcode.
    InvalidDvtOpcode = 0xB068,
    /// Temporary SRAM static already allocated.
    TemporarySramStaticAlreadyAllocated = 0xB069,
    /// Table not available cache relocation disabled.
    TableNotAvailableCacheRelocationDisabled = 0xB06A,
    /// No temporary SRAM static allocated.
    NoTemporarySramStaticAllocated = 0xB070,
    /// Invalid SMART attribute status.
    InvalidSmartAttributeStatus = 0xB071,
    /// Invalid SMART attribute ID.
    InvalidSmartAttributeId = 0xB072,
    /// Single pass tone scan not supported.
    SinglePassToneScanNotSupported = 0xB073,
    /// Invalid wear level mode.
    InvalidWearLevelMode = 0xB074,
    /// Invalid wear level config table not available.
    InvalidWearLevelConfigTableNotAvailable = 0xB075,
    /// Depop Intelligent Burn In surface 1 log not in track directory.
    DepopIbiSurface1LogNotInTrackDirectory = 0xB076,
    /// Depop Intelligent Burn In surface 1 Position Error Signal not in track
    /// directory.
    DepopIbiSurface1PesNotInTrackDirectory = 0xB077,
    /// Partial file not in Process Self-Test mode.
    PartialFileNotInPstMode = 0xB078,
    /// Dynamic Fly Height mode not enabled.
    DfhModeNotEnabled = 0xB079,
    /// Parameter out of range.
    ParameterOutOfRange = 0xB07A,
    /// Format select capacity failure.
    FmtSelectCapacityFailure = 0xB07B,
    /// Drive Configuration Matrix uninitialized.
    DcmUninitialized = 0xB07C,
    /// Capacity group definition.
    CapacityGroupDefinition = 0xB07D,
    /// Write read gap info not available.
    WriteReadGapInfoNotAvailable = 0xB07E,
    /// Drive Protect locked.
    DriveProtectLocked = 0xB080,
    /// Invalid region number.
    InvalidRegionNumber = 0xB081,
    /// Disk backend table not present.
    DiskBackendTableNotPresent = 0xB082,
    /// Compare IDs LBA miscompare.
    CompareIdsLbaMiscompare = 0xB083,
    /// Flex bias filter.
    FlexBiasFilter = 0xB084,
    /// VSC invalid config code.
    VscInvalidConfigCode = 0xB085,
    /// VSC invalid config data header.
    VscInvalidConfigDataHeader = 0xB086,
    /// VSC invalid config data.
    VscInvalidConfigData = 0xB087,
    /// VSC parameter length mismatch.
    VscParameterLengthMismatch = 0xB088,
    /// VSC parameter type mismatch.
    VscParameterTypeMismatch = 0xB089,
    /// VSC rule check fail.
    VscRuleCheckFail = 0xB08A,
    /// VSC check list empty.
    VscCheckListEmpty = 0xB08B,
    /// VSC no matching entry in table.
    VscNoMatchingEntryInTable = 0xB08C,
    /// VSC invalid entry in table.
    VscInvalidEntryInTable = 0xB08D,
    /// VSC command response threshold exceeded.
    VscCommandResponseThresholdExceeded = 0xB08E,
    /// VSC command response processing in progress.
    VscCommandResponseProcessingInProgress = 0xB08F,
    /// VSC major revision mismatch.
    VscMajorRevisionMismatch = 0xB090,
    /// VSC CVF file not found.
    VscCvfFileNotFound = 0xB091,
    /// VSC major revision already set.
    VscMajorRevisionAlreadySet = 0xB092,
    /// VSC feature set key fail.
    VscFeatureSetKeyFail = 0xB093,
    /// VSC S overlay already loaded.
    VscSOverlayAlreadyLoaded = 0xB094,
    /// VSC C overlay already loaded.
    VscCOverlayAlreadyLoaded = 0xB095,
    /// VSC mismatch family ID.
    VscMismatchFamilyId = 0xB096,
    /// VSC CVF file read.
    VscCvfFileRead = 0xB097,
    /// VSC failure in update data file.
    VscFailureInUpdateDataFile = 0xB098,
    /// VSC invalid low memory mode argument.
    VscInvalidLowMemoryModeArgument = 0xB099,
    /// VSC pre AC55 config drive.
    VscPreAc55ConfigDrive = 0xB09A,
    /// VSC UCCM total bytes expected mismatch.
    VscUccmTotalBytesExpectedMismatch = 0xB09B,
    /// VSC AC55 function code 1 support disabled.
    VscAc55FunctionCode1SupportDisabled = 0xB09C,
    /// VSC AC55 function code 2 support disabled.
    VscAc55FunctionCode2SupportDisabled = 0xB09D,
    /// VSC AC55 function code 3 support disabled.
    VscAc55FunctionCode3SupportDisabled = 0xB09E,
    /// VSC AC55 function code 4 support disabled.
    VscAc55FunctionCode4SupportDisabled = 0xB09F,
    /// VSC AC55 function code 5 support disabled.
    VscAc55FunctionCode5SupportDisabled = 0xB0A0,
    /// VSC AC55 function code 6 support disabled.
    VscAc55FunctionCode6SupportDisabled = 0xB0A1,
    /// VSC AC55 function code 7 support disabled.
    VscAc55FunctionCode7SupportDisabled = 0xB0A2,
    /// VSC AC55 function code 8 support disabled.
    VscAc55FunctionCode8SupportDisabled = 0xB0A3,
    /// VSC AC55 function code 9 support disabled.
    VscAc55FunctionCode9SupportDisabled = 0xB0A4,
    /// VSC AC55 function code 10 support disabled.
    VscAc55FunctionCode10SupportDisabled = 0xB0A5,
    /// VSC P overlay already loaded.
    VscPOverlayAlreadyLoaded = 0xB0B0,
    /// Invalid SMART EN code.
    InvalidSmartEnCode = 0xB100,
    /// SMART invalid host sector request.
    SmartInvalidHostSectorRequest = 0xB101,
    /// SMART invalid vendor sector request.
    SmartInvalidVendorSectorRequest = 0xB102,
    /// SMART feature not supported.
    SmartFeatureNotSupported = 0xB103,
    /// SMART invalid sector count.
    SmartInvalidSectorCount = 0xB104,
    /// VSC invalid RASP target.
    VscInvalidRaspTarget = 0xB105,
    /// VSC invalid RASP target track.
    VscInvalidRaspTargetTrack = 0xB106,
    /// VSC Process Self-Test RASP get buffer too small.
    VscPstRaspGetBufferTooSmall = 0xB108,
    /// SMART invalid defect list type.
    SmartInvalidDefectListType = 0xB109,
    /// SMART invalid defect list format.
    SmartInvalidDefectListFormat = 0xB10A,
    /// SMART write select test, self-test in progress.
    SmartWriteSelectTestSelfTestInProgress = 0xB302,
    /// SMART selective test invalid version.
    SmartSelectiveTestInvalidVersion = 0xB303,
    /// SMART offline immediate disabled.
    SmartOfflineImmediateDisabled = 0xB304,
    /// SMART resource allocation failed.
    SmartResourceAllocationFailed = 0xB305,
    /// SMART status failed.
    SmartStatusFailed = 0xB306,
    /// Obsolete command.
    ObsoleteCommand = 0xB400,
    /// SMART command with SMART disabled.
    SmartCommandWithSmartDisabled = 0xB401,
    /// Feature not supported.
    FeatureNotSupported = 0xB402,
    /// Security command with bad parameter.
    SecurityCommandWithBadParameter = 0xB403,
    /// Security command with security mode disabled.
    SecurityCommandSecurityModeDisabled = 0xB404,
    /// Security command with disk frozen.
    SecurityCommandWithDiskFrozen = 0xB405,
    /// Security command with disk locked.
    SecurityCommandWithDiskLocked = 0xB406,
    /// Security command with disk locked or frozen.
    SecurityCommandWithDiskLockedOrFrozen = 0xB407,
    /// Security command with disk expired or frozen.
    SecurityCommandWithDiskExpiredOrFrozen = 0xB408,
    /// Security command with receive data.
    SecurityCommandWithReceiveData = 0xB409,
    /// Security command with password miscompare.
    SecurityCommandWithPasswordMiscompare = 0xB40A,
    /// Security command with password is zero.
    SecurityCommandWithPasswordIsZero = 0xB40B,
    /// Security command with invalid master password revision.
    SecurityInvalidMasterPasswordRevision = 0xB40C,
    /// Security command no erase prepare.
    SecurityCommandNoErasePrepare = 0xB40D,
    /// Security command received in invalid state.
    SecurityCommandReceivedInInvalidState = 0xB40E,
    /// VSC security drive is locked.
    VscSecurityDriveIsLocked = 0xB40F,
    /// VSC security unlock failed after format unit.
    VscSecurityUnlockFailedAfterFormatUnit = 0xB410,
    /// VSC security invalid format unit options to unlock.
    VscSecurityInvalidFmtUnitOptions = 0xB411,
    /// CHS LBA too large.
    ChsLbaTooLarge = 0xB418,
    /// Error injection invalid function code.
    ErrorInjectionInvalidFunctionCode = 0xB430,
    /// Error injection invalid error type.
    ErrorInjectionInvalidType = 0xB431,
    /// Error injection invalid NRZ mode.
    ErrorInjectionInvalidNrzMode = 0xB432,
    /// Error injection invalid handle.
    ErrorInjectionInvalidHandle = 0xB433,
    /// Error injection invalid count.
    ErrorInjectionInvalidCount = 0xB434,
    /// Error injection invalid offset.
    ErrorInjectionInvalidOffset = 0xB435,
    /// Error injection invalid length.
    ErrorInjectionInvalidLength = 0xB436,
    /// Error injection invalid repeat count.
    ErrorInjectionInvalidRepeatCount = 0xB437,
    /// Error injection IEITBL full.
    ErrorInjectionIeitblFull = 0xB438,
    /// Error injection IEITBL empty.
    ErrorInjectionIeitblEmpty = 0xB439,
    /// Error injection same track exist.
    ErrorInjectionSameTrackExist = 0xB43A,
    /// Error injection target not found.
    ErrorInjectionTargetNotFound = 0xB43B,
    /// Error injection add target failed.
    ErrorInjectionAddTargetFailed = 0xB43C,
    /// Error injection remove target failed.
    ErrorInjectionRemoveTargetFailed = 0xB43D,
    /// Background invalid self-test selected.
    BackgroundInvalidSelfTestSelected = 0xB800,
    /// SMART Command Transport unsupported built-in self-test mode in pattern
    /// request.
    SctUnsupportedBistModeInPatternRequest = 0xC001,
    /// SMART Command Transport unsupported op code for WDLOGS.
    SctUnsupportedOpCodeForWdLogs = 0xC002,
    /// Host Protected Area invalid value specified.
    HpaInvalidValueSpecified = 0xC200,
    /// Host Protected Area lock in place.
    HpaLockInPlace = 0xC201,
    /// Host Protected Area lock not in place.
    HpaLockNotInPlace = 0xC202,
    /// Host Protected Area freeze lock in place.
    HpaFreezeLockInPlace = 0xC203,
    /// Host Protected Area command sequence.
    HpaCommandSequence = 0xC204,
    /// Host Protected Area set max address ext in place.
    HpaSetMaxAddressExtInPlace = 0xC205,
    /// Host Protected Area password active.
    HpaPasswordActive = 0xC206,
    /// Host Protected Area password not active.
    HpaPasswordNotActive = 0xC207,
    /// Host Protected Area password miscompare.
    HpaPasswordMiscompare = 0xC208,
    /// Host Protected Area second non-volatile command.
    HpaSecondNonVolatileCommand = 0xC209,
    /// Host Protected Area read max first command.
    HpaReadMaxFirstCommand = 0xC20A,
    /// Host Protected Area command sequence fault.
    HpaCommandSequenceFault = 0xC20B,
    /// Host unsupported ATA opcode.
    HostUnsupportedAtaOpcode = 0xC400,
    /// Host received LBA too big.
    HostReceivedLbaTooBig = 0xC401,
    /// Host LBA out of range.
    HostLbaOutOfRange = 0xC402,
    /// Host drive parameters Sectors Per Track not supported.
    HostDriveParametersSptNotSupported = 0xC403,
    /// Host drive parameters heads not supported.
    HostDriveParametersHeadsNotSupported = 0xC404,
    /// Host unsupported feature value.
    HostUnsupportedFeatureValue = 0xC405,
    /// Host unsupported multi count.
    HostUnsupportedMultiCount = 0xC406,
    /// Host multi not set.
    HostMultiNotSet = 0xC407,
    /// Host disabled I/O ready not supported.
    HostDisabledIordyNotSupported = 0xC408,
    /// Host unsupported command in Process Self-Test mode.
    HostUnsupportedCommandInPstMode = 0xC409,
    /// Host invalid sector count.
    HostInvalidSectorCount = 0xC40A,
    /// Host VSC command executing in background.
    HostVscCommandExecutingInBackground = 0xC40B,
    /// Host command not allowed in gain calibration mode.
    HostCommandNotAllowedInGainCalibration = 0xC40C,
    /// Host queue command intermix.
    HostQueueCommandIntermix = 0xC40D,
    /// Host unsupported set feature SATA feature.
    HostUnsupportedSetFeatureSata = 0xC40E,
    /// Host queue tag.
    HostQueueTag = 0xC40F,
    /// Host invalid CHS cylinder number.
    HostInvalidChsCylinderNumber = 0xC410,
    /// Host invalid CHS head number.
    HostInvalidChsHeadNumber = 0xC411,
    /// Host invalid CHS sector number.
    HostInvalidChsSectorNumber = 0xC412,
    /// Host unsupported ATA command in Serial IO mode.
    HostUnsupportedAtaCommandInSioMode = 0xC413,
    /// Host reserved standby timer value.
    HostReservedStandbyTimerValue = 0xC414,
    /// Host Power-Up In Standby set feature disabled in config sector.
    HostPuisSetFeatureDisabledInConfigSector = 0xC420,
    /// Host Power-Up In Standby flash set to use jumper.
    HostPuisFlashSetToUseJumper = 0xC421,
    /// Host Power-Up In Standby set feature not supported in XPM2.
    HostPuisSetFeatureNotSupportedInXpm2 = 0xC422,
    /// Host Power-Up In Standby spin up command not supported in XPM2.
    HostPuisSpinUpCommandNotSupportedInXpm2 = 0xC423,
    /// Host Power-Up In Standby jumper enabled in flash no jumper.
    HostPuisJumperEnabledInFlashNoJumper = 0xC424,
    /// Host Power-Up In Standby disabled in flash.
    HostPuisDisabledInFlash = 0xC425,
    /// Host invalid ATA stream ID.
    HostInvalidAtaStreamId = 0xC430,
    /// Host ATA stream ID not config.
    HostAtaStreamIdNotConfig = 0xC431,
    /// Host locked unit access denied.
    HostLockedUnitAccessDenied = 0xC581,
    /// Host Native Command Queuing no READ LOG 10h.
    HostNcqNoReadLog10 = 0xC586,
    /// Host unsupported chip revision.
    HostUnsupportedChipRevision = 0xC589,
    /// Host interface CRC.
    HostInterfaceCrc = 0xC601,
    /// Host interface CRC overrun underrun.
    HostInterfaceCrcOverrunUnderrun = 0xC602,
    /// Host interface overrun underrun.
    HostInterfaceOverrunUnderrun = 0xC603,
    /// Host intruding command.
    HostIntrudingCommand = 0xC604,
    /// Host SATA CRC.
    HostSataCrc = 0xC641,
    /// Host SATA RX protocol.
    HostSataRxProtocol = 0xC642,
    /// Host SATA RX sync terminate.
    HostSataRxSyncTerminate = 0xC643,
    /// Host SATA RX length.
    HostSataRxLength = 0xC644,
    /// Host SATA TX RERR.
    HostSataTxRerr = 0xC645,
    /// Host SATA TX sync terminate.
    HostSataTxSyncTerminate = 0xC646,
    /// Host SATA retransmit.
    HostSataRetransmit = 0xC647,
    /// Host SATA TM FIFO.
    HostSataTmFifo = 0xC648,
    /// Host SATA disparity.
    HostSataDisparity = 0xC649,
    /// Host SATA code violation.
    HostSataCodeViolation = 0xC64A,
    /// Host SATA link hung.
    HostSataLinkHung = 0xC64B,
    /// Host SATA unrecognized FIS.
    HostSataUnrecognizedFis = 0xC64C,
    /// Host SATA unknown.
    HostSataUnknown = 0xC64D,
    /// Host SATA RX disparity in FIS.
    HostSataRxDisparityInFis = 0xC64E,
    /// Host SATA write transfer overrun.
    HostSataWriteTransferOverrun = 0xC64F,
    /// Host SATA data FIS too long.
    HostSataDataFisTooLong = 0xC650,
    /// Host SATA data FIS too short.
    HostSataDataFisTooShort = 0xC651,
    /// Host SATA HBCRC.
    HostSataHbcrc = 0xC652,
    /// Host SATA HBCRC and RERR.
    HostSataHbcrcAndRerr = 0xC653,
    /// Host SATA hidden HBCRC.
    HostSataHiddenHbcrc = 0xC654,
    /// Host SATA data FIS wrong size.
    HostSataDataFisWrongSize = 0xC655,
    /// Host SATA TM FIFO underrun.
    HostSataTmFifoUnderrun = 0xC656,
    /// Host SATA TM FIFO overrun.
    HostSataTmFifoOverrun = 0xC657,
    /// Device Configuration Overlay invalid feature set.
    DcoInvalidFeatureSet = 0xC800,
    /// Device Configuration Overlay freeze lock in place.
    DcoFreezeLockInPlace = 0xC801,
    /// Device Configuration Overlay checksum.
    DcoChecksum = 0xC802,
    /// Device Configuration Overlay signature.
    DcoSignature = 0xC803,
    /// Device Configuration Overlay invalid UDMA mode.
    DcoInvalidUdmaMode = 0xC804,
    /// Device Configuration Overlay command aborted.
    DcoCommandAborted = 0xC805,
    /// Device Configuration Overlay restore when drive is in factory state.
    DcoDcRestoreWhenDriveIsInFactoryState = 0xC806,
    /// Device Configuration Overlay set when drive is in reduced state.
    DcoDcSetWhenDriveIsInReducedState = 0xC807,
    /// Device Configuration Overlay restore when Host Protected Area is
    /// present.
    DcoDcRestoreWhenHpaIsPresent = 0xC808,
    /// Device Configuration Overlay set with invalid conditions.
    DcoDcSetWithInvalidConditions = 0xC809,
    /// Unsupported log address.
    UnsupportedLogAddress = 0xC820,
    /// Change Definition Command invalid password.
    ChangeDefinitionInvalidPassword = 0xC840,
    /// Change Definition Command invalid config number.
    ChangeDefinitionInvalidConfigNumber = 0xC841,
    /// Change Definition Command undefined capacity.
    ChangeDefinitionUndefinedCapacity = 0xC842,
    /// Change Definition Command illegal capacity.
    ChangeDefinitionIllegalCapacity = 0xC843,
    /// Change Definition Command counter maximum.
    ChangeDefinitionCounterMaximum = 0xC844,
    /// Change Definition Command invalid config select array.
    ChangeDefinitionInvalidConfigSelectArray = 0xC845,
    /// Change Definition Command not enabled.
    ChangeDefinitionNotEnabled = 0xC846,
    /// Change Definition Command native max LBA invalid.
    ChangeDefinitionNativeMaxLbaInvalid = 0xC847,
    /// Change Definition Command field list.
    ChangeDefinitionFieldList = 0xC848,
    /// Not an error Serial IO invalid non-ATA opcode.
    NotAnErrorSioInvalidNonAtaOpcode = 0xFFFE,
    /// Not an error VSC command execution in background.
    NotAnErrorVscCommandInBackground = 0xFFFF,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SctInvalidFunctionCode => write!(f, "SCT invalid function code"),
            Self::SctInvalidLba => write!(f, "SCT invalid LBA"),
            Self::SctRequestSectorCountOverflow => write!(f, "SCT request sector count overflow"),
            Self::SctInvalidErrorRecoveryFunctionCode => {
                write!(f, "SCT invalid error recovery function code")
            },
            Self::SctInvalidErrorRecoverySelectCode => {
                write!(f, "SCT invalid error recovery select code")
            },
            Self::SctHostReadTimerLessThanMinimum => {
                write!(f, "SCT host read timer less than minimum")
            },
            Self::SctHostWriteTimerLessThanMinimum => {
                write!(f, "SCT host write timer less than minimum")
            },
            Self::SctBackgroundCommandAbortByIntHost => {
                write!(f, "SCT background command abort by INT host command")
            },
            Self::SctBackgroundTerminatedUnrecoverableServo => write!(
                f,
                "SCT background command terminated unrecoverable servo error"
            ),
            Self::SctInvalidFunctionCodeInLongSectorAccess => {
                write!(f, "SCT invalid function code in long sector access command")
            },
            Self::SctDataTransferWithoutKeySector => write!(
                f,
                "SCT data transfer command issued without a key sector command"
            ),
            Self::SctInvalidFunctionCodeInFeatureControl => {
                write!(f, "SCT invalid function code in feature control command")
            },
            Self::SctInvalidFeatureCodeInFeatureControl => {
                write!(f, "SCT invalid feature code in feature control command")
            },
            Self::SctInvalidNewStateFeatureControlCommand => {
                write!(f, "SCT invalid new state feature control command")
            },
            Self::SctInvalidOptionFlagsInFeatureControl => write!(
                f,
                "SCT invalid option flags value in feature control command"
            ),
            Self::SctInvalidActionCode => write!(f, "SCT invalid action code"),
            Self::SctInvalidTableId => write!(f, "SCT invalid table ID"),
            Self::SctCommandAbortDriveSecurityLock => {
                write!(f, "SCT command abort drive security lock")
            },
            Self::SctInvalidRevisionCode => write!(f, "SCT invalid revision code"),
            Self::SctForegroundTerminatedUnrecoverable => {
                write!(f, "SCT foreground command terminated unrecoverable")
            },
            Self::SctTlerTimeoutStatus => write!(f, "SCT TLER timeout status"),
            Self::CtlrFwRequestCommandAbort => write!(f, "CTLR FW request command abort"),
            Self::IllegalDriveModel => write!(f, "illegal drive model"),
            Self::IllegalParameter0 => write!(f, "illegal parameter 0"),
            Self::IllegalParameter1 => write!(f, "illegal parameter 1"),
            Self::IllegalParameter2 => write!(f, "illegal parameter 2"),
            Self::IllegalParameter3 => write!(f, "illegal parameter 3"),
            Self::IllegalCommand => write!(f, "illegal command"),
            Self::FatalServo => write!(f, "fatal servo"),
            Self::Timeout => write!(f, "timeout"),
            Self::IndexNotFound => write!(f, "index not found"),
            Self::SectorCountMismatch => write!(f, "sector count mismatch"),
            Self::Sam => write!(f, "SAM"),
            Self::UnableToSpinUp => write!(f, "unable to spin-up"),
            Self::UnableToSpinDown => write!(f, "unable to spin-down"),
            Self::UnableToSpeed => write!(f, "unable to speed"),
            Self::BurstSyncFail => write!(f, "burst sync fail"),
            Self::UnlatchFail => write!(f, "unlatch fail"),
            Self::CtrPdSeekFail => write!(f, "CTR PD seek fail"),
            Self::BurstSlopeCalibrationFail => write!(f, "burst slope calibration fail"),
            Self::FlexBiasCalibrationFail => write!(f, "flex bias calibration fail"),
            Self::MotorTorqueCalibrationFail => write!(f, "motor torque calibration fail"),
            Self::RroCalibrationFail => write!(f, "RRO calibration fail"),
            Self::FarGainCalibrationFail => write!(f, "far gain calibration fail"),
            Self::Track0SeekFail => write!(f, "track 0 seek fail"),
            Self::RcServoGainCalibrationFail => write!(f, "RC servo gain calibration fail"),
            Self::RcLtrOlCalibrationFail => write!(f, "RC LTR OL calibration fail"),
            Self::SnapshotGainCalibrationFail => write!(f, "snapshot gain calibration fail"),
            Self::TangentialHeadOffsetCalibrationFail => {
                write!(f, "tangential head offset calibration fail")
            },
            Self::BandwidthCalibrationFail => write!(f, "bandwidth calibration fail"),
            Self::RroLearningOverflow => write!(f, "RRO learning overflow"),
            Self::RroLearning => write!(f, "RRO learning"),
            Self::SvirUnknownB6 => write!(f, "SVIR unknown B6"),
            Self::SvirUnknownD2 => write!(f, "SVIR unknown D2"),
            Self::SvirUnknownD4 => write!(f, "SVIR unknown D4"),
            Self::DriveProtectDiskOverlay => {
                write!(f, "Drive Protect disk error, Drive Protect Overlay")
            },
            Self::DriveProtectDisk1 => write!(f, "Drive Protect disk error, Drive Protect 1"),
            Self::DriveProtectDiskConfigSector => {
                write!(f, "Drive Protect disk config sector error")
            },
            Self::DriveProtectCommandInvalidCrcChecksum => {
                write!(f, "Drive Protect command invalid CRC checksum")
            },
            Self::DriveProtectCommandInvalidOpcode => {
                write!(f, "Drive Protect command invalid opcode")
            },
            Self::DriveProtectCommandKeysAlreadyLoaded => {
                write!(f, "Drive Protect command keys already loaded")
            },
            Self::DriveProtectCommandLoadZeroKey => {
                write!(f, "Drive Protect command load zero key")
            },
            Self::DriveProtectCommandInvalidConditions => {
                write!(f, "Drive Protect command invalid conditions")
            },
            Self::DriveProtectCommandInvalidSubOpcode => {
                write!(f, "Drive Protect command invalid sub opcode")
            },
            Self::DriveProtectCommandInvalidCustomerConfig => {
                write!(f, "Drive Protect command invalid customer config")
            },
            Self::DriveProtectDriveIsUnlocked => write!(f, "Drive Protect drive is unlocked"),
            Self::DriveProtectDriveIsLocked => write!(f, "Drive Protect drive is locked"),
            Self::DriveProtectEncryptionKeysNotLoaded => {
                write!(f, "Drive Protect encryption keys not loaded")
            },
            Self::DriveProtectDataShouldNotBeEncrypted => {
                write!(f, "Drive Protect data should not be encrypted")
            },
            Self::DriveProtectDataShouldBeEncrypted => {
                write!(f, "Drive Protect data should be encrypted")
            },
            Self::DriveProtectCommandSetZeroPassword => {
                write!(f, "Drive Protect command set zero password")
            },
            Self::DriveProtectLockPrepareNotSet => write!(f, "Drive Protect lock prepare not set"),
            Self::DriveProtectChangeKeyInLockCountdown => {
                write!(f, "Drive Protect change key in lock countdown")
            },
            Self::DriveProtectMismatchHrn => write!(f, "Drive Protect mismatch HRN"),
            Self::DriveProtectMismatchDrn => write!(f, "Drive Protect mismatch DRN"),
            Self::DriveProtectMismatchPassword => write!(f, "Drive Protect mismatch password"),
            Self::DriveProtectNoHrnOrDrnWasIssued => {
                write!(f, "Drive Protect no HRN or DRN was issued")
            },
            Self::DriveProtectNoPasswordSetInDrive => {
                write!(f, "Drive Protect no password set in drive")
            },
            Self::DriveProtectSetPasswordInLockCountdown => {
                write!(f, "Drive Protect set password in lock countdown")
            },
            Self::DriveProtectLockFailUpdateConfigOrFlush => {
                write!(f, "Drive Protect lock fail update config or flush error")
            },
            Self::VscModeDisabled => write!(f, "VSC mode disabled"),
            Self::ConfigWriteFailed => write!(f, "config write failed"),
            Self::ClearDrmSectionFailed => write!(f, "clear DRM section failed"),
            Self::SetConfigLbaFailed => write!(f, "set config LBA failed"),
            Self::FormatSetFailed => write!(f, "format set failed"),
            Self::ErrorRateTableNotPresent => write!(f, "error rate table not present"),
            Self::IbiModeBitNotSet => write!(f, "IBI mode bit not set"),
            Self::PermanentOverlayAlreadyLoaded => write!(f, "permanent overlay already loaded"),
            Self::PermanentOverlayNotLoaded => write!(f, "permanent overlay not loaded"),
            Self::InvalidDrmGroupSection => write!(f, "invalid DRM group section"),
            Self::InvalidDrmGroupQSubsection => write!(f, "invalid DRM group Q-subsection"),
            Self::DrmGroupQueueEmpty => write!(f, "DRM group queue empty"),
            Self::InvalidDrmGroupLSubsection => write!(f, "invalid DRM group L-subsection"),
            Self::InvalidDrmSubsection => write!(f, "invalid DRM subsection"),
            Self::InvalidLifeQueueFlag => write!(f, "invalid life queue flag"),
            Self::ProtocolViolation => write!(f, "protocol violation"),
            Self::DepopDisabled => write!(f, "depop disabled"),
            Self::DepopInvalidApb => write!(f, "depop invalid APB"),
            Self::LastBackgroundVscCommandFailed => write!(
                f,
                "last background VSC command failed; see secondary error code for the extended \
                 error"
            ),
            Self::OdtaaNotInitialized => write!(f, "ODTAA not initialized"),
            Self::TrackListDataNotAvailable => write!(f, "track list data not available"),
            Self::OdtaaReadWatchdog => write!(f, "ODTAA read watchdog"),
            Self::RroNotInitialized => write!(f, "RRO not initialized"),
            Self::DrmSpinCountersReadFail => write!(f, "DRM spin counters read fail"),
            Self::DrmActuatorCountersReadFail => write!(f, "DRM actuator counters read fail"),
            Self::StaticFileInvalid => write!(f, "static file invalid"),
            Self::MrmRequestFailed => write!(f, "MRM request failed"),
            Self::MrmRequestTimedOut => write!(f, "MRM request timed out"),
            Self::MrmRequestRejected => write!(f, "MRM request rejected"),
            Self::MrmRequestCanceled => write!(f, "MRM request canceled"),
            Self::MrmStartCanceled => write!(f, "MRM start canceled"),
            Self::MrmWaitCanceled => write!(f, "MRM wait canceled"),
            Self::ResourceAllocationFailed => write!(f, "RSC allocation failed"),
            Self::DrmLogNotLoaded => write!(f, "DRM log not loaded"),
            Self::DfhCalibrationFailed => write!(f, "DFH calibration failed"),
            Self::DrmLogSectionsBadChecksum => write!(f, "DRM log sections bad checksum"),
            Self::VscDlg2NotActive => write!(f, "VSC DLG2 not active"),
            Self::DrmPeriodLogBadChecksum => write!(f, "DRM log period log bad checksum"),
            Self::SmartReadSelfTestLog => write!(f, "SMART read self-test log"),
            Self::SmartDrmLogHasNotBeenLoaded => write!(f, "SMART DRM log has not been loaded"),
            Self::SmartDrmLoadDisabled => write!(f, "SMART DRM load disabled"),
            Self::SmartLogPageA3NotLoaded => write!(f, "SMART log page A3 not loaded"),
            Self::CacheFlushCachedRelocationDisk => {
                write!(f, "cache flush cached relocation got disk error")
            },
            Self::CacheCannotInvalidateSdInDynamicState => {
                write!(f, "cache cannot invalidate SD in dynamic state")
            },
            Self::ResourceAllocationSectorsBeyondRange => {
                write!(f, "RSC allocation sectors beyond range")
            },
            Self::ResourceAllocationDisk => write!(f, "RSC allocation got disk"),
            Self::ResourceAllocationNoContiguousBuffer => {
                write!(f, "RSC allocation no contiguous buffer")
            },
            Self::ResourceAllocateTransientWithCacheValid => {
                write!(f, "RSC allocate transient with cache valid")
            },
            Self::ResourceAllocateTransientBufferUsed => {
                write!(f, "RSC allocate transient buffer used")
            },
            Self::ResourceAllocateSdNoneAvailable => write!(f, "RSC allocate SD, none available"),
            Self::ResourceAllocationNoBuffersAvailable => {
                write!(f, "RSC allocation no buffers available")
            },
            Self::ResourceAllocationDiskDbs => write!(f, "RSC allocation got disk error DBS"),
            Self::ResourceAllocateTdNoneAvailable => write!(f, "RSC allocate TD, none available"),
            Self::FmFileInfo => write!(f, "FM file info"),
            Self::FmDirectory => write!(f, "FM directory"),
            Self::FmFileId => write!(f, "FM file ID"),
            Self::FmChecksum => write!(f, "FM checksum"),
            Self::FmCompatibility => write!(f, "FM compatibility"),
            Self::FmTimeout => write!(f, "FM timeout"),
            Self::FmNotStaticFile => write!(f, "FM not static file"),
            Self::FmNoBuffer => write!(f, "FM no buffer"),
            Self::FmDriveNotReady => write!(f, "FM drive not ready"),
            Self::FmFileHeaderSizeZero => write!(f, "FM file header size zero"),
            Self::FmIncompatibleVersion => write!(f, "FM incompatible version"),
            Self::FmNotFlashFile => write!(f, "FM not flash file"),
            Self::FmCannotRenameToExistingFileId => {
                write!(f, "FM cannot rename to existing file ID")
            },
            Self::FmFilesGoodButChecksumsDifferent => {
                write!(f, "FM files good but checksums different")
            },
            Self::FmHeaderIiSignatureInvalid => write!(f, "FM header II signature invalid"),
            Self::FmHeaderIiTooSmall => write!(f, "FM header II too small"),
            Self::FmPartialFileRequestInvalid => write!(f, "FM partial file request invalid"),
            Self::FmSectorCountExceedsMaxAllocatedBuffer => {
                write!(f, "FM sector count exceeds maximum allocated buffer")
            },
            Self::FmFileSizeExceedsStaticBuffer => {
                write!(f, "FM file size exceeds static buffer size")
            },
            Self::FmPartialFileBufferOffsetExceedsEof => {
                write!(f, "FM partial file buffer offset exceeds EOF")
            },
            Self::FmPartialFileSectorCountExceedsEof => {
                write!(f, "FM partial file sector count exceeds EOF")
            },
            Self::FmNoSdCreatedForTheRequestedFileId => {
                write!(f, "FM no SD created for the requested file ID")
            },
            Self::FmNoMoreSpaceInDirectory => write!(f, "FM no more space in directory"),
            Self::FmNumberCopyMoreThanMaximum => write!(f, "FM number copy more than maximum copy"),
            Self::FmNoMoreSpaceInRegion => write!(f, "FM no more space in region"),
            Self::FmFileCannotCreateOnExistingFile => {
                write!(f, "FM file cannot create on existing file")
            },
            Self::FmFileEntryNotFoundInDirectorySector => {
                write!(f, "FM file entry number not found in directory sector")
            },
            Self::FmTryingToCopyToSameRegion => write!(f, "FM trying to copy to same region"),
            Self::FmFileInitializationPlaceholderBitNotSet => {
                write!(f, "FM file initialization placeholder bit not set")
            },
            Self::FmFileTargetRlbaOverlap => write!(f, "FM file target RLBA overlap"),
            Self::FmNoContiguousSpaceInRegion => write!(f, "FM no contiguous space in region"),
            Self::FmCannotResolveOverlap => write!(f, "FM cannot resolve overlap"),
            Self::FmCannotFreeEnoughSpace => write!(f, "FM cannot free enough space"),
            Self::FmCannotCreateContiguousSpaceInRegion => {
                write!(f, "FM cannot create contiguous space in region")
            },
            Self::FmRequestRlbaExceedRegionBoundary => {
                write!(f, "FM request RLBA exceed region boundary")
            },
            Self::FmDirectoryEntryNotSame => write!(f, "FM directory entry not same"),
            Self::FmDefragDetectPacketFileInReserved => {
                write!(f, "FM defrag detect packet file in reserved")
            },
            Self::FmGatherFieldFile => write!(f, "FM gather field file"),
            Self::FmSortDirectorySector => write!(f, "FM sort directory sector"),
            Self::FmDefragReserved => write!(f, "FM defrag reserved"),
            Self::FmDirectoryWhileDelete => write!(f, "FM directory error while delete"),
            Self::FmFileNotFoundInAnyDirectory3742
            | Self::FmFileNotFoundInAnyDirectory3743
            | Self::FmFileNotFoundInAnyDirectory3746 => {
                write!(f, "FM file not found in any directory")
            },
            Self::FmFile6FStructureIncorrect => write!(f, "FM file 6F structure incorrect"),
            Self::BackgroundPstAbortedByReset => write!(f, "BG PST aborted by reset"),
            Self::BackgroundPstAbortedByDeadman => write!(f, "BG PST aborted by deadman"),
            Self::BackgroundInvalidAscanConfigParameters => {
                write!(f, "BG invalid ASCAN config parameters")
            },
            Self::BackgroundTrackListDataNotAvailable => {
                write!(f, "BG track list data not available")
            },
            Self::BackgroundPstUnableToLoadPtm => write!(f, "BG PST unable to load PTM"),
            Self::BackgroundPstResourceAllocationFailed => {
                write!(f, "BG PST RSC allocation failed")
            },
            Self::BackgroundPstDisabledViaDbs => write!(f, "BG PST disabled via DBS"),
            Self::BackgroundProcessingDisabled => write!(f, "BG processing disabled"),
            Self::BackgroundPstInvalidPtmLoadAddress => {
                write!(f, "BG PST invalid PTM load address")
            },
            Self::BackgroundPstInvalidPtmStartAddress => {
                write!(f, "BG PST invalid PTM start address")
            },
            Self::BackgroundNonCaptiveMemoryTestNotAllowed => {
                write!(f, "BG non-captive memory test not allowed")
            },
            Self::BackgroundPstUnableToFlashPtm => write!(f, "BG PST unable to flash PTM"),
            Self::BackgroundSelfTestAbortedTimedOut => write!(f, "BG self-test aborted timed out"),
            Self::OvmPermanentAlreadyLoaded => write!(f, "OVM permanent already loaded"),
            Self::OvmTransientAlreadyLoaded => write!(f, "OVM transient already loaded"),
            Self::OvmPermanentNotLoaded => write!(f, "OVM permanent not loaded"),
            Self::OvmTransientNotLoaded => write!(f, "OVM transient not loaded"),
            Self::OvmNotCompatible => write!(f, "OVM not compatible"),
            Self::OvmChecksum => write!(f, "OVM checksum"),
            Self::OvmUndefinedFunction => write!(f, "OVM undefined function"),
            Self::OvmBuildIdMismatch => write!(f, "OVM build ID mismatch"),
            Self::FlashUnknown => write!(f, "flash unknown"),
            Self::FlashInvalidSectorAddress => write!(f, "flash invalid flash sector address"),
            Self::FlashWriteLatchEnable => write!(f, "flash write latch enable"),
            Self::FlashWritePageSendByte => write!(f, "flash write page send byte"),
            Self::FlashWritePageTimeout => write!(f, "flash write page timeout"),
            Self::FlashWritePageToStaticMemory => {
                write!(f, "flash write page to static memory error")
            },
            Self::FlashReadBlockGetByte => write!(f, "flash read block get byte"),
            Self::FlashByteCountExceedsDeviceLimit => {
                write!(f, "flash byte count exceeds device limit")
            },
            Self::FlashInvalidAddress => write!(f, "flash invalid flash address"),
            Self::FlashInitialBootHeaderMissing => write!(f, "flash initial boot header missing"),
            Self::FlashSendByteTimeout => write!(f, "flash send byte timeout"),
            Self::FlashReadCommand => write!(f, "flash read command"),
            Self::FlashInvalidData => write!(f, "flash invalid flash data"),
            Self::FlashDataCompare => write!(f, "flash data compare"),
            Self::FlashDeviceId => write!(f, "flash device ID"),
            Self::FlashReadInfoStartTimeout => write!(f, "flash read info start timeout"),
            Self::FlashStatusTimeout => write!(f, "flash status timeout"),
            Self::FlashCommandTimeout => write!(f, "flash command timeout"),
            Self::DptmMallocFailure => write!(f, "DPTM malloc failure"),
            Self::DptmFreeFailure => write!(f, "DPTM free failure"),
            Self::DptmInitializationFailure => write!(f, "DPTM initialization failure"),
            Self::DptmFailedToProcessDownloadedPacket => {
                write!(f, "DPTM failed to process downloaded packet")
            },
            Self::DptmFailedToBackUpFlashFiles => write!(f, "DPTM failed to back up flash files"),
            Self::DptmPreservedFileHasDiffVersionOrSize => {
                write!(f, "DPTM preserved file has different version or size")
            },
            Self::DptmUnhandledFileListException => write!(f, "DPTM unhandled file list exception"),
            Self::DptmFileIdNotInList => write!(f, "DPTM file ID not in file list"),
            Self::DptmFlashWriteBufferInvalid => write!(f, "DPTM flash write buffer invalid"),
            Self::DptmFlashImageTooBig => write!(f, "DPTM flash image too big"),
            Self::DptmFlashProgramFailure => write!(f, "DPTM flash program failure"),
            Self::DptmFailedToVerifyProgrammedFlash => {
                write!(f, "DPTM failed to verify programmed flash")
            },
            Self::DptmFailedToApplyModBytes => write!(f, "DPTM failed to apply mod bytes"),
            Self::DptmCleanupFailure => write!(f, "DPTM cleanup failure"),
            Self::DptmFailedToWriteConfig => write!(f, "DPTM failed to write config"),
            Self::DptmPacketDoesNotContainFileList => {
                write!(f, "DPTM packet does not contain file list")
            },
            Self::DptmTryToRenameToAFileThatExisted => {
                write!(f, "DPTM try to rename to a file that existed")
            },
            Self::DptmExceedsMaximumUndoList => write!(f, "DPTM exceeds maximum undo list"),
            Self::DptmReplaceOriginalFileNotExist => {
                write!(f, "DPTM replace original file not exist")
            },
            Self::DptmReplaceOriginalFileDifferentFileId => {
                write!(f, "DPTM replace original file with different file ID")
            },
            Self::DptmFlashDirectoryNotFoundInFlashFile => {
                write!(f, "DPTM flash directory not found in flash file")
            },
            Self::DptmAc55UccmTotalBytesExpectedMismatch => {
                write!(f, "DPTM AC55 UCCM total bytes expected mismatch")
            },
            Self::DptmAc55NewFile118NotIdenticalToOldOne => {
                write!(f, "DPTM AC55 new file 118 not identical to old one")
            },
            Self::DptmAc55InputNoKeySectorsDefined => {
                write!(f, "DPTM AC55 input no key sectors defined")
            },
            Self::DptmAc55InputInvalidActionCode => {
                write!(f, "DPTM AC55 input invalid action code")
            },
            Self::DptmAc55InputInvalidFunctionCodeRequest => {
                write!(f, "DPTM AC55 input invalid function code request")
            },
            Self::DptmAc55InputInvalidConfigCode => {
                write!(f, "DPTM AC55 input invalid config code")
            },
            Self::DptmAc55InputCacheFamilyMismatch => {
                write!(f, "DPTM AC55 input cache family mismatch")
            },
            Self::DptmAc55MajorRevisionMismatch => write!(f, "DPTM AC55 major revision mismatch"),
            Self::DptmAc55FwStructureRevisionMismatch => {
                write!(f, "DPTM AC55 FW structure revision mismatch")
            },
            Self::RseekMallocFailure => write!(f, "RSEEK malloc failure"),
            Self::RseekFreeFailure => write!(f, "RSEEK free failure"),
            Self::SelfTestCheckResFile => write!(f, "self-test check resident file"),
            Self::SelfTestScan => write!(f, "self-test scan"),
            Self::SelfTestSramHard => write!(f, "self-test SRAM hard"),
            Self::SelfTestSramSoft => write!(f, "self-test SRAM soft"),
            Self::SelfTestSramMultiSoft => write!(f, "self-test SRAM multi-soft"),
            Self::SelfTestDramHard => write!(f, "self-test DRAM hard"),
            Self::SelfTestDramSoft => write!(f, "self-test DRAM soft"),
            Self::SelfTestDramMultiSoft => write!(f, "self-test DRAM multi-soft"),
            Self::SelfTestTransientLoadFault => write!(f, "self-test transient load fault"),
            Self::FmtPlistNotFound => write!(f, "FMT P-list not found"),
            Self::FmtInvalidPlist => write!(f, "FMT invalid P-list"),
            Self::FmtGlistNotFound => write!(f, "FMT G-list not found"),
            Self::FmtInvalidGlist => write!(f, "FMT invalid G-list"),
            Self::FmtExceededPushDowns => write!(f, "FMT exceeded push downs"),
            Self::FmtPushDownListWriteFail => write!(f, "FMT push down list write fail"),
            Self::FmtNewBadTracks => write!(f, "FMT new bad tracks"),
            Self::FmtFailure => write!(f, "FMT failure"),
            Self::FmtExceededGlist => write!(f, "FMT exceeded G-list"),
            Self::FmtGlistWriteFail => write!(f, "FMT G-list write fail"),
            Self::FmtCapacity => write!(f, "FMT capacity"),
            Self::FmtZsdNotLoaded => write!(f, "FMT ZSD not loaded"),
            Self::FmtBufferAllocation => write!(f, "FMT buffer allocation"),
            Self::FmtRelocationListWriteFail => write!(f, "FMT relocation list write fail"),
            Self::FmtPushDownListNotFound => write!(f, "FMT push down list not found"),
            Self::FmtRelocationListNotFound => write!(f, "FMT relocation list not found"),
            Self::FmtExceededRelocationList => write!(f, "FMT exceeded relocation list"),
            Self::FmtWriteFail => write!(f, "FMT write fail"),
            Self::FmtExceededReservedPushDownList => {
                write!(f, "FMT exceeded reserved push down list")
            },
            Self::FmtReservedPushDownListWriteFail => {
                write!(f, "FMT reserved push down list write fail")
            },
            Self::FmtReservedPushDownListNotLoaded => {
                write!(f, "FMT reserved push down list not loaded")
            },
            Self::FmtPlistPsnOutOfRange => write!(f, "FMT P-list PSN out of range"),
            Self::FmtPushCountOverflow => write!(f, "FMT push count overflow"),
            Self::FmtHashTableOverflow => write!(f, "FMT hash table overflow"),
            Self::FmtInvalidZoneTable => write!(f, "FMT invalid zone table"),
            Self::FmtCannotMergePAndGList => write!(f, "FMT cannot merge P and G list"),
            Self::FmtPlistWriteFail => write!(f, "FMT P-list write fail"),
            Self::FmtSlipsExceedLimit => write!(f, "FMT slips exceed limit"),
            Self::FmtTrackPushedDown => write!(f, "FMT track pushed down"),
            Self::FmtFieldListWriteFail => write!(f, "FMT field list write fail"),
            Self::FmtPlistCylinderOutOfRange => write!(f, "FMT P-list cylinder out of range"),
            Self::FmtPlistHeadOutOfRange => write!(f, "FMT P-list head out of range"),
            Self::FmtRemergeRequired => write!(f, "FMT remerge required"),
            Self::FmtMlistWriteFail => write!(f, "FMT M-list write fail"),
            Self::DflDefectFound => write!(f, "defect list defect found"),
            Self::DflNoDefect => write!(f, "defect list no defect"),
            Self::DflTrackDefectFound => write!(f, "defect list track defect found"),
            Self::DflEmpty => write!(f, "defect list empty"),
            Self::DflNoMemory => write!(f, "defect list no memory"),
            Self::DflPlistWrite => write!(f, "defect list P-list write"),
            Self::DflGlistWrite => write!(f, "defect list G-list write"),
            Self::DflPushDownListWrite => write!(f, "defect list push down list write"),
            Self::DflRelocationListWrite => write!(f, "defect list relocation list write"),
            Self::DflGlistFull => write!(f, "defect list G-list full"),
            Self::DflPlistFull => write!(f, "defect list P-list full"),
            Self::DflClistFull => write!(f, "defect list C-list full"),
            Self::DflPlistDefect => write!(f, "defect list P-list defect"),
            Self::DflGlistDefect => write!(f, "defect list G-list defect"),
            Self::DflNoList => write!(f, "defect list no list"),
            Self::DflDef1LessThan => write!(f, "defect list DEF1 less than"),
            Self::DflDef1GreaterThan => write!(f, "defect list DEF1 greater than"),
            Self::DflDef1Equal => write!(f, "defect list DEF1 equal"),
            Self::DflInvalidLba => write!(f, "defect list invalid LBA"),
            Self::DflInvalidLbaRange => write!(f, "defect list invalid LBA range"),
            Self::DflDuplicateDefect => write!(f, "defect list duplicate defect"),
            Self::DflDlistFull => write!(f, "defect list D-list full"),
            Self::DflMlistFull => write!(f, "defect list M-list full"),
            Self::DflVfsBufferConversion => write!(f, "defect list VFS buffer conversion"),
            Self::CacheRelocationSpareRwTimeout => write!(f, "cache relocation spare RW timeout"),
            Self::CacheRelocationInsufficientSpace => {
                write!(f, "cache relocation insufficient cache space")
            },
            Self::CacheRelocationReadDoesNotExist => {
                write!(f, "cache relocation read relocation does not exist")
            },
            Self::CacheRelocationWriteDoesNotExist => {
                write!(f, "cache relocation write relocation does not exist")
            },
            Self::CacheRelocationReadNewFailure => {
                write!(f, "cache relocation read new relocation failure")
            },
            Self::CacheRelocationLoadTrackFailure => {
                write!(f, "cache relocation load track cache failure")
            },
            Self::CacheRelocationRwInProgress => write!(f, "cache relocation RW in progress"),
            Self::DmTranslationOutOfRange => write!(f, "DM translation out of range"),
            Self::DmSectorOutOfRange => write!(f, "DM sector out of range"),
            Self::DmRmNotASpareRelocation => write!(f, "DM RM not a spare relocation"),
            Self::DmRmNotASpareLba => write!(f, "DM RM not a spare LBA"),
            Self::DmRmNotARelocationListEntry => write!(f, "DM RM not a relocation list entry"),
            Self::DmRmNotAUserLba => write!(f, "DM RM not a user LBA"),
            Self::DmRmInsertingInFullRelocationList => {
                write!(f, "DM RM inserting in full relocation list")
            },
            Self::DmRmInsertingAnExistingLba => write!(f, "DM RM inserting an existing LBA"),
            Self::DmHeadCheckWrongCylinderForReservedArea => {
                write!(f, "DM head check wrong cylinder for reserved area")
            },
            Self::DmHeadCheckWrongHeadForUserArea => {
                write!(f, "DM head check wrong head for user area")
            },
            Self::DmRmNoMoreSparesForCacheRelocation => {
                write!(f, "DM RM no more spares for cache relocation")
            },
            Self::DmRmInsertingWuInFullRelocationList => {
                write!(f, "DM RM inserting WU in full relocation list")
            },
            Self::FmtReservedAreaPushDownListOverflow => {
                write!(f, "FMT reserved area reserved push down list overflow")
            },
            Self::FmtReservedAreaPlistPsnOutOfRange => {
                write!(f, "FMT reserved area P-list PSN out of range")
            },
            Self::FmtReservedAreaPlistFileIdInvalid => {
                write!(f, "FMT reserved area P-list file ID invalid")
            },
            Self::FmtReservedAreaHashBlockEmpty => write!(f, "FMT reserved area hash block empty"),
            Self::FmtReservedAreaHeadCountZero => write!(f, "FMT reserved area head count zero"),
            Self::FmtReservedAreaZoneTableSptZero => {
                write!(f, "FMT reserved area zone table SPT zero")
            },
            Self::FmtReservedAreaNoSparesAvailable => {
                write!(f, "FMT reserved area no spares available")
            },
            Self::FmtReservedAreaRegionCrossesHeadBoundary => {
                write!(f, "FMT reserved area region crosses head boundary")
            },
            Self::FmtReservedAreaRegionTooLarge => write!(f, "FMT reserved area region too large"),
            Self::FmtReservedAreaTwoRegionOnSameVirtualHead => {
                write!(f, "FMT reserved area two region on same virtual head")
            },
            Self::FmtReservedAreaIncorrectRaspControl => {
                write!(f, "FMT reserved area incorrect RASP control parameter")
            },
            Self::FmtReservedAreaInvalidFilesEncountered => {
                write!(f, "FMT reserved area invalid files encountered")
            },
            Self::FmtReservedAreaAltRpdListSizeMismatch => write!(
                f,
                "FMT reserved area alt reserved push down list size mismatch"
            ),
            Self::FmtReservedAreaInvalidTargetSize => {
                write!(f, "FMT reserved area invalid target size")
            },
            Self::FmtReservedAreaInvalidRaspRegionTable => {
                write!(f, "FMT reserved area invalid RASP region table")
            },
            Self::FmtReservedAreaInvalidTargetTable => {
                write!(f, "FMT reserved area invalid target table")
            },
            Self::FmtReservedAreaRaspInMiddleOfSurface => {
                write!(f, "FMT reserved area RASP in middle of surface")
            },
            Self::MemoryTestDataBus => write!(f, "memory test data bus"),
            Self::MemoryTestAddressBus => write!(f, "memory test address bus"),
            Self::MemoryTestDeviceBus => write!(f, "memory test device bus"),
            Self::HostDlmcBadCompChar => write!(f, "host download microcode bad COMP char"),
            Self::HostDlmcBadChecksum => write!(f, "host download microcode bad checksum"),
            Self::HostDlmcInvalidPacket => write!(f, "host download microcode invalid packet"),
            Self::HostDlmcInvalidProdFamily => {
                write!(f, "host download microcode invalid product family")
            },
            Self::HostDlmcInvalidTpiCode => write!(f, "host download microcode invalid TPI code"),
            Self::HostDlmcInvalidSectionOffset => {
                write!(f, "host download microcode invalid section offset")
            },
            Self::HostDlmcInvalidCustomerId => {
                write!(f, "host download microcode invalid customer ID")
            },
            Self::HostDlmcTransferTooLarge => {
                write!(f, "host download microcode total transferred too large")
            },
            Self::HostDlmcSavingDriveState => {
                write!(f, "host download microcode failed saving drive state")
            },
            Self::HostDlmcSizeOutOfRange => {
                write!(f, "host download microcode transfer size out of range")
            },
            Self::HostDlmcInvalidPtmStartAddress => {
                write!(f, "host download microcode invalid PTM start address")
            },
            Self::HostDlmcNeedModule19EAnd19D => {
                write!(f, "host download microcode need module 19E and 19D")
            },
            Self::HostLatchedFatalWriteFault => write!(f, "host latched fatal write fault"),
            Self::HostResidentFilesNotLoaded => write!(f, "host resident files not loaded"),
            Self::HostDbsOccurred => write!(f, "host DBS occurred"),
            Self::HostDlmc => write!(f, "host download microcode"),
            Self::HostDlmcNoSd => write!(f, "host download microcode no SD"),
            Self::HostCacheOverlayNotLoaded => write!(f, "host cache overlay not loaded"),
            Self::HostDlmcNoPtmCode => write!(f, "host download microcode no PTM code"),
            Self::HostAbortedCommand => write!(f, "host aborted command"),
            Self::HostTransferCancel => write!(f, "host transfer cancel"),
            Self::CacheFlushFailure1 => write!(f, "cache flush failure 1"),
            Self::CacheFlushFailure2 => write!(f, "cache flush failure 2"),
            Self::CommandTimeout => write!(f, "command timeout"),
            Self::CommandTimeoutAvCcto => write!(f, "command timeout AV CCTO"),
            Self::CommandTimeoutSaStreaming => write!(f, "command timeout SA streaming"),
            Self::CommandTimeoutTler => write!(f, "command timeout TLER"),
            Self::CommandTimeoutReadTler => write!(f, "command timeout read TLER"),
            Self::CommandTimeoutWriteTler => write!(f, "command timeout write TLER"),
            Self::CommandTimeoutFlushTler => write!(f, "command timeout flush TLER"),
            Self::CommandTimeoutReadTlerNetApp => write!(f, "command timeout read TLER NetApp"),
            Self::CommandTimeoutWriteTlerNetApp => write!(f, "command timeout write TLER NetApp"),
            Self::CommandTimeoutFlushTlerNetApp => write!(f, "command timeout flush TLER NetApp"),
            Self::DlgInvalidTestTrackLba => write!(f, "DLG invalid test track LBA"),
            Self::DlgNoWarehouseTracksExist => write!(f, "DLG no warehouse tracks exist"),
            Self::DlgNotEnoughBufferForCheckpointRecovery => {
                write!(f, "DLG not enough buffer for checkpoint recovery")
            },
            Self::DlgNoValidHeaderFound => write!(f, "DLG no valid header found"),
            Self::DlgInvalidCheckPointFound => write!(f, "DLG invalid check point found"),
            Self::DlgCheckpointHeaderRead => write!(f, "DLG checkpoint header read"),
            Self::DlgCheckpointTrackRead => write!(f, "DLG checkpoint track read"),
            Self::DlgNotEnoughResourcesAllocation => {
                write!(f, "DLG not enough resources allocation")
            },
            Self::DlgDiskRequestTimedOut => write!(f, "DLG disk request timed out"),
            Self::AccessDenied => write!(f, "access denied, no access rights"),
            Self::DiskEccCorrected => write!(f, "disk ECC corrected"),
            Self::DiskTaDetectStatus => write!(f, "disk TA detect status"),
            Self::DiskTa2ndSyncMark => write!(f, "disk TA 2nd sync mark"),
            Self::DiskUnsafe2ndSyncMark => write!(f, "disk unsafe 2nd sync mark"),
            Self::DiskFifoOverUnder => write!(f, "disk FIFO overrun or underrun"),
            Self::DiskFifoOverrun => write!(f, "disk FIFO overrun"),
            Self::DiskFifoUnderrun => write!(f, "disk FIFO underrun"),
            Self::DiskSectorPulseRg => write!(f, "disk sector pulse RG"),
            Self::DiskSectorPulseWg => write!(f, "disk sector pulse WG"),
            Self::DiskEccDataSize => write!(f, "disk ECC data size"),
            Self::DiskRgOverServo => write!(f, "disk RG over servo"),
            Self::DiskDam => write!(f, "disk DAM"),
            Self::DiskDamTa => write!(f, "disk DAM TA"),
            Self::DiskSectorPulseRgRecovered => write!(f, "disk sector pulse RG error recovered"),
            Self::DiskEccDataSizeRecovered => write!(f, "disk ECC data size error recovered"),
            Self::DiskRgOverServoRecovered => write!(f, "disk RG over servo error recovered"),
            Self::DiskSpba => write!(f, "disk SPBA"),
            Self::DiskTimeoutSectorNotFound => write!(f, "disk timeout sector not found"),
            Self::DiskCrc => write!(f, "disk CRC"),
            Self::DiskEcuUnsafe => write!(f, "disk ECU unsafe"),
            Self::DiskEcuUnsafeTa => write!(f, "disk ECU unsafe TA"),
            Self::DiskFwEccFailure51A4 | Self::DiskFwEccFailure5204 => {
                write!(f, "disk firmware ECC failure")
            },
            Self::DiskEcuWuPseudoLog => write!(f, "disk ECU WU pseudo log"),
            Self::DiskEcuWuPseudoNotLogged => write!(f, "disk ECU WU pseudo not logged"),
            Self::DiskEcuWuFlaggedLog => write!(f, "disk ECU WU flagged log"),
            Self::DiskEcuWuFlaggedNotLogged => write!(f, "disk ECU WU flagged not logged"),
            Self::DiskEcuTransferHwAssistRecovered => {
                write!(f, "disk ECU transfer hardware assist recovered")
            },
            Self::DiskWrite => write!(f, "disk write"),
            Self::DiskReadCrc => write!(f, "disk read CRC"),
            Self::DiskWriteCrc => write!(f, "disk write CRC"),
            Self::DiskLoggedCrc => write!(f, "disk logged CRC"),
            Self::DiskWriteRllCrc => write!(f, "disk write RLL CRC"),
            Self::DiskTimeoutDmNotActive => write!(f, "disk timeout DM not active"),
            Self::DiskTimeoutDmNotActiveRead => write!(f, "disk timeout DM not active read"),
            Self::DiskTimeoutDmNotActiveWrite => write!(f, "disk timeout DM not active write"),
            Self::DiskTimeoutBufferNotReady => write!(f, "disk timeout buffer not ready"),
            Self::DiskTimeoutDf => write!(f, "disk timeout DF"),
            Self::DiskGeneral => write!(f, "disk general"),
            Self::DiskTimeoutTler => write!(f, "disk timeout TLER"),
            Self::DiskEventTimeoutDf => write!(f, "disk event timeout DF"),
            Self::DiskBufferFull => write!(f, "disk buffer full"),
            Self::DiskTimeoutSeekNotStarted => write!(f, "disk timeout seek not started"),
            Self::DiskRecalibrationFailure => write!(f, "disk recalibration failure"),
            Self::ToneScanDefectBufferOverflow => {
                write!(f, "tone scan defect buffer overflow error")
            },
            Self::DiskWedgeCommandInProgress => write!(f, "disk wedge command in progress"),
            Self::DiskSpinUpTimeout1 => write!(f, "disk spin-up timeout 1"),
            Self::DiskSpinUpTimeout2 => write!(f, "disk spin-up timeout 2"),
            Self::DiskWedgeInvalidCount => write!(f, "disk wedge invalid wedge count"),
            Self::DiskRelocationPermanentOverlayNotLoaded => {
                write!(f, "disk relocation permanent overlay not loaded")
            },
            Self::RelocationSstFailed => write!(f, "relocation SST failed"),
            Self::RelocationNotPossibleOnSpare => write!(f, "relocation not possible on spare"),
            Self::RelocationInvalidRequestCount => write!(f, "relocation invalid request count"),
            Self::RelocationNotPossibleOnReserved => {
                write!(f, "relocation not possible on reserved")
            },
            Self::DiskStopOnCreateRelocation => write!(f, "disk stop on create relocation"),
            Self::DiskStopOnCreateTare => write!(f, "disk stop on create TARE"),
            Self::RelocationDisabled => write!(f, "relocation disabled"),
            Self::RelocationDisabledFormatUnitNotRun => {
                write!(f, "relocation disabled, format unit was not run")
            },
            Self::RelocationInPostReadCanceled => write!(f, "relocation in post read canceled"),
            Self::RelocationLastLbaInUserArea => write!(f, "relocation last LBA in user area"),
            Self::RelocationDisabledNotInitialized => {
                write!(f, "relocation disabled not initialized")
            },
            Self::RelocationEcSpareLbaWithNoUserLba => {
                write!(f, "relocation EC spare LBA with no user LBA")
            },
            Self::PmIllegalModeTransition => write!(f, "PM illegal mode transition"),
            Self::PmTransitionNotNeededOk => write!(f, "PM transition not needed ok"),
            Self::DiskChannelHeadSizeInitialization => {
                write!(f, "disk channel head size initialization error")
            },
            Self::DiskServo => write!(f, "disk servo"),
            Self::DiskServoNotReady => write!(f, "disk servo not ready"),
            Self::DiskServoHeadNotFound => write!(f, "disk servo head not found"),
            Self::DiskServoSpindle => write!(f, "disk servo spindle"),
            Self::DiskServoSpindleSpinUp => write!(f, "disk servo spindle spin-up"),
            Self::DiskServoSpindleSpinDown => write!(f, "disk servo spindle spin-down"),
            Self::DiskServoSpindleOffSpeed => write!(f, "disk servo spindle off speed"),
            Self::DiskServoSpindleSpinUpFatal => {
                write!(f, "disk servo spindle spin-up fatal error")
            },
            Self::DiskServoSpindleGetSpinUpTime => {
                write!(f, "disk servo spindle get spin-up time error")
            },
            Self::DiskServoActuator => write!(f, "disk servo actuator"),
            Self::DiskServoActuatorDriveFault => write!(f, "disk servo actuator drive fault"),
            Self::DiskServoActuatorAbort => write!(f, "disk servo actuator abort"),
            Self::DiskServoActuatorWriteInhibit => {
                write!(f, "disk servo actuator write inhibit error")
            },
            Self::DiskServoActuatorControlFault => {
                write!(f, "disk servo actuator control fault error")
            },
            Self::DiskServoActuatorShockFault => write!(f, "disk servo actuator shock fault"),
            Self::DiskServoActuatorWriteUnsafe => {
                write!(f, "disk servo actuator write unsafe error")
            },
            Self::DiskServoActuatorFault => write!(f, "disk servo actuator servo fault"),
            Self::DiskServoActuatorWgMask => write!(f, "disk servo actuator WG mask"),
            Self::DiskServoActuatorFatal => write!(f, "disk servo actuator fatal"),
            Self::DiskServoActuatorTimeout => write!(f, "disk servo actuator timeout"),
            Self::DiskServoActuatorSail => write!(f, "disk servo actuator SAIL"),
            Self::DiskServoActuatorBadWedge => write!(f, "disk servo actuator bad wedge"),
            Self::DiskServoActuatorNoScTargetWedge => {
                write!(f, "disk servo actuator no SC target wedge error")
            },
            Self::DiskServoActuatorSplitEnTimeout => {
                write!(f, "disk servo actuator split EN timeout error")
            },
            Self::DiskServoActuatorTbgUnlockDetect => {
                write!(f, "disk servo actuator TBG unlock detect")
            },
            Self::DiskServoWriteFaultUnsafe => {
                write!(f, "disk servo write fault write unsafe error")
            },
            Self::DiskServoWriteFaultSpindleAtSpeed => {
                write!(f, "disk servo write fault spindle at speed error")
            },
            Self::DiskServoWriteFaultSsmTimeout => {
                write!(f, "disk servo write fault SSM timeout error")
            },
            Self::DiskServoWriteFaultIllegalGrayCode => {
                write!(f, "disk servo write fault illegal gray code error")
            },
            Self::DiskServoWriteFaultIllegalCylinder => {
                write!(f, "disk servo write fault illegal cylinder error")
            },
            Self::DiskServoWriteFaultOffTrack => {
                write!(f, "disk servo write fault off-track error")
            },
            Self::DiskServoWriteFaultFatalPathServoDead => {
                write!(f, "disk servo write fault fatal error path servo dead")
            },
            Self::DiskServoWriteFaultReadOffTrack => {
                write!(f, "disk servo write fault read off-track error")
            },
            Self::DiskServoWriteFaultToneScanSsmTimeout => {
                write!(f, "disk servo write fault tone scan SSM timeout error")
            },
            Self::DiskServoWriteFaultLowGrayCodeQuality => {
                write!(f, "disk servo write fault low gray code quality")
            },
            Self::DiskServoWriteFaultPredictiveOffTrack => {
                write!(f, "disk servo write fault predictive off-track")
            },
            Self::DiskServoWriteFaultBadSignDetected => {
                write!(f, "disk servo write fault bad sign detected")
            },
            Self::DiskServoWriteFaultBadParityDetected => {
                write!(f, "disk servo write fault bad parity detected")
            },
            Self::DiskServoWriteFaultToneScanSsmTimeout53DE => {
                write!(f, "disk servo write fault tone scan SSM timeout")
            },
            Self::DiskServoRequiresActuatorInitialization => {
                write!(f, "disk servo requires actuator initialization")
            },
            Self::DiskServoActuatorAiNoScTargetWedge => {
                write!(f, "disk servo AI no SC target wedge error")
            },
            Self::DiskServoActuatorAiTimeout => write!(f, "disk servo AI timeout"),
            Self::DiskServoActuatorAiBadWedge => write!(f, "disk servo AI bad wedge"),
            Self::DiskServoActuatorAiFatal => write!(f, "disk servo AI fatal"),
            Self::DiskServoActuatorAiServoStateNotActive => {
                write!(f, "disk servo AI servo state not active")
            },
            Self::DiskServoWriteFaultAiUnsafe => {
                write!(f, "disk servo write fault AI write unsafe error")
            },
            Self::DiskServoWriteFaultAiSpindleAtSpeed => {
                write!(f, "disk servo write fault AI spindle at speed error")
            },
            Self::DiskServoWriteFaultAiSsmTimeout => {
                write!(f, "disk servo write fault AI SSM timeout error")
            },
            Self::DiskServoWriteFaultAiIllegalGrayCode => {
                write!(f, "disk servo write fault AI illegal gray code error")
            },
            Self::DiskServoWriteFaultAiIllegalCylinder => {
                write!(f, "disk servo write fault AI illegal cylinder error")
            },
            Self::DiskServoWriteFaultAiOffTrack => {
                write!(f, "disk servo write fault AI off-track error")
            },
            Self::DiskServoWriteFaultAiFatalPathServoDead => {
                write!(f, "disk servo write fault AI fatal error path servo dead")
            },
            Self::DiskServoWriteFaultAiReadOffTrack => {
                write!(f, "disk servo write fault AI read off-track error")
            },
            Self::DiskServoWriteFaultAiShockSensor => {
                write!(f, "disk servo write fault AI shock sensor")
            },
            Self::DiskServoWriteFaultAiLowGrayCodeQuality => {
                write!(f, "disk servo write fault AI low gray code quality")
            },
            Self::DiskServoWriteFaultAiOffTrackOccurred => {
                write!(f, "disk servo write fault AI off-track occurred")
            },
            Self::DiskServoWriteFaultAiBadSignDetected => {
                write!(f, "disk servo write fault AI bad sign detected")
            },
            Self::DiskServoWriteFaultAiBadParityDetected => {
                write!(f, "disk servo write fault AI bad parity detected")
            },
            Self::SvirOk => write!(f, "SVIR ok"),
            Self::SvirAbort => write!(f, "SVIR abort"),
            Self::SvirModel => write!(f, "SVIR model"),
            Self::SvirHead => write!(f, "SVIR head"),
            Self::SvirCylinder => write!(f, "SVIR cylinder"),
            Self::SvirParameter1 => write!(f, "SVIR parameter 1"),
            Self::SvirParameter2 => write!(f, "SVIR parameter 2"),
            Self::SvirParameter3 => write!(f, "SVIR parameter 3"),
            Self::SvirSubcommand => write!(f, "SVIR sub-command"),
            Self::SvirLength => write!(f, "SVIR length"),
            Self::SvirCommand => write!(f, "SVIR command"),
            Self::Svir => write!(f, "SVIR"),
            Self::SvirTimeout => write!(f, "SVIR timeout"),
            Self::SvirIndex => write!(f, "SVIR index"),
            Self::SvirSector => write!(f, "SVIR sector"),
            Self::SvirSam => write!(f, "SVIR SAM"),
            Self::SvirWrroOnTrack2Learn => write!(f, "SVIR WRRO on track 2 learn"),
            Self::SvirWrroOnTrack2Write => write!(f, "SVIR WRRO on track 2 write"),
            Self::SvirWrroCalibrationLearn => write!(f, "SVIR WRRO calibration learn"),
            Self::SvirWrroBurst => write!(f, "SVIR WRRO burst"),
            Self::SvirWrroReadLimitLearn => write!(f, "SVIR WRRO read limit learn"),
            Self::SvirWrroJogLearn => write!(f, "SVIR WRRO jog learn"),
            Self::SvirWrroWrite => write!(f, "SVIR WRRO write"),
            Self::SvirWrroSetup => write!(f, "SVIR WRRO setup"),
            Self::SvirWrroTroLimitLearn => write!(f, "SVIR WRRO TRO limit learn"),
            Self::SvirWrroTimeout => write!(f, "SVIR WRRO timeout"),
            Self::SvirSpinUp => write!(f, "SVIR spin up"),
            Self::SvirSpinDown => write!(f, "SVIR spin down"),
            Self::SvirActuatorSpeed => write!(f, "SVIR actuator speed"),
            Self::SvirActuatorSync => write!(f, "SVIR actuator sync"),
            Self::SvirActuatorUnlatch => write!(f, "SVIR actuator unlatch"),
            Self::SvirActuatorPdFail => write!(f, "SVIR act PD fail"),
            Self::SvirDriveNotCalibrated => write!(f, "SVIR drive not calibrated"),
            Self::SvirWrroLoad => write!(f, "SVIR WRRO load"),
            Self::SvirWrroCompare => write!(f, "SVIR WRRO compare"),
            Self::SvirPesMissSample => write!(f, "SVIR PES miss sample"),
            Self::SvirWrroDataIsrTimeout544B | Self::SvirWrroDataIsrTimeout884B => {
                write!(f, "SVIR WRRO data ISR timeout")
            },
            Self::SvirWrroDataIsrSync => write!(f, "SVIR WRRO data ISR sync"),
            Self::SvirCalibrationNormal => write!(f, "SVIR calibration normal"),
            Self::SvirCalibrationFlex => write!(f, "SVIR calibration flex"),
            Self::SvirCalibrationMotor => write!(f, "SVIR calibration motor"),
            Self::SvirCalibrationRro => write!(f, "SVIR calibration RRO"),
            Self::SvirCalibrationFGain => write!(f, "SVIR calibration F gain"),
            Self::SvirCalibrationSeek => write!(f, "SVIR calibration seek"),
            Self::SvirCalibrationApGain => write!(f, "SVIR calibration AP gain"),
            Self::SvirCalibrationLtr => write!(f, "SVIR calibration LTR"),
            Self::SvirCalibrationGainS => write!(f, "SVIR calibration gain S"),
            Self::SvirCalibrationTangentialHeadOffset => {
                write!(f, "SVIR calibration tangential head offset")
            },
            Self::SvirCalibrationBandwidth => write!(f, "SVIR calibration bandwidth"),
            Self::SvirFTemperatureInvalid => write!(f, "SVIR F temperature invalid"),
            Self::SvirAfcCalibrationF => write!(f, "SVIR AFC calibration F"),
            Self::SvirRroOverflow => write!(f, "SVIR RRO overflow"),
            Self::SvirRroAlgorithm => write!(f, "SVIR RRO algorithm"),
            Self::SvirRampLoadUnload => write!(f, "SVIR ramp load unload"),
            Self::SvirLatchHang => write!(f, "SVIR latch hang"),
            Self::SvirLoad2Fast => write!(f, "SVIR load 2 fast"),
            Self::SvirLoad2Slow => write!(f, "SVIR load 2 slow"),
            Self::SvirIrCalibration => write!(f, "SVIR IR calibration"),
            Self::SvirAdChange => write!(f, "SVIR AD change"),
            Self::SvirRampCalibrationRange => write!(f, "SVIR ramp calibration range"),
            Self::SvirCalibrationRroHead0 => write!(f, "SVIR calibration RRO head 0"),
            Self::SvirCalibrationRroHead1 => write!(f, "SVIR calibration RRO head 1"),
            Self::SvirCalibrationRroHead2 => write!(f, "SVIR calibration RRO head 2"),
            Self::SvirCalibrationRroHead3 => write!(f, "SVIR calibration RRO head 3"),
            Self::SvirCalibrationRroHead4 => write!(f, "SVIR calibration RRO head 4"),
            Self::SvirCalibrationRroHead5 => write!(f, "SVIR calibration RRO head 5"),
            Self::SvirCalibrationRroHead6 => write!(f, "SVIR calibration RRO head 6"),
            Self::SvirCalibrationRroHead7 => write!(f, "SVIR calibration RRO head 7"),
            Self::SvirSTraceFile => write!(f, "SVIR S trace file"),
            Self::SioInvalidSdIndex => write!(f, "SIO invalid SD index"),
            Self::SioInvalidBufferPointer => write!(f, "SIO invalid buffer pointer"),
            Self::SioTimeout => write!(f, "SIO timeout"),
            Self::SppDeviceInitialization => write!(f, "SPP device initialization"),
            Self::SioInvalidParameter => write!(f, "SIO invalid parameter"),
            Self::SioInvalidTransferHeaderChecksum => {
                write!(f, "SIO invalid transfer command header checksum")
            },
            Self::SioInvalidCommandFunction => write!(f, "SIO invalid command function"),
            Self::SioInvalidCrc => write!(f, "SIO invalid CRC"),
            Self::SioUnknown => write!(f, "SIO unknown"),
            Self::SioTransferRequestExceedsAvailableData => {
                write!(f, "SIO transfer request exceeds available data")
            },
            Self::SioAckSizeExceedsTransferLength => write!(
                f,
                "SIO acknowledge size request exceeds transfer length request"
            ),
            Self::SioTransferAbortRequest => write!(f, "SIO transfer abort request"),
            Self::SioTimeoutTransmitIsr => write!(f, "SIO timeout transmit ISR"),
            Self::SioTimeoutTransmitData => write!(f, "SIO timeout transmit data"),
            Self::SioTimeoutReceiveIsr => write!(f, "SIO timeout receive ISR"),
            Self::SioTimeoutTransferRequest => write!(f, "SIO timeout transfer request"),
            Self::SioProtocolCommandNotExpected => {
                write!(f, "SIO protocol error, command not expected")
            },
            Self::SioInvalidTransferPayloadLength => {
                write!(f, "SIO invalid transfer payload length")
            },
            Self::SioInvalidTransferLength => write!(f, "SIO invalid transfer length"),
            Self::SioNoDataTransferInProgress => write!(f, "SIO no data transfer in progress"),
            Self::SioTransferCommandAckSize => write!(f, "SIO transfer command acknowledge size"),
            Self::SioInvalidAbn => write!(f, "SIO invalid ABN"),
            Self::SioMaxCrcsReceiveData => write!(f, "SIO maximum CRC errors receive data"),
            Self::SioCrcReceivedRetrySent => write!(f, "SIO CRC error received retry sent"),
            Self::SioMaxRetries => write!(f, "SIO maximum retries"),
            Self::SioInvalidTransferCommandPayloadLength => {
                write!(f, "SIO invalid transfer command payload length")
            },
            Self::SioCommandAborted => write!(f, "SIO command aborted"),
            Self::SioTimeoutWaitingForAckCommand => {
                write!(f, "SIO timeout waiting for acknowledge command")
            },
            Self::SioMaxCrcsNonSequenceAbn => {
                write!(f, "SIO maximum CRC errors non sequence ABN receive data")
            },
            Self::SioWaitingForTransferComplete => write!(f, "SIO waiting for transfer complete"),
            Self::SioInvalidCommandHeaderReserveField => {
                write!(f, "SIO invalid command header reserve field")
            },
            Self::SioInvalidTransferHeaderParameter => {
                write!(f, "SIO invalid transfer command header parameter field")
            },
            Self::SioInvalidTransferCommandPayloadCrc => {
                write!(f, "SIO invalid transfer command payload CRC")
            },
            Self::SioInvalidTransferCommandDirectionField => {
                write!(f, "SIO invalid transfer command direction field")
            },
            Self::SioInvalidAckHeaderReserved => {
                write!(f, "SIO invalid acknowledge command header reserved field")
            },
            Self::SioInvalidTransferCompleteReserved => {
                write!(f, "SIO invalid transfer complete command reserved field")
            },
            Self::SioInvalidRetryCommandReservedField => {
                write!(f, "SIO invalid retry command reserved field")
            },
            Self::SioInvalidAckHeaderLength => {
                write!(f, "SIO invalid acknowledge command header length field")
            },
            Self::SioInvalidTransferCompleteLength => {
                write!(f, "SIO invalid transfer complete command length field")
            },
            Self::SioInvalidRetryCommandLengthField => {
                write!(f, "SIO invalid retry command length field")
            },
            Self::SioInvalidAckHeaderParameter => {
                write!(f, "SIO invalid acknowledge command header parameter field")
            },
            Self::SioInvalidTransferCompleteParameter => {
                write!(f, "SIO invalid transfer complete command parameter field")
            },
            Self::SioInvalidRetryCommandParameterField => {
                write!(f, "SIO invalid retry command parameter field")
            },
            Self::SioInvalidAckCommandChecksum => {
                write!(f, "SIO invalid acknowledge command checksum")
            },
            Self::SioInvalidTransferCompleteChecksum => {
                write!(f, "SIO invalid transfer complete command checksum")
            },
            Self::SioInvalidRetryCommandChecksum => write!(f, "SIO invalid retry command checksum"),
            Self::UartOther => write!(f, "UART other"),
            Self::UartOverrun => write!(f, "UART overrun"),
            Self::UartFraming => write!(f, "UART framing"),
            Self::DiskServoPztFault => write!(f, "disk servo PZT fault"),
            Self::CacheFlushAllGotCanceled => write!(f, "cache flush all got canceled"),
            Self::CacheFlushCachedRelocationGotCanceled => {
                write!(f, "cache flush cached relocation got canceled")
            },
            Self::DiskCancel => write!(f, "disk cancel"),
            Self::ExecutionOperationCanceled => write!(f, "execution operation canceled"),
            Self::ResourceAllocationGotCanceled => write!(f, "RSC allocation got canceled"),
            Self::BackgroundCanceled => write!(f, "BG canceled"),
            Self::CacheRelocationOperationCanceled => {
                write!(f, "cache relocation operation canceled")
            },
            Self::FormatUnitCanceled => write!(f, "format unit canceled"),
            Self::FmCanceled => write!(f, "FM canceled"),
            Self::DiskRemoveByRequest => write!(f, "disk remove by request"),
            Self::HostOperationCanceled => write!(f, "host operation canceled"),
            Self::AggressiveOlDrmFlushCanceled => write!(f, "aggressive OL DRM flush canceled"),
            Self::Dlg2GotCanceled => write!(f, "DLG2 got canceled"),
            Self::PtmPstInvalidParameter => write!(f, "PTM PST invalid parameter"),
            Self::PtmPstIncompatibleVersion => write!(f, "PTM PST incompatible version"),
            Self::PtmPstMem49ReadFail => write!(f, "PTM PST mem 49 read fail"),
            Self::PtmPstMem4AReadFail => write!(f, "PTM PST mem 4A read fail"),
            Self::PtmPstFile49ReadFail => write!(f, "PTM PST file 49 read fail"),
            Self::PtmPstFile49WriteFail => write!(f, "PTM PST file 49 write fail"),
            Self::PtmPstFile4AReadFail => write!(f, "PTM PST file 4A read fail"),
            Self::PtmPstFile4AWriteFail => write!(f, "PTM PST file 4A write fail"),
            Self::PtmPstFlexBiasCalibrationFail => write!(f, "PTM PST flex bias calibration fail"),
            Self::PtmPstMotorTorqueCalibrationFail => {
                write!(f, "PTM PST motor torque calibration fail")
            },
            Self::PtmPstGainCalibrationFail => write!(f, "PTM PST gain calibration fail"),
            Self::PtmPstBiasLinCalibrationFail => write!(f, "PTM PST bias lin calibration fail"),
            Self::PtmPstLoadFail => write!(f, "PTM PST load fail"),
            Self::PtmPstUnloadFail => write!(f, "PTM PST unload fail"),
            Self::PtmPstReadMemoryTableFail => write!(f, "PTM PST read memory table fail"),
            Self::PtmPstWriteReadVerifyFail => write!(f, "PTM PST write read verify fail"),
            Self::PtmPstResidentFileCreateFail => write!(f, "PTM PST resident file create fail"),
            Self::PtmPstResidentFileReadFail => write!(f, "PTM PST resident file read fail"),
            Self::PtmPstResidentFileWriteFail => write!(f, "PTM PST resident file write fail"),
            Self::PtmPstTooManyMeasurePoints => write!(f, "PTM PST too many measure points"),
            Self::PtmPstSetBodeOffsetFail => write!(f, "PTM PST set Bode offset fail"),
            Self::PtmPstAcBodeFail => write!(f, "PTM PST AC Bode fail"),
            Self::PtmPstFullStrokeServoHang => write!(f, "PTM PST full stroke servo hang"),
            Self::PtmPstFullStrokeSeekLimitFail => write!(f, "PTM PST full stroke seek limit fail"),
            Self::PtmPstFullStrokeServoLimitFail => {
                write!(f, "PTM PST full stroke servo limit fail")
            },
            Self::PtmPstRunningAverageSeekInvalidResult => {
                write!(f, "PTM PST running average seek invalid result")
            },
            Self::PtmPstDataSizeLargerThanAllocateMemory => {
                write!(f, "PTM PST data size larger than allocate memory")
            },
            Self::PtmPstNxBodeFail => write!(f, "PTM PST NX Bode fail"),
            Self::PtmPstSeamCalibrationFail => write!(f, "PTM PST seam calibration fail"),
            Self::PtmPstFile4DReadFail => write!(f, "PTM PST file 4D read fail"),
            Self::PtmPstFile4FReadFail => write!(f, "PTM PST file 4F read fail"),
            Self::PtmPstFile4DWriteFail => write!(f, "PTM PST file 4D write fail"),
            Self::PtmPstFile4FWriteFail => write!(f, "PTM PST file 4F write fail"),
            Self::PtmPstPowerUpBandwidthCalibrationFail => {
                write!(f, "PTM PST power up bandwidth calibration fail")
            },
            Self::PtmPstPowerUpBandwidthHeadExceeded => {
                write!(f, "PTM PST power up bandwidth calibration head exceeded")
            },
            Self::PtmPstPowerUpBandwidthCalibrationNotValid => {
                write!(f, "PTM PST power up bandwidth calibration not valid")
            },
            Self::PtmPstFile49CreateFail => write!(f, "PTM PST file 49 create fail"),
            Self::PtmPstFile4ACreateFail => write!(f, "PTM PST file 4A create fail"),
            Self::PtmPstMiniCalibrationInDvtFail => {
                write!(f, "PTM PST mini calibration in DVT fail")
            },
            Self::PtmPstMiniCalibrationInNotValid => {
                write!(f, "PTM PST mini calibration in not valid")
            },
            Self::PtmPstMiniCalibrationInHeadExceeded => {
                write!(f, "PTM PST mini calibration in head exceeded")
            },
            Self::PtmPstFatalWrroB5LogFull => write!(f, "PTM PST fatal WRRO B5 log full"),
            Self::PtmPstNeedClear4F => write!(f, "PTM PST need clear 4F"),
            Self::EnableDsaFail => write!(f, "enable DSA fail"),
            Self::ArcoChsWrite => write!(f, "ARCO CHS write"),
            Self::ArcoChsRead => write!(f, "ARCO CHS read"),
            Self::ArcoInvalidDcmCodes721A | Self::ArcoInvalidDcmCodes72C8 => {
                write!(f, "ARCO invalid DCM codes")
            },
            Self::ArcoInvalidConfigFileOrFormat => write!(f, "ARCO invalid config file or format"),
            Self::ArcoDirectorySectorRead => write!(f, "ARCO directory sector read"),
            Self::ArcoFile46hChecksum => write!(f, "ARCO file 46h checksum"),
            Self::ArcoInvalidCommandInCoBuffer => write!(f, "ARCO invalid command in CO buffer"),
            Self::ArcoChecksum => write!(f, "ARCO checksum"),
            Self::ArcoIncompatiblePstVersion => write!(f, "ARCO incompatible PST version"),
            Self::ArcoIncompatibleChannelFirmwareVersion => {
                write!(f, "ARCO incompatible channel firmware version")
            },
            Self::ArcoIncompatibleVscFirmwareVersion => {
                write!(f, "ARCO incompatible VSC firmware version")
            },
            Self::ArcoVscPressureSensor => write!(f, "ARCO VSC pressure sensor"),
            Self::ArcoPressureSensorDriverInitialization => {
                write!(f, "ARCO pressure sensor driver initialization")
            },
            Self::ArcoPressureSensorRefLimitExceeded => {
                write!(f, "ARCO pressure sensor ref limit exceeded")
            },
            Self::ArcoPressureSensorRefThresholdExceeded => {
                write!(f, "ARCO pressure sensor ref threshold exceeded")
            },
            Self::ArcoSetup => write!(f, "ARCO setup"),
            Self::ArcoFileId => write!(f, "ARCO file ID"),
            Self::ArcoFileRead => write!(f, "ARCO file read"),
            Self::ArcoModuleRead => write!(f, "ARCO module read"),
            Self::ArcoFileWrite => write!(f, "ARCO file write"),
            Self::ArcoInvalidHeader => write!(f, "ARCO invalid header"),
            Self::ArcoTooManyZones => write!(f, "ARCO too many zones"),
            Self::ArcoTooManyHeads => write!(f, "ARCO too many heads"),
            Self::ArcoTestTimeExceedLimit => write!(f, "ARCO test time exceed limit"),
            Self::ArcoInvalidInput => write!(f, "ARCO invalid input"),
            Self::ArcoFailedToReadCodataFile => write!(f, "ARCO failed to read codata file"),
            Self::ArcoFailedToCreateFile => write!(f, "ARCO failed to create file"),
            Self::ArcoFailedToSwitchWcs => write!(f, "ARCO failed to switch WCS"),
            Self::ArcoStandbyCommandFailed => write!(f, "ARCO standby command failed"),
            Self::ArcoRecalCommandFailed => write!(f, "ARCO recal command failed"),
            Self::ArcoLogicalToBpiZoneTranslate => write!(f, "ARCO logical to BPI zone translate"),
            Self::ArcoBpiToLogicalZoneTranslate => write!(f, "ARCO BPI to logical zone translate"),
            Self::ArcoCurveFitOrderOutOfBound => write!(f, "ARCO curve fit order out of bound"),
            Self::ArcoNotEnoughDataPointForCurveFit => {
                write!(f, "ARCO not enough data point for curve fit")
            },
            Self::ArcoUnexpectedDataInLog => write!(f, "ARCO unexpected data in log"),
            Self::ArcoFailedToReadModule => write!(f, "ARCO failed to read module"),
            Self::ArcoTdCalibrationSameFlexMinMax => {
                write!(f, "ARCO TD calibration same flex min max")
            },
            Self::ArcoRecoveryRegisterListTooSmall => {
                write!(f, "ARCO recovery register list too small")
            },
            Self::ArcoVscTranslation => write!(f, "ARCO VSC translation"),
            Self::ArcoVscGetDriveData => write!(f, "ARCO VSC get drive data"),
            Self::ArcoVscReadWriteMemoryFile46 => write!(f, "ARCO VSC read write memory file 46"),
            Self::ArcoVscExceptionControlMrrCycling => {
                write!(f, "ARCO VSC exception control during on/off MRR cycling")
            },
            Self::ArcoVscExceptionControlJogInterpolation => write!(
                f,
                "ARCO VSC exception control during on/off jog interpolation"
            ),
            Self::ArcoVscExceptionControlChannelUpdate => write!(
                f,
                "ARCO VSC exception control during select channel update mode"
            ),
            Self::ArcoVscExceptionHandlingCommand => {
                write!(f, "ARCO VSC exception handling command")
            },
            Self::ArcoVscReadErrorRateTableCommand => {
                write!(f, "ARCO VSC read error rate table command")
            },
            Self::ArcoVscMnpAccessCommand => write!(f, "ARCO VSC MNP access command"),
            Self::ArcoVscSpinDownCommand => write!(f, "ARCO VSC spin down command"),
            Self::ArcoVscSpinUpCommand => write!(f, "ARCO VSC spin up command"),
            Self::ArcoReadWriteFieldCommand => write!(f, "ARCO read write field command"),
            Self::ArcoFormatSelectCommand => write!(f, "ARCO format select command"),
            Self::ArcoVscCommandEventPending => write!(f, "ARCO VSC command event pending"),
            Self::ArcoVscSwitchFormatCommand => write!(f, "ARCO VSC switch format command"),
            Self::ArcoInitializeDefectList => write!(f, "ARCO initialize defect list"),
            Self::ArcoInitializeGoodCylinderList => write!(f, "ARCO initialize good cylinder list"),
            Self::ArcoCannotFindGoodCylinder => write!(f, "ARCO cannot find a good cylinder"),
            Self::ArcoInvalidZone => write!(f, "ARCO invalid zone"),
            Self::ArcoDeltaGreaterThanThreshold => write!(f, "ARCO delta greater than threshold"),
            Self::ArcoPrepTestTrack => write!(f, "ARCO prep test track"),
            Self::ArcoDriveTemperatureCalibration => {
                write!(f, "ARCO drive temperature calibration")
            },
            Self::ArcoInvalidPreampGainValue => write!(f, "ARCO invalid preamp gain value"),
            Self::ArcoPreampGainCalibration => write!(f, "ARCO preamp gain calibration"),
            Self::ArcoCodataAddressInvalid => write!(f, "ARCO codata address invalid"),
            Self::ArcoTemperatureAboveTargetTemperature => {
                write!(f, "ARCO temperature above target temperature")
            },
            Self::ArcoPbertDvtWrite => write!(f, "ARCO PBERT DVT write"),
            Self::ArcoIllegalOptimizationNumberRequested => {
                write!(f, "ARCO illegal optimization number requested")
            },
            Self::ArcoIllegalSptRequested => write!(f, "ARCO illegal SPT requested"),
            Self::ArcoInvalidDcmCode => write!(f, "ARCO invalid DCM code"),
            Self::ArcoBadChecksumInDataFile => write!(f, "ARCO bad checksum in data file"),
            Self::ArcoInvalidBuffer => write!(f, "ARCO invalid buffer"),
            Self::ArcoUnsupportedPreampId => write!(f, "ARCO unsupported preamp ID"),
            Self::ArcoBadOrInvalidLogInfo => write!(f, "ARCO bad or invalid log info"),
            Self::ArcoInvalidEntry => write!(f, "ARCO invalid entry"),
            Self::ArcoTooManyFormatCode => write!(f, "ARCO too many format code"),
            Self::ArcoFailedToAccessFullStroke => write!(f, "ARCO failed to access full stroke"),
            Self::ArcoInvalidModelList => write!(f, "ARCO invalid model list"),
            Self::IbiDefaultAbortCode => write!(f, "IBI default abort code"),
            Self::IbiFullHeadSurfaceLog7603
            | Self::IbiFullHeadSurfaceLog7604
            | Self::IbiFullHeadSurfaceLog7605
            | Self::IbiFullHeadSurfaceLog7608
            | Self::IbiFullHeadSurfaceLog7614 => write!(f, "IBI full head surface log"),
            Self::IbiTimeout => write!(f, "IBI timeout"),
            Self::IbiServoLogTest => write!(f, "IBI servo log test"),
            Self::IbiExceededHeadDefectsLimit760D
            | Self::IbiExceededHeadDefectsLimit761C
            | Self::IbiExceededHeadDefectsLimit764A => write!(f, "IBI exceeded head defects limit"),
            Self::IbiFmtCapacity760E | Self::IbiFmtCapacity7611 | Self::IbiFmtCapacity7613 => {
                write!(f, "IBI FMT capacity")
            },
            Self::IbiSpecifiedCapacityNotReached760F | Self::IbiSpecifiedCapacityNotReached7612 => {
                write!(f, "IBI specified capacity not reached")
            },
            Self::IbiExceededDefectsLimit7616
            | Self::IbiExceededDefectsLimit767C
            | Self::IbiExceededDefectsLimit767E
            | Self::IbiExceededDefectsLimit76FF => write!(f, "IBI exceeded defects limit"),
            Self::IbiTlist => write!(f, "IBI T-list"),
            Self::IbiIllegalParameters => write!(f, "IBI illegal parameters"),
            Self::IbiLogRead => write!(f, "IBI log read"),
            Self::IbiWrongParameters => write!(f, "IBI wrong parameters"),
            Self::IbiTestMini7652 | Self::IbiTestMini7657 => write!(f, "IBI test mini"),
            Self::IbiReservedPushDownListOverflow => {
                write!(f, "IBI reserved push down list overflow")
            },
            Self::IbiTestB9ExceededDefectsLimit => write!(f, "IBI test B9 exceeded defects limit"),
            Self::IbiTooManySoftErrors => write!(f, "IBI too many soft errors"),
            Self::IbiTestBaTooManySoftErrors => write!(f, "IBI test BA too many soft errors"),
            Self::IbiTestD1FullHeadSurfaceLog => write!(f, "IBI test D1 full head surface log"),
            Self::IbiTestB9 => write!(f, "IBI test B9"),
            Self::IbiFmtWriteFailExceededDefectsLimit => {
                write!(f, "IBI FMT write fail exceeded defects limit")
            },
            Self::IbiTooManyTracksInPlist76F6 | Self::IbiTooManyTracksInPlist76F7 => {
                write!(f, "IBI too many tracks in P-list")
            },
            Self::IbiNativeMaximumLbaTooBig => write!(f, "IBI native maximum LBA too big"),
            Self::PtmInvalidVectorTableVersion => write!(f, "PTM invalid vector table version"),
            Self::HalInvalidParameter => write!(f, "HAL invalid parameter"),
            Self::HalFlashUnknown => write!(f, "HAL flash unknown"),
            Self::HalFlashInvalidSectorAddress => {
                write!(f, "HAL flash invalid flash sector address")
            },
            Self::HalFlashWriteLatchEnable => write!(f, "HAL flash write latch enable"),
            Self::HalFlashWritePageSendByte => write!(f, "HAL flash write page send byte"),
            Self::HalFlashWritePageTimeout => write!(f, "HAL flash write page timeout"),
            Self::HalFlashWritePageToStaticMemory => {
                write!(f, "HAL flash write page to static memory error")
            },
            Self::HalFlashReadBlockGetByte => write!(f, "HAL flash read block get byte"),
            Self::HalFlashByteCountExceedsDeviceLimit => {
                write!(f, "HAL flash byte count exceeds device limit")
            },
            Self::HalFlashInvalidAddress => write!(f, "HAL flash invalid flash address"),
            Self::HalFlashInitialBootHeaderMissing => {
                write!(f, "HAL flash initial boot header missing")
            },
            Self::HalFlashSendByteTimeout => write!(f, "HAL flash send byte timeout"),
            Self::HalFlashReadCommand => write!(f, "HAL flash read command"),
            Self::HalFlashInvalidData => write!(f, "HAL flash invalid flash data"),
            Self::HalFlashDataCompare => write!(f, "HAL flash data compare"),
            Self::HalFlashDeviceId => write!(f, "HAL flash device ID"),
            Self::HalFlashReadInfoStartTimeout => write!(f, "HAL flash read info start timeout"),
            Self::HalFlashStatusTimeout => write!(f, "HAL flash status timeout"),
            Self::HalFlashCommandTimeout => write!(f, "HAL flash command timeout"),
            Self::HalFlashBadChecksum => write!(f, "HAL flash bad checksum"),
            Self::HalSystemPllLockFailure => write!(f, "HAL system PLL lock failure"),
            Self::HalSystemSppCheckFail => write!(f, "HAL system SPP check fail"),
            Self::HalSystemUartFifoFull => write!(f, "HAL system UART FIFO full"),
            Self::HalSystemUartFifoEmpty => write!(f, "HAL system UART FIFO empty"),
            Self::HalSystemUartOverrun => write!(f, "HAL system UART overrun"),
            Self::HalSystemUartOthers => write!(f, "HAL system UART others"),
            Self::HalSystemUartTransmitFifoFull => write!(f, "HAL system UART transmit FIFO full"),
            Self::VscNotSupportReadWrro => write!(f, "VSC not support read WRRO"),
            Self::SvirSeekBusy => write!(f, "SVIR seek busy"),
            Self::InvalidModActCodeRequest => write!(f, "invalid mod act code request"),
            Self::VscCommandSetNotEnabled => write!(f, "VSC command set not enabled"),
            Self::InvalidModByteInModifyConfigSector => {
                write!(f, "invalid mod byte in modify config sector command")
            },
            Self::InvalidSmartEnableCode => write!(f, "invalid SMART enable code"),
            Self::InvalidOperationRequest => write!(f, "invalid operation request"),
            Self::OffsetTooLarge => write!(f, "offset too large"),
            Self::InvalidHeadNumber => write!(f, "invalid head number"),
            Self::CylinderAboveLimit => write!(f, "cylinder above limit"),
            Self::InvalidWedgeOffset => write!(f, "invalid wedge offset"),
            Self::InvalidWedgeSize => write!(f, "invalid wedge size"),
            Self::StartAddressTooLarge => write!(f, "start address too large"),
            Self::LengthTooLarge => write!(f, "length too large"),
            Self::InvalidTableId => write!(f, "invalid table ID"),
            Self::UnsupportedActionCode => write!(f, "unsupported action code"),
            Self::UnsupportedFunction => write!(f, "unsupported function"),
            Self::ValueActionCodeUnsupportedFeature => {
                write!(f, "value action code unsupported feature")
            },
            Self::ValueActionCodeUnsupportedOperation => {
                write!(f, "value action code unsupported operation")
            },
            Self::InvalidFunctionCodeRequest => write!(f, "invalid function code request"),
            Self::TableOffsetTooLarge => write!(f, "table offset too large"),
            Self::InvalidExceptionFeature => write!(f, "invalid exception feature"),
            Self::InvalidOffset => write!(f, "invalid offset"),
            Self::InvalidKeySectorSize => write!(f, "invalid key sector size"),
            Self::TransferRequestExceedAvailableData => {
                write!(f, "transfer request exceeds available data")
            },
            Self::InvalidVscSource => write!(f, "invalid VSC source"),
            Self::ActionCodeOutOfRange => write!(f, "action code out of range"),
            Self::KeySectorMustPrecedeDataTransferRequest => {
                write!(f, "key sector must precede data transfer request")
            },
            Self::InvalidSettleMode => write!(f, "invalid settle mode"),
            Self::InvalidEnableDisableKeyInFeatureRegisters => {
                write!(f, "invalid enable disable key in feature registers")
            },
            Self::FunctionNotSupportedOnSocPlatform => {
                write!(f, "function not supported on SOC platform")
            },
            Self::InvalidSectorRequest => write!(f, "invalid sector request"),
            Self::FlashLengthTooSmall => write!(f, "flash length too small"),
            Self::FlashStartAddressTooSmall => write!(f, "flash start address too small"),
            Self::FlashStartSectorTooSmall => write!(f, "flash start sector too small"),
            Self::FlashAccessRangeRequestTooLarge => {
                write!(f, "flash access range request too large")
            },
            Self::InvalidLbaRequest => write!(f, "invalid LBA request"),
            Self::PstBufferNotAllocated => write!(f, "PST buffer not allocated"),
            Self::CommandNotAllowedFromPst => write!(f, "command not allowed from PST"),
            Self::VscInvalidPstTestId => write!(f, "VSC invalid PST test ID"),
            Self::VscInvalidPstVectorAddress => write!(f, "VSC invalid PST vector address"),
            Self::PstVscdBufferTooSmall => write!(f, "PST VSCD buffer too small"),
            Self::SectorOffsetNotFromZero => write!(f, "sector offset not from zero"),
            Self::InvalidResourceMemoryRequest => write!(f, "invalid RSC memory request"),
            Self::InvalidPstTestModeRequest => write!(f, "invalid PST test mode request"),
            Self::ClearDrmLogFailed => write!(f, "clear DRM log failed"),
            Self::ClearFactoryFileFailed => write!(f, "clear factory file failed"),
            Self::WarningWearLevelWithBackgroundDisabled => {
                write!(f, "warning wear level with BG disabled")
            },
            Self::InvalidWearLevelArgument => write!(f, "invalid wear level argument"),
            Self::InvalidPeriodShiftCtlrArgument => write!(f, "invalid period shift CTLR argument"),
            Self::WarningDrmFlushWithBackgroundDisabled => {
                write!(f, "warning DRM flush with BG disabled")
            },
            Self::InvalidDrmFlushControlArgument => write!(f, "invalid DRM flush control argument"),
            Self::InvalidSmartBackdoorArgument => write!(f, "invalid SMART backdoor argument"),
            Self::InvalidBackgroundActivityArgument => write!(f, "invalid BG activity argument"),
            Self::InvalidDriveTemperatureSamplingArgument => {
                write!(f, "invalid drive temperature sampling argument")
            },
            Self::InvalidClearDrmSection => write!(f, "invalid clear DRM section"),
            Self::DepopInvalidHeadId => write!(f, "depop invalid head ID"),
            Self::RequestEndLbaLessThanStart => write!(f, "request end LBA less than start LBA"),
            Self::DepopOnlyOneHead => write!(f, "depop only one head"),
            Self::InvalidPstModeArgument => write!(f, "invalid PST mode argument"),
            Self::PushDownsOnTrack => write!(f, "push downs on track"),
            Self::InvalidPeriodSumParameter => write!(f, "invalid period sum parameter"),
            Self::HostDataTransferDidNotOccur => write!(f, "host data transfer did not occur"),
            Self::InvalidClearDrmAgentCode => write!(f, "invalid clear DRM agent code"),
            Self::FeatureControlInvalidArgument => write!(f, "feature control invalid argument"),
            Self::FeatureControlReadUnsupported => {
                write!(f, "feature control read feature unsupported")
            },
            Self::MemoryTableIsReadOnly => write!(f, "memory table is read only"),
            Self::DebugStopOccurred => write!(f, "DBS occurred"),
            Self::ReadWriteFieldInvalidLength => write!(f, "read write field invalid length"),
            Self::RequestedRelocationsGreaterThanAvailable => {
                write!(f, "requested relocations greater than available")
            },
            Self::InvalidLength => write!(f, "invalid length"),
            Self::InvalidCountValue => write!(f, "invalid count value"),
            Self::InvalidAddressMode => write!(f, "invalid address mode"),
            Self::ServoTraceDisabled => write!(f, "servo trace disabled"),
            Self::ConfigServoTraceAlreadyEnabled => write!(f, "config servo trace already enabled"),
            Self::InvalidStartWedge => write!(f, "invalid start wedge"),
            Self::InvalidZoneNumber => write!(f, "invalid zone number"),
            Self::CylinderNotInGainCalibrationZone => {
                write!(f, "cylinder not in gain calibration zone")
            },
            Self::DisableGainCalibrationToRunThisCommand => {
                write!(f, "disable gain calibration to run this command")
            },
            Self::InvalidConfigSection => write!(f, "invalid config section"),
            Self::SvirInvalidTableSize => write!(f, "SVIR invalid table size"),
            Self::GainCalibrationTableNotInitialized => {
                write!(f, "gain calibration table not initialized")
            },
            Self::GainCalibrationTrainingNotStarted => {
                write!(f, "gain calibration training not started")
            },
            Self::GainCalibrationValueNotTrained => write!(f, "gain calibration value not trained"),
            Self::GainCalibrationFeatureNotImplemented => {
                write!(f, "gain calibration feature not implemented")
            },
            Self::PartialFileRequestPastEof => write!(f, "partial file request past EOF"),
            Self::InvalidDvtOpcode => write!(f, "invalid DVT opcode"),
            Self::TemporarySramStaticAlreadyAllocated => {
                write!(f, "temporary SRAM static already allocated")
            },
            Self::TableNotAvailableCacheRelocationDisabled => {
                write!(f, "table not available cache relocation disabled")
            },
            Self::NoTemporarySramStaticAllocated => write!(f, "no temporary SRAM static allocated"),
            Self::InvalidSmartAttributeStatus => write!(f, "invalid SMART attribute status"),
            Self::InvalidSmartAttributeId => write!(f, "invalid SMART attribute ID"),
            Self::SinglePassToneScanNotSupported => {
                write!(f, "single pass tone scan not supported")
            },
            Self::InvalidWearLevelMode => write!(f, "invalid wear level mode"),
            Self::InvalidWearLevelConfigTableNotAvailable => {
                write!(f, "invalid wear level config table not available")
            },
            Self::DepopIbiSurface1LogNotInTrackDirectory => {
                write!(f, "depop IBI surface 1 log not in track directory")
            },
            Self::DepopIbiSurface1PesNotInTrackDirectory => {
                write!(f, "depop IBI surface 1 PES not in track directory")
            },
            Self::PartialFileNotInPstMode => write!(f, "partial file not in PST mode"),
            Self::DfhModeNotEnabled => write!(f, "DFH mode not enabled"),
            Self::ParameterOutOfRange => write!(f, "parameter out of range"),
            Self::FmtSelectCapacityFailure => write!(f, "FMT select capacity failure"),
            Self::DcmUninitialized => write!(f, "DCM uninitialized"),
            Self::CapacityGroupDefinition => write!(f, "capacity group definition"),
            Self::WriteReadGapInfoNotAvailable => write!(f, "write read gap info not available"),
            Self::DriveProtectLocked => write!(f, "drive protect locked"),
            Self::InvalidRegionNumber => write!(f, "invalid region number"),
            Self::DiskBackendTableNotPresent => write!(f, "disk backend table not present"),
            Self::CompareIdsLbaMiscompare => write!(f, "compare IDs LBA miscompare"),
            Self::FlexBiasFilter => write!(f, "flex bias filter"),
            Self::VscInvalidConfigCode => write!(f, "VSC invalid config code"),
            Self::VscInvalidConfigDataHeader => write!(f, "VSC invalid config data header"),
            Self::VscInvalidConfigData => write!(f, "VSC invalid config data"),
            Self::VscParameterLengthMismatch => write!(f, "VSC parameter length mismatch"),
            Self::VscParameterTypeMismatch => write!(f, "VSC parameter type mismatch"),
            Self::VscRuleCheckFail => write!(f, "VSC rule check fail"),
            Self::VscCheckListEmpty => write!(f, "VSC check list empty"),
            Self::VscNoMatchingEntryInTable => write!(f, "VSC no matching entry in table"),
            Self::VscInvalidEntryInTable => write!(f, "VSC invalid entry in table"),
            Self::VscCommandResponseThresholdExceeded => {
                write!(f, "VSC command response threshold exceeded")
            },
            Self::VscCommandResponseProcessingInProgress => {
                write!(f, "VSC command response processing in progress")
            },
            Self::VscMajorRevisionMismatch => write!(f, "VSC major revision mismatch"),
            Self::VscCvfFileNotFound => write!(f, "VSC CVF file not found"),
            Self::VscMajorRevisionAlreadySet => write!(f, "VSC major revision already set"),
            Self::VscFeatureSetKeyFail => write!(f, "VSC feature set key fail"),
            Self::VscSOverlayAlreadyLoaded => write!(f, "VSC S overlay already loaded"),
            Self::VscCOverlayAlreadyLoaded => write!(f, "VSC C overlay already loaded"),
            Self::VscMismatchFamilyId => write!(f, "VSC mismatch family ID"),
            Self::VscCvfFileRead => write!(f, "VSC CVF file read"),
            Self::VscFailureInUpdateDataFile => write!(f, "VSC failure in update data file"),
            Self::VscInvalidLowMemoryModeArgument => {
                write!(f, "VSC invalid low memory mode argument")
            },
            Self::VscPreAc55ConfigDrive => write!(f, "VSC pre AC55 config drive"),
            Self::VscUccmTotalBytesExpectedMismatch => {
                write!(f, "VSC UCCM total bytes expected mismatch")
            },
            Self::VscAc55FunctionCode1SupportDisabled => {
                write!(f, "VSC AC55 function code 1 support disabled")
            },
            Self::VscAc55FunctionCode2SupportDisabled => {
                write!(f, "VSC AC55 function code 2 support disabled")
            },
            Self::VscAc55FunctionCode3SupportDisabled => {
                write!(f, "VSC AC55 function code 3 support disabled")
            },
            Self::VscAc55FunctionCode4SupportDisabled => {
                write!(f, "VSC AC55 function code 4 support disabled")
            },
            Self::VscAc55FunctionCode5SupportDisabled => {
                write!(f, "VSC AC55 function code 5 support disabled")
            },
            Self::VscAc55FunctionCode6SupportDisabled => {
                write!(f, "VSC AC55 function code 6 support disabled")
            },
            Self::VscAc55FunctionCode7SupportDisabled => {
                write!(f, "VSC AC55 function code 7 support disabled")
            },
            Self::VscAc55FunctionCode8SupportDisabled => {
                write!(f, "VSC AC55 function code 8 support disabled")
            },
            Self::VscAc55FunctionCode9SupportDisabled => {
                write!(f, "VSC AC55 function code 9 support disabled")
            },
            Self::VscAc55FunctionCode10SupportDisabled => {
                write!(f, "VSC AC55 function code 10 support disabled")
            },
            Self::VscPOverlayAlreadyLoaded => write!(f, "VSC P overlay already loaded"),
            Self::InvalidSmartEnCode => write!(f, "invalid SMART EN code"),
            Self::SmartInvalidHostSectorRequest => write!(f, "SMART invalid host sector request"),
            Self::SmartInvalidVendorSectorRequest => {
                write!(f, "SMART invalid vendor sector request")
            },
            Self::SmartFeatureNotSupported => write!(f, "SMART feature not supported"),
            Self::SmartInvalidSectorCount => write!(f, "SMART invalid sector count"),
            Self::VscInvalidRaspTarget => write!(f, "VSC invalid RASP target"),
            Self::VscInvalidRaspTargetTrack => write!(f, "VSC invalid RASP target track"),
            Self::VscPstRaspGetBufferTooSmall => write!(f, "VSC PST RASP get buffer too small"),
            Self::SmartInvalidDefectListType => write!(f, "SMART invalid defect list type"),
            Self::SmartInvalidDefectListFormat => write!(f, "SMART invalid defect list format"),
            Self::SmartWriteSelectTestSelfTestInProgress => {
                write!(f, "SMART write select test, self-test in progress")
            },
            Self::SmartSelectiveTestInvalidVersion => {
                write!(f, "SMART selective test invalid version")
            },
            Self::SmartOfflineImmediateDisabled => write!(f, "SMART offline immediate disabled"),
            Self::SmartResourceAllocationFailed => write!(f, "SMART RSC allocation failed"),
            Self::SmartStatusFailed => write!(f, "SMART status failed"),
            Self::ObsoleteCommand => write!(f, "obsolete command"),
            Self::SmartCommandWithSmartDisabled => write!(f, "SMART command with SMART disabled"),
            Self::FeatureNotSupported => write!(f, "feature not supported"),
            Self::SecurityCommandWithBadParameter => {
                write!(f, "security command with bad parameter")
            },
            Self::SecurityCommandSecurityModeDisabled => {
                write!(f, "security command with security mode disabled")
            },
            Self::SecurityCommandWithDiskFrozen => write!(f, "security command with disk frozen"),
            Self::SecurityCommandWithDiskLocked => write!(f, "security command with disk locked"),
            Self::SecurityCommandWithDiskLockedOrFrozen => {
                write!(f, "security command with disk locked or frozen")
            },
            Self::SecurityCommandWithDiskExpiredOrFrozen => {
                write!(f, "security command with disk expired or frozen")
            },
            Self::SecurityCommandWithReceiveData => {
                write!(f, "security command with receive data error")
            },
            Self::SecurityCommandWithPasswordMiscompare => {
                write!(f, "security command with password miscompare")
            },
            Self::SecurityCommandWithPasswordIsZero => {
                write!(f, "security command with password is zero")
            },
            Self::SecurityInvalidMasterPasswordRevision => {
                write!(f, "security command with invalid master password revision")
            },
            Self::SecurityCommandNoErasePrepare => write!(f, "security command no erase prepare"),
            Self::SecurityCommandReceivedInInvalidState => {
                write!(f, "security command received in invalid state")
            },
            Self::VscSecurityDriveIsLocked => write!(f, "VSC security drive is locked"),
            Self::VscSecurityUnlockFailedAfterFormatUnit => {
                write!(f, "VSC security unlock failed after format unit")
            },
            Self::VscSecurityInvalidFmtUnitOptions => {
                write!(f, "VSC security invalid format unit options to unlock")
            },
            Self::ChsLbaTooLarge => write!(f, "CHS LBA too large"),
            Self::ErrorInjectionInvalidFunctionCode => {
                write!(f, "error injection invalid function code")
            },
            Self::ErrorInjectionInvalidType => write!(f, "error injection invalid error type"),
            Self::ErrorInjectionInvalidNrzMode => write!(f, "error injection invalid NRZ mode"),
            Self::ErrorInjectionInvalidHandle => write!(f, "error injection invalid handle"),
            Self::ErrorInjectionInvalidCount => write!(f, "error injection invalid count"),
            Self::ErrorInjectionInvalidOffset => write!(f, "error injection invalid offset"),
            Self::ErrorInjectionInvalidLength => write!(f, "error injection invalid length"),
            Self::ErrorInjectionInvalidRepeatCount => {
                write!(f, "error injection invalid repeat count")
            },
            Self::ErrorInjectionIeitblFull => write!(f, "error injection IEITBL full"),
            Self::ErrorInjectionIeitblEmpty => write!(f, "error injection IEITBL empty"),
            Self::ErrorInjectionSameTrackExist => write!(f, "error injection same track exist"),
            Self::ErrorInjectionTargetNotFound => write!(f, "error injection target not found"),
            Self::ErrorInjectionAddTargetFailed => write!(f, "error injection add target failed"),
            Self::ErrorInjectionRemoveTargetFailed => {
                write!(f, "error injection remove target failed")
            },
            Self::BackgroundInvalidSelfTestSelected => write!(f, "BG invalid self-test selected"),
            Self::SctUnsupportedBistModeInPatternRequest => {
                write!(f, "SCT unsupported BIST mode in pattern request")
            },
            Self::SctUnsupportedOpCodeForWdLogs => write!(f, "SCT unsupported op code for WDLOGS"),
            Self::HpaInvalidValueSpecified => write!(f, "HPA invalid value specified"),
            Self::HpaLockInPlace => write!(f, "HPA lock in place"),
            Self::HpaLockNotInPlace => write!(f, "HPA lock not in place"),
            Self::HpaFreezeLockInPlace => write!(f, "HPA freeze lock in place"),
            Self::HpaCommandSequence => write!(f, "HPA command sequence"),
            Self::HpaSetMaxAddressExtInPlace => write!(f, "HPA set max address ext in place"),
            Self::HpaPasswordActive => write!(f, "HPA password active"),
            Self::HpaPasswordNotActive => write!(f, "HPA password not active"),
            Self::HpaPasswordMiscompare => write!(f, "HPA password miscompare"),
            Self::HpaSecondNonVolatileCommand => write!(f, "HPA second non-volatile command"),
            Self::HpaReadMaxFirstCommand => write!(f, "HPA read max first command"),
            Self::HpaCommandSequenceFault => write!(f, "HPA command sequence fault"),
            Self::HostUnsupportedAtaOpcode => write!(f, "host unsupported ATA opcode"),
            Self::HostReceivedLbaTooBig => write!(f, "host received LBA too big"),
            Self::HostLbaOutOfRange => write!(f, "host LBA out of range"),
            Self::HostDriveParametersSptNotSupported => {
                write!(f, "host drive parameters SPT not supported")
            },
            Self::HostDriveParametersHeadsNotSupported => {
                write!(f, "host drive parameters heads not supported")
            },
            Self::HostUnsupportedFeatureValue => write!(f, "host unsupported feature value"),
            Self::HostUnsupportedMultiCount => write!(f, "host unsupported multi count"),
            Self::HostMultiNotSet => write!(f, "host multi not set"),
            Self::HostDisabledIordyNotSupported => write!(f, "host disabled IORDY not supported"),
            Self::HostUnsupportedCommandInPstMode => {
                write!(f, "host unsupported command in PST mode")
            },
            Self::HostInvalidSectorCount => write!(f, "host invalid sector count"),
            Self::HostVscCommandExecutingInBackground => {
                write!(f, "host VSC command executing in background")
            },
            Self::HostCommandNotAllowedInGainCalibration => {
                write!(f, "host command not allowed in gain calibration mode")
            },
            Self::HostQueueCommandIntermix => write!(f, "host queue command intermix"),
            Self::HostUnsupportedSetFeatureSata => {
                write!(f, "host unsupported set feature SATA feature")
            },
            Self::HostQueueTag => write!(f, "host queue tag"),
            Self::HostInvalidChsCylinderNumber => write!(f, "host invalid CHS cylinder number"),
            Self::HostInvalidChsHeadNumber => write!(f, "host invalid CHS head number"),
            Self::HostInvalidChsSectorNumber => write!(f, "host invalid CHS sector number"),
            Self::HostUnsupportedAtaCommandInSioMode => {
                write!(f, "host unsupported ATA command in SIO mode")
            },
            Self::HostReservedStandbyTimerValue => write!(f, "host reserved standby timer value"),
            Self::HostPuisSetFeatureDisabledInConfigSector => {
                write!(f, "host PUIS set feature disabled in config sector")
            },
            Self::HostPuisFlashSetToUseJumper => write!(f, "host PUIS flash set to use jumper"),
            Self::HostPuisSetFeatureNotSupportedInXpm2 => {
                write!(f, "host PUIS set feature not supported in XPM2")
            },
            Self::HostPuisSpinUpCommandNotSupportedInXpm2 => {
                write!(f, "host PUIS spin up command not supported in XPM2")
            },
            Self::HostPuisJumperEnabledInFlashNoJumper => {
                write!(f, "host PUIS jumper enabled in flash no jumper")
            },
            Self::HostPuisDisabledInFlash => write!(f, "host PUIS disabled in flash"),
            Self::HostInvalidAtaStreamId => write!(f, "host invalid ATA stream ID"),
            Self::HostAtaStreamIdNotConfig => write!(f, "host ATA stream ID not config"),
            Self::HostLockedUnitAccessDenied => write!(f, "host locked unit access denied"),
            Self::HostNcqNoReadLog10 => write!(f, "host NCQ no READ LOG 10h"),
            Self::HostUnsupportedChipRevision => write!(f, "host unsupported chip revision"),
            Self::HostInterfaceCrc => write!(f, "host interface CRC"),
            Self::HostInterfaceCrcOverrunUnderrun => {
                write!(f, "host interface CRC overrun underrun error")
            },
            Self::HostInterfaceOverrunUnderrun => {
                write!(f, "host interface overrun underrun error")
            },
            Self::HostIntrudingCommand => write!(f, "host intruding command"),
            Self::HostSataCrc => write!(f, "host SATA CRC"),
            Self::HostSataRxProtocol => write!(f, "host SATA RX protocol"),
            Self::HostSataRxSyncTerminate => write!(f, "host SATA RX sync terminate"),
            Self::HostSataRxLength => write!(f, "host SATA RX length"),
            Self::HostSataTxRerr => write!(f, "host SATA TX RERR"),
            Self::HostSataTxSyncTerminate => write!(f, "host SATA TX sync terminate"),
            Self::HostSataRetransmit => write!(f, "host SATA retransmit"),
            Self::HostSataTmFifo => write!(f, "host SATA TM FIFO"),
            Self::HostSataDisparity => write!(f, "host SATA disparity"),
            Self::HostSataCodeViolation => write!(f, "host SATA code violation"),
            Self::HostSataLinkHung => write!(f, "host SATA link hung"),
            Self::HostSataUnrecognizedFis => write!(f, "host SATA unrecognized FIS"),
            Self::HostSataUnknown => write!(f, "host SATA unknown"),
            Self::HostSataRxDisparityInFis => write!(f, "host SATA RX disparity in FIS"),
            Self::HostSataWriteTransferOverrun => write!(f, "host SATA write transfer overrun"),
            Self::HostSataDataFisTooLong => write!(f, "host SATA data FIS too long"),
            Self::HostSataDataFisTooShort => write!(f, "host SATA data FIS too short"),
            Self::HostSataHbcrc => write!(f, "host SATA HBCRC"),
            Self::HostSataHbcrcAndRerr => write!(f, "host SATA HBCRC and RERR"),
            Self::HostSataHiddenHbcrc => write!(f, "host SATA hidden HBCRC"),
            Self::HostSataDataFisWrongSize => write!(f, "host SATA data FIS wrong size"),
            Self::HostSataTmFifoUnderrun => write!(f, "host SATA TM FIFO underrun"),
            Self::HostSataTmFifoOverrun => write!(f, "host SATA TM FIFO overrun"),
            Self::DcoInvalidFeatureSet => write!(f, "DCO invalid feature set"),
            Self::DcoFreezeLockInPlace => write!(f, "DCO freeze lock in place"),
            Self::DcoChecksum => write!(f, "DCO checksum"),
            Self::DcoSignature => write!(f, "DCO signature"),
            Self::DcoInvalidUdmaMode => write!(f, "DCO invalid UDMA mode"),
            Self::DcoCommandAborted => write!(f, "DCO command aborted"),
            Self::DcoDcRestoreWhenDriveIsInFactoryState => {
                write!(f, "DCO restore when drive is in factory state")
            },
            Self::DcoDcSetWhenDriveIsInReducedState => {
                write!(f, "DCO set when drive is in reduced state")
            },
            Self::DcoDcRestoreWhenHpaIsPresent => write!(f, "DCO restore when HPA is present"),
            Self::DcoDcSetWithInvalidConditions => write!(f, "DCO set with invalid conditions"),
            Self::UnsupportedLogAddress => write!(f, "unsupported log address"),
            Self::ChangeDefinitionInvalidPassword => {
                write!(f, "change definition command invalid password error")
            },
            Self::ChangeDefinitionInvalidConfigNumber => {
                write!(f, "change definition command invalid config number error")
            },
            Self::ChangeDefinitionUndefinedCapacity => {
                write!(f, "change definition command undefined capacity error")
            },
            Self::ChangeDefinitionIllegalCapacity => {
                write!(f, "change definition command illegal capacity error")
            },
            Self::ChangeDefinitionCounterMaximum => {
                write!(f, "change definition command counter maximum error")
            },
            Self::ChangeDefinitionInvalidConfigSelectArray => write!(
                f,
                "change definition command invalid config select array error"
            ),
            Self::ChangeDefinitionNotEnabled => {
                write!(f, "change definition command not enabled error")
            },
            Self::ChangeDefinitionNativeMaxLbaInvalid => {
                write!(f, "change definition command native max LBA invalid error")
            },
            Self::ChangeDefinitionFieldList => {
                write!(f, "change definition command field list error")
            },
            Self::NotAnErrorSioInvalidNonAtaOpcode => {
                write!(f, "not an error SIO invalid non-ATA opcode")
            },
            Self::NotAnErrorVscCommandInBackground => {
                write!(f, "not an error VSC command execution in background")
            },
            Self::Unknown(x) => write!(f, "unknown {x:#x}"),
        }
    }
}

impl From<ErrorCode> for u32 {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::Unknown(x) => x,
            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

impl ErrorCode {
    /// Parse error code.
    pub(crate) fn parse(value: u32) -> Option<Self> {
        const CONST_VARIANTS: &[ErrorCode] = &[
            ErrorCode::SctInvalidFunctionCode,
            ErrorCode::SctInvalidLba,
            ErrorCode::SctRequestSectorCountOverflow,
            ErrorCode::SctInvalidErrorRecoveryFunctionCode,
            ErrorCode::SctInvalidErrorRecoverySelectCode,
            ErrorCode::SctHostReadTimerLessThanMinimum,
            ErrorCode::SctHostWriteTimerLessThanMinimum,
            ErrorCode::SctBackgroundCommandAbortByIntHost,
            ErrorCode::SctBackgroundTerminatedUnrecoverableServo,
            ErrorCode::SctInvalidFunctionCodeInLongSectorAccess,
            ErrorCode::SctDataTransferWithoutKeySector,
            ErrorCode::SctInvalidFunctionCodeInFeatureControl,
            ErrorCode::SctInvalidFeatureCodeInFeatureControl,
            ErrorCode::SctInvalidNewStateFeatureControlCommand,
            ErrorCode::SctInvalidOptionFlagsInFeatureControl,
            ErrorCode::SctInvalidActionCode,
            ErrorCode::SctInvalidTableId,
            ErrorCode::SctCommandAbortDriveSecurityLock,
            ErrorCode::SctInvalidRevisionCode,
            ErrorCode::SctForegroundTerminatedUnrecoverable,
            ErrorCode::SctTlerTimeoutStatus,
            ErrorCode::CtlrFwRequestCommandAbort,
            ErrorCode::IllegalDriveModel,
            ErrorCode::IllegalParameter0,
            ErrorCode::IllegalParameter1,
            ErrorCode::IllegalParameter2,
            ErrorCode::IllegalParameter3,
            ErrorCode::IllegalCommand,
            ErrorCode::FatalServo,
            ErrorCode::Timeout,
            ErrorCode::IndexNotFound,
            ErrorCode::SectorCountMismatch,
            ErrorCode::Sam,
            ErrorCode::UnableToSpinUp,
            ErrorCode::UnableToSpinDown,
            ErrorCode::UnableToSpeed,
            ErrorCode::BurstSyncFail,
            ErrorCode::UnlatchFail,
            ErrorCode::CtrPdSeekFail,
            ErrorCode::BurstSlopeCalibrationFail,
            ErrorCode::FlexBiasCalibrationFail,
            ErrorCode::MotorTorqueCalibrationFail,
            ErrorCode::RroCalibrationFail,
            ErrorCode::FarGainCalibrationFail,
            ErrorCode::Track0SeekFail,
            ErrorCode::RcServoGainCalibrationFail,
            ErrorCode::RcLtrOlCalibrationFail,
            ErrorCode::SnapshotGainCalibrationFail,
            ErrorCode::TangentialHeadOffsetCalibrationFail,
            ErrorCode::BandwidthCalibrationFail,
            ErrorCode::RroLearningOverflow,
            ErrorCode::RroLearning,
            ErrorCode::SvirUnknownB6,
            ErrorCode::SvirUnknownD2,
            ErrorCode::SvirUnknownD4,
            ErrorCode::DriveProtectDiskOverlay,
            ErrorCode::DriveProtectDisk1,
            ErrorCode::DriveProtectDiskConfigSector,
            ErrorCode::DriveProtectCommandInvalidCrcChecksum,
            ErrorCode::DriveProtectCommandInvalidOpcode,
            ErrorCode::DriveProtectCommandKeysAlreadyLoaded,
            ErrorCode::DriveProtectCommandLoadZeroKey,
            ErrorCode::DriveProtectCommandInvalidConditions,
            ErrorCode::DriveProtectCommandInvalidSubOpcode,
            ErrorCode::DriveProtectCommandInvalidCustomerConfig,
            ErrorCode::DriveProtectDriveIsUnlocked,
            ErrorCode::DriveProtectDriveIsLocked,
            ErrorCode::DriveProtectEncryptionKeysNotLoaded,
            ErrorCode::DriveProtectDataShouldNotBeEncrypted,
            ErrorCode::DriveProtectDataShouldBeEncrypted,
            ErrorCode::DriveProtectCommandSetZeroPassword,
            ErrorCode::DriveProtectLockPrepareNotSet,
            ErrorCode::DriveProtectChangeKeyInLockCountdown,
            ErrorCode::DriveProtectMismatchHrn,
            ErrorCode::DriveProtectMismatchDrn,
            ErrorCode::DriveProtectMismatchPassword,
            ErrorCode::DriveProtectNoHrnOrDrnWasIssued,
            ErrorCode::DriveProtectNoPasswordSetInDrive,
            ErrorCode::DriveProtectSetPasswordInLockCountdown,
            ErrorCode::DriveProtectLockFailUpdateConfigOrFlush,
            ErrorCode::VscModeDisabled,
            ErrorCode::ConfigWriteFailed,
            ErrorCode::ClearDrmSectionFailed,
            ErrorCode::SetConfigLbaFailed,
            ErrorCode::FormatSetFailed,
            ErrorCode::ErrorRateTableNotPresent,
            ErrorCode::IbiModeBitNotSet,
            ErrorCode::PermanentOverlayAlreadyLoaded,
            ErrorCode::PermanentOverlayNotLoaded,
            ErrorCode::InvalidDrmGroupSection,
            ErrorCode::InvalidDrmGroupQSubsection,
            ErrorCode::DrmGroupQueueEmpty,
            ErrorCode::InvalidDrmGroupLSubsection,
            ErrorCode::InvalidDrmSubsection,
            ErrorCode::InvalidLifeQueueFlag,
            ErrorCode::ProtocolViolation,
            ErrorCode::DepopDisabled,
            ErrorCode::DepopInvalidApb,
            ErrorCode::LastBackgroundVscCommandFailed,
            ErrorCode::OdtaaNotInitialized,
            ErrorCode::TrackListDataNotAvailable,
            ErrorCode::OdtaaReadWatchdog,
            ErrorCode::RroNotInitialized,
            ErrorCode::DrmSpinCountersReadFail,
            ErrorCode::DrmActuatorCountersReadFail,
            ErrorCode::StaticFileInvalid,
            ErrorCode::MrmRequestFailed,
            ErrorCode::MrmRequestTimedOut,
            ErrorCode::MrmRequestRejected,
            ErrorCode::MrmRequestCanceled,
            ErrorCode::MrmStartCanceled,
            ErrorCode::MrmWaitCanceled,
            ErrorCode::ResourceAllocationFailed,
            ErrorCode::DrmLogNotLoaded,
            ErrorCode::DfhCalibrationFailed,
            ErrorCode::DrmLogSectionsBadChecksum,
            ErrorCode::VscDlg2NotActive,
            ErrorCode::DrmPeriodLogBadChecksum,
            ErrorCode::SmartReadSelfTestLog,
            ErrorCode::SmartDrmLogHasNotBeenLoaded,
            ErrorCode::SmartDrmLoadDisabled,
            ErrorCode::SmartLogPageA3NotLoaded,
            ErrorCode::CacheFlushCachedRelocationDisk,
            ErrorCode::CacheCannotInvalidateSdInDynamicState,
            ErrorCode::ResourceAllocationSectorsBeyondRange,
            ErrorCode::ResourceAllocationDisk,
            ErrorCode::ResourceAllocationNoContiguousBuffer,
            ErrorCode::ResourceAllocateTransientWithCacheValid,
            ErrorCode::ResourceAllocateTransientBufferUsed,
            ErrorCode::ResourceAllocateSdNoneAvailable,
            ErrorCode::ResourceAllocationNoBuffersAvailable,
            ErrorCode::ResourceAllocationDiskDbs,
            ErrorCode::ResourceAllocateTdNoneAvailable,
            ErrorCode::FmFileInfo,
            ErrorCode::FmDirectory,
            ErrorCode::FmFileId,
            ErrorCode::FmChecksum,
            ErrorCode::FmCompatibility,
            ErrorCode::FmTimeout,
            ErrorCode::FmNotStaticFile,
            ErrorCode::FmNoBuffer,
            ErrorCode::FmDriveNotReady,
            ErrorCode::FmFileHeaderSizeZero,
            ErrorCode::FmIncompatibleVersion,
            ErrorCode::FmNotFlashFile,
            ErrorCode::FmCannotRenameToExistingFileId,
            ErrorCode::FmFilesGoodButChecksumsDifferent,
            ErrorCode::FmHeaderIiSignatureInvalid,
            ErrorCode::FmHeaderIiTooSmall,
            ErrorCode::FmPartialFileRequestInvalid,
            ErrorCode::FmSectorCountExceedsMaxAllocatedBuffer,
            ErrorCode::FmFileSizeExceedsStaticBuffer,
            ErrorCode::FmPartialFileBufferOffsetExceedsEof,
            ErrorCode::FmPartialFileSectorCountExceedsEof,
            ErrorCode::FmNoSdCreatedForTheRequestedFileId,
            ErrorCode::FmNoMoreSpaceInDirectory,
            ErrorCode::FmNumberCopyMoreThanMaximum,
            ErrorCode::FmNoMoreSpaceInRegion,
            ErrorCode::FmFileCannotCreateOnExistingFile,
            ErrorCode::FmFileEntryNotFoundInDirectorySector,
            ErrorCode::FmTryingToCopyToSameRegion,
            ErrorCode::FmFileInitializationPlaceholderBitNotSet,
            ErrorCode::FmFileTargetRlbaOverlap,
            ErrorCode::FmNoContiguousSpaceInRegion,
            ErrorCode::FmCannotResolveOverlap,
            ErrorCode::FmCannotFreeEnoughSpace,
            ErrorCode::FmCannotCreateContiguousSpaceInRegion,
            ErrorCode::FmRequestRlbaExceedRegionBoundary,
            ErrorCode::FmDirectoryEntryNotSame,
            ErrorCode::FmDefragDetectPacketFileInReserved,
            ErrorCode::FmGatherFieldFile,
            ErrorCode::FmSortDirectorySector,
            ErrorCode::FmDefragReserved,
            ErrorCode::FmDirectoryWhileDelete,
            ErrorCode::FmFileNotFoundInAnyDirectory3742,
            ErrorCode::FmFileNotFoundInAnyDirectory3743,
            ErrorCode::FmFileNotFoundInAnyDirectory3746,
            ErrorCode::FmFile6FStructureIncorrect,
            ErrorCode::BackgroundPstAbortedByReset,
            ErrorCode::BackgroundPstAbortedByDeadman,
            ErrorCode::BackgroundInvalidAscanConfigParameters,
            ErrorCode::BackgroundTrackListDataNotAvailable,
            ErrorCode::BackgroundPstUnableToLoadPtm,
            ErrorCode::BackgroundPstResourceAllocationFailed,
            ErrorCode::BackgroundPstDisabledViaDbs,
            ErrorCode::BackgroundProcessingDisabled,
            ErrorCode::BackgroundPstInvalidPtmLoadAddress,
            ErrorCode::BackgroundPstInvalidPtmStartAddress,
            ErrorCode::BackgroundNonCaptiveMemoryTestNotAllowed,
            ErrorCode::BackgroundPstUnableToFlashPtm,
            ErrorCode::BackgroundSelfTestAbortedTimedOut,
            ErrorCode::OvmPermanentAlreadyLoaded,
            ErrorCode::OvmTransientAlreadyLoaded,
            ErrorCode::OvmPermanentNotLoaded,
            ErrorCode::OvmTransientNotLoaded,
            ErrorCode::OvmNotCompatible,
            ErrorCode::OvmChecksum,
            ErrorCode::OvmUndefinedFunction,
            ErrorCode::OvmBuildIdMismatch,
            ErrorCode::FlashUnknown,
            ErrorCode::FlashInvalidSectorAddress,
            ErrorCode::FlashWriteLatchEnable,
            ErrorCode::FlashWritePageSendByte,
            ErrorCode::FlashWritePageTimeout,
            ErrorCode::FlashWritePageToStaticMemory,
            ErrorCode::FlashReadBlockGetByte,
            ErrorCode::FlashByteCountExceedsDeviceLimit,
            ErrorCode::FlashInvalidAddress,
            ErrorCode::FlashInitialBootHeaderMissing,
            ErrorCode::FlashSendByteTimeout,
            ErrorCode::FlashReadCommand,
            ErrorCode::FlashInvalidData,
            ErrorCode::FlashDataCompare,
            ErrorCode::FlashDeviceId,
            ErrorCode::FlashReadInfoStartTimeout,
            ErrorCode::FlashStatusTimeout,
            ErrorCode::FlashCommandTimeout,
            ErrorCode::DptmMallocFailure,
            ErrorCode::DptmFreeFailure,
            ErrorCode::DptmInitializationFailure,
            ErrorCode::DptmFailedToProcessDownloadedPacket,
            ErrorCode::DptmFailedToBackUpFlashFiles,
            ErrorCode::DptmPreservedFileHasDiffVersionOrSize,
            ErrorCode::DptmUnhandledFileListException,
            ErrorCode::DptmFileIdNotInList,
            ErrorCode::DptmFlashWriteBufferInvalid,
            ErrorCode::DptmFlashImageTooBig,
            ErrorCode::DptmFlashProgramFailure,
            ErrorCode::DptmFailedToVerifyProgrammedFlash,
            ErrorCode::DptmFailedToApplyModBytes,
            ErrorCode::DptmCleanupFailure,
            ErrorCode::DptmFailedToWriteConfig,
            ErrorCode::DptmPacketDoesNotContainFileList,
            ErrorCode::DptmTryToRenameToAFileThatExisted,
            ErrorCode::DptmExceedsMaximumUndoList,
            ErrorCode::DptmReplaceOriginalFileNotExist,
            ErrorCode::DptmReplaceOriginalFileDifferentFileId,
            ErrorCode::DptmFlashDirectoryNotFoundInFlashFile,
            ErrorCode::DptmAc55UccmTotalBytesExpectedMismatch,
            ErrorCode::DptmAc55NewFile118NotIdenticalToOldOne,
            ErrorCode::DptmAc55InputNoKeySectorsDefined,
            ErrorCode::DptmAc55InputInvalidActionCode,
            ErrorCode::DptmAc55InputInvalidFunctionCodeRequest,
            ErrorCode::DptmAc55InputInvalidConfigCode,
            ErrorCode::DptmAc55InputCacheFamilyMismatch,
            ErrorCode::DptmAc55MajorRevisionMismatch,
            ErrorCode::DptmAc55FwStructureRevisionMismatch,
            ErrorCode::RseekMallocFailure,
            ErrorCode::RseekFreeFailure,
            ErrorCode::SelfTestCheckResFile,
            ErrorCode::SelfTestScan,
            ErrorCode::SelfTestSramHard,
            ErrorCode::SelfTestSramSoft,
            ErrorCode::SelfTestSramMultiSoft,
            ErrorCode::SelfTestDramHard,
            ErrorCode::SelfTestDramSoft,
            ErrorCode::SelfTestDramMultiSoft,
            ErrorCode::SelfTestTransientLoadFault,
            ErrorCode::FmtPlistNotFound,
            ErrorCode::FmtInvalidPlist,
            ErrorCode::FmtGlistNotFound,
            ErrorCode::FmtInvalidGlist,
            ErrorCode::FmtExceededPushDowns,
            ErrorCode::FmtPushDownListWriteFail,
            ErrorCode::FmtNewBadTracks,
            ErrorCode::FmtFailure,
            ErrorCode::FmtExceededGlist,
            ErrorCode::FmtGlistWriteFail,
            ErrorCode::FmtCapacity,
            ErrorCode::FmtZsdNotLoaded,
            ErrorCode::FmtBufferAllocation,
            ErrorCode::FmtRelocationListWriteFail,
            ErrorCode::FmtPushDownListNotFound,
            ErrorCode::FmtRelocationListNotFound,
            ErrorCode::FmtExceededRelocationList,
            ErrorCode::FmtWriteFail,
            ErrorCode::FmtExceededReservedPushDownList,
            ErrorCode::FmtReservedPushDownListWriteFail,
            ErrorCode::FmtReservedPushDownListNotLoaded,
            ErrorCode::FmtPlistPsnOutOfRange,
            ErrorCode::FmtPushCountOverflow,
            ErrorCode::FmtHashTableOverflow,
            ErrorCode::FmtInvalidZoneTable,
            ErrorCode::FmtCannotMergePAndGList,
            ErrorCode::FmtPlistWriteFail,
            ErrorCode::FmtSlipsExceedLimit,
            ErrorCode::FmtTrackPushedDown,
            ErrorCode::FmtFieldListWriteFail,
            ErrorCode::FmtPlistCylinderOutOfRange,
            ErrorCode::FmtPlistHeadOutOfRange,
            ErrorCode::FmtRemergeRequired,
            ErrorCode::FmtMlistWriteFail,
            ErrorCode::DflDefectFound,
            ErrorCode::DflNoDefect,
            ErrorCode::DflTrackDefectFound,
            ErrorCode::DflEmpty,
            ErrorCode::DflNoMemory,
            ErrorCode::DflPlistWrite,
            ErrorCode::DflGlistWrite,
            ErrorCode::DflPushDownListWrite,
            ErrorCode::DflRelocationListWrite,
            ErrorCode::DflGlistFull,
            ErrorCode::DflPlistFull,
            ErrorCode::DflClistFull,
            ErrorCode::DflPlistDefect,
            ErrorCode::DflGlistDefect,
            ErrorCode::DflNoList,
            ErrorCode::DflDef1LessThan,
            ErrorCode::DflDef1GreaterThan,
            ErrorCode::DflDef1Equal,
            ErrorCode::DflInvalidLba,
            ErrorCode::DflInvalidLbaRange,
            ErrorCode::DflDuplicateDefect,
            ErrorCode::DflDlistFull,
            ErrorCode::DflMlistFull,
            ErrorCode::DflVfsBufferConversion,
            ErrorCode::CacheRelocationSpareRwTimeout,
            ErrorCode::CacheRelocationInsufficientSpace,
            ErrorCode::CacheRelocationReadDoesNotExist,
            ErrorCode::CacheRelocationWriteDoesNotExist,
            ErrorCode::CacheRelocationReadNewFailure,
            ErrorCode::CacheRelocationLoadTrackFailure,
            ErrorCode::CacheRelocationRwInProgress,
            ErrorCode::DmTranslationOutOfRange,
            ErrorCode::DmSectorOutOfRange,
            ErrorCode::DmRmNotASpareRelocation,
            ErrorCode::DmRmNotASpareLba,
            ErrorCode::DmRmNotARelocationListEntry,
            ErrorCode::DmRmNotAUserLba,
            ErrorCode::DmRmInsertingInFullRelocationList,
            ErrorCode::DmRmInsertingAnExistingLba,
            ErrorCode::DmHeadCheckWrongCylinderForReservedArea,
            ErrorCode::DmHeadCheckWrongHeadForUserArea,
            ErrorCode::DmRmNoMoreSparesForCacheRelocation,
            ErrorCode::DmRmInsertingWuInFullRelocationList,
            ErrorCode::FmtReservedAreaPushDownListOverflow,
            ErrorCode::FmtReservedAreaPlistPsnOutOfRange,
            ErrorCode::FmtReservedAreaPlistFileIdInvalid,
            ErrorCode::FmtReservedAreaHashBlockEmpty,
            ErrorCode::FmtReservedAreaHeadCountZero,
            ErrorCode::FmtReservedAreaZoneTableSptZero,
            ErrorCode::FmtReservedAreaNoSparesAvailable,
            ErrorCode::FmtReservedAreaRegionCrossesHeadBoundary,
            ErrorCode::FmtReservedAreaRegionTooLarge,
            ErrorCode::FmtReservedAreaTwoRegionOnSameVirtualHead,
            ErrorCode::FmtReservedAreaIncorrectRaspControl,
            ErrorCode::FmtReservedAreaInvalidFilesEncountered,
            ErrorCode::FmtReservedAreaAltRpdListSizeMismatch,
            ErrorCode::FmtReservedAreaInvalidTargetSize,
            ErrorCode::FmtReservedAreaInvalidRaspRegionTable,
            ErrorCode::FmtReservedAreaInvalidTargetTable,
            ErrorCode::FmtReservedAreaRaspInMiddleOfSurface,
            ErrorCode::MemoryTestDataBus,
            ErrorCode::MemoryTestAddressBus,
            ErrorCode::MemoryTestDeviceBus,
            ErrorCode::HostDlmcBadCompChar,
            ErrorCode::HostDlmcBadChecksum,
            ErrorCode::HostDlmcInvalidPacket,
            ErrorCode::HostDlmcInvalidProdFamily,
            ErrorCode::HostDlmcInvalidTpiCode,
            ErrorCode::HostDlmcInvalidSectionOffset,
            ErrorCode::HostDlmcInvalidCustomerId,
            ErrorCode::HostDlmcTransferTooLarge,
            ErrorCode::HostDlmcSavingDriveState,
            ErrorCode::HostDlmcSizeOutOfRange,
            ErrorCode::HostDlmcInvalidPtmStartAddress,
            ErrorCode::HostDlmcNeedModule19EAnd19D,
            ErrorCode::HostLatchedFatalWriteFault,
            ErrorCode::HostResidentFilesNotLoaded,
            ErrorCode::HostDbsOccurred,
            ErrorCode::HostDlmc,
            ErrorCode::HostDlmcNoSd,
            ErrorCode::HostCacheOverlayNotLoaded,
            ErrorCode::HostDlmcNoPtmCode,
            ErrorCode::HostAbortedCommand,
            ErrorCode::HostTransferCancel,
            ErrorCode::CacheFlushFailure1,
            ErrorCode::CacheFlushFailure2,
            ErrorCode::CommandTimeout,
            ErrorCode::CommandTimeoutAvCcto,
            ErrorCode::CommandTimeoutSaStreaming,
            ErrorCode::CommandTimeoutTler,
            ErrorCode::CommandTimeoutReadTler,
            ErrorCode::CommandTimeoutWriteTler,
            ErrorCode::CommandTimeoutFlushTler,
            ErrorCode::CommandTimeoutReadTlerNetApp,
            ErrorCode::CommandTimeoutWriteTlerNetApp,
            ErrorCode::CommandTimeoutFlushTlerNetApp,
            ErrorCode::DlgInvalidTestTrackLba,
            ErrorCode::DlgNoWarehouseTracksExist,
            ErrorCode::DlgNotEnoughBufferForCheckpointRecovery,
            ErrorCode::DlgNoValidHeaderFound,
            ErrorCode::DlgInvalidCheckPointFound,
            ErrorCode::DlgCheckpointHeaderRead,
            ErrorCode::DlgCheckpointTrackRead,
            ErrorCode::DlgNotEnoughResourcesAllocation,
            ErrorCode::DlgDiskRequestTimedOut,
            ErrorCode::AccessDenied,
            ErrorCode::DiskEccCorrected,
            ErrorCode::DiskTaDetectStatus,
            ErrorCode::DiskTa2ndSyncMark,
            ErrorCode::DiskUnsafe2ndSyncMark,
            ErrorCode::DiskFifoOverUnder,
            ErrorCode::DiskFifoOverrun,
            ErrorCode::DiskFifoUnderrun,
            ErrorCode::DiskSectorPulseRg,
            ErrorCode::DiskSectorPulseWg,
            ErrorCode::DiskEccDataSize,
            ErrorCode::DiskRgOverServo,
            ErrorCode::DiskDam,
            ErrorCode::DiskDamTa,
            ErrorCode::DiskSectorPulseRgRecovered,
            ErrorCode::DiskEccDataSizeRecovered,
            ErrorCode::DiskRgOverServoRecovered,
            ErrorCode::DiskSpba,
            ErrorCode::DiskTimeoutSectorNotFound,
            ErrorCode::DiskCrc,
            ErrorCode::DiskEcuUnsafe,
            ErrorCode::DiskEcuUnsafeTa,
            ErrorCode::DiskFwEccFailure51A4,
            ErrorCode::DiskEcuWuPseudoLog,
            ErrorCode::DiskEcuWuPseudoNotLogged,
            ErrorCode::DiskEcuWuFlaggedLog,
            ErrorCode::DiskEcuWuFlaggedNotLogged,
            ErrorCode::DiskEcuTransferHwAssistRecovered,
            ErrorCode::DiskWrite,
            ErrorCode::DiskReadCrc,
            ErrorCode::DiskWriteCrc,
            ErrorCode::DiskLoggedCrc,
            ErrorCode::DiskWriteRllCrc,
            ErrorCode::DiskTimeoutDmNotActive,
            ErrorCode::DiskTimeoutDmNotActiveRead,
            ErrorCode::DiskTimeoutDmNotActiveWrite,
            ErrorCode::DiskTimeoutBufferNotReady,
            ErrorCode::DiskTimeoutDf,
            ErrorCode::DiskGeneral,
            ErrorCode::DiskTimeoutTler,
            ErrorCode::DiskEventTimeoutDf,
            ErrorCode::DiskBufferFull,
            ErrorCode::DiskTimeoutSeekNotStarted,
            ErrorCode::DiskFwEccFailure5204,
            ErrorCode::DiskRecalibrationFailure,
            ErrorCode::ToneScanDefectBufferOverflow,
            ErrorCode::DiskWedgeCommandInProgress,
            ErrorCode::DiskSpinUpTimeout1,
            ErrorCode::DiskSpinUpTimeout2,
            ErrorCode::DiskWedgeInvalidCount,
            ErrorCode::DiskRelocationPermanentOverlayNotLoaded,
            ErrorCode::RelocationSstFailed,
            ErrorCode::RelocationNotPossibleOnSpare,
            ErrorCode::RelocationInvalidRequestCount,
            ErrorCode::RelocationNotPossibleOnReserved,
            ErrorCode::DiskStopOnCreateRelocation,
            ErrorCode::DiskStopOnCreateTare,
            ErrorCode::RelocationDisabled,
            ErrorCode::RelocationDisabledFormatUnitNotRun,
            ErrorCode::RelocationInPostReadCanceled,
            ErrorCode::RelocationLastLbaInUserArea,
            ErrorCode::RelocationDisabledNotInitialized,
            ErrorCode::RelocationEcSpareLbaWithNoUserLba,
            ErrorCode::PmIllegalModeTransition,
            ErrorCode::PmTransitionNotNeededOk,
            ErrorCode::DiskChannelHeadSizeInitialization,
            ErrorCode::DiskServo,
            ErrorCode::DiskServoNotReady,
            ErrorCode::DiskServoHeadNotFound,
            ErrorCode::DiskServoSpindle,
            ErrorCode::DiskServoSpindleSpinUp,
            ErrorCode::DiskServoSpindleSpinDown,
            ErrorCode::DiskServoSpindleOffSpeed,
            ErrorCode::DiskServoSpindleSpinUpFatal,
            ErrorCode::DiskServoSpindleGetSpinUpTime,
            ErrorCode::DiskServoActuator,
            ErrorCode::DiskServoActuatorDriveFault,
            ErrorCode::DiskServoActuatorAbort,
            ErrorCode::DiskServoActuatorWriteInhibit,
            ErrorCode::DiskServoActuatorControlFault,
            ErrorCode::DiskServoActuatorShockFault,
            ErrorCode::DiskServoActuatorWriteUnsafe,
            ErrorCode::DiskServoActuatorFault,
            ErrorCode::DiskServoActuatorWgMask,
            ErrorCode::DiskServoActuatorFatal,
            ErrorCode::DiskServoActuatorTimeout,
            ErrorCode::DiskServoActuatorSail,
            ErrorCode::DiskServoActuatorBadWedge,
            ErrorCode::DiskServoActuatorNoScTargetWedge,
            ErrorCode::DiskServoActuatorSplitEnTimeout,
            ErrorCode::DiskServoActuatorTbgUnlockDetect,
            ErrorCode::DiskServoWriteFaultUnsafe,
            ErrorCode::DiskServoWriteFaultSpindleAtSpeed,
            ErrorCode::DiskServoWriteFaultSsmTimeout,
            ErrorCode::DiskServoWriteFaultIllegalGrayCode,
            ErrorCode::DiskServoWriteFaultIllegalCylinder,
            ErrorCode::DiskServoWriteFaultOffTrack,
            ErrorCode::DiskServoWriteFaultFatalPathServoDead,
            ErrorCode::DiskServoWriteFaultReadOffTrack,
            ErrorCode::DiskServoWriteFaultToneScanSsmTimeout,
            ErrorCode::DiskServoWriteFaultLowGrayCodeQuality,
            ErrorCode::DiskServoWriteFaultPredictiveOffTrack,
            ErrorCode::DiskServoWriteFaultBadSignDetected,
            ErrorCode::DiskServoWriteFaultBadParityDetected,
            ErrorCode::DiskServoWriteFaultToneScanSsmTimeout53DE,
            ErrorCode::DiskServoRequiresActuatorInitialization,
            ErrorCode::DiskServoActuatorAiNoScTargetWedge,
            ErrorCode::DiskServoActuatorAiTimeout,
            ErrorCode::DiskServoActuatorAiBadWedge,
            ErrorCode::DiskServoActuatorAiFatal,
            ErrorCode::DiskServoActuatorAiServoStateNotActive,
            ErrorCode::DiskServoWriteFaultAiUnsafe,
            ErrorCode::DiskServoWriteFaultAiSpindleAtSpeed,
            ErrorCode::DiskServoWriteFaultAiSsmTimeout,
            ErrorCode::DiskServoWriteFaultAiIllegalGrayCode,
            ErrorCode::DiskServoWriteFaultAiIllegalCylinder,
            ErrorCode::DiskServoWriteFaultAiOffTrack,
            ErrorCode::DiskServoWriteFaultAiFatalPathServoDead,
            ErrorCode::DiskServoWriteFaultAiReadOffTrack,
            ErrorCode::DiskServoWriteFaultAiShockSensor,
            ErrorCode::DiskServoWriteFaultAiLowGrayCodeQuality,
            ErrorCode::DiskServoWriteFaultAiOffTrackOccurred,
            ErrorCode::DiskServoWriteFaultAiBadSignDetected,
            ErrorCode::DiskServoWriteFaultAiBadParityDetected,
            ErrorCode::SvirOk,
            ErrorCode::SvirAbort,
            ErrorCode::SvirModel,
            ErrorCode::SvirHead,
            ErrorCode::SvirCylinder,
            ErrorCode::SvirParameter1,
            ErrorCode::SvirParameter2,
            ErrorCode::SvirParameter3,
            ErrorCode::SvirSubcommand,
            ErrorCode::SvirLength,
            ErrorCode::SvirCommand,
            ErrorCode::Svir,
            ErrorCode::SvirTimeout,
            ErrorCode::SvirIndex,
            ErrorCode::SvirSector,
            ErrorCode::SvirSam,
            ErrorCode::SvirWrroOnTrack2Learn,
            ErrorCode::SvirWrroOnTrack2Write,
            ErrorCode::SvirWrroCalibrationLearn,
            ErrorCode::SvirWrroBurst,
            ErrorCode::SvirWrroReadLimitLearn,
            ErrorCode::SvirWrroJogLearn,
            ErrorCode::SvirWrroWrite,
            ErrorCode::SvirWrroSetup,
            ErrorCode::SvirWrroTroLimitLearn,
            ErrorCode::SvirWrroTimeout,
            ErrorCode::SvirSpinUp,
            ErrorCode::SvirSpinDown,
            ErrorCode::SvirActuatorSpeed,
            ErrorCode::SvirActuatorSync,
            ErrorCode::SvirActuatorUnlatch,
            ErrorCode::SvirActuatorPdFail,
            ErrorCode::SvirDriveNotCalibrated,
            ErrorCode::SvirWrroLoad,
            ErrorCode::SvirWrroCompare,
            ErrorCode::SvirPesMissSample,
            ErrorCode::SvirWrroDataIsrTimeout544B,
            ErrorCode::SvirWrroDataIsrSync,
            ErrorCode::SvirCalibrationNormal,
            ErrorCode::SvirCalibrationFlex,
            ErrorCode::SvirCalibrationMotor,
            ErrorCode::SvirCalibrationRro,
            ErrorCode::SvirCalibrationFGain,
            ErrorCode::SvirCalibrationSeek,
            ErrorCode::SvirCalibrationApGain,
            ErrorCode::SvirCalibrationLtr,
            ErrorCode::SvirCalibrationGainS,
            ErrorCode::SvirCalibrationTangentialHeadOffset,
            ErrorCode::SvirCalibrationBandwidth,
            ErrorCode::SvirFTemperatureInvalid,
            ErrorCode::SvirAfcCalibrationF,
            ErrorCode::SvirRroOverflow,
            ErrorCode::SvirRroAlgorithm,
            ErrorCode::SvirRampLoadUnload,
            ErrorCode::SvirLatchHang,
            ErrorCode::SvirLoad2Fast,
            ErrorCode::SvirLoad2Slow,
            ErrorCode::SvirIrCalibration,
            ErrorCode::SvirAdChange,
            ErrorCode::SvirRampCalibrationRange,
            ErrorCode::SvirCalibrationRroHead0,
            ErrorCode::SvirCalibrationRroHead1,
            ErrorCode::SvirCalibrationRroHead2,
            ErrorCode::SvirCalibrationRroHead3,
            ErrorCode::SvirCalibrationRroHead4,
            ErrorCode::SvirCalibrationRroHead5,
            ErrorCode::SvirCalibrationRroHead6,
            ErrorCode::SvirCalibrationRroHead7,
            ErrorCode::SvirSTraceFile,
            ErrorCode::SioInvalidSdIndex,
            ErrorCode::SioInvalidBufferPointer,
            ErrorCode::SioTimeout,
            ErrorCode::SppDeviceInitialization,
            ErrorCode::SioInvalidParameter,
            ErrorCode::SioInvalidTransferHeaderChecksum,
            ErrorCode::SioInvalidCommandFunction,
            ErrorCode::SioInvalidCrc,
            ErrorCode::SioUnknown,
            ErrorCode::SioTransferRequestExceedsAvailableData,
            ErrorCode::SioAckSizeExceedsTransferLength,
            ErrorCode::SioTransferAbortRequest,
            ErrorCode::SioTimeoutTransmitIsr,
            ErrorCode::SioTimeoutTransmitData,
            ErrorCode::SioTimeoutReceiveIsr,
            ErrorCode::SioTimeoutTransferRequest,
            ErrorCode::SioProtocolCommandNotExpected,
            ErrorCode::SioInvalidTransferPayloadLength,
            ErrorCode::SioInvalidTransferLength,
            ErrorCode::SioNoDataTransferInProgress,
            ErrorCode::SioTransferCommandAckSize,
            ErrorCode::SioInvalidAbn,
            ErrorCode::SioMaxCrcsReceiveData,
            ErrorCode::SioCrcReceivedRetrySent,
            ErrorCode::SioMaxRetries,
            ErrorCode::SioInvalidTransferCommandPayloadLength,
            ErrorCode::SioCommandAborted,
            ErrorCode::SioTimeoutWaitingForAckCommand,
            ErrorCode::SioMaxCrcsNonSequenceAbn,
            ErrorCode::SioWaitingForTransferComplete,
            ErrorCode::SioInvalidCommandHeaderReserveField,
            ErrorCode::SioInvalidTransferHeaderParameter,
            ErrorCode::SioInvalidTransferCommandPayloadCrc,
            ErrorCode::SioInvalidTransferCommandDirectionField,
            ErrorCode::SioInvalidAckHeaderReserved,
            ErrorCode::SioInvalidTransferCompleteReserved,
            ErrorCode::SioInvalidRetryCommandReservedField,
            ErrorCode::SioInvalidAckHeaderLength,
            ErrorCode::SioInvalidTransferCompleteLength,
            ErrorCode::SioInvalidRetryCommandLengthField,
            ErrorCode::SioInvalidAckHeaderParameter,
            ErrorCode::SioInvalidTransferCompleteParameter,
            ErrorCode::SioInvalidRetryCommandParameterField,
            ErrorCode::SioInvalidAckCommandChecksum,
            ErrorCode::SioInvalidTransferCompleteChecksum,
            ErrorCode::SioInvalidRetryCommandChecksum,
            ErrorCode::UartOther,
            ErrorCode::UartOverrun,
            ErrorCode::UartFraming,
            ErrorCode::DiskServoPztFault,
            ErrorCode::CacheFlushAllGotCanceled,
            ErrorCode::CacheFlushCachedRelocationGotCanceled,
            ErrorCode::DiskCancel,
            ErrorCode::ExecutionOperationCanceled,
            ErrorCode::ResourceAllocationGotCanceled,
            ErrorCode::BackgroundCanceled,
            ErrorCode::CacheRelocationOperationCanceled,
            ErrorCode::FormatUnitCanceled,
            ErrorCode::FmCanceled,
            ErrorCode::DiskRemoveByRequest,
            ErrorCode::HostOperationCanceled,
            ErrorCode::AggressiveOlDrmFlushCanceled,
            ErrorCode::Dlg2GotCanceled,
            ErrorCode::PtmPstInvalidParameter,
            ErrorCode::PtmPstIncompatibleVersion,
            ErrorCode::PtmPstMem49ReadFail,
            ErrorCode::PtmPstMem4AReadFail,
            ErrorCode::PtmPstFile49ReadFail,
            ErrorCode::PtmPstFile49WriteFail,
            ErrorCode::PtmPstFile4AReadFail,
            ErrorCode::PtmPstFile4AWriteFail,
            ErrorCode::PtmPstFlexBiasCalibrationFail,
            ErrorCode::PtmPstMotorTorqueCalibrationFail,
            ErrorCode::PtmPstGainCalibrationFail,
            ErrorCode::PtmPstBiasLinCalibrationFail,
            ErrorCode::PtmPstLoadFail,
            ErrorCode::PtmPstUnloadFail,
            ErrorCode::PtmPstReadMemoryTableFail,
            ErrorCode::PtmPstWriteReadVerifyFail,
            ErrorCode::PtmPstResidentFileCreateFail,
            ErrorCode::PtmPstResidentFileReadFail,
            ErrorCode::PtmPstResidentFileWriteFail,
            ErrorCode::PtmPstTooManyMeasurePoints,
            ErrorCode::PtmPstSetBodeOffsetFail,
            ErrorCode::PtmPstAcBodeFail,
            ErrorCode::PtmPstFullStrokeServoHang,
            ErrorCode::PtmPstFullStrokeSeekLimitFail,
            ErrorCode::PtmPstFullStrokeServoLimitFail,
            ErrorCode::PtmPstRunningAverageSeekInvalidResult,
            ErrorCode::PtmPstDataSizeLargerThanAllocateMemory,
            ErrorCode::PtmPstNxBodeFail,
            ErrorCode::PtmPstSeamCalibrationFail,
            ErrorCode::PtmPstFile4DReadFail,
            ErrorCode::PtmPstFile4FReadFail,
            ErrorCode::PtmPstFile4DWriteFail,
            ErrorCode::PtmPstFile4FWriteFail,
            ErrorCode::PtmPstPowerUpBandwidthCalibrationFail,
            ErrorCode::PtmPstPowerUpBandwidthHeadExceeded,
            ErrorCode::PtmPstPowerUpBandwidthCalibrationNotValid,
            ErrorCode::PtmPstFile49CreateFail,
            ErrorCode::PtmPstFile4ACreateFail,
            ErrorCode::PtmPstMiniCalibrationInDvtFail,
            ErrorCode::PtmPstMiniCalibrationInNotValid,
            ErrorCode::PtmPstMiniCalibrationInHeadExceeded,
            ErrorCode::PtmPstFatalWrroB5LogFull,
            ErrorCode::PtmPstNeedClear4F,
            ErrorCode::EnableDsaFail,
            ErrorCode::ArcoChsWrite,
            ErrorCode::ArcoChsRead,
            ErrorCode::ArcoInvalidDcmCodes721A,
            ErrorCode::ArcoInvalidConfigFileOrFormat,
            ErrorCode::ArcoDirectorySectorRead,
            ErrorCode::ArcoFile46hChecksum,
            ErrorCode::ArcoInvalidCommandInCoBuffer,
            ErrorCode::ArcoChecksum,
            ErrorCode::ArcoIncompatiblePstVersion,
            ErrorCode::ArcoIncompatibleChannelFirmwareVersion,
            ErrorCode::ArcoIncompatibleVscFirmwareVersion,
            ErrorCode::ArcoVscPressureSensor,
            ErrorCode::ArcoPressureSensorDriverInitialization,
            ErrorCode::ArcoPressureSensorRefLimitExceeded,
            ErrorCode::ArcoPressureSensorRefThresholdExceeded,
            ErrorCode::ArcoSetup,
            ErrorCode::ArcoFileId,
            ErrorCode::ArcoFileRead,
            ErrorCode::ArcoModuleRead,
            ErrorCode::ArcoFileWrite,
            ErrorCode::ArcoInvalidHeader,
            ErrorCode::ArcoTooManyZones,
            ErrorCode::ArcoTooManyHeads,
            ErrorCode::ArcoTestTimeExceedLimit,
            ErrorCode::ArcoInvalidInput,
            ErrorCode::ArcoFailedToReadCodataFile,
            ErrorCode::ArcoFailedToCreateFile,
            ErrorCode::ArcoFailedToSwitchWcs,
            ErrorCode::ArcoStandbyCommandFailed,
            ErrorCode::ArcoRecalCommandFailed,
            ErrorCode::ArcoLogicalToBpiZoneTranslate,
            ErrorCode::ArcoBpiToLogicalZoneTranslate,
            ErrorCode::ArcoCurveFitOrderOutOfBound,
            ErrorCode::ArcoNotEnoughDataPointForCurveFit,
            ErrorCode::ArcoUnexpectedDataInLog,
            ErrorCode::ArcoFailedToReadModule,
            ErrorCode::ArcoTdCalibrationSameFlexMinMax,
            ErrorCode::ArcoRecoveryRegisterListTooSmall,
            ErrorCode::ArcoInvalidDcmCodes72C8,
            ErrorCode::ArcoVscTranslation,
            ErrorCode::ArcoVscGetDriveData,
            ErrorCode::ArcoVscReadWriteMemoryFile46,
            ErrorCode::ArcoVscExceptionControlMrrCycling,
            ErrorCode::ArcoVscExceptionControlJogInterpolation,
            ErrorCode::ArcoVscExceptionControlChannelUpdate,
            ErrorCode::ArcoVscExceptionHandlingCommand,
            ErrorCode::ArcoVscReadErrorRateTableCommand,
            ErrorCode::ArcoVscMnpAccessCommand,
            ErrorCode::ArcoVscSpinDownCommand,
            ErrorCode::ArcoVscSpinUpCommand,
            ErrorCode::ArcoReadWriteFieldCommand,
            ErrorCode::ArcoFormatSelectCommand,
            ErrorCode::ArcoVscCommandEventPending,
            ErrorCode::ArcoVscSwitchFormatCommand,
            ErrorCode::ArcoInitializeDefectList,
            ErrorCode::ArcoInitializeGoodCylinderList,
            ErrorCode::ArcoCannotFindGoodCylinder,
            ErrorCode::ArcoInvalidZone,
            ErrorCode::ArcoDeltaGreaterThanThreshold,
            ErrorCode::ArcoPrepTestTrack,
            ErrorCode::ArcoDriveTemperatureCalibration,
            ErrorCode::ArcoInvalidPreampGainValue,
            ErrorCode::ArcoPreampGainCalibration,
            ErrorCode::ArcoCodataAddressInvalid,
            ErrorCode::ArcoTemperatureAboveTargetTemperature,
            ErrorCode::ArcoPbertDvtWrite,
            ErrorCode::ArcoIllegalOptimizationNumberRequested,
            ErrorCode::ArcoIllegalSptRequested,
            ErrorCode::ArcoInvalidDcmCode,
            ErrorCode::ArcoBadChecksumInDataFile,
            ErrorCode::ArcoInvalidBuffer,
            ErrorCode::ArcoUnsupportedPreampId,
            ErrorCode::ArcoBadOrInvalidLogInfo,
            ErrorCode::ArcoInvalidEntry,
            ErrorCode::ArcoTooManyFormatCode,
            ErrorCode::ArcoFailedToAccessFullStroke,
            ErrorCode::ArcoInvalidModelList,
            ErrorCode::IbiDefaultAbortCode,
            ErrorCode::IbiFullHeadSurfaceLog7603,
            ErrorCode::IbiFullHeadSurfaceLog7604,
            ErrorCode::IbiFullHeadSurfaceLog7605,
            ErrorCode::IbiFullHeadSurfaceLog7608,
            ErrorCode::IbiTimeout,
            ErrorCode::IbiServoLogTest,
            ErrorCode::IbiExceededHeadDefectsLimit760D,
            ErrorCode::IbiFmtCapacity760E,
            ErrorCode::IbiSpecifiedCapacityNotReached760F,
            ErrorCode::IbiFmtCapacity7611,
            ErrorCode::IbiSpecifiedCapacityNotReached7612,
            ErrorCode::IbiFmtCapacity7613,
            ErrorCode::IbiFullHeadSurfaceLog7614,
            ErrorCode::IbiExceededDefectsLimit7616,
            ErrorCode::IbiTlist,
            ErrorCode::IbiExceededHeadDefectsLimit761C,
            ErrorCode::IbiIllegalParameters,
            ErrorCode::IbiLogRead,
            ErrorCode::IbiExceededHeadDefectsLimit764A,
            ErrorCode::IbiWrongParameters,
            ErrorCode::IbiTestMini7652,
            ErrorCode::IbiReservedPushDownListOverflow,
            ErrorCode::IbiTestMini7657,
            ErrorCode::IbiTestB9ExceededDefectsLimit,
            ErrorCode::IbiTooManySoftErrors,
            ErrorCode::IbiTestBaTooManySoftErrors,
            ErrorCode::IbiExceededDefectsLimit767C,
            ErrorCode::IbiExceededDefectsLimit767E,
            ErrorCode::IbiTestD1FullHeadSurfaceLog,
            ErrorCode::IbiTestB9,
            ErrorCode::IbiFmtWriteFailExceededDefectsLimit,
            ErrorCode::IbiTooManyTracksInPlist76F6,
            ErrorCode::IbiTooManyTracksInPlist76F7,
            ErrorCode::IbiNativeMaximumLbaTooBig,
            ErrorCode::IbiExceededDefectsLimit76FF,
            ErrorCode::PtmInvalidVectorTableVersion,
            ErrorCode::HalInvalidParameter,
            ErrorCode::HalFlashUnknown,
            ErrorCode::HalFlashInvalidSectorAddress,
            ErrorCode::HalFlashWriteLatchEnable,
            ErrorCode::HalFlashWritePageSendByte,
            ErrorCode::HalFlashWritePageTimeout,
            ErrorCode::HalFlashWritePageToStaticMemory,
            ErrorCode::HalFlashReadBlockGetByte,
            ErrorCode::HalFlashByteCountExceedsDeviceLimit,
            ErrorCode::HalFlashInvalidAddress,
            ErrorCode::HalFlashInitialBootHeaderMissing,
            ErrorCode::HalFlashSendByteTimeout,
            ErrorCode::HalFlashReadCommand,
            ErrorCode::HalFlashInvalidData,
            ErrorCode::HalFlashDataCompare,
            ErrorCode::HalFlashDeviceId,
            ErrorCode::HalFlashReadInfoStartTimeout,
            ErrorCode::HalFlashStatusTimeout,
            ErrorCode::HalFlashCommandTimeout,
            ErrorCode::HalFlashBadChecksum,
            ErrorCode::HalSystemPllLockFailure,
            ErrorCode::HalSystemSppCheckFail,
            ErrorCode::HalSystemUartFifoFull,
            ErrorCode::HalSystemUartFifoEmpty,
            ErrorCode::HalSystemUartOverrun,
            ErrorCode::HalSystemUartOthers,
            ErrorCode::HalSystemUartTransmitFifoFull,
            ErrorCode::VscNotSupportReadWrro,
            ErrorCode::SvirWrroDataIsrTimeout884B,
            ErrorCode::SvirSeekBusy,
            ErrorCode::InvalidModActCodeRequest,
            ErrorCode::VscCommandSetNotEnabled,
            ErrorCode::InvalidModByteInModifyConfigSector,
            ErrorCode::InvalidSmartEnableCode,
            ErrorCode::InvalidOperationRequest,
            ErrorCode::OffsetTooLarge,
            ErrorCode::InvalidHeadNumber,
            ErrorCode::CylinderAboveLimit,
            ErrorCode::InvalidWedgeOffset,
            ErrorCode::InvalidWedgeSize,
            ErrorCode::StartAddressTooLarge,
            ErrorCode::LengthTooLarge,
            ErrorCode::InvalidTableId,
            ErrorCode::UnsupportedActionCode,
            ErrorCode::UnsupportedFunction,
            ErrorCode::ValueActionCodeUnsupportedFeature,
            ErrorCode::ValueActionCodeUnsupportedOperation,
            ErrorCode::InvalidFunctionCodeRequest,
            ErrorCode::TableOffsetTooLarge,
            ErrorCode::InvalidExceptionFeature,
            ErrorCode::InvalidOffset,
            ErrorCode::InvalidKeySectorSize,
            ErrorCode::TransferRequestExceedAvailableData,
            ErrorCode::InvalidVscSource,
            ErrorCode::ActionCodeOutOfRange,
            ErrorCode::KeySectorMustPrecedeDataTransferRequest,
            ErrorCode::InvalidSettleMode,
            ErrorCode::InvalidEnableDisableKeyInFeatureRegisters,
            ErrorCode::FunctionNotSupportedOnSocPlatform,
            ErrorCode::InvalidSectorRequest,
            ErrorCode::FlashLengthTooSmall,
            ErrorCode::FlashStartAddressTooSmall,
            ErrorCode::FlashStartSectorTooSmall,
            ErrorCode::FlashAccessRangeRequestTooLarge,
            ErrorCode::InvalidLbaRequest,
            ErrorCode::PstBufferNotAllocated,
            ErrorCode::CommandNotAllowedFromPst,
            ErrorCode::VscInvalidPstTestId,
            ErrorCode::VscInvalidPstVectorAddress,
            ErrorCode::PstVscdBufferTooSmall,
            ErrorCode::SectorOffsetNotFromZero,
            ErrorCode::InvalidResourceMemoryRequest,
            ErrorCode::InvalidPstTestModeRequest,
            ErrorCode::ClearDrmLogFailed,
            ErrorCode::ClearFactoryFileFailed,
            ErrorCode::WarningWearLevelWithBackgroundDisabled,
            ErrorCode::InvalidWearLevelArgument,
            ErrorCode::InvalidPeriodShiftCtlrArgument,
            ErrorCode::WarningDrmFlushWithBackgroundDisabled,
            ErrorCode::InvalidDrmFlushControlArgument,
            ErrorCode::InvalidSmartBackdoorArgument,
            ErrorCode::InvalidBackgroundActivityArgument,
            ErrorCode::InvalidDriveTemperatureSamplingArgument,
            ErrorCode::InvalidClearDrmSection,
            ErrorCode::DepopInvalidHeadId,
            ErrorCode::RequestEndLbaLessThanStart,
            ErrorCode::DepopOnlyOneHead,
            ErrorCode::InvalidPstModeArgument,
            ErrorCode::PushDownsOnTrack,
            ErrorCode::InvalidPeriodSumParameter,
            ErrorCode::HostDataTransferDidNotOccur,
            ErrorCode::InvalidClearDrmAgentCode,
            ErrorCode::FeatureControlInvalidArgument,
            ErrorCode::FeatureControlReadUnsupported,
            ErrorCode::MemoryTableIsReadOnly,
            ErrorCode::DebugStopOccurred,
            ErrorCode::ReadWriteFieldInvalidLength,
            ErrorCode::RequestedRelocationsGreaterThanAvailable,
            ErrorCode::InvalidLength,
            ErrorCode::InvalidCountValue,
            ErrorCode::InvalidAddressMode,
            ErrorCode::ServoTraceDisabled,
            ErrorCode::ConfigServoTraceAlreadyEnabled,
            ErrorCode::InvalidStartWedge,
            ErrorCode::InvalidZoneNumber,
            ErrorCode::CylinderNotInGainCalibrationZone,
            ErrorCode::DisableGainCalibrationToRunThisCommand,
            ErrorCode::InvalidConfigSection,
            ErrorCode::SvirInvalidTableSize,
            ErrorCode::GainCalibrationTableNotInitialized,
            ErrorCode::GainCalibrationTrainingNotStarted,
            ErrorCode::GainCalibrationValueNotTrained,
            ErrorCode::GainCalibrationFeatureNotImplemented,
            ErrorCode::PartialFileRequestPastEof,
            ErrorCode::InvalidDvtOpcode,
            ErrorCode::TemporarySramStaticAlreadyAllocated,
            ErrorCode::TableNotAvailableCacheRelocationDisabled,
            ErrorCode::NoTemporarySramStaticAllocated,
            ErrorCode::InvalidSmartAttributeStatus,
            ErrorCode::InvalidSmartAttributeId,
            ErrorCode::SinglePassToneScanNotSupported,
            ErrorCode::InvalidWearLevelMode,
            ErrorCode::InvalidWearLevelConfigTableNotAvailable,
            ErrorCode::DepopIbiSurface1LogNotInTrackDirectory,
            ErrorCode::DepopIbiSurface1PesNotInTrackDirectory,
            ErrorCode::PartialFileNotInPstMode,
            ErrorCode::DfhModeNotEnabled,
            ErrorCode::ParameterOutOfRange,
            ErrorCode::FmtSelectCapacityFailure,
            ErrorCode::DcmUninitialized,
            ErrorCode::CapacityGroupDefinition,
            ErrorCode::WriteReadGapInfoNotAvailable,
            ErrorCode::DriveProtectLocked,
            ErrorCode::InvalidRegionNumber,
            ErrorCode::DiskBackendTableNotPresent,
            ErrorCode::CompareIdsLbaMiscompare,
            ErrorCode::FlexBiasFilter,
            ErrorCode::VscInvalidConfigCode,
            ErrorCode::VscInvalidConfigDataHeader,
            ErrorCode::VscInvalidConfigData,
            ErrorCode::VscParameterLengthMismatch,
            ErrorCode::VscParameterTypeMismatch,
            ErrorCode::VscRuleCheckFail,
            ErrorCode::VscCheckListEmpty,
            ErrorCode::VscNoMatchingEntryInTable,
            ErrorCode::VscInvalidEntryInTable,
            ErrorCode::VscCommandResponseThresholdExceeded,
            ErrorCode::VscCommandResponseProcessingInProgress,
            ErrorCode::VscMajorRevisionMismatch,
            ErrorCode::VscCvfFileNotFound,
            ErrorCode::VscMajorRevisionAlreadySet,
            ErrorCode::VscFeatureSetKeyFail,
            ErrorCode::VscSOverlayAlreadyLoaded,
            ErrorCode::VscCOverlayAlreadyLoaded,
            ErrorCode::VscMismatchFamilyId,
            ErrorCode::VscCvfFileRead,
            ErrorCode::VscFailureInUpdateDataFile,
            ErrorCode::VscInvalidLowMemoryModeArgument,
            ErrorCode::VscPreAc55ConfigDrive,
            ErrorCode::VscUccmTotalBytesExpectedMismatch,
            ErrorCode::VscAc55FunctionCode1SupportDisabled,
            ErrorCode::VscAc55FunctionCode2SupportDisabled,
            ErrorCode::VscAc55FunctionCode3SupportDisabled,
            ErrorCode::VscAc55FunctionCode4SupportDisabled,
            ErrorCode::VscAc55FunctionCode5SupportDisabled,
            ErrorCode::VscAc55FunctionCode6SupportDisabled,
            ErrorCode::VscAc55FunctionCode7SupportDisabled,
            ErrorCode::VscAc55FunctionCode8SupportDisabled,
            ErrorCode::VscAc55FunctionCode9SupportDisabled,
            ErrorCode::VscAc55FunctionCode10SupportDisabled,
            ErrorCode::VscPOverlayAlreadyLoaded,
            ErrorCode::InvalidSmartEnCode,
            ErrorCode::SmartInvalidHostSectorRequest,
            ErrorCode::SmartInvalidVendorSectorRequest,
            ErrorCode::SmartFeatureNotSupported,
            ErrorCode::SmartInvalidSectorCount,
            ErrorCode::VscInvalidRaspTarget,
            ErrorCode::VscInvalidRaspTargetTrack,
            ErrorCode::VscPstRaspGetBufferTooSmall,
            ErrorCode::SmartInvalidDefectListType,
            ErrorCode::SmartInvalidDefectListFormat,
            ErrorCode::SmartWriteSelectTestSelfTestInProgress,
            ErrorCode::SmartSelectiveTestInvalidVersion,
            ErrorCode::SmartOfflineImmediateDisabled,
            ErrorCode::SmartResourceAllocationFailed,
            ErrorCode::SmartStatusFailed,
            ErrorCode::ObsoleteCommand,
            ErrorCode::SmartCommandWithSmartDisabled,
            ErrorCode::FeatureNotSupported,
            ErrorCode::SecurityCommandWithBadParameter,
            ErrorCode::SecurityCommandSecurityModeDisabled,
            ErrorCode::SecurityCommandWithDiskFrozen,
            ErrorCode::SecurityCommandWithDiskLocked,
            ErrorCode::SecurityCommandWithDiskLockedOrFrozen,
            ErrorCode::SecurityCommandWithDiskExpiredOrFrozen,
            ErrorCode::SecurityCommandWithReceiveData,
            ErrorCode::SecurityCommandWithPasswordMiscompare,
            ErrorCode::SecurityCommandWithPasswordIsZero,
            ErrorCode::SecurityInvalidMasterPasswordRevision,
            ErrorCode::SecurityCommandNoErasePrepare,
            ErrorCode::SecurityCommandReceivedInInvalidState,
            ErrorCode::VscSecurityDriveIsLocked,
            ErrorCode::VscSecurityUnlockFailedAfterFormatUnit,
            ErrorCode::VscSecurityInvalidFmtUnitOptions,
            ErrorCode::ChsLbaTooLarge,
            ErrorCode::ErrorInjectionInvalidFunctionCode,
            ErrorCode::ErrorInjectionInvalidType,
            ErrorCode::ErrorInjectionInvalidNrzMode,
            ErrorCode::ErrorInjectionInvalidHandle,
            ErrorCode::ErrorInjectionInvalidCount,
            ErrorCode::ErrorInjectionInvalidOffset,
            ErrorCode::ErrorInjectionInvalidLength,
            ErrorCode::ErrorInjectionInvalidRepeatCount,
            ErrorCode::ErrorInjectionIeitblFull,
            ErrorCode::ErrorInjectionIeitblEmpty,
            ErrorCode::ErrorInjectionSameTrackExist,
            ErrorCode::ErrorInjectionTargetNotFound,
            ErrorCode::ErrorInjectionAddTargetFailed,
            ErrorCode::ErrorInjectionRemoveTargetFailed,
            ErrorCode::BackgroundInvalidSelfTestSelected,
            ErrorCode::SctUnsupportedBistModeInPatternRequest,
            ErrorCode::SctUnsupportedOpCodeForWdLogs,
            ErrorCode::HpaInvalidValueSpecified,
            ErrorCode::HpaLockInPlace,
            ErrorCode::HpaLockNotInPlace,
            ErrorCode::HpaFreezeLockInPlace,
            ErrorCode::HpaCommandSequence,
            ErrorCode::HpaSetMaxAddressExtInPlace,
            ErrorCode::HpaPasswordActive,
            ErrorCode::HpaPasswordNotActive,
            ErrorCode::HpaPasswordMiscompare,
            ErrorCode::HpaSecondNonVolatileCommand,
            ErrorCode::HpaReadMaxFirstCommand,
            ErrorCode::HpaCommandSequenceFault,
            ErrorCode::HostUnsupportedAtaOpcode,
            ErrorCode::HostReceivedLbaTooBig,
            ErrorCode::HostLbaOutOfRange,
            ErrorCode::HostDriveParametersSptNotSupported,
            ErrorCode::HostDriveParametersHeadsNotSupported,
            ErrorCode::HostUnsupportedFeatureValue,
            ErrorCode::HostUnsupportedMultiCount,
            ErrorCode::HostMultiNotSet,
            ErrorCode::HostDisabledIordyNotSupported,
            ErrorCode::HostUnsupportedCommandInPstMode,
            ErrorCode::HostInvalidSectorCount,
            ErrorCode::HostVscCommandExecutingInBackground,
            ErrorCode::HostCommandNotAllowedInGainCalibration,
            ErrorCode::HostQueueCommandIntermix,
            ErrorCode::HostUnsupportedSetFeatureSata,
            ErrorCode::HostQueueTag,
            ErrorCode::HostInvalidChsCylinderNumber,
            ErrorCode::HostInvalidChsHeadNumber,
            ErrorCode::HostInvalidChsSectorNumber,
            ErrorCode::HostUnsupportedAtaCommandInSioMode,
            ErrorCode::HostReservedStandbyTimerValue,
            ErrorCode::HostPuisSetFeatureDisabledInConfigSector,
            ErrorCode::HostPuisFlashSetToUseJumper,
            ErrorCode::HostPuisSetFeatureNotSupportedInXpm2,
            ErrorCode::HostPuisSpinUpCommandNotSupportedInXpm2,
            ErrorCode::HostPuisJumperEnabledInFlashNoJumper,
            ErrorCode::HostPuisDisabledInFlash,
            ErrorCode::HostInvalidAtaStreamId,
            ErrorCode::HostAtaStreamIdNotConfig,
            ErrorCode::HostLockedUnitAccessDenied,
            ErrorCode::HostNcqNoReadLog10,
            ErrorCode::HostUnsupportedChipRevision,
            ErrorCode::HostInterfaceCrc,
            ErrorCode::HostInterfaceCrcOverrunUnderrun,
            ErrorCode::HostInterfaceOverrunUnderrun,
            ErrorCode::HostIntrudingCommand,
            ErrorCode::HostSataCrc,
            ErrorCode::HostSataRxProtocol,
            ErrorCode::HostSataRxSyncTerminate,
            ErrorCode::HostSataRxLength,
            ErrorCode::HostSataTxRerr,
            ErrorCode::HostSataTxSyncTerminate,
            ErrorCode::HostSataRetransmit,
            ErrorCode::HostSataTmFifo,
            ErrorCode::HostSataDisparity,
            ErrorCode::HostSataCodeViolation,
            ErrorCode::HostSataLinkHung,
            ErrorCode::HostSataUnrecognizedFis,
            ErrorCode::HostSataUnknown,
            ErrorCode::HostSataRxDisparityInFis,
            ErrorCode::HostSataWriteTransferOverrun,
            ErrorCode::HostSataDataFisTooLong,
            ErrorCode::HostSataDataFisTooShort,
            ErrorCode::HostSataHbcrc,
            ErrorCode::HostSataHbcrcAndRerr,
            ErrorCode::HostSataHiddenHbcrc,
            ErrorCode::HostSataDataFisWrongSize,
            ErrorCode::HostSataTmFifoUnderrun,
            ErrorCode::HostSataTmFifoOverrun,
            ErrorCode::DcoInvalidFeatureSet,
            ErrorCode::DcoFreezeLockInPlace,
            ErrorCode::DcoChecksum,
            ErrorCode::DcoSignature,
            ErrorCode::DcoInvalidUdmaMode,
            ErrorCode::DcoCommandAborted,
            ErrorCode::DcoDcRestoreWhenDriveIsInFactoryState,
            ErrorCode::DcoDcSetWhenDriveIsInReducedState,
            ErrorCode::DcoDcRestoreWhenHpaIsPresent,
            ErrorCode::DcoDcSetWithInvalidConditions,
            ErrorCode::UnsupportedLogAddress,
            ErrorCode::ChangeDefinitionInvalidPassword,
            ErrorCode::ChangeDefinitionInvalidConfigNumber,
            ErrorCode::ChangeDefinitionUndefinedCapacity,
            ErrorCode::ChangeDefinitionIllegalCapacity,
            ErrorCode::ChangeDefinitionCounterMaximum,
            ErrorCode::ChangeDefinitionInvalidConfigSelectArray,
            ErrorCode::ChangeDefinitionNotEnabled,
            ErrorCode::ChangeDefinitionNativeMaxLbaInvalid,
            ErrorCode::ChangeDefinitionFieldList,
            ErrorCode::NotAnErrorSioInvalidNonAtaOpcode,
            ErrorCode::NotAnErrorVscCommandInBackground,
        ];

        if value == 0 {
            return None;
        }

        if let Some(x) = CONST_VARIANTS
            .iter()
            .find(|&&y| u32::from(y) == value)
            .copied()
        {
            return Some(x);
        }

        Some(Self::Unknown(value))
    }
}
