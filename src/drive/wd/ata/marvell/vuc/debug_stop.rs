//! Debug Stop codes.

/// Debug Stop code, when firmware halts on an internal assertion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DebugStopCode {
    /// Unknown.
    Unknown(u32),
    /// Operating system error.
    OsError = 0x10000,
    /// Operating system nested interrupt.
    OsNestedInterrupt = 0x10100,
    /// Operating system undefined instruction.
    OsUndefInstruction = 0x10101,
    /// Operating system prefetch abort.
    OsPrefetchAbort = 0x10102,
    /// Operating system data abort.
    OsDataAbort = 0x10103,
    /// Operating system stack overflow.
    OsStackOverflow = 0x10104,
    /// Operating system interrupt request stack overflow.
    OsIrqStackOverflow = 0x10105,
    /// Operating system flag set from fast interrupt request.
    OsFlagSetFromFiq = 0x10106,
    /// Operating system unmatched exit task unsafe.
    OsUnmatchedExitTaskUnsafe = 0x10107,
    /// Operating system null pointer assignment.
    OsNullPtrAssignment = 0x10108,
    /// Operating system task unsafe timeout.
    OsTaskUnsafeTimeout = 0x10109,
    /// Operating system pend invalid group.
    OsPendInvalidGroup = 0x1010A,
    /// Operating system post invalid group.
    OsPostInvalidGroup = 0x1010B,
    /// Operating system query invalid group.
    OsQueryInvalidGroup = 0x1010C,
    /// CRT exit.
    CrtExit = 0x11000,
    /// CRT raise.
    CrtRaise = 0x11001,
    /// Memcpy into code.
    MemcpyIntoCode = 0x11002,
    /// Memcpy unaligned source pointer.
    MemcpyUnalignedSrcPtr = 0x11003,
    /// Memcpy unaligned drive self-test pointer.
    MemcpyUnalignedDstPtr = 0x11004,
    /// Memset unaligned drive self-test pointer.
    MemsetUnalignedDstPtr = 0x11005,
    /// Invalid software interrupt.
    InvalidSwi = 0x11006,
    /// FIFO collision FW timeout.
    FifoCollisionFwTimeout = 0x11007,
    /// CPU HDC wait timeout.
    CpuHdcWaitTimeout = 0x11008,
    /// Jump to zero.
    JumpToZero = 0x11009,
    /// Memset into code.
    MemsetIntoCode = 0x1100A,
    /// Memclr into code.
    MemclrIntoCode = 0x1100B,
    /// Memmove into code.
    MemmoveIntoCode = 0x1100C,
    /// Drive error history log.
    DriveErrHistLog = 0x1100D,
    /// Get extend error.
    GetExtErr = 0x1100E,
    /// Clear period log.
    ClrPeriodLog = 0x1100F,
    /// Set self test timer.
    SetSelfTestTimer = 0x11010,
    /// Relocation code to Drive Reliability Monitor code.
    ReloCodeToDrmCode = 0x11011,
    /// Update drive life time error counters.
    UpdateDrvLifeTimeErrCounters = 0x11012,
    /// Self-test scan.
    SelftestScan = 0x11013,
    /// Self-test read write.
    SelftestRdWr = 0x11014,
    /// Servo core init wrong core.
    ServocoreInitWrongCore = 0x11015,
    /// Servo core init premature.
    ServocoreInitPremature = 0x11016,
    /// Overlay guard undefined function.
    OvlGuardUndefinedFn = 0x11100,
    /// PM invalid mode select.
    PmInvalidModeSel = 0x11200,
    /// PM invalid history index.
    PmInvalidHistoryIndex = 0x11201,
    /// PM invalid DCV state at disk seek.
    PmInvalidDcvStateAtDiskSeek = 0x11210,
    /// PM invalid DCV state at disk exit.
    PmInvalidDcvStateAtDiskExit = 0x11211,
    /// PM invalid DCV state entering sleep.
    PmInvalidDcvStateEnteringSleep = 0x11212,
    /// PM invalid DCV state exiting sleep.
    PmInvalidDcvStateExitingSleep = 0x11213,
    /// PM invalid voltage request.
    PmInvalidVoltageRequest = 0x11214,
    /// Unsupported SOC.
    UnsupportedSoc = 0x11300,
    /// Interrupt service routine HBI.
    IsrHbi = 0x12001,
    /// Interrupt service routine DF.
    IsrDf = 0x12002,
    /// Interrupt service routine BM.
    IsrBm = 0x12003,
    /// Interrupt service routine HBI tonescan.
    IsrHbiTonescan = 0x12004,
    /// Interrupt service routine HBI FIFO error.
    IsrHbiFifoErr = 0x12005,
    /// Interrupt service routine watchdog timeout.
    IsrWatchdogTimeout = 0x12006,
    /// Interrupt service routine external trigger.
    IsrExternalTrigger = 0x12007,
    /// Disk unhandled DF error.
    DiskUnhandledDfError = 0x13003,
    /// Disk invalid SD chain.
    DiskInvalidSdChain = 0x13004,
    /// Disk invalid insert.
    DiskInvalidInsert = 0x13005,
    /// Disk invalid call forward.
    DiskInvalidCallForward = 0x13006,
    /// Disk invalid opcode.
    DiskInvalidOpcode = 0x13007,
    /// Disk no CDD present.
    DiskNoCddPresent = 0x13008,
    /// Disk services invalid opcode.
    DiskServicesInvalidOpcode = 0x13009,
    /// Invalid TD parameter.
    InvalidTdParam = 0x1300A,
    /// Disk hardware check hang.
    DiskHardwareCheckHang = 0x1300B,
    /// Corrupt track status.
    CorruptTrackStatus = 0x1300C,
    /// Disk invalid buffer xfer pointer.
    DiskInvalidBfrXferPtr = 0x1300D,
    /// Disk SD not present.
    DiskSdNotPresent = 0x1300E,
    /// No more seek setup structs.
    NoMoreSeekSetupStructs = 0x1300F,
    /// Invalid servo event.
    InvalidServoEvent = 0x13010,
    /// Disk invalid spinup event.
    DiskInvalidSpinupEvent = 0x13011,
    /// Disk invalid head number.
    DiskInvalidHeadNumber = 0x13012,
    /// Disk debug stop unsupported request.
    DiskDebugstopUnsupportedRequest = 0x13013,
    /// Disk debug stop invalid extend SD.
    DiskDebugstopInvalidExtendSd = 0x13014,
    /// Disk debug stop invalid extend track info.
    DiskDebugstopInvalidExtendTrackInfo = 0x13015,
    /// Disk debug stop write extend not sequential.
    DiskDebugstopWriteExtendNotSequential = 0x13016,
    /// Disk debug stop invalid wedge data size.
    DiskDebugstopInvalidWedgeDataSize = 0x13017,
    /// Disk invalid extend request.
    DiskInvalidExtendRequest = 0x13018,
    /// Disk spinup timeout event.
    DiskSpinupTimeoutEvent = 0x13019,
    /// Disk invalid uninitialised CDD.
    DiskInvalidUninitCdd = 0x1301A,
    /// Disk tonescan defect buffer empty.
    DiskTonescanDefectBufferEmpty = 0x1301B,
    /// Disk tonescan bad defective wedge count.
    DiskTonescanBadDefectiveWedgeCount = 0x1301C,
    /// Disk debug stop max logical sector number mismatch.
    DiskDebugstopMaxLsnMismatch = 0x1301D,
    /// Disk invalid head count from ch.
    DiskInvalidHeadCountFromCh = 0x1301E,
    /// Disk invalid loopcount register value.
    DiskInvalidLoopcountRegValue = 0x1301F,
    /// Disk debug stop DF is active after DF stop.
    DiskDebugstopDfIsActiveAfterDfStop = 0x13020,
    /// Disk invalid completion count.
    DiskInvalidCompletionCount = 0x13030,
    /// Disk invalid wedge command.
    DiskInvalidWedgeCommand = 0x13031,
    /// Disk invalid address to rc update in spinup.
    DiskInvalidAddrToRcUpdateInSpinup = 0x13032,
    /// File Manager debug stop wrong mode.
    FmDebugstopWrongMode = 0x13300,
    /// File Manager debug stop invalid opcode.
    FmDebugstopInvalidOpcode = 0x13301,
    /// File Manager debug stop wrong calling task.
    FmDebugstopWrongCallingTask = 0x13302,
    /// File Manager copy temperature to permanent failed.
    FmCopyTempToPermFailed = 0x13303,
    /// File Manager disk task not called.
    FmDiskTaskNotCalled = 0x13304,
    /// File Manager too many flash files.
    FmTooManyFlashFiles = 0x13305,
    /// File Manager download microcode open failed.
    FmDlmcOpenFailed = 0x13306,
    /// File Manager debug stop not enough FMDS.
    FmDebugstopNotEnoughFmds = 0x13307,
    /// File Manager debug stop file IDS dont match.
    FmDebugstopFileIdsDontMatch = 0x13308,
    /// File Manager debug stop already freed FMD.
    FmDebugstopAlreadyFreedFmd = 0x13309,
    /// File Manager debug stop FMD index overwritten.
    FmDebugstopFmdIndexOverwritten = 0x1330A,
    /// File Manager debug stop invalid origin.
    FmDebugstopInvalidOrigin = 0x1330B,
    /// File Manager debug stop invalid offset.
    FmDebugstopInvalidOffset = 0x1330C,
    /// File Manager debug stop invalid LBA offset.
    FmDebugstopInvalidLbaOffset = 0x1330D,
    /// File Manager debug stop request count too big.
    FmDebugstopRequestCountTooBig = 0x1330E,
    /// File Manager debug stop partial on flash files.
    FmDebugstopPartialOnFlashFiles = 0x1330F,
    /// File Manager debug stop too many dirty files.
    FmDebugstopTooManyDirtyFiles = 0x13310,
    /// File Manager debug stop number copy more then max copy.
    FmDebugstopNumCopyMoreThenMaxCopy = 0x13311,
    /// File Manager open empty slot no can do.
    FmOpenEmptySlotNoCanDo = 0x13312,
    /// File Manager FMD index exceed allocated FMD space.
    FmFmdIndexExceedAllocatedFmdSpace = 0x13313,
    /// File Manager open zipcode dlg file fail.
    FmOpenZipcodeDlgFileFail = 0x13314,
    /// Disk debug stop unexpected end of SD group.
    DiskDebugstopUnexpectedEndOfSdGroup = 0x13400,
    /// Disk debug stop unexpected SD values.
    DiskDebugstopUnexpectedSdValues = 0x13401,
    /// Disk debug stop next list out of range.
    DiskDebugstopNextListOutOfRange = 0x13402,
    /// Disk debug stop unexpected remaining count values.
    DiskDebugstopUnexpectedRemCntValues = 0x13403,
    /// Disk debug stop invalid completed count.
    DiskDebugstopInvalidCompletedCnt = 0x13404,
    /// Disk debug stop invalid read operation.
    DiskDebugstopInvalidReadOperation = 0x13405,
    /// Disk debug stop invalid SSC adjustment.
    DiskDebugstopInvalidSscAdjustment = 0x13406,
    /// Disk debug stop mismatch for first SD loaded.
    DiskDebugstopMismatchForFirstSdLoaded = 0x13407,
    /// Disk debug stop invalid SD chain.
    DiskDebugstopInvalidSdChain = 0x13408,
    /// Disk debug stop mismatch total ARM count.
    DiskDebugstopMismatchTotalArmCnt = 0x13409,
    /// Disk debug stop invalid remaining ARM count.
    DiskDebugstopInvalidRemainingArmCnt = 0x1340A,
    /// Disk debug stop write backup size greater than disk segment size.
    DiskDebugstopWriteBackupSizeGreaterThanDiskSegmentSize = 0x1340B,
    /// Disk debug stop writable control store still running on check disk
    /// write.
    DiskDebugstopWcsStillRunningOnCheckDiskWrite = 0x1340C,
    /// Disk debug stop formatter timeout on read.
    DiskDebugstopFormatterTimeoutOnRead = 0x1340D,
    /// Disk debug stop formatter timeout on write.
    DiskDebugstopFormatterTimeoutOnWrite = 0x1340E,
    /// Disk debug stop no buffer present.
    DiskDebugstopNoBufferPresent = 0x1340F,
    /// Disk debug stop segment end address invalid.
    DiskDebugstopSegEndAddrInvalid = 0x13410,
    /// Disk debug stop SD count not zero for read.
    DiskDebugstopSdCountNotZeroForRead = 0x13411,
    /// Disk debug stop pseudo set SSC too large.
    DiskDebugstopPseudoSetSscTooLarge = 0x13412,
    /// Disk debug stop SSC not equal to remaining request count.
    DiskDebugstopSscNotEqualToRemReqCnt = 0x13413,
    /// Disk debug stop DAB remaining request count underflow.
    DiskDebugstopDabRemReqCntUnderflow = 0x13414,
    /// Disk debug stop unexpected error status.
    DiskDebugstopUnexpectedErrorStatus = 0x13415,
    /// Disk debug stop illegal cylinder seek.
    DiskDebugstopIllegalCylSeek = 0x13416,
    /// Disk debug stop illegal head seek.
    DiskDebugstopIllegalHeadSeek = 0x13417,
    /// Disk debug stop read extend needs connect to host.
    DiskDebugstopReadExtendNeedsConnectToHost = 0x13418,
    /// Disk debug stop no SD attached.
    DiskDebugstopNoSdAttached = 0x13419,
    /// Disk debug stop no CD attached.
    DiskDebugstopNoCdAttached = 0x1341A,
    /// Disk debug stop seek not complete.
    DiskDebugstopSeekNotComplete = 0x1341B,
    /// Disk debug stop read overflow size greater than disk segment size.
    DiskDebugstopReadOverflowSizeGreaterThanDiskSegmentSize = 0x1341C,
    /// Disk debug stop wrong seek issued.
    DiskDebugstopWrongSeekIssued = 0x1341D,
    /// Disk debug stop linked sequential thread.
    DiskDebugstopLinkedSequentialThread = 0x1341E,
    /// Disk debug stop hanging disk events.
    DiskDebugstopHangingDiskEvents = 0x1341F,
    /// Disk debug stop negative remaining request count.
    DiskDebugstopNegativeRemRequestCount = 0x13420,
    /// Disk debug stop invalid TREX pattern1.
    DiskDebugstopInvalidTrexPattern1 = 0x13421,
    /// Disk debug stop invalid TREX pattern2.
    DiskDebugstopInvalidTrexPattern2 = 0x13422,
    /// Disk debug stop invalid TREX pattern3.
    DiskDebugstopInvalidTrexPattern3 = 0x13423,
    /// Disk debug stop no error from gatherstatus.
    DiskDebugstopNoErrorFromGatherstatus = 0x13424,
    /// Disk debug stop servo event timeout1.
    DiskDebugstopServoEventTimeout1 = 0x13425,
    /// Disk debug stop servo event timeout2.
    DiskDebugstopServoEventTimeout2 = 0x13426,
    /// Disk debug stop cache memory test failed.
    DiskDebugstopCacheMemoryTestFailed = 0x13427,
    /// Disk debug stop mismatched remaining request count.
    DiskDebugstopMismatchedRemReqCnt = 0x13428,
    /// Disk debug stop physical sector number logical sector number mismatch.
    DiskDebugstopPsnLsnMismatch = 0x13429,
    /// Disk debug stop servo timeout.
    DiskDebugstopServoTimeout = 0x1342A,
    /// Disk debug stop servo wrong head selected.
    DiskDebugstopServoWrongHeadSelected = 0x1342B,
    /// Disk debug stop er bad translation.
    DiskDebugstopErBadTranslation = 0x1342C,
    /// Disk debug stop no DD attached.
    DiskDebugstopNoDdAttached = 0x1342D,
    /// Disk debug stop misaligned xfer pointer.
    DiskDebugstopMisalignedXferPtr = 0x1342E,
    /// Disk debug stop unexpected segment count.
    DiskDebugstopUnexpectedSegmentCnt = 0x1342F,
    /// Disk debug stop bad test track.
    DiskDebugstopBadTestTrack = 0x13430,
    /// Disk debug stop findmaxlba no current TD.
    DiskDebugstopFindmaxlbaNoCurTd = 0x13431,
    /// Disk debug stop findmaxlba invalid request.
    DiskDebugstopFindmaxlbaInvalidReq = 0x13432,
    /// Disk debug stop xfer wake at LBA write.
    DiskDebugstopXferWakeAtLbaWrite = 0x13433,
    /// Disk debug stop illegal command for head of queue.
    DiskDebugstopIllegalCmdForHeadOfQueue = 0x13434,
    /// Disk debug stop illegal queue depth.
    DiskDebugstopIllegalQueueDepth = 0x13435,
    /// Disk debug stop after cancel still very busy.
    DiskDebugstopAfterCancelStillVeryBusy = 0x13436,
    /// Disk debug stop queue greater than one.
    DiskDebugstopQueueGreaterThanOne = 0x13437,
    /// Disk debug stop sequential with linked SDS.
    DiskDebugstopSequentialWithLinkedSds = 0x13438,
    /// Disk debug stop invalid wedge down counter calculated.
    DiskDebugstopInvalidWedgeDownCounterCalculated = 0x13439,
    /// Disk debug stop invalid wedge down counter detected when decrement.
    DiskDebugstopInvalidWedgeDownCounterDetectedWhenDecrement = 0x1343A,
    /// Disk debug stop invalid wedge down counter detected when adjust.
    DiskDebugstopInvalidWedgeDownCounterDetectedWhenAdjust = 0x1343B,
    /// Disk debug stop seek latency table not supported.
    DiskDebugstopSeekLatencyTableNotSupported = 0x1343C,
    /// Disk debug stop read thread count mismatch.
    DiskDebugstopReadThreadCountMismatch = 0x1343D,
    /// Disk debug stop read er resume pointer is null.
    DiskDebugstopReadErResumePtrIsNull = 0x1343E,
    /// Disk debug stop no head switch.
    DiskDebugstopNoHeadSwitch = 0x1343F,
    /// Disk debug stop DSSS unaligned LBA.
    DiskDebugstopDsssUnalignedLba = 0x13440,
    /// Disk debug stop invalid timer ID.
    DiskDebugstopInvalidTimerId = 0x13441,
    /// Disk debug stop get temperature timeout exceeded.
    DiskDebugstopGetTempTimeoutExceeded = 0x13442,
    /// Disk debug stop invalid servo scan state.
    DiskDebugstopInvalidServoScanState = 0x13443,
    /// Disk debug stop invalid cluster index.
    DiskDebugstopInvalidClusterIndex = 0x13444,
    /// Disk debug stop arming mishap.
    DiskDebugstopArmingMishap = 0x13445,
    /// Disk debug stop stillbusy ARM.
    DiskDebugstopStillbusyArm = 0x13446,
    /// Disk debug stop first LBA not found.
    DiskDebugstopFirstLbaNotFound = 0x13447,
    /// Disk debug stop zero count decrement attempt.
    DiskDebugstopZeroCountDecrmentAttmp = 0x13448,
    /// Disk debug stop zero count write zip high.
    DiskDebugstopZeroCountWrtZipHigh = 0x13449,
    /// Disk debug stop zero count write zip low.
    DiskDebugstopZeroCountWrtZipLow = 0x1344A,
    /// Disk debug stop zip file write during flush.
    DiskDebugstopZipFileWrtDuringFlush = 0x1344B,
    /// Disk debug stop get temperature invalid task ID.
    DiskDebugstopGetTempInvalidTaskId = 0x1344C,
    /// Disk debug stop invalid channel scan state.
    DiskDebugstopInvalidChannelScanState = 0x13450,
    /// Disk debug stop read offtrack limits conflict.
    DiskDebugstopReadOfftrackLimitsConflict = 0x13451,
    /// Disk debug stop write offtrack limits conflict.
    DiskDebugstopWriteOfftrackLimitsConflict = 0x13452,
    /// Disk debug stop predict offtrack limits conflict.
    DiskDebugstopPredictOfftrackLimitsConflict = 0x13453,
    /// Disk debug stop fine trackoffsets conflict.
    DiskDebugstopFineTrackoffsetsConflict = 0x13454,
    /// Dynamic Fly Height debug preheat check register c zero.
    DfhDebugPreheatCheckRegCZero = 0x13470,
    /// Dynamic Fly Height debug DF stopped DFH register d changed.
    DfhDebugDfStoppedDfhRegDChanged = 0x13471,
    /// Dynamic Fly Height debug DF stopped DFH register c changed.
    DfhDebugDfStoppedDfhRegCChanged = 0x13472,
    /// Dynamic Fly Height debug state monitor bad preheat count.
    DfhDebugStateMonitorBadPreheatCount = 0x13473,
    /// Dynamic Fly Height debug state idle bad preheat count.
    DfhDebugStateIdleBadPreheatCount = 0x13474,
    /// Dynamic Fly Height debug condition met bad DFH state.
    DfhDebugOnditionMetBadDfhState = 0x13475,
    /// Relocation invalid error status.
    ReloInvalidErrorStatus = 0x13500,
    /// Relocation invalid TD opcode.
    ReloInvalidTdOpcode = 0x13501,
    /// Relocation no bad wedge info.
    ReloNoBadWedgeInfo = 0x13502,
    /// Relocation SST not possible on read.
    ReloSstNotPossibleOnRead = 0x13503,
    /// Relocation spare LBA with no user LBA.
    ReloSpareLbaWithNoUserLba = 0x13504,
    /// Relocation invalid to use er DABS.
    ReloInvalidToUseErDabs = 0x13505,
    /// Relocation handler lost in space.
    ReloHandlerLostInSpace = 0x13506,
    /// Relocation experiment no transparent auto relocations allowed.
    ReloExperimentNoTaresAllowed = 0x13507,
    /// Relocation experiment no relos allowed.
    ReloExperimentNoRelosAllowed = 0x13508,
    /// Relocation experiment no thermal asperity transparent auto relocations
    /// allowed.
    ReloExperimentNoTaTaresAllowed = 0x13509,
    /// Relocation experiment no reserved bit3 allowed.
    ReloExperimentNoRsvdBit3Allowed = 0x1350A,
    /// Relocation experiment no reserved bit4 allowed.
    ReloExperimentNoRsvdBit4Allowed = 0x1350B,
    /// Relocation experiment no reserved bit5 allowed.
    ReloExperimentNoRsvdBit5Allowed = 0x1350C,
    /// Relocation experiment no reserved bit6 allowed.
    ReloExperimentNoRsvdBit6Allowed = 0x1350D,
    /// Relocation experiment no reserved bit7 allowed.
    ReloExperimentNoRsvdBit7Allowed = 0x1350E,
    /// Relocation bad wedge config out of range.
    ReloBadWedgeCfgOutOfRange = 0x1350F,
    /// Advanced Peripheral Bus read error.
    ApbReadError = 0x13600,
    /// Invalid Advanced Peripheral Bus checksum.
    InvalidApbChecksum = 0x13601,
    /// Invalid format surface ID.
    InvalidFormatSurfaceId = 0x13602,
    /// DAB nested disconnect.
    DabNestedDisconnect = 0x13700,
    /// DAB connect without disconnect.
    DabConnectWithoutDisconnect = 0x13701,
    /// Disk disconnected on cache command.
    DiskDisconnectedOnCacheCmd = 0x13702,
    /// Disk internal error injection hardware abstraction layer invalid error
    /// type.
    DiskIeiHalInvalidErrorType = 0x13703,
    /// Disk internal error injection invalid counter.
    DiskIeiInvalidCounter = 0x13704,
    /// Resource debug stop no free CD.
    RscDebugstopNoFreeCd = 0x14000,
    /// Resource debug stop no free XD.
    RscDebugstopNoFreeXd = 0x14001,
    /// Resource debug stop put XD already free.
    RscDebugstopPutXdAlreadyFree = 0x14002,
    /// Resource debug stop no free DD.
    RscDebugstopNoFreeDd = 0x14003,
    /// Resource debug stop put DD already free.
    RscDebugstopPutDdAlreadyFree = 0x14004,
    /// Resource debug stop no free TD.
    RscDebugstopNoFreeTd = 0x14006,
    /// Resource debug stop TD disk queue full.
    RscDebugstopTdDiskQueueFull = 0x14008,
    /// Resource debug stop dequeue TD queue empty.
    RscDebugstopDequeueTdQueueEmpty = 0x14009,
    /// Resource debug stop enqueue TD index used.
    RscDebugstopEnqueueTdIndexUsed = 0x1400A,
    /// Resource debug stop remove TD queue empty.
    RscDebugstopRemoveTdQueueEmpty = 0x1400B,
    /// Resource debug stop remove TD bad index.
    RscDebugstopRemoveTdBadIndex = 0x1400C,
    /// Resource debug stop no SD to deallocate.
    RscDebugstopNoSdToDeallocate = 0x1400E,
    /// Resource debug stop no CD for verify buffer.
    RscDebugstopNoCdForVerifyBuffer = 0x1400F,
    /// Resource debug stop put TD already free.
    RscDebugstopPutTdAlreadyFree = 0x14010,
    /// Resource debug stop put CD already free.
    RscDebugstopPutCdAlreadyFree = 0x14011,
    /// Resource debug stop invalid SD state.
    RscDebugstopInvalidSdState = 0x14012,
    /// Resource debug stop get null buffer address SD.
    RscDebugstopGetNullBufferAddressSd = 0x14013,
    /// Resource debug stop get null buffer address CD1.
    RscDebugstopGetNullBufferAddressCd1 = 0x14014,
    /// Resource debug stop get null buffer address CD2.
    RscDebugstopGetNullBufferAddressCd2 = 0x14015,
    /// Resource debug stop cluster chain too long.
    RscDebugstopClusterChainTooLong = 0x14016,
    /// Resource debug stop allocate buffers not by exec task.
    RscDebugstopAllocBuffersNotByExecTask = 0x14017,
    /// Resource debug stop allocate SD not by exec task.
    RscDebugstopAllocSdNotByExecTask = 0x14018,
    /// Resource debug stop get null buffer address DD1.
    RscDebugstopGetNullBufferAddressDd1 = 0x14019,
    /// Resource debug stop get null buffer address DD2.
    RscDebugstopGetNullBufferAddressDd2 = 0x1401A,
    /// Resource debug stop invalid lock count state.
    RscDebugstopInvalidLockCntState = 0x1401B,
    /// Resource debug stop invalid total cluster count.
    RscDebugstopInvalidTotalClusterCount = 0x14100,
    /// Resource debug stop invalid cluster index.
    RscDebugstopInvalidClusterIndex = 0x14102,
    /// Resource debug stop invalid free cluster count.
    RscDebugstopInvalidFreeClusterCount = 0x14103,
    /// Resource debug stop deallocate free cluster.
    RscDebugstopDeallocateFreeCluster = 0x14104,
    /// Resource debug stop allocate empty free cluster list.
    RscDebugstopAlloEmptyFreeClusterList = 0x14105,
    /// Resource debug stop allocate cluster not in free list.
    RscDebugstopAlloClusterNotInFreeList = 0x14106,
    /// Resource debug stop init clusters not all free.
    RscDebugstopInitClustersNotAllFree = 0x14107,
    /// Resource debug stop permanent allocate too large.
    RscDebugstopPermAllocTooLarge = 0x14108,
    /// Resource debug stop cluster count beyond range.
    RscDebugstopClusterCountBeyondRange = 0x14109,
    /// Resource debug stop cluster count not enough.
    RscDebugstopClusterCountNotEnough = 0x1410A,
    /// Resource debug stop download microcode buffer too small.
    RscDebugstopDlmcBufferTooSmall = 0x1410B,
    /// Resource debug stop request clusters beyond capacity.
    RscDebugstopReqClustersBeyondCapacity = 0x1410C,
    /// Resource debug stop unable flush clusters.
    RscDebugstopUnableFlushClusters = 0x1410D,
    /// Resource debug stop flush clusters timeout.
    RscDebugstopFlushClustersTimeout = 0x1410E,
    /// Resource debug stop invalid CD index.
    RscDebugstopInvalidCdIndex = 0x14200,
    /// Resource debug stop sequential SD in wrong state.
    RscDebugstopSequentialSdInWrongState = 0x14201,
    /// Resource debug stop no buffer requested flag not set.
    RscDebugstopNoBufferRequestedFlagNotSet = 0x14202,
    /// Resource debug stop invalid SD index.
    RscDebugstopInvalidSdIndex = 0x14203,
    /// Resource debug stop invalid TD index.
    RscDebugstopInvalidTdIndex = 0x14204,
    /// Resource debug stop invalid TD task ID.
    RscDebugstopInvalidTdTaskId = 0x14205,
    /// Resource debug stop free SD in sequence stream.
    RscDebugstopFreeSdInSeqStream = 0x14206,
    /// Resource debug stop send to disk with sequence stream.
    RscDebugstopSendToDiskWithSeqStream = 0x14207,
    /// Resource debug stop flush with sequence stream.
    RscDebugstopFlushWithSeqStream = 0x14208,
    /// Resource debug stop invalid DD index.
    RscDebugstopInvalidDdIndex = 0x14209,
    /// Resource debug stop invalid XD index.
    RscDebugstopInvalidXdIndex = 0x1420A,
    /// Resource debug stop invalid state QCMD bit and CD index.
    RscDebugstopInvalidStateQcmdBitAndCdIndex = 0x1420B,
    /// Resource debug stop invalid trim parameters.
    RscDebugstopInvalidTrimParameters = 0x14300,
    /// Resource debug stop trim with zero buffer.
    RscDebugstopTrimWithZeroBuffer = 0x14301,
    /// Resource debug stop invalid insert index.
    RscDebugstopInvalidInsertIndex = 0x14302,
    /// Resource debug stop allocate CD when CD already allocated.
    RscDebugstopAllocateCdWhenCdAlreadyAllocated = 0x14400,
    /// Resource debug stop allocate DD when DD already allocated.
    RscDebugstopAllocateDdWhenDdAlreadyAllocated = 0x14401,
    /// Resource debug stop allocate XD when XD already allocated.
    RscDebugstopAllocateXdWhenXdAlreadyAllocated = 0x14402,
    /// Resource debug stop cyclic SD state queue check fail.
    RscDebugstopCyclicSdStateqCheckFail = 0x14501,
    /// Resource debug stop cyclic SD cluster check fail.
    RscDebugstopCyclicSdClusterCheckFail = 0x14502,
    /// Resource debug stop cyclic cluster chain check fail.
    RscDebugstopCyclicClusterChainCheckFail = 0x14503,
    /// Resource debug stop cyclic fr cluster check fail.
    RscDebugstopCyclicFrClusterCheckFail = 0x14504,
    /// Resource debug stop total cluster count check fail.
    RscDebugstopTotalClCountCheckFail = 0x14505,
    /// Resource debug stop total SD count check fail.
    RscDebugstopTotalSdCountCheckFail = 0x14506,
    /// Resource debug stop fell off end of SD chain.
    RscDebugstopFellOffEndOfSdChain = 0x14507,
    /// Resource dynamic available count underflow.
    RscDynamicAvailCountUnderflow = 0x14601,
    /// Resource dynamic valid count underflow.
    RscDynamicValidCountUnderflow = 0x14602,
    /// Resource SD unlock callback error1.
    RscSdUnlockCallbackError1 = 0x14603,
    /// Resource SD unlock callback error2.
    RscSdUnlockCallbackError2 = 0x14604,
    /// Resource debug stop bad xfer reserve count.
    RscDebugstopBadXferReserveCnt = 0x14605,
    /// Resource invalid SD unlock.
    RscInvalidSdUnlock = 0x14606,
    /// Head disk assembly realnumheads invalid.
    HdaRealnumheadsInvalid = 0x15001,
    /// Head disk assembly debug stop no more seek setup structs.
    HdaDebugstopNoMoreSeekSetupStructs = 0x15002,
    /// Head disk assembly debug stop deallocate null seek setup pointer.
    HdaDebugstopDeallocateNullSeekSetupPtr = 0x15003,
    /// Head disk assembly debug stop deallocate free seek setup structure.
    HdaDebugstopDeallocateFreeSeekSetupStructure = 0x15004,
    /// Servo API debug stop invalid state transition.
    ServoApiDebugstopInvalidStateTransition = 0x15100,
    /// MRM debug stop submit invalid state queue.
    MrmDebugstopSubmitInvalidStateQueue = 0x16000,
    /// MRM debug stop submit no buffer.
    MrmDebugstopSubmitNoBuffer = 0x16001,
    /// MRM debug stop remove invalid state queue.
    MrmDebugstopRemoveInvalidStateQueue = 0x16002,
    /// MRM debug stop sendtodisk invalid state queue.
    MrmDebugstopSendtodiskInvalidStateQueue = 0x16004,
    /// MRM debug stop sendtodisk no buffer.
    MrmDebugstopSendtodiskNoBuffer = 0x16005,
    /// MRM debug stop sendtodisk SD in group.
    MrmDebugstopSendtodiskSdInGroup = 0x16006,
    /// MRM debug stop flushspecific no buffer.
    MrmDebugstopFlushspecificNoBuffer = 0x16007,
    /// MRM debug stop flushspecific invalid state queue.
    MrmDebugstopFlushspecificInvalidStateQueue = 0x16008,
    /// MRM debug stop flushspecific invalid group.
    MrmDebugstopFlushspecificInvalidGroup = 0x16009,
    /// MRM debug stop filemgr invalid state queue.
    MrmDebugstopFilemgrInvalidStateQueue = 0x1600A,
    /// MRM debug stop addtogroup invalid group.
    MrmDebugstopAddtogroupInvalidGroup = 0x1600B,
    /// MRM debug stop TD without CD.
    MrmDebugstopTdWithoutCd = 0x1600C,
    /// MRM debug stop releasecallback wrong state.
    MrmDebugstopReleasecallbackWrongState = 0x1600D,
    /// MRM debug stop processcommand wrong state.
    MrmDebugstopProcesscommandWrongState = 0x1600E,
    /// MRM debug stop defaultcallback wrong state.
    MrmDebugstopDefaultcallbackWrongState = 0x1600F,
    /// MRM debug stop wedge count valid timeout.
    MrmDebugstopWedgeCountValidTimeout = 0x16010,
    /// MRM debug stop flush rotational position optimisation cylinder head
    /// wedge not valid.
    MrmDebugstopFlushRpoChwNotValid = 0x16011,
    /// MRM start timed out.
    MrmStartTimedOut = 0x16012,
    /// MRM debug stop invalid op MRM service request.
    MrmDebugstopInvalidOpMrmServiceRequest = 0x16013,
    /// MRM debug stop invalid DD callback state.
    MrmDebugstopInvalidDdCallbackState = 0x16014,
    /// MRM debug stop TD without SD.
    MrmDebugstopTdWithoutSd = 0x16015,
    /// MRM unexpected SD in independent state.
    MrmUnexpectedSdInIndependentState = 0x16016,
    /// MRM unexpected SD wait state.
    MrmUnexpectedSdWaitState = 0x16017,
    /// MRM disk cancel queue failure.
    MrmDiskCancelQueueFailure = 0x16018,
    /// MRM disk invalid SD state.
    MrmDiskInvalidSdState = 0x16019,
    /// MRM disk invalid DD count.
    MrmDiskInvalidDdCount = 0x1601A,
    /// MRM disk odd LBA count request.
    MrmDiskOddLbaCountRequest = 0x1601B,
    /// MRM invalid DSSS merge dirty operation.
    MrmInvalidDsssMergeDirtyOperation = 0x1601C,
    /// MRM invalid task ID.
    MrmInvalidTaskId = 0x16080,
    /// Rotational position optimisation debug stop preread translate error.
    RpoDebugstopPrereadXlatError = 0x16100,
    /// Rotational position optimisation debug stop seek profile table error.
    RpoDebugstopSeekProfileTableError = 0x16101,
    /// Execution debug stop section start.
    ExecDebugstopSectionStart = 0x17000,
    /// Execution debug stop invalid TD opcode.
    ExecDebugstopInvalidTdOpcode = 0x17001,
    /// Execution debug stop invalid fm request.
    ExecDebugstopInvalidFmRequest = 0x17002,
    /// Execution debug stop CD required.
    ExecDebugstopCdRequired = 0x17003,
    /// Execution DLG2 invalid area.
    ExecDlg2InvalidArea = 0x17004,
    /// Execution DLG2 RSC failure.
    ExecDlg2RscFailure = 0x17005,
    /// Execution DLG2 invalid request count.
    ExecDlg2InvalidRequestCount = 0x17006,
    /// Execution DLG2 invalid LBA.
    ExecDlg2InvalidLba = 0x17007,
    /// Execution DLG2 read SPBA error.
    ExecDlg2ReadSpbaError = 0x17008,
    /// Execution DLG2 verify SPBA error.
    ExecDlg2VerifySpbaError = 0x17009,
    /// Execution DLG2 sector count bigger than config default count.
    ExecDlg2SectorCntBiggerThanConfigDefaultCnt = 0x1700A,
    /// Execution invalid factory selftest customer ID.
    ExecInvalidFactorySelftestCustomerId = 0x1700B,
    /// Background debug stop drive self-test recordresults invalid sub-action.
    BgDebugstopDstRecordresultsInvalidSubact = 0x18000,
    /// Background debug stop OL recordresults invalid sub-action.
    BgDebugstopOlRecordresultsInvalidSubact = 0x18001,
    /// Background debug stop invalid current activity.
    BgDebugstopInvalidCurrActivity = 0x18002,
    /// Background debug stop invalid TD opcode.
    BgDebugstopInvalidTdOpcode = 0x18003,
    /// Background debug stop invalid timer ID.
    BgDebugstopInvalidTimerId = 0x18004,
    /// Background debug stop invalid bg state.
    BgDebugstopInvalidBgState = 0x18005,
    /// Background debug stop invalid testinfo status.
    BgDebugstopInvalidTestinfoStatus = 0x18006,
    /// Background debug stop Process Self-Test invalid trb index.
    BgDebugstopPstInvalidTrbIndex = 0x18007,
    /// Background debug stop Process Self-Test unable to load sequence table.
    BgDebugstopPstUnableToLoadSeqTable = 0x18008,
    /// Background debug stop Process Self-Test unable to save sequence table.
    BgDebugstopPstUnableToSaveSeqTable = 0x18009,
    /// Background debug stop Process Self-Test invalid test ID.
    BgDebugstopPstInvalidTestId = 0x1800A,
    /// Background debug stop Process Self-Test invalid status.
    BgDebugstopPstInvalidStatus = 0x1800B,
    /// Background debug stop Process Self-Test interface version mismatch.
    BgDebugstopPstInterfaceVersionMismatch = 0x1800C,
    /// Background debug stop Process Self-Test invalid trb size.
    BgDebugstopPstInvalidTrbSize = 0x1800D,
    /// Background debug stop TD already allocated.
    BgDebugstopTdAlreadyAllocated = 0x1800E,
    /// Background debug stop instruction SRAM checksum failed.
    BgDebugstopIsramChksumFailed = 0x1800F,
    /// Background debug stop RVA MRM start timed out.
    BgDebugstopRvaMrmStartTimedOut = 0x18010,
    /// Background debug stop RVA requires CD.
    BgDebugstopRvaRequiresCd = 0x18011,
    /// Background debug stop RVA requires CDD.
    BgDebugstopRvaRequiresCdd = 0x18012,
    /// Background debug stop RVA invalid SD.
    BgDebugstopRvaInvalidSd = 0x18013,
    /// Background debug stop RVA illegal request size.
    BgDebugstopRvaIllegalRequestSize = 0x18014,
    /// Background debug stop WA MRM start timed out.
    BgDebugstopWaMrmStartTimedOut = 0x18015,
    /// Background debug stop WA requires CD.
    BgDebugstopWaRequiresCd = 0x18016,
    /// Background debug stop WA requires CDD.
    BgDebugstopWaRequiresCdd = 0x18017,
    /// Background debug stop WA request size too large.
    BgDebugstopWaRequestSizeTooLarge = 0x18018,
    /// Background debug stop preempted under busy protect.
    BgDebugstopPreemptedUnderBsyProtect = 0x18019,
    /// Background debug stop dramtest null testinfo pointer.
    BgDebugstopDramtestNullTestinfoPointer = 0x1801A,
    /// Background debug stop errorcode null testinfo pointer.
    BgDebugstopErrorcodeNullTestinfoPointer = 0x1801B,
    /// Background debug stop resfilecheck null testinfo pointer.
    BgDebugstopResfilecheckNullTestinfoPointer = 0x1801C,
    /// Background debug stop memcheck null testinfo pointer.
    BgDebugstopMemcheckNullTestinfoPointer = 0x1801D,
    /// Background debug stop thermal asperity null testinfo pointer.
    BgDebugstopTaNullTestinfoPointer = 0x1801E,
    /// Background debug stop scan null testinfo pointer.
    BgDebugstopScanNullTestinfoPointer = 0x1801F,
    /// Background debug stop Process Self-Test stack check failed.
    BgDebugstopPstStackCheckFailed = 0x18020,
    /// Background debug stop WA illegal zero request size.
    BgDebugstopWaIllegalZeroRequestSize = 0x18021,
    /// Background debug stop WA all LBA request size too large.
    BgDebugstopWaAllLbaRequestSizeTooLarge = 0x18022,
    /// Background debug stop release resource callback timeout.
    BgDebugstopReleaseResourceCallbackTimeout = 0x18023,
    /// Background debug stop Process Self-Test unable to flash Process Test
    /// Module.
    BgDebugstopPstUnableToFlashPtm = 0x18024,
    /// Background debug stop WA missing release resource info.
    BgDebugstopWaMissingReleaseResourceInfo = 0x18025,
    /// Background debug stop trace hang.
    BgDebugstopTraceHang = 0x18026,
    /// Background debug stop Process Self-Test postup failure.
    BgDebugstopPstPostupFailure = 0x18027,
    /// Background debug stop Process Self-Test permanent allocation failed.
    BgDebugstopPstPermAllocationFailed = 0x18028,
    /// Background debug stop invalid idle type.
    BgDebugstopInvalidIdleType = 0x18029,
    /// Background debug stop component ID requires CD.
    BgDebugstopCompidRequiresCd = 0x1802A,
    /// Background debug stop component ID requires CDD.
    BgDebugstopCompidRequiresCdd = 0x1802B,
    /// Background debug stop component ID request size too large.
    BgDebugstopCompidRequestSizeTooLarge = 0x1802C,
    /// Background debug stop component ID MRM start timed out.
    BgDebugstopCompidMrmStartTimedOut = 0x1802D,
    /// Background debug stop component ID MRM request error.
    BgDebugstopCompidMrmRequestError = 0x1802E,
    /// Background debug stop component ID illegal zero request size.
    BgDebugstopCompidIllegalZeroRequestSize = 0x1802F,
    /// Background debug stop component ID LBA request size too large.
    BgDebugstopCompidLbaRequestSizeTooLarge = 0x18030,
    /// Background debug stop drive self-test recordresults invalid testlevel.
    BgDebugstopDstRecordresultsInvalidTestlevel = 0x18031,
    /// Background debug stop scan invalid sub active drive self-test quick.
    BgDebugstopScanInvalidSubActiDstQuick = 0x18032,
    /// Background debug stop scan invalid sub active drive self-test extended.
    BgDebugstopScanInvalidSubActiDstExtended = 0x18033,
    /// Background debug stop scan invalid sub active drive self-test
    /// conveyance.
    BgDebugstopScanInvalidSubActiDstConveyance = 0x18034,
    /// Background debug stop scan invalid sub active drive self-test selective.
    BgDebugstopScanInvalidSubActiDstSelective = 0x18035,
    /// Background debug stop dlg clrzip failed.
    BgDebugstopDlgClrzipFailed = 0x18036,
    /// Background debug stop refresh LBA return count invalid.
    BgDebugstopRefreshLbaReturnCountInvalid = 0x18037,
    /// Background debug stop dlg low threshold greater than high threshold.
    BgDebugstopDlgLowThresholdGreaterThanHighThreshold = 0x18038,
    /// Background debug stop dlg illegal scheduler state.
    BgDebugstopDlgIllegalSchedulerState = 0x18039,
    /// Background debug stop dlg Process Self-Test mode controller failed.
    BgDebugstopDlgPstModeCtlrFailed = 0x1803A,
    /// Background debug stop remaining LBA count less than refresher count.
    BgDebugstopRemainingLbaCountLessThanRefresherCount = 0x1803B,
    /// Background debug stop refresh start LBA greater than max zip LBA.
    BgDebugstopRefreshStartLbaGreaterThanMaxZipLba = 0x1803C,
    /// Background debug stop Process Self-Test index overrun static.
    BgDebugstopPstIndexOverrunStatic = 0x1803D,
    /// Background debug stop ATA index overrun static.
    BgDebugstopAtaIndexOverrunStatic = 0x1803E,
    /// Background debug stop WA illegal request size.
    BgDebugstopWaIllegalRequestSize = 0x18F00,
    /// AR debug stop Hard Sector Descriptor Table invalid mapping result.
    ArDebugstopHsdtInvalidMappingResult = 0x19000,
    /// Cached relocation debug stop read write in progress.
    CrDebugstopReadWriteInProgress = 0x19800,
    /// Cached relocation debug stop read relo does not exist.
    CrDebugstopReadReloDoesNotExist = 0x19801,
    /// Cached relocation debug stop write relo does not exist.
    CrDebugstopWriteReloDoesNotExist = 0x19802,
    /// Cached relocation debug stop spare RW invalid index.
    CrDebugstopSpareRwInvalidIndex = 0x19803,
    /// Cached relocation debug stop reading relo never written.
    CrDebugstopReadingReloNeverWritten = 0x19804,
    /// Cached relocation debug stop buffer allocation failed.
    CrDebugstopBufferAllocationFailed = 0x19805,
    /// Cached relocation debug stop buffer not allocated.
    CrDebugstopBufferNotAllocated = 0x19806,
    /// Cached relocation debug stop unable to allocate permanent SD.
    CrDebugstopUnableToAllocatePermanentSd = 0x19807,
    /// Host debug stop xfer invalid count left.
    HostDebugstopXferInvalidCountLeft = 0x1A000,
    /// Host debug stop copy invalid count left.
    HostDebugstopCopyInvalidCountLeft = 0x1A001,
    /// Host debug stop xfer invalid first cluster ID.
    HostDebugstopXferInvalidFirstClusterId = 0x1A002,
    /// Host debug stop xfer invalid remaining count.
    HostDebugstopXferInvalidRemainingCnt = 0x1A003,
    /// Host debug stop xfer unexpected AW count.
    HostDebugstopXferUnexpectedAwCount = 0x1A004,
    /// Host debug stop invalid xfer for wait.
    HostDebugstopInvalidXferForWait = 0x1A005,
    /// Host debug stop unexpected xfer operation.
    HostDebugstopUnexpectedXferOperation = 0x1A006,
    /// Host debug stop invalid SD count update request.
    HostDebugstopInvalidSdCountUpdateReq = 0x1A007,
    /// Host debug stop end protocol unmasked unexpectedly.
    HostDebugstopEndProtocolUnmaskedUnexpectedly = 0x1A008,
    /// Host debug stop UDMA mode out of range.
    HostDebugstopUdmaModeOutOfRange = 0x1A009,
    /// Host debug stop unexpected HBI interrupts unmasked.
    HostDebugstopUnexpectedHbiIntsUnmasked = 0x1A00A,
    /// Host debug stop multiple SD multi mode xfer.
    HostDebugstopMultipleSdMultiModeXfer = 0x1A00B,
    /// Host debug stop AW xfer pointer misaligned.
    HostDebugstopAwXferPtrMisaligned = 0x1A00C,
    /// Host debug stop xfer not supported without permovl.
    HostDebugstopXferNotSupportedWithoutPermovl = 0x1A00D,
    /// Host debug stop xfer buffer not 32 bit aligned.
    HostDebugstopXferBufferNot32BitAligned = 0x1A00E,
    /// Host debug stop xfer buffer not sector aligned.
    HostDebugstopXferBufferNotSectorAligned = 0x1A00F,
    /// SOC debug stop subsegment out of range.
    SocDebugstopSubsegmentOutOfRange = 0x1A010,
    /// SOC debug stop invalid xfer offset.
    SocDebugstopInvalidXferOffset = 0x1A011,
    /// Host debug stop CD required.
    HostDebugstopCdRequired = 0x1A020,
    /// Host debug stop invalid number bytes for DMA copy.
    HostDebugstopInvalidNumBytesForDmaCopy = 0x1A021,
    /// Host debug stop invalid ATA status.
    HostDebugstopInvalidAtaStatus = 0x1A030,
    /// Host debug stop invalid xfer wait callback.
    HostDebugstopInvalidXferWaitCallback = 0x1A031,
    /// SATA bridge SOC UART write error.
    SatabridgeSocUartWriteError = 0x1A040,
    /// SATA bridge SOC UART read error.
    SatabridgeSocUartReadError = 0x1A041,
    /// SATA interface stuck in partial.
    SataInterfaceStuckInPartial = 0x1A050,
    /// SATA interface stuck in slumber.
    SataInterfaceStuckInSlumber = 0x1A051,
    /// SATA unhandled error interrupt.
    SataUnhandledErrorInterrupt = 0x1A060,
    /// SATA temperature interface CRC debug stop.
    SataTempIcrcDebugstop = 0x1A070,
    /// Host debug stop command aborted by reset.
    HostDebugstopCmdAbortedByReset = 0x1A100,
    /// Host debug stop command reparse error.
    HostDebugstopCmdReparseError = 0x1A101,
    /// Host debug stop reset timeout.
    HostDebugstopResetTimeout = 0x1A102,
    /// Host debug stop invalid TREX pattern1.
    HostDebugstopInvalidTrexPattern1 = 0x1A110,
    /// Host debug stop invalid TREX pattern2.
    HostDebugstopInvalidTrexPattern2 = 0x1A111,
    /// Host debug stop invalid TREX pattern3.
    HostDebugstopInvalidTrexPattern3 = 0x1A112,
    /// Host debug stop unexpected command timeout.
    HostDebugstopUnexpectedCommandTimeout = 0x1A200,
    /// Host debug stop command timeout not enabled.
    HostDebugstopCommandTimeoutNotEnabled = 0x1A201,
    /// Host debug stop command timer already enabled.
    HostDebugstopCommandTimerAlreadyEnabled = 0x1A202,
    /// Host debug stop command timeout error.
    HostDebugstopCmdTimeoutError = 0x1A203,
    /// Host debug stop invalid queued interrupt.
    HostDebugstopInvalidQueuedInterrupt = 0x1A300,
    /// Host debug stop next queue pointer invalid.
    HostDebugstopNextqPtrInvalid = 0x1A301,
    /// Host debug stop cpulock unexpected.
    HostDebugstopCpulockUnexpected = 0x1A302,
    /// Host debug stop QCD not present.
    HostDebugstopQcdNotPresent = 0x1A303,
    /// Host debug stop QRAM empty.
    HostDebugstopQramEmpty = 0x1A304,
    /// Host debug stop FUA queued write unsupported.
    HostDebugstopFuaQueuedWriteUnsupported = 0x1A305,
    /// Interrupt service routine queue HBI.
    IsrQueueHbi = 0x1A306,
    /// Hardware abstraction layer native command queuing debug stop sactive
    /// frame information structure timeout.
    HalNcqDebugstopSactiveFisTimeout = 0x1A307,
    /// Interrupt workaround fail.
    IntrWorkaroundFail = 0x1A308,
    /// Native command queuing next queue non zero when queue full.
    NcqNextqNonZeroWhenQueueFull = 0x1A309,
    /// La jolla 3 0 broken.
    LaJolla30Broken = 0x1A30A,
    /// Hardware abstraction layer native command queuing send Set Device Bits
    /// FIS while queue state is not active.
    HalNcqSendSdbfisWhileQstateIsNotActive = 0x1A30B,
    /// Hardware abstraction layer native command queuing debug stop QRAM active
    /// bit not set.
    HalNcqDebugstopQramActiveBitNotSet = 0x1A30C,
    /// Hardware abstraction layer native command queuing Set Device Bits frame
    /// information structure timeout.
    HalNcqSdbFisTimeout = 0x1A30D,
    /// Host debug stop QRAM execution enable.
    HostDebugstopQramExecutionEnable = 0x1A313,
    /// Stream log error SD not present.
    StreamLogErrorSdNotPresent = 0x1A400,
    /// Stream log error CD not present.
    StreamLogErrorCdNotPresent = 0x1A401,
    /// Depop invalid file ID.
    DepopInvalidFileId = 0x1B901,
    /// Depop unexpected file ID.
    DepopUnexpectedFileId = 0x1B902,
    /// TM debug stop slipped absolute block address overflow.
    TmDebugstopSlippedAbaOvfl = 0x1BC00,
    /// Format debug stop new bad track write drive quick.
    FmtDebugstopNewBadTrkWriteDriveQuick = 0x1BC01,
    /// Format debug stop new bad track write drive.
    FmtDebugstopNewBadTrkWriteDrive = 0x1BC02,
    /// Format debug stop illegal request size.
    FmtDebugstopIllegalRequestSize = 0x1BC03,
    /// Format debug stop format unit fatal error.
    FmtDebugstopFormatUnitFatalError = 0x1BC04,
    /// Defect Manager virtual head out of range.
    DmVirHeadOutOfRange = 0x1BC05,
    /// Format debug stop illegal WD WA request.
    FmtDebugstopIllegalWdWaRequest = 0x1BC06,
    /// Format debug stop illegal WDQ WA request.
    FmtDebugstopIllegalWdqWaRequest = 0x1BC07,
    /// Defect Manager not enough memory allocate.
    DmNotEnoughMemoryAlloc = 0x1BC08,
    /// Format reserved cylinder in G-list.
    FmtRsvdCylinderInGlist = 0x1BC09,
    /// Format debug stop illegal WDQ WA request 1.
    FmtDebugstopIllegalWdqWaRequest1 = 0x1BC0A,
    /// Format debug stop illegal WDQ WA request 2.
    FmtDebugstopIllegalWdqWaRequest2 = 0x1BC0B,
    /// Format debug stop illegal WDQ WA request 3.
    FmtDebugstopIllegalWdqWaRequest3 = 0x1BC0C,
    /// Defect Manager absolute block address out of range.
    DmAbaOutOfRange = 0x1BC0D,
    /// Defect Manager physical sector number out of range.
    DmPsnOutOfRange = 0x1BC0E,
    /// TM debug stop pushdown wedge overflow.
    TmDebugstopPushdownWedgeOvfl = 0x1BC0F,
    /// Defect Manager debug stop cyloffset out of range.
    DmDebugstopCyloffsetOutOfRange = 0x1BC10,
    /// Format P-list full.
    FmtPlistFull = 0x1BC11,
    /// TM debug stop too many pushdown groups.
    TmDebugstopTooManyPushdownGroups = 0x1BC12,
    /// Translation block info bad head.
    TranslationBlockInfoBadHead = 0x1BC13,
    /// Format start LBA out of sequence.
    FmtStartLbaOutOfSequence = 0x1BC14,
    /// Defect Manager invalid head skew calculation.
    DmInvalidHeadSkewCalculation = 0x1BC15,
    /// Defect Manager clist full.
    DmClistFull = 0x1BC16,
    /// Format reserved rplist physical sector number out of range.
    FmtRsvdRplistPsnOutOfRange = 0x1BC17,
    /// Defect Manager invalid reverse search.
    DmInvalidReverseSearch = 0x1BC18,
    /// Translation bad data track.
    TranslationBadDataTrack = 0x1BC19,
    /// Defect Manager translation test fail.
    DmTranslationTestFail = 0x1BC1A,
    /// Cache debug stop delete bad SD index.
    CaDebugstopDeleteBadSdIndex = 0x1C000,
    /// Cache debug stop insert bad SD index.
    CaDebugstopInsertBadSdIndex = 0x1C001,
    /// Cache debug stop insert table full.
    CaDebugstopInsertTableFull = 0x1C002,
    /// Cache debug stop sector count beyond range.
    CaDebugstopSectorCountBeyongRange = 0x1C003,
    /// Cache debug stop sequential stream bad parameter.
    CaDebugstopSequentialStreamBadParameter = 0x1C004,
    /// Cache debug stop command opcode for xfermode.
    CaDebugstopCmdOpcodeForXfermode = 0x1C005,
    /// Cache debug stop invalid sequential state.
    CaDebugstopInvalidSequentialState = 0x1C006,
    /// Cache debug stop sequential write no matching SD.
    CaDebugstopSequentialWriteNoMatchingSd = 0x1C007,
    /// Cache debug stop unexpected state write full hit.
    CaDebugstopUnexpectedStateWriteFullHit = 0x1C008,
    /// Cache debug stop unexpected opcode.
    CaDebugstopUnexpectedOpcode = 0x1C009,
    /// Cache debug stop clear host data out of range.
    CaDebugstopClearHostDataOutOfRange = 0x1C00A,
    /// Cache debug stop mismatched sequential stream.
    CaDebugstopMismatchedSequentialStream = 0x1C00B,
    /// Cache debug stop unexpected sequence service type.
    CaDebugstopUnexpectedSeqServiceType = 0x1C00C,
    /// Cache debug stop mismatched delete SD index.
    CaDebugstopMismatchedDeleteSdIndex = 0x1C00D,
    /// Cache debug stop set host data out of range.
    CaDebugstopSetHostDataOutOfRange = 0x1C00E,
    /// Cache debug stop read write command RSC allocate failed.
    CaDebugstopReadWriteCmdRscAllocFailed = 0x1C00F,
    /// Cache debug stop unexpected cancel while flushing.
    CaDebugstopUnexpectedCancelWhileFlushing = 0x1C010,
    /// Cache debug stop cacheoff wrong return state.
    CaDebugstopCacheoffWrongReturnState = 0x1C011,
    /// Cache debug stop unexpected return SD hit info call.
    CaDebugstopUnexpectedReturnSdHitInfoCall = 0x1C012,
    /// Cache debug stop SD scan index out of range.
    CaDebugstopSdScanIndexOutOfRange = 0x1C013,
    /// Cache debug stop unexpected SD state.
    CaDebugstopUnexpectedSdState = 0x1C014,
    /// Cache debug stop invalid cache.
    CaDebugstopInvalidCache = 0x1C015,
    /// Cache debug stop no CD or SD attached.
    CaDebugstopNoCdOrSdAttached = 0x1C016,
    /// Cache debug stop unexpected read scan state.
    CaDebugstopUnexpectedReadScanState = 0x1C017,
    /// Cache debug stop unexpected write scan state.
    CaDebugstopUnexpectedWriteScanState = 0x1C018,
    /// Cache debug stop unexpected host xfer state.
    CaDebugstopUnexpectedHostXferState = 0x1C019,
    /// Cache debug stop bad aggregation count.
    CaDebugstopBadAggregationCount = 0x1C01A,
    /// Cache debug stop missing SD resource.
    CaDebugstopMissingSdResource = 0x1C01B,
    /// Cache debug stop missing buffer resource.
    CaDebugstopMissingBufferResource = 0x1C01C,
    /// Cache DSSS debug stop cannot find fragment SD.
    CaDsssDebugstopCannotFindFragmentSd = 0x1C01D,
    /// Cache debug stop invalid max potential LBA.
    CaDebugstopInvalidMaxPotentialLba = 0x1C01E,
    /// Cache debug stop invalid coherency command.
    CaDebugstopInvalidCoherencyCmd = 0x1C100,
    /// Cache debug stop invalid overlap count.
    CaDebugstopInvalidOverlapCount = 0x1C101,
    /// Cache debug stop invalid coherency SD state.
    CaDebugstopInvalidCoherencySdState = 0x1C102,
    /// Cache debug stop invalid post read count.
    CaDebugstopInvalidPostReadCount = 0x1C103,
    /// Cache debug stop invalid pre read count.
    CaDebugstopInvalidPreReadCount = 0x1C104,
    /// Cache debug stop invalid coherency SD index.
    CaDebugstopInvalidCoherencySdIndex = 0x1C105,
    /// Drive not parked blind write.
    DrvNotParkedBlndwr = 0x1E000,
    /// DXDWCS index skirting.
    DxdwcsIndexSkirting = 0x1E001,
    /// UBAD on.
    UbadOn = 0x1E002,
    /// MRERR debug stop gen bimodal process.
    MrerrDebugstopGenBimodalProc = 0x1E003,
    /// Read channel timeout.
    ReadChannelTimeout = 0x1E004,
    /// Error Recovery invalid table row size.
    ErInvalidTableRowSize = 0x1E100,
    /// Error Recovery invalid repeat count.
    ErInvalidRepeatCount = 0x1E101,
    /// Error Recovery invalid recovery path.
    ErInvalidRecoveryPath = 0x1E102,
    /// Error Recovery invalid table directory entry.
    ErInvalidTableDirectoryEntry = 0x1E103,
    /// Util invalid TREX pattern.
    UtilInvalidTrexPattern = 0x1E104,
    /// FW ECC all CRC bytes are zero.
    FwEccAllCrcBytesAreZero = 0x1E105,
    /// FW ECC need new code for new sector size.
    FwEccNeedNewCodeForNewSecSize = 0x1E106,
    /// FW ECC buffer xfer failure.
    FwEccBufferXferFailure = 0x1E107,
    /// FW ECC buffer read failure.
    FwEccBufferReadFailure = 0x1E108,
    /// FW ECC corrected sector not transferred.
    FwEccCorrectedSectorNotTransferred = 0x1E109,
    /// Error Recovery DFH levels error.
    ErDfhLevelsError = 0x1E110,
    /// Error Recovery invalid virtual memory manager count.
    ErInvalidVmmCount = 0x1E111,
    /// Error Recovery invalid LCT read thermal asperity count.
    ErInvalidLctReadTaCount = 0x1E120,
    /// Error Recovery invalid LCT read non thermal asperity count.
    ErInvalidLctReadNontaCount = 0x1E121,
    /// Error Recovery invalid LCT write count.
    ErInvalidLctWriteCount = 0x1E123,
    /// Error Recovery invalid LCT index count.
    ErInvalidLctIndexCount = 0x1E124,
    /// Vendor specific command protocol violation.
    VscProtocolViolation = 0x1F000,
    /// Vendor specific command Process Self-Test debug stop.
    VscPstDebugstop = 0x1F001,
    /// Vendor specific command too many copies.
    VscTooManyCopies = 0x1F002,
    /// Vendor specific command fieldlist temperature buffer access.
    VscFieldlistTempBuffAccess = 0x1F003,
    /// Vendor specific command write 2t buffer allocate failed.
    VscWrite2tBufferAlocFailed = 0x1F004,
    /// Vendor specific command compareids thread not allocated.
    VscCompareidsThreadNotAllocated = 0x1F005,
    /// Vendor specific command error injection error type.
    VscErrInjectionErrType = 0x1F006,
    /// Vendor specific command error injection function code.
    VscErrInjectionFuncCode = 0x1F007,
    /// Vendor specific command low level command thread not allocated.
    VscLowLevelCommandThreadNotAllocated = 0x1F008,
    /// Vendor specific command max allocation reached.
    VscMaxAllocationReached = 0x1F009,
    /// Vendor specific command internal error injection invalid counter.
    VscIeiInvalidCounter = 0x1F00A,
    /// Vendor specific command init SD allocation failed.
    VscInitSdAllocationFailed = 0x1F010,
    /// Vendor specific command force debug stop.
    VscForceDebugstop = 0x1FFFF,
    /// Transfer debug stop unsupported op code.
    XferDebugstopUnsupportedOpCode = 0x20000,
    /// Transfer debug stop unexpected additional XD request.
    XferDebugstopUnexpectedAdditonalXdRequest = 0x20001,
    /// Transfer debug stop invalid xfer request count.
    XferDebugstopInvalidXferReqCnt = 0x20002,
    /// Transfer debug stop cannot lock thread required.
    XferDebugstopCannotLockThreadRequired = 0x20003,
    /// Transfer debug stop task not idle during rotational vibration.
    XferDebugstopTaskNotIdleDuringRv = 0x20004,
    /// Transfer debug stop invalid insert event.
    XferDebugstopInvalidInsertEvent = 0x20005,
    /// Transfer debug stop data not available for xfer.
    XferDebugstopDataNotAvailForXfer = 0x20006,
    /// Hardware abstraction layer BM debug stop transfer address not found.
    HalBmDebugstopTransferAddrNotFound = 0x20101,
    /// Hardware abstraction layer BM debug stop subsegment not available.
    HalBmDebugstopSubsegmentNotAvailable = 0x20102,
    /// Hardware abstraction layer BM debug stop invalid thread number.
    HalBmDebugstopInvalidThreadNum = 0x20103,
    /// Hardware abstraction layer BM debug stop thread not allocated.
    HalBmDebugstopThreadNotAllocated = 0x20104,
    /// Hardware abstraction layer BM debug stop lost some subsegments.
    HalBmDebugstopLostSomeSubsegments = 0x20105,
    /// Hardware abstraction layer BM debug stop current cluster not found.
    HalBmDebugstopCurrentClusterNotFound = 0x20106,
    /// Hardware abstraction layer BM debug stop current SD not found.
    HalBmDebugstopCurrentSdNotFound = 0x20107,
    /// Hardware abstraction layer BM debug stop non reentrant code executed.
    HalBmDebugstopNonReentrantCodeExecuted = 0x20108,
    /// Transfer debug stop multi xfer in priority queue.
    XferDebugstopMultiXferInPriorityQueue = 0x20180,
    /// Transfer debug stop unexpected p available.
    XferDebugstopUnexpectedPAvailable = 0x20181,
    /// Transfer debug stop AW not expected.
    XferDebugstopAwNotExpected = 0x20182,
    /// Transfer debug stop intruding nonlba.
    XferDebugstopIntrudingNonlba = 0x20183,
    /// Transfer debug stop nonlba thread invalid.
    XferDebugstopNonlbaThreadInvalid = 0x20184,
    /// Transfer debug stop unexpected FW message.
    XferDebugstopUnexpectedFwMsg = 0x20185,
    /// Transfer debug stop unexpected multi done message.
    XferDebugstopUnexpectedMultiDoneMsg = 0x20186,
    /// Transfer debug stop unknown FW message type.
    XferDebugstopUnknownFwMsgType = 0x20187,
    /// Transfer debug stop next xfer TD not present.
    XferDebugstopNextXferTdNotPresent = 0x20188,
    /// Transfer debug stop unexpected AW done.
    XferDebugstopUnexpectedAwDone = 0x20189,
    /// Transfer debug stop unexpected non LBA xfer done.
    XferDebugstopUnexpectedNonLbaXferDone = 0x2018A,
    /// Transfer debug stop unknown xfer message type.
    XferDebugstopUnknownXferMsgType = 0x2018B,
    /// Transfer debug stop xfer error.
    XferDebugstopXferError = 0x2018C,
    /// Transfer debug stop empty FUA queue.
    XferDebugstopEmptyFuaQueue = 0x2018D,
    /// Transfer debug stop EOC request in multi state.
    XferDebugstopEocReqInMultiState = 0x2018E,
    /// Transfer debug stop TD not in queue.
    XferDebugstopTdNotInQueue = 0x2018F,
    /// Transfer debug stop AW transfer message.
    XferDebugstopAwXfrMsg = 0x20190,
    /// Transfer debug stop FUA write message.
    XferDebugstopFuaWriteMsg = 0x20191,
    /// Transfer debug stop nonp not available.
    XferDebugstopNonpNotAvailable = 0x20192,
    /// Transfer debug stop p not available.
    XferDebugstopPNotAvailable = 0x20193,
    /// Interface power management debug stop command interval queue overflow.
    IpmDebugstopCommandIntervalQueueOverflow = 0x21100,
    /// Interface power management debug stop command bin overflow.
    IpmDebugstopCommandBinOverflow = 0x21101,
    /// Data Lifeguard debug stop test track translation failed.
    DlgDebugstopTestTrackTranslationFailed = 0x21200,
    /// Data Lifeguard debug stop invalid warehouse index.
    DlgDebugstopInvalidWarehouseIndex = 0x21201,
}

impl std::fmt::Display for DebugStopCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OsError => write!(f, "OS error"),
            Self::OsNestedInterrupt => write!(f, "OS nested interrupt"),
            Self::OsUndefInstruction => write!(f, "OS undefined instruction"),
            Self::OsPrefetchAbort => write!(f, "OS prefetch abort"),
            Self::OsDataAbort => write!(f, "OS data abort"),
            Self::OsStackOverflow => write!(f, "OS stack overflow"),
            Self::OsIrqStackOverflow => write!(f, "OS interrupt request stack overflow"),
            Self::OsFlagSetFromFiq => write!(f, "OS flag set from fast interrupt request"),
            Self::OsUnmatchedExitTaskUnsafe => write!(f, "OS unmatched exit task unsafe"),
            Self::OsNullPtrAssignment => write!(f, "OS null pointer assignment"),
            Self::OsTaskUnsafeTimeout => write!(f, "OS task unsafe timeout"),
            Self::OsPendInvalidGroup => write!(f, "OS pend invalid group"),
            Self::OsPostInvalidGroup => write!(f, "OS post invalid group"),
            Self::OsQueryInvalidGroup => write!(f, "OS query invalid group"),
            Self::CrtExit => write!(f, "CRT exit"),
            Self::CrtRaise => write!(f, "CRT raise"),
            Self::MemcpyIntoCode => write!(f, "memcpy into code"),
            Self::MemcpyUnalignedSrcPtr => write!(f, "memcpy unaligned source pointer"),
            Self::MemcpyUnalignedDstPtr => write!(f, "memcpy unaligned drive self-test pointer"),
            Self::MemsetUnalignedDstPtr => write!(f, "memset unaligned drive self-test pointer"),
            Self::InvalidSwi => write!(f, "invalid software interrupt"),
            Self::FifoCollisionFwTimeout => write!(f, "FIFO collision FW timeout"),
            Self::CpuHdcWaitTimeout => write!(f, "CPU HDC wait timeout"),
            Self::JumpToZero => write!(f, "jump to zero"),
            Self::MemsetIntoCode => write!(f, "memset into code"),
            Self::MemclrIntoCode => write!(f, "memclr into code"),
            Self::MemmoveIntoCode => write!(f, "memmove into code"),
            Self::DriveErrHistLog => write!(f, "drive error history log"),
            Self::GetExtErr => write!(f, "get extend error"),
            Self::ClrPeriodLog => write!(f, "clear period log"),
            Self::SetSelfTestTimer => write!(f, "set self test timer"),
            Self::ReloCodeToDrmCode => {
                write!(f, "relocation code to Drive Reliability Monitor code")
            },
            Self::UpdateDrvLifeTimeErrCounters => {
                write!(f, "update drive life time error counters")
            },
            Self::SelftestScan => write!(f, "self-test scan"),
            Self::SelftestRdWr => write!(f, "self-test read write"),
            Self::ServocoreInitWrongCore => write!(f, "servo core init wrong core"),
            Self::ServocoreInitPremature => write!(f, "servo core init premature"),
            Self::OvlGuardUndefinedFn => write!(f, "overlay guard undefined function"),
            Self::PmInvalidModeSel => write!(f, "PM invalid mode select"),
            Self::PmInvalidHistoryIndex => write!(f, "PM invalid history index"),
            Self::PmInvalidDcvStateAtDiskSeek => write!(f, "PM invalid DCV state at disk seek"),
            Self::PmInvalidDcvStateAtDiskExit => write!(f, "PM invalid DCV state at disk exit"),
            Self::PmInvalidDcvStateEnteringSleep => {
                write!(f, "PM invalid DCV state entering sleep")
            },
            Self::PmInvalidDcvStateExitingSleep => write!(f, "PM invalid DCV state exiting sleep"),
            Self::PmInvalidVoltageRequest => write!(f, "PM invalid voltage request"),
            Self::UnsupportedSoc => write!(f, "unsupported SOC"),
            Self::IsrHbi => write!(f, "interrupt service routine HBI"),
            Self::IsrDf => write!(f, "interrupt service routine DF"),
            Self::IsrBm => write!(f, "interrupt service routine BM"),
            Self::IsrHbiTonescan => write!(f, "interrupt service routine HBI tonescan"),
            Self::IsrHbiFifoErr => write!(f, "interrupt service routine HBI FIFO error"),
            Self::IsrWatchdogTimeout => write!(f, "interrupt service routine watchdog timeout"),
            Self::IsrExternalTrigger => write!(f, "interrupt service routine external trigger"),
            Self::DiskUnhandledDfError => write!(f, "disk unhandled DF error"),
            Self::DiskInvalidSdChain => write!(f, "disk invalid SD chain"),
            Self::DiskInvalidInsert => write!(f, "disk invalid insert"),
            Self::DiskInvalidCallForward => write!(f, "disk invalid call forward"),
            Self::DiskInvalidOpcode => write!(f, "disk invalid opcode"),
            Self::DiskNoCddPresent => write!(f, "disk no CDD present"),
            Self::DiskServicesInvalidOpcode => write!(f, "disk services invalid opcode"),
            Self::InvalidTdParam => write!(f, "invalid TD parameter"),
            Self::DiskHardwareCheckHang => write!(f, "disk hardware check hang"),
            Self::CorruptTrackStatus => write!(f, "corrupt track status"),
            Self::DiskInvalidBfrXferPtr => write!(f, "disk invalid buffer xfer pointer"),
            Self::DiskSdNotPresent => write!(f, "disk SD not present"),
            Self::NoMoreSeekSetupStructs => write!(f, "no more seek setup structs"),
            Self::InvalidServoEvent => write!(f, "invalid servo event"),
            Self::DiskInvalidSpinupEvent => write!(f, "disk invalid spinup event"),
            Self::DiskInvalidHeadNumber => write!(f, "disk invalid head number"),
            Self::DiskDebugstopUnsupportedRequest => {
                write!(f, "disk debug stop unsupported request")
            },
            Self::DiskDebugstopInvalidExtendSd => write!(f, "disk debug stop invalid extend SD"),
            Self::DiskDebugstopInvalidExtendTrackInfo => {
                write!(f, "disk debug stop invalid extend track info")
            },
            Self::DiskDebugstopWriteExtendNotSequential => {
                write!(f, "disk debug stop write extend not sequential")
            },
            Self::DiskDebugstopInvalidWedgeDataSize => {
                write!(f, "disk debug stop invalid wedge data size")
            },
            Self::DiskInvalidExtendRequest => write!(f, "disk invalid extend request"),
            Self::DiskSpinupTimeoutEvent => write!(f, "disk spinup timeout event"),
            Self::DiskInvalidUninitCdd => write!(f, "disk invalid uninitialised CDD"),
            Self::DiskTonescanDefectBufferEmpty => write!(f, "disk tonescan defect buffer empty"),
            Self::DiskTonescanBadDefectiveWedgeCount => {
                write!(f, "disk tonescan bad defective wedge count")
            },
            Self::DiskDebugstopMaxLsnMismatch => {
                write!(f, "disk debug stop max logical sector number mismatch")
            },
            Self::DiskInvalidHeadCountFromCh => write!(f, "disk invalid head count from ch"),
            Self::DiskInvalidLoopcountRegValue => {
                write!(f, "disk invalid loopcount register value")
            },
            Self::DiskDebugstopDfIsActiveAfterDfStop => {
                write!(f, "disk debug stop DF is active after DF stop")
            },
            Self::DiskInvalidCompletionCount => write!(f, "disk invalid completion count"),
            Self::DiskInvalidWedgeCommand => write!(f, "disk invalid wedge command"),
            Self::DiskInvalidAddrToRcUpdateInSpinup => {
                write!(f, "disk invalid address to rc update in spinup")
            },
            Self::FmDebugstopWrongMode => write!(f, "FM debug stop wrong mode"),
            Self::FmDebugstopInvalidOpcode => write!(f, "FM debug stop invalid opcode"),
            Self::FmDebugstopWrongCallingTask => write!(f, "FM debug stop wrong calling task"),
            Self::FmCopyTempToPermFailed => write!(f, "FM copy temperature to permanent failed"),
            Self::FmDiskTaskNotCalled => write!(f, "FM disk task not called"),
            Self::FmTooManyFlashFiles => write!(f, "FM too many flash files"),
            Self::FmDlmcOpenFailed => write!(f, "FM download microcode open failed"),
            Self::FmDebugstopNotEnoughFmds => write!(f, "FM debug stop not enough FMDS"),
            Self::FmDebugstopFileIdsDontMatch => write!(f, "FM debug stop file IDS dont match"),
            Self::FmDebugstopAlreadyFreedFmd => write!(f, "FM debug stop already freed FMD"),
            Self::FmDebugstopFmdIndexOverwritten => {
                write!(f, "FM debug stop FMD index overwritten")
            },
            Self::FmDebugstopInvalidOrigin => write!(f, "FM debug stop invalid origin"),
            Self::FmDebugstopInvalidOffset => write!(f, "FM debug stop invalid offset"),
            Self::FmDebugstopInvalidLbaOffset => write!(f, "FM debug stop invalid LBA offset"),
            Self::FmDebugstopRequestCountTooBig => write!(f, "FM debug stop request count too big"),
            Self::FmDebugstopPartialOnFlashFiles => {
                write!(f, "FM debug stop partial on flash files")
            },
            Self::FmDebugstopTooManyDirtyFiles => write!(f, "FM debug stop too many dirty files"),
            Self::FmDebugstopNumCopyMoreThenMaxCopy => {
                write!(f, "FM debug stop number copy more then max copy")
            },
            Self::FmOpenEmptySlotNoCanDo => write!(f, "FM open empty slot no can do"),
            Self::FmFmdIndexExceedAllocatedFmdSpace => {
                write!(f, "FM FMD index exceed allocated FMD space")
            },
            Self::FmOpenZipcodeDlgFileFail => write!(f, "FM open zipcode dlg file fail"),
            Self::DiskDebugstopUnexpectedEndOfSdGroup => {
                write!(f, "disk debug stop unexpected end of SD group")
            },
            Self::DiskDebugstopUnexpectedSdValues => {
                write!(f, "disk debug stop unexpected SD values")
            },
            Self::DiskDebugstopNextListOutOfRange => {
                write!(f, "disk debug stop next list out of range")
            },
            Self::DiskDebugstopUnexpectedRemCntValues => {
                write!(f, "disk debug stop unexpected remaining count values")
            },
            Self::DiskDebugstopInvalidCompletedCnt => {
                write!(f, "disk debug stop invalid completed count")
            },
            Self::DiskDebugstopInvalidReadOperation => {
                write!(f, "disk debug stop invalid read operation")
            },
            Self::DiskDebugstopInvalidSscAdjustment => {
                write!(f, "disk debug stop invalid SSC adjustment")
            },
            Self::DiskDebugstopMismatchForFirstSdLoaded => {
                write!(f, "disk debug stop mismatch for first SD loaded")
            },
            Self::DiskDebugstopInvalidSdChain => write!(f, "disk debug stop invalid SD chain"),
            Self::DiskDebugstopMismatchTotalArmCnt => {
                write!(f, "disk debug stop mismatch total ARM count")
            },
            Self::DiskDebugstopInvalidRemainingArmCnt => {
                write!(f, "disk debug stop invalid remaining ARM count")
            },
            Self::DiskDebugstopWriteBackupSizeGreaterThanDiskSegmentSize => write!(
                f,
                "disk debug stop write backup size greater than disk segment size"
            ),
            Self::DiskDebugstopWcsStillRunningOnCheckDiskWrite => write!(
                f,
                "disk debug stop writable control store still running on check disk write"
            ),
            Self::DiskDebugstopFormatterTimeoutOnRead => {
                write!(f, "disk debug stop formatter timeout on read")
            },
            Self::DiskDebugstopFormatterTimeoutOnWrite => {
                write!(f, "disk debug stop formatter timeout on write")
            },
            Self::DiskDebugstopNoBufferPresent => write!(f, "disk debug stop no buffer present"),
            Self::DiskDebugstopSegEndAddrInvalid => {
                write!(f, "disk debug stop segment end address invalid")
            },
            Self::DiskDebugstopSdCountNotZeroForRead => {
                write!(f, "disk debug stop SD count not zero for read")
            },
            Self::DiskDebugstopPseudoSetSscTooLarge => {
                write!(f, "disk debug stop pseudo set SSC too large")
            },
            Self::DiskDebugstopSscNotEqualToRemReqCnt => write!(
                f,
                "disk debug stop SSC not equal to remaining request count"
            ),
            Self::DiskDebugstopDabRemReqCntUnderflow => {
                write!(f, "disk debug stop DAB remaining request count underflow")
            },
            Self::DiskDebugstopUnexpectedErrorStatus => {
                write!(f, "disk debug stop unexpected error status")
            },
            Self::DiskDebugstopIllegalCylSeek => write!(f, "disk debug stop illegal cylinder seek"),
            Self::DiskDebugstopIllegalHeadSeek => write!(f, "disk debug stop illegal head seek"),
            Self::DiskDebugstopReadExtendNeedsConnectToHost => {
                write!(f, "disk debug stop read extend needs connect to host")
            },
            Self::DiskDebugstopNoSdAttached => write!(f, "disk debug stop no SD attached"),
            Self::DiskDebugstopNoCdAttached => write!(f, "disk debug stop no CD attached"),
            Self::DiskDebugstopSeekNotComplete => write!(f, "disk debug stop seek not complete"),
            Self::DiskDebugstopReadOverflowSizeGreaterThanDiskSegmentSize => write!(
                f,
                "disk debug stop read overflow size greater than disk segment size"
            ),
            Self::DiskDebugstopWrongSeekIssued => write!(f, "disk debug stop wrong seek issued"),
            Self::DiskDebugstopLinkedSequentialThread => {
                write!(f, "disk debug stop linked sequential thread")
            },
            Self::DiskDebugstopHangingDiskEvents => {
                write!(f, "disk debug stop hanging disk events")
            },
            Self::DiskDebugstopNegativeRemRequestCount => {
                write!(f, "disk debug stop negative remaining request count")
            },
            Self::DiskDebugstopInvalidTrexPattern1 => {
                write!(f, "disk debug stop invalid TREX pattern1")
            },
            Self::DiskDebugstopInvalidTrexPattern2 => {
                write!(f, "disk debug stop invalid TREX pattern2")
            },
            Self::DiskDebugstopInvalidTrexPattern3 => {
                write!(f, "disk debug stop invalid TREX pattern3")
            },
            Self::DiskDebugstopNoErrorFromGatherstatus => {
                write!(f, "disk debug stop no error from gatherstatus")
            },
            Self::DiskDebugstopServoEventTimeout1 => {
                write!(f, "disk debug stop servo event timeout1")
            },
            Self::DiskDebugstopServoEventTimeout2 => {
                write!(f, "disk debug stop servo event timeout2")
            },
            Self::DiskDebugstopCacheMemoryTestFailed => {
                write!(f, "disk debug stop cache memory test failed")
            },
            Self::DiskDebugstopMismatchedRemReqCnt => {
                write!(f, "disk debug stop mismatched remaining request count")
            },
            Self::DiskDebugstopPsnLsnMismatch => write!(
                f,
                "disk debug stop physical sector number logical sector number mismatch"
            ),
            Self::DiskDebugstopServoTimeout => write!(f, "disk debug stop servo timeout"),
            Self::DiskDebugstopServoWrongHeadSelected => {
                write!(f, "disk debug stop servo wrong head selected")
            },
            Self::DiskDebugstopErBadTranslation => write!(f, "disk debug stop er bad translation"),
            Self::DiskDebugstopNoDdAttached => write!(f, "disk debug stop no DD attached"),
            Self::DiskDebugstopMisalignedXferPtr => {
                write!(f, "disk debug stop misaligned xfer pointer")
            },
            Self::DiskDebugstopUnexpectedSegmentCnt => {
                write!(f, "disk debug stop unexpected segment count")
            },
            Self::DiskDebugstopBadTestTrack => write!(f, "disk debug stop bad test track"),
            Self::DiskDebugstopFindmaxlbaNoCurTd => {
                write!(f, "disk debug stop findmaxlba no current TD")
            },
            Self::DiskDebugstopFindmaxlbaInvalidReq => {
                write!(f, "disk debug stop findmaxlba invalid request")
            },
            Self::DiskDebugstopXferWakeAtLbaWrite => {
                write!(f, "disk debug stop xfer wake at LBA write")
            },
            Self::DiskDebugstopIllegalCmdForHeadOfQueue => {
                write!(f, "disk debug stop illegal command for head of queue")
            },
            Self::DiskDebugstopIllegalQueueDepth => {
                write!(f, "disk debug stop illegal queue depth")
            },
            Self::DiskDebugstopAfterCancelStillVeryBusy => {
                write!(f, "disk debug stop after cancel still very busy")
            },
            Self::DiskDebugstopQueueGreaterThanOne => {
                write!(f, "disk debug stop queue greater than one")
            },
            Self::DiskDebugstopSequentialWithLinkedSds => {
                write!(f, "disk debug stop sequential with linked SDS")
            },
            Self::DiskDebugstopInvalidWedgeDownCounterCalculated => {
                write!(f, "disk debug stop invalid wedge down counter calculated")
            },
            Self::DiskDebugstopInvalidWedgeDownCounterDetectedWhenDecrement => write!(
                f,
                "disk debug stop invalid wedge down counter detected when decrement"
            ),
            Self::DiskDebugstopInvalidWedgeDownCounterDetectedWhenAdjust => write!(
                f,
                "disk debug stop invalid wedge down counter detected when adjust"
            ),
            Self::DiskDebugstopSeekLatencyTableNotSupported => {
                write!(f, "disk debug stop seek latency table not supported")
            },
            Self::DiskDebugstopReadThreadCountMismatch => {
                write!(f, "disk debug stop read thread count mismatch")
            },
            Self::DiskDebugstopReadErResumePtrIsNull => {
                write!(f, "disk debug stop read er resume pointer is null")
            },
            Self::DiskDebugstopNoHeadSwitch => write!(f, "disk debug stop no head switch"),
            Self::DiskDebugstopDsssUnalignedLba => write!(f, "disk debug stop DSSS unaligned LBA"),
            Self::DiskDebugstopInvalidTimerId => write!(f, "disk debug stop invalid timer ID"),
            Self::DiskDebugstopGetTempTimeoutExceeded => {
                write!(f, "disk debug stop get temperature timeout exceeded")
            },
            Self::DiskDebugstopInvalidServoScanState => {
                write!(f, "disk debug stop invalid servo scan state")
            },
            Self::DiskDebugstopInvalidClusterIndex => {
                write!(f, "disk debug stop invalid cluster index")
            },
            Self::DiskDebugstopArmingMishap => write!(f, "disk debug stop arming mishap"),
            Self::DiskDebugstopStillbusyArm => write!(f, "disk debug stop stillbusy ARM"),
            Self::DiskDebugstopFirstLbaNotFound => write!(f, "disk debug stop first LBA not found"),
            Self::DiskDebugstopZeroCountDecrmentAttmp => {
                write!(f, "disk debug stop zero count decrement attempt")
            },
            Self::DiskDebugstopZeroCountWrtZipHigh => {
                write!(f, "disk debug stop zero count write zip high")
            },
            Self::DiskDebugstopZeroCountWrtZipLow => {
                write!(f, "disk debug stop zero count write zip low")
            },
            Self::DiskDebugstopZipFileWrtDuringFlush => {
                write!(f, "disk debug stop zip file write during flush")
            },
            Self::DiskDebugstopGetTempInvalidTaskId => {
                write!(f, "disk debug stop get temperature invalid task ID")
            },
            Self::DiskDebugstopInvalidChannelScanState => {
                write!(f, "disk debug stop invalid channel scan state")
            },
            Self::DiskDebugstopReadOfftrackLimitsConflict => {
                write!(f, "disk debug stop read offtrack limits conflict")
            },
            Self::DiskDebugstopWriteOfftrackLimitsConflict => {
                write!(f, "disk debug stop write offtrack limits conflict")
            },
            Self::DiskDebugstopPredictOfftrackLimitsConflict => {
                write!(f, "disk debug stop predict offtrack limits conflict")
            },
            Self::DiskDebugstopFineTrackoffsetsConflict => {
                write!(f, "disk debug stop fine trackoffsets conflict")
            },
            Self::DfhDebugPreheatCheckRegCZero => {
                write!(f, "DFH debug preheat check register c zero")
            },
            Self::DfhDebugDfStoppedDfhRegDChanged => {
                write!(f, "DFH debug DF stopped DFH register d changed")
            },
            Self::DfhDebugDfStoppedDfhRegCChanged => {
                write!(f, "DFH debug DF stopped DFH register c changed")
            },
            Self::DfhDebugStateMonitorBadPreheatCount => {
                write!(f, "DFH debug state monitor bad preheat count")
            },
            Self::DfhDebugStateIdleBadPreheatCount => {
                write!(f, "DFH debug state idle bad preheat count")
            },
            Self::DfhDebugOnditionMetBadDfhState => {
                write!(f, "DFH debug condition met bad DFH state")
            },
            Self::ReloInvalidErrorStatus => write!(f, "relocation invalid error status"),
            Self::ReloInvalidTdOpcode => write!(f, "relocation invalid TD opcode"),
            Self::ReloNoBadWedgeInfo => write!(f, "relocation no bad wedge info"),
            Self::ReloSstNotPossibleOnRead => write!(f, "relocation SST not possible on read"),
            Self::ReloSpareLbaWithNoUserLba => write!(f, "relocation spare LBA with no user LBA"),
            Self::ReloInvalidToUseErDabs => write!(f, "relocation invalid to use er DABS"),
            Self::ReloHandlerLostInSpace => write!(f, "relocation handler lost in space"),
            Self::ReloExperimentNoTaresAllowed => write!(
                f,
                "relocation experiment no transparent auto relocations allowed"
            ),
            Self::ReloExperimentNoRelosAllowed => {
                write!(f, "relocation experiment no relos allowed")
            },
            Self::ReloExperimentNoTaTaresAllowed => write!(
                f,
                "relocation experiment no thermal asperity transparent auto relocations allowed"
            ),
            Self::ReloExperimentNoRsvdBit3Allowed => {
                write!(f, "relocation experiment no reserved bit3 allowed")
            },
            Self::ReloExperimentNoRsvdBit4Allowed => {
                write!(f, "relocation experiment no reserved bit4 allowed")
            },
            Self::ReloExperimentNoRsvdBit5Allowed => {
                write!(f, "relocation experiment no reserved bit5 allowed")
            },
            Self::ReloExperimentNoRsvdBit6Allowed => {
                write!(f, "relocation experiment no reserved bit6 allowed")
            },
            Self::ReloExperimentNoRsvdBit7Allowed => {
                write!(f, "relocation experiment no reserved bit7 allowed")
            },
            Self::ReloBadWedgeCfgOutOfRange => {
                write!(f, "relocation bad wedge config out of range")
            },
            Self::ApbReadError => write!(f, "advanced Peripheral Bus read error"),
            Self::InvalidApbChecksum => write!(f, "invalid Advanced Peripheral Bus checksum"),
            Self::InvalidFormatSurfaceId => write!(f, "invalid format surface ID"),
            Self::DabNestedDisconnect => write!(f, "DAB nested disconnect"),
            Self::DabConnectWithoutDisconnect => write!(f, "DAB connect without disconnect"),
            Self::DiskDisconnectedOnCacheCmd => write!(f, "disk disconnected on cache command"),
            Self::DiskIeiHalInvalidErrorType => write!(
                f,
                "disk internal error injection hardware abstraction layer invalid error type"
            ),
            Self::DiskIeiInvalidCounter => {
                write!(f, "disk internal error injection invalid counter")
            },
            Self::RscDebugstopNoFreeCd => write!(f, "resource debug stop no free CD"),
            Self::RscDebugstopNoFreeXd => write!(f, "resource debug stop no free XD"),
            Self::RscDebugstopPutXdAlreadyFree => {
                write!(f, "resource debug stop put XD already free")
            },
            Self::RscDebugstopNoFreeDd => write!(f, "resource debug stop no free DD"),
            Self::RscDebugstopPutDdAlreadyFree => {
                write!(f, "resource debug stop put DD already free")
            },
            Self::RscDebugstopNoFreeTd => write!(f, "resource debug stop no free TD"),
            Self::RscDebugstopTdDiskQueueFull => {
                write!(f, "resource debug stop TD disk queue full")
            },
            Self::RscDebugstopDequeueTdQueueEmpty => {
                write!(f, "resource debug stop dequeue TD queue empty")
            },
            Self::RscDebugstopEnqueueTdIndexUsed => {
                write!(f, "resource debug stop enqueue TD index used")
            },
            Self::RscDebugstopRemoveTdQueueEmpty => {
                write!(f, "resource debug stop remove TD queue empty")
            },
            Self::RscDebugstopRemoveTdBadIndex => {
                write!(f, "resource debug stop remove TD bad index")
            },
            Self::RscDebugstopNoSdToDeallocate => {
                write!(f, "resource debug stop no SD to deallocate")
            },
            Self::RscDebugstopNoCdForVerifyBuffer => {
                write!(f, "resource debug stop no CD for verify buffer")
            },
            Self::RscDebugstopPutTdAlreadyFree => {
                write!(f, "resource debug stop put TD already free")
            },
            Self::RscDebugstopPutCdAlreadyFree => {
                write!(f, "resource debug stop put CD already free")
            },
            Self::RscDebugstopInvalidSdState => write!(f, "resource debug stop invalid SD state"),
            Self::RscDebugstopGetNullBufferAddressSd => {
                write!(f, "resource debug stop get null buffer address SD")
            },
            Self::RscDebugstopGetNullBufferAddressCd1 => {
                write!(f, "resource debug stop get null buffer address CD1")
            },
            Self::RscDebugstopGetNullBufferAddressCd2 => {
                write!(f, "resource debug stop get null buffer address CD2")
            },
            Self::RscDebugstopClusterChainTooLong => {
                write!(f, "resource debug stop cluster chain too long")
            },
            Self::RscDebugstopAllocBuffersNotByExecTask => {
                write!(f, "resource debug stop allocate buffers not by exec task")
            },
            Self::RscDebugstopAllocSdNotByExecTask => {
                write!(f, "resource debug stop allocate SD not by exec task")
            },
            Self::RscDebugstopGetNullBufferAddressDd1 => {
                write!(f, "resource debug stop get null buffer address DD1")
            },
            Self::RscDebugstopGetNullBufferAddressDd2 => {
                write!(f, "resource debug stop get null buffer address DD2")
            },
            Self::RscDebugstopInvalidLockCntState => {
                write!(f, "resource debug stop invalid lock count state")
            },
            Self::RscDebugstopInvalidTotalClusterCount => {
                write!(f, "resource debug stop invalid total cluster count")
            },
            Self::RscDebugstopInvalidClusterIndex => {
                write!(f, "resource debug stop invalid cluster index")
            },
            Self::RscDebugstopInvalidFreeClusterCount => {
                write!(f, "resource debug stop invalid free cluster count")
            },
            Self::RscDebugstopDeallocateFreeCluster => {
                write!(f, "resource debug stop deallocate free cluster")
            },
            Self::RscDebugstopAlloEmptyFreeClusterList => {
                write!(f, "resource debug stop allocate empty free cluster list")
            },
            Self::RscDebugstopAlloClusterNotInFreeList => {
                write!(f, "resource debug stop allocate cluster not in free list")
            },
            Self::RscDebugstopInitClustersNotAllFree => {
                write!(f, "resource debug stop init clusters not all free")
            },
            Self::RscDebugstopPermAllocTooLarge => {
                write!(f, "resource debug stop permanent allocate too large")
            },
            Self::RscDebugstopClusterCountBeyondRange => {
                write!(f, "resource debug stop cluster count beyond range")
            },
            Self::RscDebugstopClusterCountNotEnough => {
                write!(f, "resource debug stop cluster count not enough")
            },
            Self::RscDebugstopDlmcBufferTooSmall => {
                write!(f, "resource debug stop download microcode buffer too small")
            },
            Self::RscDebugstopReqClustersBeyondCapacity => {
                write!(f, "resource debug stop request clusters beyond capacity")
            },
            Self::RscDebugstopUnableFlushClusters => {
                write!(f, "resource debug stop unable flush clusters")
            },
            Self::RscDebugstopFlushClustersTimeout => {
                write!(f, "resource debug stop flush clusters timeout")
            },
            Self::RscDebugstopInvalidCdIndex => write!(f, "resource debug stop invalid CD index"),
            Self::RscDebugstopSequentialSdInWrongState => {
                write!(f, "resource debug stop sequential SD in wrong state")
            },
            Self::RscDebugstopNoBufferRequestedFlagNotSet => {
                write!(f, "resource debug stop no buffer requested flag not set")
            },
            Self::RscDebugstopInvalidSdIndex => write!(f, "resource debug stop invalid SD index"),
            Self::RscDebugstopInvalidTdIndex => write!(f, "resource debug stop invalid TD index"),
            Self::RscDebugstopInvalidTdTaskId => {
                write!(f, "resource debug stop invalid TD task ID")
            },
            Self::RscDebugstopFreeSdInSeqStream => {
                write!(f, "resource debug stop free SD in sequence stream")
            },
            Self::RscDebugstopSendToDiskWithSeqStream => {
                write!(f, "resource debug stop send to disk with sequence stream")
            },
            Self::RscDebugstopFlushWithSeqStream => {
                write!(f, "resource debug stop flush with sequence stream")
            },
            Self::RscDebugstopInvalidDdIndex => write!(f, "resource debug stop invalid DD index"),
            Self::RscDebugstopInvalidXdIndex => write!(f, "resource debug stop invalid XD index"),
            Self::RscDebugstopInvalidStateQcmdBitAndCdIndex => {
                write!(f, "resource debug stop invalid state QCMD bit and CD index")
            },
            Self::RscDebugstopInvalidTrimParameters => {
                write!(f, "resource debug stop invalid trim parameters")
            },
            Self::RscDebugstopTrimWithZeroBuffer => {
                write!(f, "resource debug stop trim with zero buffer")
            },
            Self::RscDebugstopInvalidInsertIndex => {
                write!(f, "resource debug stop invalid insert index")
            },
            Self::RscDebugstopAllocateCdWhenCdAlreadyAllocated => write!(
                f,
                "resource debug stop allocate CD when CD already allocated"
            ),
            Self::RscDebugstopAllocateDdWhenDdAlreadyAllocated => write!(
                f,
                "resource debug stop allocate DD when DD already allocated"
            ),
            Self::RscDebugstopAllocateXdWhenXdAlreadyAllocated => write!(
                f,
                "resource debug stop allocate XD when XD already allocated"
            ),
            Self::RscDebugstopCyclicSdStateqCheckFail => {
                write!(f, "resource debug stop cyclic SD state queue check fail")
            },
            Self::RscDebugstopCyclicSdClusterCheckFail => {
                write!(f, "resource debug stop cyclic SD cluster check fail")
            },
            Self::RscDebugstopCyclicClusterChainCheckFail => {
                write!(f, "resource debug stop cyclic cluster chain check fail")
            },
            Self::RscDebugstopCyclicFrClusterCheckFail => {
                write!(f, "resource debug stop cyclic fr cluster check fail")
            },
            Self::RscDebugstopTotalClCountCheckFail => {
                write!(f, "resource debug stop total cluster count check fail")
            },
            Self::RscDebugstopTotalSdCountCheckFail => {
                write!(f, "resource debug stop total SD count check fail")
            },
            Self::RscDebugstopFellOffEndOfSdChain => {
                write!(f, "resource debug stop fell off end of SD chain")
            },
            Self::RscDynamicAvailCountUnderflow => {
                write!(f, "resource dynamic available count underflow")
            },
            Self::RscDynamicValidCountUnderflow => {
                write!(f, "resource dynamic valid count underflow")
            },
            Self::RscSdUnlockCallbackError1 => write!(f, "resource SD unlock callback error1"),
            Self::RscSdUnlockCallbackError2 => write!(f, "resource SD unlock callback error2"),
            Self::RscDebugstopBadXferReserveCnt => {
                write!(f, "resource debug stop bad xfer reserve count")
            },
            Self::RscInvalidSdUnlock => write!(f, "resource invalid SD unlock"),
            Self::HdaRealnumheadsInvalid => write!(f, "head disk assembly realnumheads invalid"),
            Self::HdaDebugstopNoMoreSeekSetupStructs => write!(
                f,
                "head disk assembly debug stop no more seek setup structs"
            ),
            Self::HdaDebugstopDeallocateNullSeekSetupPtr => write!(
                f,
                "head disk assembly debug stop deallocate null seek setup pointer"
            ),
            Self::HdaDebugstopDeallocateFreeSeekSetupStructure => write!(
                f,
                "head disk assembly debug stop deallocate free seek setup structure"
            ),
            Self::ServoApiDebugstopInvalidStateTransition => {
                write!(f, "servo API debug stop invalid state transition")
            },
            Self::MrmDebugstopSubmitInvalidStateQueue => {
                write!(f, "MRM debug stop submit invalid state queue")
            },
            Self::MrmDebugstopSubmitNoBuffer => write!(f, "MRM debug stop submit no buffer"),
            Self::MrmDebugstopRemoveInvalidStateQueue => {
                write!(f, "MRM debug stop remove invalid state queue")
            },
            Self::MrmDebugstopSendtodiskInvalidStateQueue => {
                write!(f, "MRM debug stop sendtodisk invalid state queue")
            },
            Self::MrmDebugstopSendtodiskNoBuffer => {
                write!(f, "MRM debug stop sendtodisk no buffer")
            },
            Self::MrmDebugstopSendtodiskSdInGroup => {
                write!(f, "MRM debug stop sendtodisk SD in group")
            },
            Self::MrmDebugstopFlushspecificNoBuffer => {
                write!(f, "MRM debug stop flushspecific no buffer")
            },
            Self::MrmDebugstopFlushspecificInvalidStateQueue => {
                write!(f, "MRM debug stop flushspecific invalid state queue")
            },
            Self::MrmDebugstopFlushspecificInvalidGroup => {
                write!(f, "MRM debug stop flushspecific invalid group")
            },
            Self::MrmDebugstopFilemgrInvalidStateQueue => {
                write!(f, "MRM debug stop filemgr invalid state queue")
            },
            Self::MrmDebugstopAddtogroupInvalidGroup => {
                write!(f, "MRM debug stop addtogroup invalid group")
            },
            Self::MrmDebugstopTdWithoutCd => write!(f, "MRM debug stop TD without CD"),
            Self::MrmDebugstopReleasecallbackWrongState => {
                write!(f, "MRM debug stop releasecallback wrong state")
            },
            Self::MrmDebugstopProcesscommandWrongState => {
                write!(f, "MRM debug stop processcommand wrong state")
            },
            Self::MrmDebugstopDefaultcallbackWrongState => {
                write!(f, "MRM debug stop defaultcallback wrong state")
            },
            Self::MrmDebugstopWedgeCountValidTimeout => {
                write!(f, "MRM debug stop wedge count valid timeout")
            },
            Self::MrmDebugstopFlushRpoChwNotValid => write!(
                f,
                "MRM debug stop flush rotational position optimisation cylinder head wedge not \
                 valid"
            ),
            Self::MrmStartTimedOut => write!(f, "MRM start timed out"),
            Self::MrmDebugstopInvalidOpMrmServiceRequest => {
                write!(f, "MRM debug stop invalid op MRM service request")
            },
            Self::MrmDebugstopInvalidDdCallbackState => {
                write!(f, "MRM debug stop invalid DD callback state")
            },
            Self::MrmDebugstopTdWithoutSd => write!(f, "MRM debug stop TD without SD"),
            Self::MrmUnexpectedSdInIndependentState => {
                write!(f, "MRM unexpected SD in independent state")
            },
            Self::MrmUnexpectedSdWaitState => write!(f, "MRM unexpected SD wait state"),
            Self::MrmDiskCancelQueueFailure => write!(f, "MRM disk cancel queue failure"),
            Self::MrmDiskInvalidSdState => write!(f, "MRM disk invalid SD state"),
            Self::MrmDiskInvalidDdCount => write!(f, "MRM disk invalid DD count"),
            Self::MrmDiskOddLbaCountRequest => write!(f, "MRM disk odd LBA count request"),
            Self::MrmInvalidDsssMergeDirtyOperation => {
                write!(f, "MRM invalid DSSS merge dirty operation")
            },
            Self::MrmInvalidTaskId => write!(f, "MRM invalid task ID"),
            Self::RpoDebugstopPrereadXlatError => write!(
                f,
                "rotational position optimisation debug stop preread translate error"
            ),
            Self::RpoDebugstopSeekProfileTableError => write!(
                f,
                "rotational position optimisation debug stop seek profile table error"
            ),
            Self::ExecDebugstopSectionStart => write!(f, "execution debug stop section start"),
            Self::ExecDebugstopInvalidTdOpcode => {
                write!(f, "execution debug stop invalid TD opcode")
            },
            Self::ExecDebugstopInvalidFmRequest => {
                write!(f, "execution debug stop invalid fm request")
            },
            Self::ExecDebugstopCdRequired => write!(f, "execution debug stop CD required"),
            Self::ExecDlg2InvalidArea => write!(f, "execution DLG2 invalid area"),
            Self::ExecDlg2RscFailure => write!(f, "execution DLG2 RSC failure"),
            Self::ExecDlg2InvalidRequestCount => write!(f, "execution DLG2 invalid request count"),
            Self::ExecDlg2InvalidLba => write!(f, "execution DLG2 invalid LBA"),
            Self::ExecDlg2ReadSpbaError => write!(f, "execution DLG2 read SPBA error"),
            Self::ExecDlg2VerifySpbaError => write!(f, "execution DLG2 verify SPBA error"),
            Self::ExecDlg2SectorCntBiggerThanConfigDefaultCnt => write!(
                f,
                "execution DLG2 sector count bigger than config default count"
            ),
            Self::ExecInvalidFactorySelftestCustomerId => {
                write!(f, "execution invalid factory selftest customer ID")
            },
            Self::BgDebugstopDstRecordresultsInvalidSubact => write!(
                f,
                "BG debug stop drive self-test recordresults invalid sub-action"
            ),
            Self::BgDebugstopOlRecordresultsInvalidSubact => {
                write!(f, "BG debug stop OL recordresults invalid sub-action")
            },
            Self::BgDebugstopInvalidCurrActivity => {
                write!(f, "BG debug stop invalid current activity")
            },
            Self::BgDebugstopInvalidTdOpcode => write!(f, "BG debug stop invalid TD opcode"),
            Self::BgDebugstopInvalidTimerId => write!(f, "BG debug stop invalid timer ID"),
            Self::BgDebugstopInvalidBgState => write!(f, "BG debug stop invalid bg state"),
            Self::BgDebugstopInvalidTestinfoStatus => {
                write!(f, "BG debug stop invalid testinfo status")
            },
            Self::BgDebugstopPstInvalidTrbIndex => {
                write!(f, "BG debug stop Process Self-Test invalid trb index")
            },
            Self::BgDebugstopPstUnableToLoadSeqTable => write!(
                f,
                "BG debug stop Process Self-Test unable to load sequence table"
            ),
            Self::BgDebugstopPstUnableToSaveSeqTable => write!(
                f,
                "BG debug stop Process Self-Test unable to save sequence table"
            ),
            Self::BgDebugstopPstInvalidTestId => {
                write!(f, "BG debug stop Process Self-Test invalid test ID")
            },
            Self::BgDebugstopPstInvalidStatus => {
                write!(f, "BG debug stop Process Self-Test invalid status")
            },
            Self::BgDebugstopPstInterfaceVersionMismatch => write!(
                f,
                "BG debug stop Process Self-Test interface version mismatch"
            ),
            Self::BgDebugstopPstInvalidTrbSize => {
                write!(f, "BG debug stop Process Self-Test invalid trb size")
            },
            Self::BgDebugstopTdAlreadyAllocated => write!(f, "BG debug stop TD already allocated"),
            Self::BgDebugstopIsramChksumFailed => {
                write!(f, "BG debug stop instruction SRAM checksum failed")
            },
            Self::BgDebugstopRvaMrmStartTimedOut => {
                write!(f, "BG debug stop RVA MRM start timed out")
            },
            Self::BgDebugstopRvaRequiresCd => write!(f, "BG debug stop RVA requires CD"),
            Self::BgDebugstopRvaRequiresCdd => write!(f, "BG debug stop RVA requires CDD"),
            Self::BgDebugstopRvaInvalidSd => write!(f, "BG debug stop RVA invalid SD"),
            Self::BgDebugstopRvaIllegalRequestSize => {
                write!(f, "BG debug stop RVA illegal request size")
            },
            Self::BgDebugstopWaMrmStartTimedOut => {
                write!(f, "BG debug stop WA MRM start timed out")
            },
            Self::BgDebugstopWaRequiresCd => write!(f, "BG debug stop WA requires CD"),
            Self::BgDebugstopWaRequiresCdd => write!(f, "BG debug stop WA requires CDD"),
            Self::BgDebugstopWaRequestSizeTooLarge => {
                write!(f, "BG debug stop WA request size too large")
            },
            Self::BgDebugstopPreemptedUnderBsyProtect => {
                write!(f, "BG debug stop preempted under busy protect")
            },
            Self::BgDebugstopDramtestNullTestinfoPointer => {
                write!(f, "BG debug stop dramtest null testinfo pointer")
            },
            Self::BgDebugstopErrorcodeNullTestinfoPointer => {
                write!(f, "BG debug stop errorcode null testinfo pointer")
            },
            Self::BgDebugstopResfilecheckNullTestinfoPointer => {
                write!(f, "BG debug stop resfilecheck null testinfo pointer")
            },
            Self::BgDebugstopMemcheckNullTestinfoPointer => {
                write!(f, "BG debug stop memcheck null testinfo pointer")
            },
            Self::BgDebugstopTaNullTestinfoPointer => {
                write!(f, "BG debug stop thermal asperity null testinfo pointer")
            },
            Self::BgDebugstopScanNullTestinfoPointer => {
                write!(f, "BG debug stop scan null testinfo pointer")
            },
            Self::BgDebugstopPstStackCheckFailed => {
                write!(f, "BG debug stop Process Self-Test stack check failed")
            },
            Self::BgDebugstopWaIllegalZeroRequestSize => {
                write!(f, "BG debug stop WA illegal zero request size")
            },
            Self::BgDebugstopWaAllLbaRequestSizeTooLarge => {
                write!(f, "BG debug stop WA all LBA request size too large")
            },
            Self::BgDebugstopReleaseResourceCallbackTimeout => {
                write!(f, "BG debug stop release resource callback timeout")
            },
            Self::BgDebugstopPstUnableToFlashPtm => write!(
                f,
                "BG debug stop Process Self-Test unable to flash Process Test Module"
            ),
            Self::BgDebugstopWaMissingReleaseResourceInfo => {
                write!(f, "BG debug stop WA missing release resource info")
            },
            Self::BgDebugstopTraceHang => write!(f, "BG debug stop trace hang"),
            Self::BgDebugstopPstPostupFailure => {
                write!(f, "BG debug stop Process Self-Test postup failure")
            },
            Self::BgDebugstopPstPermAllocationFailed => write!(
                f,
                "BG debug stop Process Self-Test permanent allocation failed"
            ),
            Self::BgDebugstopInvalidIdleType => write!(f, "BG debug stop invalid idle type"),
            Self::BgDebugstopCompidRequiresCd => {
                write!(f, "BG debug stop component ID requires CD")
            },
            Self::BgDebugstopCompidRequiresCdd => {
                write!(f, "BG debug stop component ID requires CDD")
            },
            Self::BgDebugstopCompidRequestSizeTooLarge => {
                write!(f, "BG debug stop component ID request size too large")
            },
            Self::BgDebugstopCompidMrmStartTimedOut => {
                write!(f, "BG debug stop component ID MRM start timed out")
            },
            Self::BgDebugstopCompidMrmRequestError => {
                write!(f, "BG debug stop component ID MRM request error")
            },
            Self::BgDebugstopCompidIllegalZeroRequestSize => {
                write!(f, "BG debug stop component ID illegal zero request size")
            },
            Self::BgDebugstopCompidLbaRequestSizeTooLarge => {
                write!(f, "BG debug stop component ID LBA request size too large")
            },
            Self::BgDebugstopDstRecordresultsInvalidTestlevel => write!(
                f,
                "BG debug stop drive self-test recordresults invalid testlevel"
            ),
            Self::BgDebugstopScanInvalidSubActiDstQuick => write!(
                f,
                "BG debug stop scan invalid sub active drive self-test quick"
            ),
            Self::BgDebugstopScanInvalidSubActiDstExtended => write!(
                f,
                "BG debug stop scan invalid sub active drive self-test extended"
            ),
            Self::BgDebugstopScanInvalidSubActiDstConveyance => write!(
                f,
                "BG debug stop scan invalid sub active drive self-test conveyance"
            ),
            Self::BgDebugstopScanInvalidSubActiDstSelective => write!(
                f,
                "BG debug stop scan invalid sub active drive self-test selective"
            ),
            Self::BgDebugstopDlgClrzipFailed => write!(f, "BG debug stop dlg clrzip failed"),
            Self::BgDebugstopRefreshLbaReturnCountInvalid => {
                write!(f, "BG debug stop refresh LBA return count invalid")
            },
            Self::BgDebugstopDlgLowThresholdGreaterThanHighThreshold => write!(
                f,
                "BG debug stop dlg low threshold greater than high threshold"
            ),
            Self::BgDebugstopDlgIllegalSchedulerState => {
                write!(f, "BG debug stop dlg illegal scheduler state")
            },
            Self::BgDebugstopDlgPstModeCtlrFailed => write!(
                f,
                "BG debug stop dlg Process Self-Test mode controller failed"
            ),
            Self::BgDebugstopRemainingLbaCountLessThanRefresherCount => write!(
                f,
                "BG debug stop remaining LBA count less than refresher count"
            ),
            Self::BgDebugstopRefreshStartLbaGreaterThanMaxZipLba => write!(
                f,
                "BG debug stop refresh start LBA greater than max zip LBA"
            ),
            Self::BgDebugstopPstIndexOverrunStatic => {
                write!(f, "BG debug stop Process Self-Test index overrun static")
            },
            Self::BgDebugstopAtaIndexOverrunStatic => {
                write!(f, "BG debug stop ATA index overrun static")
            },
            Self::BgDebugstopWaIllegalRequestSize => {
                write!(f, "BG debug stop WA illegal request size")
            },
            Self::ArDebugstopHsdtInvalidMappingResult => write!(
                f,
                "AR debug stop Hard Sector Descriptor Table invalid mapping result"
            ),
            Self::CrDebugstopReadWriteInProgress => {
                write!(f, "cached relocation debug stop read write in progress")
            },
            Self::CrDebugstopReadReloDoesNotExist => {
                write!(f, "cached relocation debug stop read relo does not exist")
            },
            Self::CrDebugstopWriteReloDoesNotExist => {
                write!(f, "cached relocation debug stop write relo does not exist")
            },
            Self::CrDebugstopSpareRwInvalidIndex => {
                write!(f, "cached relocation debug stop spare RW invalid index")
            },
            Self::CrDebugstopReadingReloNeverWritten => {
                write!(f, "cached relocation debug stop reading relo never written")
            },
            Self::CrDebugstopBufferAllocationFailed => {
                write!(f, "cached relocation debug stop buffer allocation failed")
            },
            Self::CrDebugstopBufferNotAllocated => {
                write!(f, "cached relocation debug stop buffer not allocated")
            },
            Self::CrDebugstopUnableToAllocatePermanentSd => write!(
                f,
                "cached relocation debug stop unable to allocate permanent SD"
            ),
            Self::HostDebugstopXferInvalidCountLeft => {
                write!(f, "host debug stop xfer invalid count left")
            },
            Self::HostDebugstopCopyInvalidCountLeft => {
                write!(f, "host debug stop copy invalid count left")
            },
            Self::HostDebugstopXferInvalidFirstClusterId => {
                write!(f, "host debug stop xfer invalid first cluster ID")
            },
            Self::HostDebugstopXferInvalidRemainingCnt => {
                write!(f, "host debug stop xfer invalid remaining count")
            },
            Self::HostDebugstopXferUnexpectedAwCount => {
                write!(f, "host debug stop xfer unexpected AW count")
            },
            Self::HostDebugstopInvalidXferForWait => {
                write!(f, "host debug stop invalid xfer for wait")
            },
            Self::HostDebugstopUnexpectedXferOperation => {
                write!(f, "host debug stop unexpected xfer operation")
            },
            Self::HostDebugstopInvalidSdCountUpdateReq => {
                write!(f, "host debug stop invalid SD count update request")
            },
            Self::HostDebugstopEndProtocolUnmaskedUnexpectedly => {
                write!(f, "host debug stop end protocol unmasked unexpectedly")
            },
            Self::HostDebugstopUdmaModeOutOfRange => {
                write!(f, "host debug stop UDMA mode out of range")
            },
            Self::HostDebugstopUnexpectedHbiIntsUnmasked => {
                write!(f, "host debug stop unexpected HBI interrupts unmasked")
            },
            Self::HostDebugstopMultipleSdMultiModeXfer => {
                write!(f, "host debug stop multiple SD multi mode xfer")
            },
            Self::HostDebugstopAwXferPtrMisaligned => {
                write!(f, "host debug stop AW xfer pointer misaligned")
            },
            Self::HostDebugstopXferNotSupportedWithoutPermovl => {
                write!(f, "host debug stop xfer not supported without permovl")
            },
            Self::HostDebugstopXferBufferNot32BitAligned => {
                write!(f, "host debug stop xfer buffer not 32 bit aligned")
            },
            Self::HostDebugstopXferBufferNotSectorAligned => {
                write!(f, "host debug stop xfer buffer not sector aligned")
            },
            Self::SocDebugstopSubsegmentOutOfRange => {
                write!(f, "SOC debug stop subsegment out of range")
            },
            Self::SocDebugstopInvalidXferOffset => write!(f, "SOC debug stop invalid xfer offset"),
            Self::HostDebugstopCdRequired => write!(f, "host debug stop CD required"),
            Self::HostDebugstopInvalidNumBytesForDmaCopy => {
                write!(f, "host debug stop invalid number bytes for DMA copy")
            },
            Self::HostDebugstopInvalidAtaStatus => write!(f, "host debug stop invalid ATA status"),
            Self::HostDebugstopInvalidXferWaitCallback => {
                write!(f, "host debug stop invalid xfer wait callback")
            },
            Self::SatabridgeSocUartWriteError => write!(f, "SATA bridge SOC UART write error"),
            Self::SatabridgeSocUartReadError => write!(f, "SATA bridge SOC UART read error"),
            Self::SataInterfaceStuckInPartial => write!(f, "SATA interface stuck in partial"),
            Self::SataInterfaceStuckInSlumber => write!(f, "SATA interface stuck in slumber"),
            Self::SataUnhandledErrorInterrupt => write!(f, "SATA unhandled error interrupt"),
            Self::SataTempIcrcDebugstop => write!(f, "SATA temperature interface CRC debug stop"),
            Self::HostDebugstopCmdAbortedByReset => {
                write!(f, "host debug stop command aborted by reset")
            },
            Self::HostDebugstopCmdReparseError => {
                write!(f, "host debug stop command reparse error")
            },
            Self::HostDebugstopResetTimeout => write!(f, "host debug stop reset timeout"),
            Self::HostDebugstopInvalidTrexPattern1 => {
                write!(f, "host debug stop invalid TREX pattern1")
            },
            Self::HostDebugstopInvalidTrexPattern2 => {
                write!(f, "host debug stop invalid TREX pattern2")
            },
            Self::HostDebugstopInvalidTrexPattern3 => {
                write!(f, "host debug stop invalid TREX pattern3")
            },
            Self::HostDebugstopUnexpectedCommandTimeout => {
                write!(f, "host debug stop unexpected command timeout")
            },
            Self::HostDebugstopCommandTimeoutNotEnabled => {
                write!(f, "host debug stop command timeout not enabled")
            },
            Self::HostDebugstopCommandTimerAlreadyEnabled => {
                write!(f, "host debug stop command timer already enabled")
            },
            Self::HostDebugstopCmdTimeoutError => {
                write!(f, "host debug stop command timeout error")
            },
            Self::HostDebugstopInvalidQueuedInterrupt => {
                write!(f, "host debug stop invalid queued interrupt")
            },
            Self::HostDebugstopNextqPtrInvalid => {
                write!(f, "host debug stop next queue pointer invalid")
            },
            Self::HostDebugstopCpulockUnexpected => write!(f, "host debug stop cpulock unexpected"),
            Self::HostDebugstopQcdNotPresent => write!(f, "host debug stop QCD not present"),
            Self::HostDebugstopQramEmpty => write!(f, "host debug stop QRAM empty"),
            Self::HostDebugstopFuaQueuedWriteUnsupported => {
                write!(f, "host debug stop FUA queued write unsupported")
            },
            Self::IsrQueueHbi => write!(f, "interrupt service routine queue HBI"),
            Self::HalNcqDebugstopSactiveFisTimeout => write!(
                f,
                "hardware abstraction layer native command queuing debug stop sactive frame \
                 information structure timeout"
            ),
            Self::IntrWorkaroundFail => write!(f, "interrupt workaround fail"),
            Self::NcqNextqNonZeroWhenQueueFull => write!(
                f,
                "native command queuing next queue non zero when queue full"
            ),
            Self::LaJolla30Broken => write!(f, "la jolla 3 0 broken"),
            Self::HalNcqSendSdbfisWhileQstateIsNotActive => write!(
                f,
                "hardware abstraction layer native command queuing send Set Device Bits FIS while \
                 queue state is not active"
            ),
            Self::HalNcqDebugstopQramActiveBitNotSet => write!(
                f,
                "hardware abstraction layer native command queuing debug stop QRAM active bit not \
                 set"
            ),
            Self::HalNcqSdbFisTimeout => write!(
                f,
                "hardware abstraction layer native command queuing Set Device Bits frame \
                 information structure timeout"
            ),
            Self::HostDebugstopQramExecutionEnable => {
                write!(f, "host debug stop QRAM execution enable")
            },
            Self::StreamLogErrorSdNotPresent => write!(f, "stream log error SD not present"),
            Self::StreamLogErrorCdNotPresent => write!(f, "stream log error CD not present"),
            Self::DepopInvalidFileId => write!(f, "depop invalid file ID"),
            Self::DepopUnexpectedFileId => write!(f, "depop unexpected file ID"),
            Self::TmDebugstopSlippedAbaOvfl => {
                write!(f, "TM debug stop slipped absolute block address overflow")
            },
            Self::FmtDebugstopNewBadTrkWriteDriveQuick => {
                write!(f, "format debug stop new bad track write drive quick")
            },
            Self::FmtDebugstopNewBadTrkWriteDrive => {
                write!(f, "format debug stop new bad track write drive")
            },
            Self::FmtDebugstopIllegalRequestSize => {
                write!(f, "format debug stop illegal request size")
            },
            Self::FmtDebugstopFormatUnitFatalError => {
                write!(f, "format debug stop format unit fatal error")
            },
            Self::DmVirHeadOutOfRange => write!(f, "DM virtual head out of range"),
            Self::FmtDebugstopIllegalWdWaRequest => {
                write!(f, "format debug stop illegal WD WA request")
            },
            Self::FmtDebugstopIllegalWdqWaRequest => {
                write!(f, "format debug stop illegal WDQ WA request")
            },
            Self::DmNotEnoughMemoryAlloc => write!(f, "DM not enough memory allocate"),
            Self::FmtRsvdCylinderInGlist => write!(f, "format reserved cylinder in G-list"),
            Self::FmtDebugstopIllegalWdqWaRequest1 => {
                write!(f, "format debug stop illegal WDQ WA request 1")
            },
            Self::FmtDebugstopIllegalWdqWaRequest2 => {
                write!(f, "format debug stop illegal WDQ WA request 2")
            },
            Self::FmtDebugstopIllegalWdqWaRequest3 => {
                write!(f, "format debug stop illegal WDQ WA request 3")
            },
            Self::DmAbaOutOfRange => write!(f, "DM absolute block address out of range"),
            Self::DmPsnOutOfRange => write!(f, "DM physical sector number out of range"),
            Self::TmDebugstopPushdownWedgeOvfl => {
                write!(f, "TM debug stop pushdown wedge overflow")
            },
            Self::DmDebugstopCyloffsetOutOfRange => {
                write!(f, "DM debug stop cyloffset out of range")
            },
            Self::FmtPlistFull => write!(f, "format P-list full"),
            Self::TmDebugstopTooManyPushdownGroups => {
                write!(f, "TM debug stop too many pushdown groups")
            },
            Self::TranslationBlockInfoBadHead => write!(f, "translation block info bad head"),
            Self::FmtStartLbaOutOfSequence => write!(f, "format start LBA out of sequence"),
            Self::DmInvalidHeadSkewCalculation => write!(f, "DM invalid head skew calculation"),
            Self::DmClistFull => write!(f, "DM clist full"),
            Self::FmtRsvdRplistPsnOutOfRange => write!(
                f,
                "format reserved rplist physical sector number out of range"
            ),
            Self::DmInvalidReverseSearch => write!(f, "DM invalid reverse search"),
            Self::TranslationBadDataTrack => write!(f, "translation bad data track"),
            Self::DmTranslationTestFail => write!(f, "DM translation test fail"),
            Self::CaDebugstopDeleteBadSdIndex => write!(f, "cache debug stop delete bad SD index"),
            Self::CaDebugstopInsertBadSdIndex => write!(f, "cache debug stop insert bad SD index"),
            Self::CaDebugstopInsertTableFull => write!(f, "cache debug stop insert table full"),
            Self::CaDebugstopSectorCountBeyongRange => {
                write!(f, "cache debug stop sector count beyond range")
            },
            Self::CaDebugstopSequentialStreamBadParameter => {
                write!(f, "cache debug stop sequential stream bad parameter")
            },
            Self::CaDebugstopCmdOpcodeForXfermode => {
                write!(f, "cache debug stop command opcode for xfermode")
            },
            Self::CaDebugstopInvalidSequentialState => {
                write!(f, "cache debug stop invalid sequential state")
            },
            Self::CaDebugstopSequentialWriteNoMatchingSd => {
                write!(f, "cache debug stop sequential write no matching SD")
            },
            Self::CaDebugstopUnexpectedStateWriteFullHit => {
                write!(f, "cache debug stop unexpected state write full hit")
            },
            Self::CaDebugstopUnexpectedOpcode => write!(f, "cache debug stop unexpected opcode"),
            Self::CaDebugstopClearHostDataOutOfRange => {
                write!(f, "cache debug stop clear host data out of range")
            },
            Self::CaDebugstopMismatchedSequentialStream => {
                write!(f, "cache debug stop mismatched sequential stream")
            },
            Self::CaDebugstopUnexpectedSeqServiceType => {
                write!(f, "cache debug stop unexpected sequence service type")
            },
            Self::CaDebugstopMismatchedDeleteSdIndex => {
                write!(f, "cache debug stop mismatched delete SD index")
            },
            Self::CaDebugstopSetHostDataOutOfRange => {
                write!(f, "cache debug stop set host data out of range")
            },
            Self::CaDebugstopReadWriteCmdRscAllocFailed => {
                write!(f, "cache debug stop read write command RSC allocate failed")
            },
            Self::CaDebugstopUnexpectedCancelWhileFlushing => {
                write!(f, "cache debug stop unexpected cancel while flushing")
            },
            Self::CaDebugstopCacheoffWrongReturnState => {
                write!(f, "cache debug stop cacheoff wrong return state")
            },
            Self::CaDebugstopUnexpectedReturnSdHitInfoCall => {
                write!(f, "cache debug stop unexpected return SD hit info call")
            },
            Self::CaDebugstopSdScanIndexOutOfRange => {
                write!(f, "cache debug stop SD scan index out of range")
            },
            Self::CaDebugstopUnexpectedSdState => write!(f, "cache debug stop unexpected SD state"),
            Self::CaDebugstopInvalidCache => write!(f, "cache debug stop invalid cache"),
            Self::CaDebugstopNoCdOrSdAttached => write!(f, "cache debug stop no CD or SD attached"),
            Self::CaDebugstopUnexpectedReadScanState => {
                write!(f, "cache debug stop unexpected read scan state")
            },
            Self::CaDebugstopUnexpectedWriteScanState => {
                write!(f, "cache debug stop unexpected write scan state")
            },
            Self::CaDebugstopUnexpectedHostXferState => {
                write!(f, "cache debug stop unexpected host xfer state")
            },
            Self::CaDebugstopBadAggregationCount => {
                write!(f, "cache debug stop bad aggregation count")
            },
            Self::CaDebugstopMissingSdResource => write!(f, "cache debug stop missing SD resource"),
            Self::CaDebugstopMissingBufferResource => {
                write!(f, "cache debug stop missing buffer resource")
            },
            Self::CaDsssDebugstopCannotFindFragmentSd => {
                write!(f, "cache DSSS debug stop cannot find fragment SD")
            },
            Self::CaDebugstopInvalidMaxPotentialLba => {
                write!(f, "cache debug stop invalid max potential LBA")
            },
            Self::CaDebugstopInvalidCoherencyCmd => {
                write!(f, "cache debug stop invalid coherency command")
            },
            Self::CaDebugstopInvalidOverlapCount => {
                write!(f, "cache debug stop invalid overlap count")
            },
            Self::CaDebugstopInvalidCoherencySdState => {
                write!(f, "cache debug stop invalid coherency SD state")
            },
            Self::CaDebugstopInvalidPostReadCount => {
                write!(f, "cache debug stop invalid post read count")
            },
            Self::CaDebugstopInvalidPreReadCount => {
                write!(f, "cache debug stop invalid pre read count")
            },
            Self::CaDebugstopInvalidCoherencySdIndex => {
                write!(f, "cache debug stop invalid coherency SD index")
            },
            Self::DrvNotParkedBlndwr => write!(f, "drive not parked blind write"),
            Self::DxdwcsIndexSkirting => write!(f, "DXDWCS index skirting"),
            Self::UbadOn => write!(f, "UBAD on"),
            Self::MrerrDebugstopGenBimodalProc => write!(f, "MRERR debug stop gen bimodal process"),
            Self::ReadChannelTimeout => write!(f, "read channel timeout"),
            Self::ErInvalidTableRowSize => write!(f, "ER invalid table row size"),
            Self::ErInvalidRepeatCount => write!(f, "ER invalid repeat count"),
            Self::ErInvalidRecoveryPath => write!(f, "ER invalid recovery path"),
            Self::ErInvalidTableDirectoryEntry => write!(f, "ER invalid table directory entry"),
            Self::UtilInvalidTrexPattern => write!(f, "util invalid TREX pattern"),
            Self::FwEccAllCrcBytesAreZero => write!(f, "FW ECC all CRC bytes are zero"),
            Self::FwEccNeedNewCodeForNewSecSize => {
                write!(f, "FW ECC need new code for new sector size")
            },
            Self::FwEccBufferXferFailure => write!(f, "FW ECC buffer xfer failure"),
            Self::FwEccBufferReadFailure => write!(f, "FW ECC buffer read failure"),
            Self::FwEccCorrectedSectorNotTransferred => {
                write!(f, "FW ECC corrected sector not transferred")
            },
            Self::ErDfhLevelsError => write!(f, "ER DFH levels error"),
            Self::ErInvalidVmmCount => write!(f, "ER invalid virtual memory manager count"),
            Self::ErInvalidLctReadTaCount => {
                write!(f, "ER invalid LCT read thermal asperity count")
            },
            Self::ErInvalidLctReadNontaCount => {
                write!(f, "ER invalid LCT read non thermal asperity count")
            },
            Self::ErInvalidLctWriteCount => write!(f, "ER invalid LCT write count"),
            Self::ErInvalidLctIndexCount => write!(f, "ER invalid LCT index count"),
            Self::VscProtocolViolation => write!(f, "vendor specific command protocol violation"),
            Self::VscPstDebugstop => {
                write!(f, "vendor specific command Process Self-Test debug stop")
            },
            Self::VscTooManyCopies => write!(f, "vendor specific command too many copies"),
            Self::VscFieldlistTempBuffAccess => write!(
                f,
                "vendor specific command fieldlist temperature buffer access"
            ),
            Self::VscWrite2tBufferAlocFailed => {
                write!(f, "vendor specific command write 2t buffer allocate failed")
            },
            Self::VscCompareidsThreadNotAllocated => {
                write!(f, "vendor specific command compareids thread not allocated")
            },
            Self::VscErrInjectionErrType => {
                write!(f, "vendor specific command error injection error type")
            },
            Self::VscErrInjectionFuncCode => {
                write!(f, "vendor specific command error injection function code")
            },
            Self::VscLowLevelCommandThreadNotAllocated => write!(
                f,
                "vendor specific command low level command thread not allocated"
            ),
            Self::VscMaxAllocationReached => {
                write!(f, "vendor specific command max allocation reached")
            },
            Self::VscIeiInvalidCounter => write!(
                f,
                "vendor specific command internal error injection invalid counter"
            ),
            Self::VscInitSdAllocationFailed => {
                write!(f, "vendor specific command init SD allocation failed")
            },
            Self::VscForceDebugstop => write!(f, "vendor specific command force debug stop"),
            Self::XferDebugstopUnsupportedOpCode => {
                write!(f, "transfer debug stop unsupported op code")
            },
            Self::XferDebugstopUnexpectedAdditonalXdRequest => {
                write!(f, "transfer debug stop unexpected additional XD request")
            },
            Self::XferDebugstopInvalidXferReqCnt => {
                write!(f, "transfer debug stop invalid xfer request count")
            },
            Self::XferDebugstopCannotLockThreadRequired => {
                write!(f, "transfer debug stop cannot lock thread required")
            },
            Self::XferDebugstopTaskNotIdleDuringRv => write!(
                f,
                "transfer debug stop task not idle during rotational vibration"
            ),
            Self::XferDebugstopInvalidInsertEvent => {
                write!(f, "transfer debug stop invalid insert event")
            },
            Self::XferDebugstopDataNotAvailForXfer => {
                write!(f, "transfer debug stop data not available for xfer")
            },
            Self::HalBmDebugstopTransferAddrNotFound => write!(
                f,
                "hardware abstraction layer BM debug stop transfer address not found"
            ),
            Self::HalBmDebugstopSubsegmentNotAvailable => write!(
                f,
                "hardware abstraction layer BM debug stop subsegment not available"
            ),
            Self::HalBmDebugstopInvalidThreadNum => write!(
                f,
                "hardware abstraction layer BM debug stop invalid thread number"
            ),
            Self::HalBmDebugstopThreadNotAllocated => write!(
                f,
                "hardware abstraction layer BM debug stop thread not allocated"
            ),
            Self::HalBmDebugstopLostSomeSubsegments => write!(
                f,
                "hardware abstraction layer BM debug stop lost some subsegments"
            ),
            Self::HalBmDebugstopCurrentClusterNotFound => write!(
                f,
                "hardware abstraction layer BM debug stop current cluster not found"
            ),
            Self::HalBmDebugstopCurrentSdNotFound => write!(
                f,
                "hardware abstraction layer BM debug stop current SD not found"
            ),
            Self::HalBmDebugstopNonReentrantCodeExecuted => write!(
                f,
                "hardware abstraction layer BM debug stop non reentrant code executed"
            ),
            Self::XferDebugstopMultiXferInPriorityQueue => {
                write!(f, "transfer debug stop multi xfer in priority queue")
            },
            Self::XferDebugstopUnexpectedPAvailable => {
                write!(f, "transfer debug stop unexpected p available")
            },
            Self::XferDebugstopAwNotExpected => write!(f, "transfer debug stop AW not expected"),
            Self::XferDebugstopIntrudingNonlba => write!(f, "transfer debug stop intruding nonlba"),
            Self::XferDebugstopNonlbaThreadInvalid => {
                write!(f, "transfer debug stop nonlba thread invalid")
            },
            Self::XferDebugstopUnexpectedFwMsg => {
                write!(f, "transfer debug stop unexpected FW message")
            },
            Self::XferDebugstopUnexpectedMultiDoneMsg => {
                write!(f, "transfer debug stop unexpected multi done message")
            },
            Self::XferDebugstopUnknownFwMsgType => {
                write!(f, "transfer debug stop unknown FW message type")
            },
            Self::XferDebugstopNextXferTdNotPresent => {
                write!(f, "transfer debug stop next xfer TD not present")
            },
            Self::XferDebugstopUnexpectedAwDone => {
                write!(f, "transfer debug stop unexpected AW done")
            },
            Self::XferDebugstopUnexpectedNonLbaXferDone => {
                write!(f, "transfer debug stop unexpected non LBA xfer done")
            },
            Self::XferDebugstopUnknownXferMsgType => {
                write!(f, "transfer debug stop unknown xfer message type")
            },
            Self::XferDebugstopXferError => write!(f, "transfer debug stop xfer error"),
            Self::XferDebugstopEmptyFuaQueue => write!(f, "transfer debug stop empty FUA queue"),
            Self::XferDebugstopEocReqInMultiState => {
                write!(f, "transfer debug stop EOC request in multi state")
            },
            Self::XferDebugstopTdNotInQueue => write!(f, "transfer debug stop TD not in queue"),
            Self::XferDebugstopAwXfrMsg => write!(f, "transfer debug stop AW transfer message"),
            Self::XferDebugstopFuaWriteMsg => write!(f, "transfer debug stop FUA write message"),
            Self::XferDebugstopNonpNotAvailable => {
                write!(f, "transfer debug stop nonp not available")
            },
            Self::XferDebugstopPNotAvailable => write!(f, "transfer debug stop p not available"),
            Self::IpmDebugstopCommandIntervalQueueOverflow => write!(
                f,
                "interface power management debug stop command interval queue overflow"
            ),
            Self::IpmDebugstopCommandBinOverflow => write!(
                f,
                "interface power management debug stop command bin overflow"
            ),
            Self::DlgDebugstopTestTrackTranslationFailed => {
                write!(f, "DLG debug stop test track translation failed")
            },
            Self::DlgDebugstopInvalidWarehouseIndex => {
                write!(f, "DLG debug stop invalid warehouse index")
            },
            Self::Unknown(x) => write!(f, "unknown {x:#x}"),
        }
    }
}

impl From<DebugStopCode> for u32 {
    fn from(value: DebugStopCode) -> Self {
        match value {
            DebugStopCode::Unknown(x) => x,
            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

impl DebugStopCode {
    /// Parse Debug Stop code.
    pub(crate) fn parse(value: u32) -> Option<Self> {
        const CONST_VARIANTS: &[DebugStopCode] = &[
            DebugStopCode::OsError,
            DebugStopCode::OsNestedInterrupt,
            DebugStopCode::OsUndefInstruction,
            DebugStopCode::OsPrefetchAbort,
            DebugStopCode::OsDataAbort,
            DebugStopCode::OsStackOverflow,
            DebugStopCode::OsIrqStackOverflow,
            DebugStopCode::OsFlagSetFromFiq,
            DebugStopCode::OsUnmatchedExitTaskUnsafe,
            DebugStopCode::OsNullPtrAssignment,
            DebugStopCode::OsTaskUnsafeTimeout,
            DebugStopCode::OsPendInvalidGroup,
            DebugStopCode::OsPostInvalidGroup,
            DebugStopCode::OsQueryInvalidGroup,
            DebugStopCode::CrtExit,
            DebugStopCode::CrtRaise,
            DebugStopCode::MemcpyIntoCode,
            DebugStopCode::MemcpyUnalignedSrcPtr,
            DebugStopCode::MemcpyUnalignedDstPtr,
            DebugStopCode::MemsetUnalignedDstPtr,
            DebugStopCode::InvalidSwi,
            DebugStopCode::FifoCollisionFwTimeout,
            DebugStopCode::CpuHdcWaitTimeout,
            DebugStopCode::JumpToZero,
            DebugStopCode::MemsetIntoCode,
            DebugStopCode::MemclrIntoCode,
            DebugStopCode::MemmoveIntoCode,
            DebugStopCode::DriveErrHistLog,
            DebugStopCode::GetExtErr,
            DebugStopCode::ClrPeriodLog,
            DebugStopCode::SetSelfTestTimer,
            DebugStopCode::ReloCodeToDrmCode,
            DebugStopCode::UpdateDrvLifeTimeErrCounters,
            DebugStopCode::SelftestScan,
            DebugStopCode::SelftestRdWr,
            DebugStopCode::ServocoreInitWrongCore,
            DebugStopCode::ServocoreInitPremature,
            DebugStopCode::OvlGuardUndefinedFn,
            DebugStopCode::PmInvalidModeSel,
            DebugStopCode::PmInvalidHistoryIndex,
            DebugStopCode::PmInvalidDcvStateAtDiskSeek,
            DebugStopCode::PmInvalidDcvStateAtDiskExit,
            DebugStopCode::PmInvalidDcvStateEnteringSleep,
            DebugStopCode::PmInvalidDcvStateExitingSleep,
            DebugStopCode::PmInvalidVoltageRequest,
            DebugStopCode::UnsupportedSoc,
            DebugStopCode::IsrHbi,
            DebugStopCode::IsrDf,
            DebugStopCode::IsrBm,
            DebugStopCode::IsrHbiTonescan,
            DebugStopCode::IsrHbiFifoErr,
            DebugStopCode::IsrWatchdogTimeout,
            DebugStopCode::IsrExternalTrigger,
            DebugStopCode::DiskUnhandledDfError,
            DebugStopCode::DiskInvalidSdChain,
            DebugStopCode::DiskInvalidInsert,
            DebugStopCode::DiskInvalidCallForward,
            DebugStopCode::DiskInvalidOpcode,
            DebugStopCode::DiskNoCddPresent,
            DebugStopCode::DiskServicesInvalidOpcode,
            DebugStopCode::InvalidTdParam,
            DebugStopCode::DiskHardwareCheckHang,
            DebugStopCode::CorruptTrackStatus,
            DebugStopCode::DiskInvalidBfrXferPtr,
            DebugStopCode::DiskSdNotPresent,
            DebugStopCode::NoMoreSeekSetupStructs,
            DebugStopCode::InvalidServoEvent,
            DebugStopCode::DiskInvalidSpinupEvent,
            DebugStopCode::DiskInvalidHeadNumber,
            DebugStopCode::DiskDebugstopUnsupportedRequest,
            DebugStopCode::DiskDebugstopInvalidExtendSd,
            DebugStopCode::DiskDebugstopInvalidExtendTrackInfo,
            DebugStopCode::DiskDebugstopWriteExtendNotSequential,
            DebugStopCode::DiskDebugstopInvalidWedgeDataSize,
            DebugStopCode::DiskInvalidExtendRequest,
            DebugStopCode::DiskSpinupTimeoutEvent,
            DebugStopCode::DiskInvalidUninitCdd,
            DebugStopCode::DiskTonescanDefectBufferEmpty,
            DebugStopCode::DiskTonescanBadDefectiveWedgeCount,
            DebugStopCode::DiskDebugstopMaxLsnMismatch,
            DebugStopCode::DiskInvalidHeadCountFromCh,
            DebugStopCode::DiskInvalidLoopcountRegValue,
            DebugStopCode::DiskDebugstopDfIsActiveAfterDfStop,
            DebugStopCode::DiskInvalidCompletionCount,
            DebugStopCode::DiskInvalidWedgeCommand,
            DebugStopCode::DiskInvalidAddrToRcUpdateInSpinup,
            DebugStopCode::FmDebugstopWrongMode,
            DebugStopCode::FmDebugstopInvalidOpcode,
            DebugStopCode::FmDebugstopWrongCallingTask,
            DebugStopCode::FmCopyTempToPermFailed,
            DebugStopCode::FmDiskTaskNotCalled,
            DebugStopCode::FmTooManyFlashFiles,
            DebugStopCode::FmDlmcOpenFailed,
            DebugStopCode::FmDebugstopNotEnoughFmds,
            DebugStopCode::FmDebugstopFileIdsDontMatch,
            DebugStopCode::FmDebugstopAlreadyFreedFmd,
            DebugStopCode::FmDebugstopFmdIndexOverwritten,
            DebugStopCode::FmDebugstopInvalidOrigin,
            DebugStopCode::FmDebugstopInvalidOffset,
            DebugStopCode::FmDebugstopInvalidLbaOffset,
            DebugStopCode::FmDebugstopRequestCountTooBig,
            DebugStopCode::FmDebugstopPartialOnFlashFiles,
            DebugStopCode::FmDebugstopTooManyDirtyFiles,
            DebugStopCode::FmDebugstopNumCopyMoreThenMaxCopy,
            DebugStopCode::FmOpenEmptySlotNoCanDo,
            DebugStopCode::FmFmdIndexExceedAllocatedFmdSpace,
            DebugStopCode::FmOpenZipcodeDlgFileFail,
            DebugStopCode::DiskDebugstopUnexpectedEndOfSdGroup,
            DebugStopCode::DiskDebugstopUnexpectedSdValues,
            DebugStopCode::DiskDebugstopNextListOutOfRange,
            DebugStopCode::DiskDebugstopUnexpectedRemCntValues,
            DebugStopCode::DiskDebugstopInvalidCompletedCnt,
            DebugStopCode::DiskDebugstopInvalidReadOperation,
            DebugStopCode::DiskDebugstopInvalidSscAdjustment,
            DebugStopCode::DiskDebugstopMismatchForFirstSdLoaded,
            DebugStopCode::DiskDebugstopInvalidSdChain,
            DebugStopCode::DiskDebugstopMismatchTotalArmCnt,
            DebugStopCode::DiskDebugstopInvalidRemainingArmCnt,
            DebugStopCode::DiskDebugstopWriteBackupSizeGreaterThanDiskSegmentSize,
            DebugStopCode::DiskDebugstopWcsStillRunningOnCheckDiskWrite,
            DebugStopCode::DiskDebugstopFormatterTimeoutOnRead,
            DebugStopCode::DiskDebugstopFormatterTimeoutOnWrite,
            DebugStopCode::DiskDebugstopNoBufferPresent,
            DebugStopCode::DiskDebugstopSegEndAddrInvalid,
            DebugStopCode::DiskDebugstopSdCountNotZeroForRead,
            DebugStopCode::DiskDebugstopPseudoSetSscTooLarge,
            DebugStopCode::DiskDebugstopSscNotEqualToRemReqCnt,
            DebugStopCode::DiskDebugstopDabRemReqCntUnderflow,
            DebugStopCode::DiskDebugstopUnexpectedErrorStatus,
            DebugStopCode::DiskDebugstopIllegalCylSeek,
            DebugStopCode::DiskDebugstopIllegalHeadSeek,
            DebugStopCode::DiskDebugstopReadExtendNeedsConnectToHost,
            DebugStopCode::DiskDebugstopNoSdAttached,
            DebugStopCode::DiskDebugstopNoCdAttached,
            DebugStopCode::DiskDebugstopSeekNotComplete,
            DebugStopCode::DiskDebugstopReadOverflowSizeGreaterThanDiskSegmentSize,
            DebugStopCode::DiskDebugstopWrongSeekIssued,
            DebugStopCode::DiskDebugstopLinkedSequentialThread,
            DebugStopCode::DiskDebugstopHangingDiskEvents,
            DebugStopCode::DiskDebugstopNegativeRemRequestCount,
            DebugStopCode::DiskDebugstopInvalidTrexPattern1,
            DebugStopCode::DiskDebugstopInvalidTrexPattern2,
            DebugStopCode::DiskDebugstopInvalidTrexPattern3,
            DebugStopCode::DiskDebugstopNoErrorFromGatherstatus,
            DebugStopCode::DiskDebugstopServoEventTimeout1,
            DebugStopCode::DiskDebugstopServoEventTimeout2,
            DebugStopCode::DiskDebugstopCacheMemoryTestFailed,
            DebugStopCode::DiskDebugstopMismatchedRemReqCnt,
            DebugStopCode::DiskDebugstopPsnLsnMismatch,
            DebugStopCode::DiskDebugstopServoTimeout,
            DebugStopCode::DiskDebugstopServoWrongHeadSelected,
            DebugStopCode::DiskDebugstopErBadTranslation,
            DebugStopCode::DiskDebugstopNoDdAttached,
            DebugStopCode::DiskDebugstopMisalignedXferPtr,
            DebugStopCode::DiskDebugstopUnexpectedSegmentCnt,
            DebugStopCode::DiskDebugstopBadTestTrack,
            DebugStopCode::DiskDebugstopFindmaxlbaNoCurTd,
            DebugStopCode::DiskDebugstopFindmaxlbaInvalidReq,
            DebugStopCode::DiskDebugstopXferWakeAtLbaWrite,
            DebugStopCode::DiskDebugstopIllegalCmdForHeadOfQueue,
            DebugStopCode::DiskDebugstopIllegalQueueDepth,
            DebugStopCode::DiskDebugstopAfterCancelStillVeryBusy,
            DebugStopCode::DiskDebugstopQueueGreaterThanOne,
            DebugStopCode::DiskDebugstopSequentialWithLinkedSds,
            DebugStopCode::DiskDebugstopInvalidWedgeDownCounterCalculated,
            DebugStopCode::DiskDebugstopInvalidWedgeDownCounterDetectedWhenDecrement,
            DebugStopCode::DiskDebugstopInvalidWedgeDownCounterDetectedWhenAdjust,
            DebugStopCode::DiskDebugstopSeekLatencyTableNotSupported,
            DebugStopCode::DiskDebugstopReadThreadCountMismatch,
            DebugStopCode::DiskDebugstopReadErResumePtrIsNull,
            DebugStopCode::DiskDebugstopNoHeadSwitch,
            DebugStopCode::DiskDebugstopDsssUnalignedLba,
            DebugStopCode::DiskDebugstopInvalidTimerId,
            DebugStopCode::DiskDebugstopGetTempTimeoutExceeded,
            DebugStopCode::DiskDebugstopInvalidServoScanState,
            DebugStopCode::DiskDebugstopInvalidClusterIndex,
            DebugStopCode::DiskDebugstopArmingMishap,
            DebugStopCode::DiskDebugstopStillbusyArm,
            DebugStopCode::DiskDebugstopFirstLbaNotFound,
            DebugStopCode::DiskDebugstopZeroCountDecrmentAttmp,
            DebugStopCode::DiskDebugstopZeroCountWrtZipHigh,
            DebugStopCode::DiskDebugstopZeroCountWrtZipLow,
            DebugStopCode::DiskDebugstopZipFileWrtDuringFlush,
            DebugStopCode::DiskDebugstopGetTempInvalidTaskId,
            DebugStopCode::DiskDebugstopInvalidChannelScanState,
            DebugStopCode::DiskDebugstopReadOfftrackLimitsConflict,
            DebugStopCode::DiskDebugstopWriteOfftrackLimitsConflict,
            DebugStopCode::DiskDebugstopPredictOfftrackLimitsConflict,
            DebugStopCode::DiskDebugstopFineTrackoffsetsConflict,
            DebugStopCode::DfhDebugPreheatCheckRegCZero,
            DebugStopCode::DfhDebugDfStoppedDfhRegDChanged,
            DebugStopCode::DfhDebugDfStoppedDfhRegCChanged,
            DebugStopCode::DfhDebugStateMonitorBadPreheatCount,
            DebugStopCode::DfhDebugStateIdleBadPreheatCount,
            DebugStopCode::DfhDebugOnditionMetBadDfhState,
            DebugStopCode::ReloInvalidErrorStatus,
            DebugStopCode::ReloInvalidTdOpcode,
            DebugStopCode::ReloNoBadWedgeInfo,
            DebugStopCode::ReloSstNotPossibleOnRead,
            DebugStopCode::ReloSpareLbaWithNoUserLba,
            DebugStopCode::ReloInvalidToUseErDabs,
            DebugStopCode::ReloHandlerLostInSpace,
            DebugStopCode::ReloExperimentNoTaresAllowed,
            DebugStopCode::ReloExperimentNoRelosAllowed,
            DebugStopCode::ReloExperimentNoTaTaresAllowed,
            DebugStopCode::ReloExperimentNoRsvdBit3Allowed,
            DebugStopCode::ReloExperimentNoRsvdBit4Allowed,
            DebugStopCode::ReloExperimentNoRsvdBit5Allowed,
            DebugStopCode::ReloExperimentNoRsvdBit6Allowed,
            DebugStopCode::ReloExperimentNoRsvdBit7Allowed,
            DebugStopCode::ReloBadWedgeCfgOutOfRange,
            DebugStopCode::ApbReadError,
            DebugStopCode::InvalidApbChecksum,
            DebugStopCode::InvalidFormatSurfaceId,
            DebugStopCode::DabNestedDisconnect,
            DebugStopCode::DabConnectWithoutDisconnect,
            DebugStopCode::DiskDisconnectedOnCacheCmd,
            DebugStopCode::DiskIeiHalInvalidErrorType,
            DebugStopCode::DiskIeiInvalidCounter,
            DebugStopCode::RscDebugstopNoFreeCd,
            DebugStopCode::RscDebugstopNoFreeXd,
            DebugStopCode::RscDebugstopPutXdAlreadyFree,
            DebugStopCode::RscDebugstopNoFreeDd,
            DebugStopCode::RscDebugstopPutDdAlreadyFree,
            DebugStopCode::RscDebugstopNoFreeTd,
            DebugStopCode::RscDebugstopTdDiskQueueFull,
            DebugStopCode::RscDebugstopDequeueTdQueueEmpty,
            DebugStopCode::RscDebugstopEnqueueTdIndexUsed,
            DebugStopCode::RscDebugstopRemoveTdQueueEmpty,
            DebugStopCode::RscDebugstopRemoveTdBadIndex,
            DebugStopCode::RscDebugstopNoSdToDeallocate,
            DebugStopCode::RscDebugstopNoCdForVerifyBuffer,
            DebugStopCode::RscDebugstopPutTdAlreadyFree,
            DebugStopCode::RscDebugstopPutCdAlreadyFree,
            DebugStopCode::RscDebugstopInvalidSdState,
            DebugStopCode::RscDebugstopGetNullBufferAddressSd,
            DebugStopCode::RscDebugstopGetNullBufferAddressCd1,
            DebugStopCode::RscDebugstopGetNullBufferAddressCd2,
            DebugStopCode::RscDebugstopClusterChainTooLong,
            DebugStopCode::RscDebugstopAllocBuffersNotByExecTask,
            DebugStopCode::RscDebugstopAllocSdNotByExecTask,
            DebugStopCode::RscDebugstopGetNullBufferAddressDd1,
            DebugStopCode::RscDebugstopGetNullBufferAddressDd2,
            DebugStopCode::RscDebugstopInvalidLockCntState,
            DebugStopCode::RscDebugstopInvalidTotalClusterCount,
            DebugStopCode::RscDebugstopInvalidClusterIndex,
            DebugStopCode::RscDebugstopInvalidFreeClusterCount,
            DebugStopCode::RscDebugstopDeallocateFreeCluster,
            DebugStopCode::RscDebugstopAlloEmptyFreeClusterList,
            DebugStopCode::RscDebugstopAlloClusterNotInFreeList,
            DebugStopCode::RscDebugstopInitClustersNotAllFree,
            DebugStopCode::RscDebugstopPermAllocTooLarge,
            DebugStopCode::RscDebugstopClusterCountBeyondRange,
            DebugStopCode::RscDebugstopClusterCountNotEnough,
            DebugStopCode::RscDebugstopDlmcBufferTooSmall,
            DebugStopCode::RscDebugstopReqClustersBeyondCapacity,
            DebugStopCode::RscDebugstopUnableFlushClusters,
            DebugStopCode::RscDebugstopFlushClustersTimeout,
            DebugStopCode::RscDebugstopInvalidCdIndex,
            DebugStopCode::RscDebugstopSequentialSdInWrongState,
            DebugStopCode::RscDebugstopNoBufferRequestedFlagNotSet,
            DebugStopCode::RscDebugstopInvalidSdIndex,
            DebugStopCode::RscDebugstopInvalidTdIndex,
            DebugStopCode::RscDebugstopInvalidTdTaskId,
            DebugStopCode::RscDebugstopFreeSdInSeqStream,
            DebugStopCode::RscDebugstopSendToDiskWithSeqStream,
            DebugStopCode::RscDebugstopFlushWithSeqStream,
            DebugStopCode::RscDebugstopInvalidDdIndex,
            DebugStopCode::RscDebugstopInvalidXdIndex,
            DebugStopCode::RscDebugstopInvalidStateQcmdBitAndCdIndex,
            DebugStopCode::RscDebugstopInvalidTrimParameters,
            DebugStopCode::RscDebugstopTrimWithZeroBuffer,
            DebugStopCode::RscDebugstopInvalidInsertIndex,
            DebugStopCode::RscDebugstopAllocateCdWhenCdAlreadyAllocated,
            DebugStopCode::RscDebugstopAllocateDdWhenDdAlreadyAllocated,
            DebugStopCode::RscDebugstopAllocateXdWhenXdAlreadyAllocated,
            DebugStopCode::RscDebugstopCyclicSdStateqCheckFail,
            DebugStopCode::RscDebugstopCyclicSdClusterCheckFail,
            DebugStopCode::RscDebugstopCyclicClusterChainCheckFail,
            DebugStopCode::RscDebugstopCyclicFrClusterCheckFail,
            DebugStopCode::RscDebugstopTotalClCountCheckFail,
            DebugStopCode::RscDebugstopTotalSdCountCheckFail,
            DebugStopCode::RscDebugstopFellOffEndOfSdChain,
            DebugStopCode::RscDynamicAvailCountUnderflow,
            DebugStopCode::RscDynamicValidCountUnderflow,
            DebugStopCode::RscSdUnlockCallbackError1,
            DebugStopCode::RscSdUnlockCallbackError2,
            DebugStopCode::RscDebugstopBadXferReserveCnt,
            DebugStopCode::RscInvalidSdUnlock,
            DebugStopCode::HdaRealnumheadsInvalid,
            DebugStopCode::HdaDebugstopNoMoreSeekSetupStructs,
            DebugStopCode::HdaDebugstopDeallocateNullSeekSetupPtr,
            DebugStopCode::HdaDebugstopDeallocateFreeSeekSetupStructure,
            DebugStopCode::ServoApiDebugstopInvalidStateTransition,
            DebugStopCode::MrmDebugstopSubmitInvalidStateQueue,
            DebugStopCode::MrmDebugstopSubmitNoBuffer,
            DebugStopCode::MrmDebugstopRemoveInvalidStateQueue,
            DebugStopCode::MrmDebugstopSendtodiskInvalidStateQueue,
            DebugStopCode::MrmDebugstopSendtodiskNoBuffer,
            DebugStopCode::MrmDebugstopSendtodiskSdInGroup,
            DebugStopCode::MrmDebugstopFlushspecificNoBuffer,
            DebugStopCode::MrmDebugstopFlushspecificInvalidStateQueue,
            DebugStopCode::MrmDebugstopFlushspecificInvalidGroup,
            DebugStopCode::MrmDebugstopFilemgrInvalidStateQueue,
            DebugStopCode::MrmDebugstopAddtogroupInvalidGroup,
            DebugStopCode::MrmDebugstopTdWithoutCd,
            DebugStopCode::MrmDebugstopReleasecallbackWrongState,
            DebugStopCode::MrmDebugstopProcesscommandWrongState,
            DebugStopCode::MrmDebugstopDefaultcallbackWrongState,
            DebugStopCode::MrmDebugstopWedgeCountValidTimeout,
            DebugStopCode::MrmDebugstopFlushRpoChwNotValid,
            DebugStopCode::MrmStartTimedOut,
            DebugStopCode::MrmDebugstopInvalidOpMrmServiceRequest,
            DebugStopCode::MrmDebugstopInvalidDdCallbackState,
            DebugStopCode::MrmDebugstopTdWithoutSd,
            DebugStopCode::MrmUnexpectedSdInIndependentState,
            DebugStopCode::MrmUnexpectedSdWaitState,
            DebugStopCode::MrmDiskCancelQueueFailure,
            DebugStopCode::MrmDiskInvalidSdState,
            DebugStopCode::MrmDiskInvalidDdCount,
            DebugStopCode::MrmDiskOddLbaCountRequest,
            DebugStopCode::MrmInvalidDsssMergeDirtyOperation,
            DebugStopCode::MrmInvalidTaskId,
            DebugStopCode::RpoDebugstopPrereadXlatError,
            DebugStopCode::RpoDebugstopSeekProfileTableError,
            DebugStopCode::ExecDebugstopSectionStart,
            DebugStopCode::ExecDebugstopInvalidTdOpcode,
            DebugStopCode::ExecDebugstopInvalidFmRequest,
            DebugStopCode::ExecDebugstopCdRequired,
            DebugStopCode::ExecDlg2InvalidArea,
            DebugStopCode::ExecDlg2RscFailure,
            DebugStopCode::ExecDlg2InvalidRequestCount,
            DebugStopCode::ExecDlg2InvalidLba,
            DebugStopCode::ExecDlg2ReadSpbaError,
            DebugStopCode::ExecDlg2VerifySpbaError,
            DebugStopCode::ExecDlg2SectorCntBiggerThanConfigDefaultCnt,
            DebugStopCode::ExecInvalidFactorySelftestCustomerId,
            DebugStopCode::BgDebugstopDstRecordresultsInvalidSubact,
            DebugStopCode::BgDebugstopOlRecordresultsInvalidSubact,
            DebugStopCode::BgDebugstopInvalidCurrActivity,
            DebugStopCode::BgDebugstopInvalidTdOpcode,
            DebugStopCode::BgDebugstopInvalidTimerId,
            DebugStopCode::BgDebugstopInvalidBgState,
            DebugStopCode::BgDebugstopInvalidTestinfoStatus,
            DebugStopCode::BgDebugstopPstInvalidTrbIndex,
            DebugStopCode::BgDebugstopPstUnableToLoadSeqTable,
            DebugStopCode::BgDebugstopPstUnableToSaveSeqTable,
            DebugStopCode::BgDebugstopPstInvalidTestId,
            DebugStopCode::BgDebugstopPstInvalidStatus,
            DebugStopCode::BgDebugstopPstInterfaceVersionMismatch,
            DebugStopCode::BgDebugstopPstInvalidTrbSize,
            DebugStopCode::BgDebugstopTdAlreadyAllocated,
            DebugStopCode::BgDebugstopIsramChksumFailed,
            DebugStopCode::BgDebugstopRvaMrmStartTimedOut,
            DebugStopCode::BgDebugstopRvaRequiresCd,
            DebugStopCode::BgDebugstopRvaRequiresCdd,
            DebugStopCode::BgDebugstopRvaInvalidSd,
            DebugStopCode::BgDebugstopRvaIllegalRequestSize,
            DebugStopCode::BgDebugstopWaMrmStartTimedOut,
            DebugStopCode::BgDebugstopWaRequiresCd,
            DebugStopCode::BgDebugstopWaRequiresCdd,
            DebugStopCode::BgDebugstopWaRequestSizeTooLarge,
            DebugStopCode::BgDebugstopPreemptedUnderBsyProtect,
            DebugStopCode::BgDebugstopDramtestNullTestinfoPointer,
            DebugStopCode::BgDebugstopErrorcodeNullTestinfoPointer,
            DebugStopCode::BgDebugstopResfilecheckNullTestinfoPointer,
            DebugStopCode::BgDebugstopMemcheckNullTestinfoPointer,
            DebugStopCode::BgDebugstopTaNullTestinfoPointer,
            DebugStopCode::BgDebugstopScanNullTestinfoPointer,
            DebugStopCode::BgDebugstopPstStackCheckFailed,
            DebugStopCode::BgDebugstopWaIllegalZeroRequestSize,
            DebugStopCode::BgDebugstopWaAllLbaRequestSizeTooLarge,
            DebugStopCode::BgDebugstopReleaseResourceCallbackTimeout,
            DebugStopCode::BgDebugstopPstUnableToFlashPtm,
            DebugStopCode::BgDebugstopWaMissingReleaseResourceInfo,
            DebugStopCode::BgDebugstopTraceHang,
            DebugStopCode::BgDebugstopPstPostupFailure,
            DebugStopCode::BgDebugstopPstPermAllocationFailed,
            DebugStopCode::BgDebugstopInvalidIdleType,
            DebugStopCode::BgDebugstopCompidRequiresCd,
            DebugStopCode::BgDebugstopCompidRequiresCdd,
            DebugStopCode::BgDebugstopCompidRequestSizeTooLarge,
            DebugStopCode::BgDebugstopCompidMrmStartTimedOut,
            DebugStopCode::BgDebugstopCompidMrmRequestError,
            DebugStopCode::BgDebugstopCompidIllegalZeroRequestSize,
            DebugStopCode::BgDebugstopCompidLbaRequestSizeTooLarge,
            DebugStopCode::BgDebugstopDstRecordresultsInvalidTestlevel,
            DebugStopCode::BgDebugstopScanInvalidSubActiDstQuick,
            DebugStopCode::BgDebugstopScanInvalidSubActiDstExtended,
            DebugStopCode::BgDebugstopScanInvalidSubActiDstConveyance,
            DebugStopCode::BgDebugstopScanInvalidSubActiDstSelective,
            DebugStopCode::BgDebugstopDlgClrzipFailed,
            DebugStopCode::BgDebugstopRefreshLbaReturnCountInvalid,
            DebugStopCode::BgDebugstopDlgLowThresholdGreaterThanHighThreshold,
            DebugStopCode::BgDebugstopDlgIllegalSchedulerState,
            DebugStopCode::BgDebugstopDlgPstModeCtlrFailed,
            DebugStopCode::BgDebugstopRemainingLbaCountLessThanRefresherCount,
            DebugStopCode::BgDebugstopRefreshStartLbaGreaterThanMaxZipLba,
            DebugStopCode::BgDebugstopPstIndexOverrunStatic,
            DebugStopCode::BgDebugstopAtaIndexOverrunStatic,
            DebugStopCode::BgDebugstopWaIllegalRequestSize,
            DebugStopCode::ArDebugstopHsdtInvalidMappingResult,
            DebugStopCode::CrDebugstopReadWriteInProgress,
            DebugStopCode::CrDebugstopReadReloDoesNotExist,
            DebugStopCode::CrDebugstopWriteReloDoesNotExist,
            DebugStopCode::CrDebugstopSpareRwInvalidIndex,
            DebugStopCode::CrDebugstopReadingReloNeverWritten,
            DebugStopCode::CrDebugstopBufferAllocationFailed,
            DebugStopCode::CrDebugstopBufferNotAllocated,
            DebugStopCode::CrDebugstopUnableToAllocatePermanentSd,
            DebugStopCode::HostDebugstopXferInvalidCountLeft,
            DebugStopCode::HostDebugstopCopyInvalidCountLeft,
            DebugStopCode::HostDebugstopXferInvalidFirstClusterId,
            DebugStopCode::HostDebugstopXferInvalidRemainingCnt,
            DebugStopCode::HostDebugstopXferUnexpectedAwCount,
            DebugStopCode::HostDebugstopInvalidXferForWait,
            DebugStopCode::HostDebugstopUnexpectedXferOperation,
            DebugStopCode::HostDebugstopInvalidSdCountUpdateReq,
            DebugStopCode::HostDebugstopEndProtocolUnmaskedUnexpectedly,
            DebugStopCode::HostDebugstopUdmaModeOutOfRange,
            DebugStopCode::HostDebugstopUnexpectedHbiIntsUnmasked,
            DebugStopCode::HostDebugstopMultipleSdMultiModeXfer,
            DebugStopCode::HostDebugstopAwXferPtrMisaligned,
            DebugStopCode::HostDebugstopXferNotSupportedWithoutPermovl,
            DebugStopCode::HostDebugstopXferBufferNot32BitAligned,
            DebugStopCode::HostDebugstopXferBufferNotSectorAligned,
            DebugStopCode::SocDebugstopSubsegmentOutOfRange,
            DebugStopCode::SocDebugstopInvalidXferOffset,
            DebugStopCode::HostDebugstopCdRequired,
            DebugStopCode::HostDebugstopInvalidNumBytesForDmaCopy,
            DebugStopCode::HostDebugstopInvalidAtaStatus,
            DebugStopCode::HostDebugstopInvalidXferWaitCallback,
            DebugStopCode::SatabridgeSocUartWriteError,
            DebugStopCode::SatabridgeSocUartReadError,
            DebugStopCode::SataInterfaceStuckInPartial,
            DebugStopCode::SataInterfaceStuckInSlumber,
            DebugStopCode::SataUnhandledErrorInterrupt,
            DebugStopCode::SataTempIcrcDebugstop,
            DebugStopCode::HostDebugstopCmdAbortedByReset,
            DebugStopCode::HostDebugstopCmdReparseError,
            DebugStopCode::HostDebugstopResetTimeout,
            DebugStopCode::HostDebugstopInvalidTrexPattern1,
            DebugStopCode::HostDebugstopInvalidTrexPattern2,
            DebugStopCode::HostDebugstopInvalidTrexPattern3,
            DebugStopCode::HostDebugstopUnexpectedCommandTimeout,
            DebugStopCode::HostDebugstopCommandTimeoutNotEnabled,
            DebugStopCode::HostDebugstopCommandTimerAlreadyEnabled,
            DebugStopCode::HostDebugstopCmdTimeoutError,
            DebugStopCode::HostDebugstopInvalidQueuedInterrupt,
            DebugStopCode::HostDebugstopNextqPtrInvalid,
            DebugStopCode::HostDebugstopCpulockUnexpected,
            DebugStopCode::HostDebugstopQcdNotPresent,
            DebugStopCode::HostDebugstopQramEmpty,
            DebugStopCode::HostDebugstopFuaQueuedWriteUnsupported,
            DebugStopCode::IsrQueueHbi,
            DebugStopCode::HalNcqDebugstopSactiveFisTimeout,
            DebugStopCode::IntrWorkaroundFail,
            DebugStopCode::NcqNextqNonZeroWhenQueueFull,
            DebugStopCode::LaJolla30Broken,
            DebugStopCode::HalNcqSendSdbfisWhileQstateIsNotActive,
            DebugStopCode::HalNcqDebugstopQramActiveBitNotSet,
            DebugStopCode::HalNcqSdbFisTimeout,
            DebugStopCode::HostDebugstopQramExecutionEnable,
            DebugStopCode::StreamLogErrorSdNotPresent,
            DebugStopCode::StreamLogErrorCdNotPresent,
            DebugStopCode::DepopInvalidFileId,
            DebugStopCode::DepopUnexpectedFileId,
            DebugStopCode::TmDebugstopSlippedAbaOvfl,
            DebugStopCode::FmtDebugstopNewBadTrkWriteDriveQuick,
            DebugStopCode::FmtDebugstopNewBadTrkWriteDrive,
            DebugStopCode::FmtDebugstopIllegalRequestSize,
            DebugStopCode::FmtDebugstopFormatUnitFatalError,
            DebugStopCode::DmVirHeadOutOfRange,
            DebugStopCode::FmtDebugstopIllegalWdWaRequest,
            DebugStopCode::FmtDebugstopIllegalWdqWaRequest,
            DebugStopCode::DmNotEnoughMemoryAlloc,
            DebugStopCode::FmtRsvdCylinderInGlist,
            DebugStopCode::FmtDebugstopIllegalWdqWaRequest1,
            DebugStopCode::FmtDebugstopIllegalWdqWaRequest2,
            DebugStopCode::FmtDebugstopIllegalWdqWaRequest3,
            DebugStopCode::DmAbaOutOfRange,
            DebugStopCode::DmPsnOutOfRange,
            DebugStopCode::TmDebugstopPushdownWedgeOvfl,
            DebugStopCode::DmDebugstopCyloffsetOutOfRange,
            DebugStopCode::FmtPlistFull,
            DebugStopCode::TmDebugstopTooManyPushdownGroups,
            DebugStopCode::TranslationBlockInfoBadHead,
            DebugStopCode::FmtStartLbaOutOfSequence,
            DebugStopCode::DmInvalidHeadSkewCalculation,
            DebugStopCode::DmClistFull,
            DebugStopCode::FmtRsvdRplistPsnOutOfRange,
            DebugStopCode::DmInvalidReverseSearch,
            DebugStopCode::TranslationBadDataTrack,
            DebugStopCode::DmTranslationTestFail,
            DebugStopCode::CaDebugstopDeleteBadSdIndex,
            DebugStopCode::CaDebugstopInsertBadSdIndex,
            DebugStopCode::CaDebugstopInsertTableFull,
            DebugStopCode::CaDebugstopSectorCountBeyongRange,
            DebugStopCode::CaDebugstopSequentialStreamBadParameter,
            DebugStopCode::CaDebugstopCmdOpcodeForXfermode,
            DebugStopCode::CaDebugstopInvalidSequentialState,
            DebugStopCode::CaDebugstopSequentialWriteNoMatchingSd,
            DebugStopCode::CaDebugstopUnexpectedStateWriteFullHit,
            DebugStopCode::CaDebugstopUnexpectedOpcode,
            DebugStopCode::CaDebugstopClearHostDataOutOfRange,
            DebugStopCode::CaDebugstopMismatchedSequentialStream,
            DebugStopCode::CaDebugstopUnexpectedSeqServiceType,
            DebugStopCode::CaDebugstopMismatchedDeleteSdIndex,
            DebugStopCode::CaDebugstopSetHostDataOutOfRange,
            DebugStopCode::CaDebugstopReadWriteCmdRscAllocFailed,
            DebugStopCode::CaDebugstopUnexpectedCancelWhileFlushing,
            DebugStopCode::CaDebugstopCacheoffWrongReturnState,
            DebugStopCode::CaDebugstopUnexpectedReturnSdHitInfoCall,
            DebugStopCode::CaDebugstopSdScanIndexOutOfRange,
            DebugStopCode::CaDebugstopUnexpectedSdState,
            DebugStopCode::CaDebugstopInvalidCache,
            DebugStopCode::CaDebugstopNoCdOrSdAttached,
            DebugStopCode::CaDebugstopUnexpectedReadScanState,
            DebugStopCode::CaDebugstopUnexpectedWriteScanState,
            DebugStopCode::CaDebugstopUnexpectedHostXferState,
            DebugStopCode::CaDebugstopBadAggregationCount,
            DebugStopCode::CaDebugstopMissingSdResource,
            DebugStopCode::CaDebugstopMissingBufferResource,
            DebugStopCode::CaDsssDebugstopCannotFindFragmentSd,
            DebugStopCode::CaDebugstopInvalidMaxPotentialLba,
            DebugStopCode::CaDebugstopInvalidCoherencyCmd,
            DebugStopCode::CaDebugstopInvalidOverlapCount,
            DebugStopCode::CaDebugstopInvalidCoherencySdState,
            DebugStopCode::CaDebugstopInvalidPostReadCount,
            DebugStopCode::CaDebugstopInvalidPreReadCount,
            DebugStopCode::CaDebugstopInvalidCoherencySdIndex,
            DebugStopCode::DrvNotParkedBlndwr,
            DebugStopCode::DxdwcsIndexSkirting,
            DebugStopCode::UbadOn,
            DebugStopCode::MrerrDebugstopGenBimodalProc,
            DebugStopCode::ReadChannelTimeout,
            DebugStopCode::ErInvalidTableRowSize,
            DebugStopCode::ErInvalidRepeatCount,
            DebugStopCode::ErInvalidRecoveryPath,
            DebugStopCode::ErInvalidTableDirectoryEntry,
            DebugStopCode::UtilInvalidTrexPattern,
            DebugStopCode::FwEccAllCrcBytesAreZero,
            DebugStopCode::FwEccNeedNewCodeForNewSecSize,
            DebugStopCode::FwEccBufferXferFailure,
            DebugStopCode::FwEccBufferReadFailure,
            DebugStopCode::FwEccCorrectedSectorNotTransferred,
            DebugStopCode::ErDfhLevelsError,
            DebugStopCode::ErInvalidVmmCount,
            DebugStopCode::ErInvalidLctReadTaCount,
            DebugStopCode::ErInvalidLctReadNontaCount,
            DebugStopCode::ErInvalidLctWriteCount,
            DebugStopCode::ErInvalidLctIndexCount,
            DebugStopCode::VscProtocolViolation,
            DebugStopCode::VscPstDebugstop,
            DebugStopCode::VscTooManyCopies,
            DebugStopCode::VscFieldlistTempBuffAccess,
            DebugStopCode::VscWrite2tBufferAlocFailed,
            DebugStopCode::VscCompareidsThreadNotAllocated,
            DebugStopCode::VscErrInjectionErrType,
            DebugStopCode::VscErrInjectionFuncCode,
            DebugStopCode::VscLowLevelCommandThreadNotAllocated,
            DebugStopCode::VscMaxAllocationReached,
            DebugStopCode::VscIeiInvalidCounter,
            DebugStopCode::VscInitSdAllocationFailed,
            DebugStopCode::VscForceDebugstop,
            DebugStopCode::XferDebugstopUnsupportedOpCode,
            DebugStopCode::XferDebugstopUnexpectedAdditonalXdRequest,
            DebugStopCode::XferDebugstopInvalidXferReqCnt,
            DebugStopCode::XferDebugstopCannotLockThreadRequired,
            DebugStopCode::XferDebugstopTaskNotIdleDuringRv,
            DebugStopCode::XferDebugstopInvalidInsertEvent,
            DebugStopCode::XferDebugstopDataNotAvailForXfer,
            DebugStopCode::HalBmDebugstopTransferAddrNotFound,
            DebugStopCode::HalBmDebugstopSubsegmentNotAvailable,
            DebugStopCode::HalBmDebugstopInvalidThreadNum,
            DebugStopCode::HalBmDebugstopThreadNotAllocated,
            DebugStopCode::HalBmDebugstopLostSomeSubsegments,
            DebugStopCode::HalBmDebugstopCurrentClusterNotFound,
            DebugStopCode::HalBmDebugstopCurrentSdNotFound,
            DebugStopCode::HalBmDebugstopNonReentrantCodeExecuted,
            DebugStopCode::XferDebugstopMultiXferInPriorityQueue,
            DebugStopCode::XferDebugstopUnexpectedPAvailable,
            DebugStopCode::XferDebugstopAwNotExpected,
            DebugStopCode::XferDebugstopIntrudingNonlba,
            DebugStopCode::XferDebugstopNonlbaThreadInvalid,
            DebugStopCode::XferDebugstopUnexpectedFwMsg,
            DebugStopCode::XferDebugstopUnexpectedMultiDoneMsg,
            DebugStopCode::XferDebugstopUnknownFwMsgType,
            DebugStopCode::XferDebugstopNextXferTdNotPresent,
            DebugStopCode::XferDebugstopUnexpectedAwDone,
            DebugStopCode::XferDebugstopUnexpectedNonLbaXferDone,
            DebugStopCode::XferDebugstopUnknownXferMsgType,
            DebugStopCode::XferDebugstopXferError,
            DebugStopCode::XferDebugstopEmptyFuaQueue,
            DebugStopCode::XferDebugstopEocReqInMultiState,
            DebugStopCode::XferDebugstopTdNotInQueue,
            DebugStopCode::XferDebugstopAwXfrMsg,
            DebugStopCode::XferDebugstopFuaWriteMsg,
            DebugStopCode::XferDebugstopNonpNotAvailable,
            DebugStopCode::XferDebugstopPNotAvailable,
            DebugStopCode::IpmDebugstopCommandIntervalQueueOverflow,
            DebugStopCode::IpmDebugstopCommandBinOverflow,
            DebugStopCode::DlgDebugstopTestTrackTranslationFailed,
            DebugStopCode::DlgDebugstopInvalidWarehouseIndex,
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
