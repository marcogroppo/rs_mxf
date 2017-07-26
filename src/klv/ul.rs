
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Ul {
  HeaderPartition,
  BodyPartition,
  FooterPartition,
  PrimerPack,
  RandomIndexMetadata,
  SequenceSet,
  SourceClipSet,
  TimecodeComponentSet,
  ContentStorageSet,
  EssenceContainerDataSet,
  FileDescriptorSet,
  GenericPictureEssenceDescriptor,
  CdciVideoDescriptor,
  RgbaVideoDescriptor,
  PrefaceSet,
  IdentificationSet,
  NetworkLocatorSet,
  TextLocatorSet,
  MaterialPackageSet,
  FilePackageSet,
  StaticTrackSet,
  TrackSet,
  EventTrackSet,
  DmSegmentDescriptorSet,
  GenericSoundEssenceDescriptorSet,
  GenericDataEssenceDescriptorSet,
  MultipleDescriptorSet,
  DmSourceClipSet,
  Aes3AudioDescriptorSet,
  WaveAudioDescriptorSet,
  MpegVideoDescriptorSet,
  Jpeg2000SubDescriptorSet,
  McaLabelSubDescriptorSet,
  AudioChannelLabelSubDescriptorSet,
  SoundfieldGroupLabelSubDescriptorSet,
  PictureItemMpegFrameWrappedPictureElement,
  Jpeg2000FrameWrapped,
  Jpeg2000ClipWrapped,
  SoundItemDataWrappedSoundElement,
  SoundItemWaveDataWrappedSoundElement,
  SoundItemBwfDataWrappedSoundElement,
  FillItem,
  FillItemAvid,

  StructuralComponentDataDefinition,
  StructuralComponentDuration,
  SequenceStructuralComponents,
  SourceClipSourcePackageID,
  SourceClipSourceTrackID,
  SourceClipStartPosition,
  TimecodeComponentStartTimecode,
  TimecodeComponentRoundedTimecodeBase,
  TimecodeComponentDropFrame,
  ContentStoragePackages,
  ContentStorageEssenceContainerData,
  EssenceContainerDataLinkedPackageUID,

  IndexTableSegment_IndexEditRate,
  IndexTableSegment_IndexStartPosition,
  IndexTableSegment_IndexDuration,
  IndexTableSegment_EditUnitByteCount,
  IndexTableSegment_IndexSID,
  IndexTableSegment_BodySID,
  IndexTableSegment_SliceCount,
  IndexTableSegment_PositionTableCount,
  IndexTableSegment_DeltaEntryArray,
  IndexTableSegment_IndexEntryArray,

  IndexTableLayout_SingleIndexLocation,
  IndexTableLayout_SingleEssenceLocation,
  IndexTableLayout_ForwardIndexDirection,

  FileDescriptor_SampleRate,
  FileDescriptor_ContainerDuration,
  FileDescriptor_EssenceContainer,
  FileDescriptor_LinkedTrackID,
  FileDescriptor_Codec,
  FileDescriptor_Locators,

  NetworkLocator_Url,

  GenericPictureEssenceDescriptor_SignalStandard,
  GenericPictureEssenceDescriptor_FrameLayout,
  GenericPictureEssenceDescriptor_StoredWidth,
  GenericPictureEssenceDescriptor_StoredHeight,
  GenericPictureEssenceDescriptor_StoredF2Offset,
  GenericPictureEssenceDescriptor_SampledWidth,
  GenericPictureEssenceDescriptor_SampledHeight,
  GenericPictureEssenceDescriptor_SampledXOffset,
  GenericPictureEssenceDescriptor_SampledYOffset,
  GenericPictureEssenceDescriptor_DisplayHeight,
  GenericPictureEssenceDescriptor_DisplayWidth,
  GenericPictureEssenceDescriptor_DisplayXOffset,
  GenericPictureEssenceDescriptor_DisplayYOffset,
  GenericPictureEssenceDescriptor_DisplayF2Offset,
  GenericPictureEssenceDescriptor_AspectRatio,
  GenericPictureEssenceDescriptor_ActiveFormatDescriptor,
  GenericPictureEssenceDescriptor_VideoLineMap,
  GenericPictureEssenceDescriptor_AlphaTransparency,
  GenericPictureEssenceDescriptor_TransferCharacteristic,
  GenericPictureEssenceDescriptor_ImageAlignmentOffset,
  GenericPictureEssenceDescriptor_ImageStartOffset,
  GenericPictureEssenceDescriptor_ImageEndOffset,
  GenericPictureEssenceDescriptor_FieldDominance,
  GenericPictureEssenceDescriptor_PictureEssenceCoding,
  GenericPictureEssenceDescriptor_CodingEquations,

  CdciEssenceDescriptor_ComponentDepth,
  CdciEssenceDescriptor_HorizontalSubsampling,
  CdciEssenceDescriptor_VerticalSubsampling,
  CdciEssenceDescriptor_ColorSiting,
  CdciEssenceDescriptor_ReversedByteOrder,
  CdciEssenceDescriptor_PaddingBits,
  CdciEssenceDescriptor_AlphaSampleDepth,
  CdciEssenceDescriptor_BlackRefLevel,
  CdciEssenceDescriptor_WhiteReflevel,
  CdciEssenceDescriptor_ColorRange,
  CdciEssenceDescriptor_ComponentMaxRef,
  CdciEssenceDescriptor_ComponentMinRef,
  CdciEssenceDescriptor_AlphaMaxRef,
  CdciEssenceDescriptor_AlphaMinRef,
  CdciEssenceDescriptor_ScanningDirection,
  CdciEssenceDescriptor_PixelLayout,
  CdciEssenceDescriptor_Palette,
  CdciEssenceDescriptor_PaletteLayout,

  Preface_LastModifiedDate,
  Preface_ContentStorage,
  Preface_Version,
  Preface_Identifications,
  Preface_DMSchemes,

  Preface_ObjectModelVersion,
  Preface_PrimaryPackage,

  LinkedGenerationID,

  OperationalPattern,
  EssenceContainers,
  IsRipPresent,
  Identification_CompanyName,
  Identification_ProductName,
  Identification_ProductVersion,
  Identification_VersionString,
  Identification_ProductUid,
  Identification_ModificationDate,
  Identification_ToolkitVersion,
  Identification_Platform,
  Identification_ThisGenerationUid,
  InterchangeObjectInstanceUid,

  GenericSoundEssenceDescriptorQuantizationBits,
  GenericSoundEssenceDescriptorLocked,
  GenericSoundEssenceDescriptorAudioSamplingRate,
  GenericSoundEssenceDescriptorAudioRefLevel,
  GenericSoundEssenceDescriptorChannelCount,

  WaveAudioDescriptorAvgBps,
  WaveAudioDescriptorBlockAlign,

  Aes3AudioDescriptorAuxBitsMode,
  Aes3AudioDescriptorChannelStatusMode,
  Aes3AudioDescriptorFixedChannelStatusData,
  Aes3AudioDescriptorUserDataMode,
  Aes3AudioDescriptorFixedUserData,
  Aes3AudioDescriptorEmphasis,

  MultipleDescriptorSubDescriptorUids,
  IndexSid,
  BodySid,
  GenericPackagePackageUid,
  GenericPackageTracks,
  GenericPackagePackageModifiedDate,
  GenericPackagePackageCreationDate,
  SourcePackageDescriptor,
  GenericTrackTrackId,
  GenericTrackTrackName,
  GenericTrackSequence,
  GenericTrackTrackNumber,

  ColorPrimaries,
  ElectrospatialFormulation,
  SoundCompression,
  DialNorm,
  ChannelAssignment,
  RFC5646SpokenLanguage,

  Mpeg2VideoDescriptorSingleSequence,
  Mpeg2VideoDescriptorConstantBframes,
  Mpeg2VideoDescriptorCodedContentType,
  Mpeg2VideoDescriptorLowDelay,
  Mpeg2VideoDescriptorClosedGOP,
  Mpeg2VideoDescriptorIdenticalGOP,
  Mpeg2VideoDescriptorMaxGOP,
  Mpeg2VideoDescriptorBPictureCount,
  Mpeg2VideoDescriptorProfileAndLevel,
  Mpeg2VideoDescriptorBitRate,

  Jpeg2000VideoDescriptor_Rsiz,
  Jpeg2000VideoDescriptor_Xsiz,
  Jpeg2000VideoDescriptor_Ysiz,
  Jpeg2000VideoDescriptor_XOsiz,
  Jpeg2000VideoDescriptor_YOsiz,
  Jpeg2000VideoDescriptor_XTsiz,
  Jpeg2000VideoDescriptor_YTsiz,
  Jpeg2000VideoDescriptor_XTOsiz,
  Jpeg2000VideoDescriptor_YTOsiz,
  Jpeg2000VideoDescriptor_Csiz,
  Jpeg2000VideoDescriptor_PictureComponentSizing,
  Jpeg2000VideoDescriptor_CodingStyleDefault,
  Jpeg2000VideoDescriptor_QuantizationDefault,
  Jpeg2000VideoDescriptor_J2CLayout,

  TrackEditRate,
  TrackOrigin,
  DMSegmentDMFramework,
  AS10CoreFramework,
  AS10ShimName,
  SubDescriptors,

  SourceDescriptor_ChannelIDs,
  SourceDescriptor_MonoSourceTrackIDs,

  PackageName,

  Eidr_CanonicalDOIName,
  Eidr_CanonicalEIDRIdentifier,
  Eidr_CompactAdIDIdentifier,
  Eidr_DMSEssenceID,

  MCALabelDictionaryID,
  MCATagSymbol,
  MCATagname,
  GroupOfSoundfieldGroupsLinkID,
  MCALinkID,
  SoundfieldGroupLinkID,
  MCAChannelID,
  MCATitle,
  MCATitleVersion,
  MCATitleSubVersion,
  MCAEpisode,
  MCAPartitionKind,
  MCAPartitionNumber,
  MCAAudioContentKind,
  MCAAudioElementKind,

  ReferenceImageEditRate,
  ReferenceAudioAlignmentLevel,
}

#[derive(Debug)]
pub enum SmpteRegitery {
  Partition,
  Set,
  Essence,
  Fill
}

macro_rules! tuple_to_vec {
  ($ul:expr) => (
    vec![$ul.0, $ul.1, $ul.2, $ul.3, $ul.4, $ul.5, $ul.6, $ul.7, $ul.8, $ul.9, $ul.10, $ul.11, $ul.12, $ul.13, $ul.14, $ul.15]
  );
}

macro_rules! vec_ul {
  (Ul::HeaderPartition, $status:expr) => (tuple_to_vec!(partition_identifier!(Ul::HeaderPartition, $status)););
  (Ul::BodyPartition, $status:expr) => (tuple_to_vec!(partition_identifier!(Ul::BodyPartition, $status)));
  (Ul::FooterPartition, $status:expr) => (tuple_to_vec!(partition_identifier!(Ul::FooterPartition, $status)));
  (Ul::PrimerPack) => (tuple_to_vec!(partition_identifier!(Ul::PrimerPack)));
  (Ul::RandomIndexMetadata) => (tuple_to_vec!(partition_identifier!(Ul::RandomIndexMetadata)));
  (Ul::StaticTrack) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x3a)));
  (Ul::Track) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x3b)));
  (Ul::PictureItemMpegFrameWrappedPictureElement, $stream:tt) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x05, $stream)));
  (Ul::Jpeg2000FrameWrapped, $stream:tt) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x08, $stream)));
  (Ul::Jpeg2000ClipWrapped, $stream:tt) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x09, $stream)));
  (Ul::SoundItemDataWrappedSoundElement, $stream:tt) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x16, 0x00, $stream)));
  (Ul::SoundItemWaveDataWrappedSoundElement, $stream:tt) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x16, 0x00, $stream)));
  (Ul::SoundItemBwfDataWrappedSoundElement, $stream:tt) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Essence, 0x06, 0x00, $stream)));
  (Ul::FillItem) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Fill, 0x02)));
  (Ul::FillItemAvid) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Fill, 0x01)));

  (Ul::SequenceSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x0f)));
  (Ul::SourceClipSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x11)));
  (Ul::TimecodeComponentSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x14)));
  (Ul::ContentStorageSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x18)));
  (Ul::EssenceContainerDataSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x23)));
  (Ul::FileDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x25)));
  (Ul::GenericPictureEssenceDescriptor) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x27)));
  (Ul::CdciVideoDescriptor) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x28)));
  (Ul::RgbaVideoDescriptor) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x29)));
  (Ul::PrefaceSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x2f)));
  (Ul::IdentificationSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x30)));
  (Ul::NetworkLocatorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x32)));
  (Ul::TextLocatorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x33)));
  (Ul::MaterialPackageSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x36)));
  (Ul::FilePackageSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x37)));
  (Ul::StaticTrackSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x3a)));
  (Ul::TrackSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x3b)));
  (Ul::EventTrackSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x39)));
  (Ul::DmSegmentDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x41)));
  (Ul::GenericSoundEssenceDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x42)));
  (Ul::GenericDataEssenceDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x43)));
  (Ul::MultipleDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x44)));
  (Ul::DmSourceClipSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x45)));
  (Ul::Aes3AudioDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x47)));
  (Ul::WaveAudioDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x48)));
  (Ul::MpegVideoDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x51)));
  (Ul::Jpeg2000SubDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x5a)));
  (Ul::McaLabelSubDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x6a)));
  (Ul::AudioChannelLabelSubDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x6b)));
  (Ul::SoundfieldGroupLabelSubDescriptorSet) => (tuple_to_vec!(smpte_identifier!(SmpteRegitery::Set, 0x6c)));
}

macro_rules! ul_filter {
  (Ul::HeaderPartition) => (partition_identifier!(Ul::HeaderPartition));
  (Ul::BodyPartition) => (partition_identifier!(Ul::BodyPartition));
  (Ul::FooterPartition) => (partition_identifier!(Ul::FooterPartition));
  (Ul::PrimerPack) => (partition_identifier!(Ul::PrimerPack));
  (Ul::RandomIndexMetadata) => (partition_identifier!(Ul::RandomIndexMetadata));
  (Ul::FillItem) => (smpte_identifier!(SmpteRegitery::Fill, 0x02));
  (Ul::FillItemAvid) => (smpte_identifier!(SmpteRegitery::Fill, 0x01));
  (Ul::PictureItemMpegFrameWrappedPictureElement) => (smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x05));
  (Ul::Jpeg2000FrameWrapped) => (smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x08));
  (Ul::Jpeg2000ClipWrapped) => (smpte_identifier!(SmpteRegitery::Essence, 0x15, 0x09));
  (SmpteRegitery::Set) => (smpte_identifier!(SmpteRegitery::Set));
}

macro_rules! partition_status_identifier {
  ($ul:tt, $status:expr) => (
    match $status {
      PartitionStatus::OpenAndIncomplete => smpte_identifier!(SmpteRegitery::Partition, $ul, 0x01),
      PartitionStatus::ClosedAndIncomplete => smpte_identifier!(SmpteRegitery::Partition, $ul, 0x02),
      PartitionStatus::OpenAndComplete => smpte_identifier!(SmpteRegitery::Partition, $ul, 0x03),
      PartitionStatus::ClosedAndComplete => smpte_identifier!(SmpteRegitery::Partition, $ul, 0x04),
    }
  );
}

macro_rules! partition_identifier {
  ($ul:expr, $status:expr) => (
    match $ul {
      Ul::HeaderPartition => partition_status_identifier!(0x02, $status),
      Ul::BodyPartition => partition_status_identifier!(0x03, $status),
      Ul::FooterPartition => partition_status_identifier!(0x04, $status),
      _ => panic!("Not covered Partition identifier")
    }
  );
  (Ul::HeaderPartition) => (
    smpte_identifier!(SmpteRegitery::Partition, 0x02)
  );
  (Ul::BodyPartition) => (
    smpte_identifier!(SmpteRegitery::Partition, 0x03)
  );
  (Ul::FooterPartition) => (
    smpte_identifier!(SmpteRegitery::Partition, 0x04)
  );
  (Ul::PrimerPack) => (
    smpte_identifier!(SmpteRegitery::Partition, 0x05)
  );
  (Ul::RandomIndexMetadata) => (
    smpte_identifier!(SmpteRegitery::Partition, 0x11)
  );
}

macro_rules! smpte_identifier {
  (SmpteRegitery::Partition, $x:tt, $status:tt) => (
    smpte_identifier!(0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, $x, $status, 0x00)
  );
  (SmpteRegitery::Partition, $x:tt) => (
    smpte_identifier!(0x02, 0x05, 0x01, 0x01, 0x0d, 0x01, 0x02, 0x01, 0x01, $x, _, 0x00)
  );
  (SmpteRegitery::Set, $x:tt) => (
    smpte_identifier!(0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x01, 0x01, 0x01, 0x01, $x, 0x00)
  );
  (SmpteRegitery::Set) => (
    smpte_identifier!(0x02, 0x53, 0x01, 0x01, 0x0d, 0x01, 0x01, 0x01, 0x01, 0x01, _, 0x00)
  );
  (SmpteRegitery::Essence, $x:tt, $y:tt, $stream:tt) => (
    smpte_identifier!(0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, $x, 0x01, $y, $stream)
  );
  (SmpteRegitery::Essence, $x:tt, $y:tt) => (
    smpte_identifier!(0x01, 0x02, 0x01, 0x01, 0x0d, 0x01, 0x03, 0x01, $x, 0x01, $y, _)
  );
  (SmpteRegitery::Fill, $x:tt) => (
    smpte_identifier!(0x01, 0x01, 0x01, $x, 0x03, 0x01, 0x02, 0x10, 0x01, 0x00, 0x00, 0x00)
  );

  ($x0:tt, $x1:tt, $x2:tt, $x3:tt, $x4:tt, $x5:tt, $x6:tt, $x7:tt, $x8:tt, $x9:tt, $x10:tt, $x11:tt) => (
    (0x06, 0x0e, 0x2b, 0x34, $x0, $x1, $x2, $x3, $x4, $x5, $x6, $x7, $x8, $x9, $x10, $x11)
  );
}


pub fn match_ul(data: Vec<u8>) -> Option<Ul> {
  match_smpte_ul(data)
}

fn match_smpte_ul(data: Vec<u8>) -> Option<Ul> {
  match (data[0], data[1], data[2], data[3]) {
    (0x06, 0x0e, 0x2b, 0x34) => get_ul(data),
    _ => {
      println!("Unknown UL: {}", format_ul(&data));
      // Err("UL not match to SMPTE registery".to_string())
      None
    }
  }
}

fn get_ul(data: Vec<u8>) -> Option<Ul> {
  let ul =
    match (data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]) {
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x07, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00) =>
        Ul::StructuralComponentDataDefinition,
      (0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x02, 0x01, 0x01, 0x03, 0x00, 0x00) =>
        Ul::StructuralComponentDuration,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x06, 0x09, 0x00, 0x00) =>
        Ul::SequenceStructuralComponents,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x03, 0x01, 0x00, 0x00, 0x00) =>
        Ul::SourceClipSourcePackageID,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x03, 0x02, 0x00, 0x00, 0x00) =>
        Ul::SourceClipSourceTrackID,
      (0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x03, 0x01, 0x04, 0x00, 0x00) =>
        Ul::SourceClipStartPosition,
      (0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x03, 0x01, 0x05, 0x00, 0x00) =>
        Ul::TimecodeComponentStartTimecode,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x04, 0x01, 0x01, 0x02, 0x06, 0x00, 0x00) =>
        Ul::TimecodeComponentRoundedTimecodeBase,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x04, 0x01, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::TimecodeComponentDropFrame,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x05, 0x01, 0x00, 0x00) =>
        Ul::ContentStoragePackages,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x05, 0x02, 0x00, 0x00) =>
        Ul::ContentStorageEssenceContainerData,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x06, 0x01, 0x00, 0x00, 0x00) =>
        Ul::EssenceContainerDataLinkedPackageUID,

      (0x01, 0x01, 0x01, 0x05, 0x05, 0x30, 0x04, 0x06, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IndexTableSegment_IndexEditRate,
      (0x01, 0x01, 0x01, 0x05, 0x07, 0x02, 0x01, 0x03, 0x01, 0x0a, 0x00, 0x00) =>
        Ul::IndexTableSegment_IndexStartPosition,
      (0x01, 0x01, 0x01, 0x05, 0x07, 0x02, 0x02, 0x01, 0x01, 0x02, 0x00, 0x00) =>
        Ul::IndexTableSegment_IndexDuration,
      (0x01, 0x01, 0x01, 0x04, 0x04, 0x06, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IndexTableSegment_EditUnitByteCount,
      (0x01, 0x01, 0x01, 0x04, 0x04, 0x04, 0x04, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::IndexTableSegment_SliceCount,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x04, 0x04, 0x01, 0x07, 0x00, 0x00, 0x00) =>
        Ul::IndexTableSegment_PositionTableCount,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x04, 0x04, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::IndexTableSegment_DeltaEntryArray,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x04, 0x04, 0x02, 0x05, 0x00, 0x00, 0x00) =>
        Ul::IndexTableSegment_IndexEntryArray,

      (0x01, 0x01, 0x01, 0x0e, 0x04, 0x04, 0x05, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IndexTableLayout_SingleIndexLocation,
      (0x01, 0x01, 0x01, 0x0e, 0x04, 0x06, 0x02, 0x06, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IndexTableLayout_SingleEssenceLocation,
      (0x01, 0x01, 0x01, 0x0e, 0x04, 0x04, 0x05, 0x02, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IndexTableLayout_ForwardIndexDirection,

      (0x01, 0x01, 0x01, 0x01, 0x04, 0x06, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::FileDescriptor_SampleRate,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x06, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00) =>
        Ul::FileDescriptor_ContainerDuration,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x01, 0x02, 0x00, 0x00) =>
        Ul::FileDescriptor_EssenceContainer,
      (0x01, 0x01, 0x01, 0x05, 0x06, 0x01, 0x01, 0x03, 0x05, 0x00, 0x00, 0x00) =>
        Ul::FileDescriptor_LinkedTrackID,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x01, 0x03, 0x00, 0x00) =>
        Ul::FileDescriptor_Codec,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x06, 0x03, 0x00, 0x00) =>
        Ul::FileDescriptor_Locators,

      (0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::NetworkLocator_Url,

      (0x01, 0x01, 0x01, 0x05, 0x04, 0x05, 0x01, 0x13, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_SignalStandard,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x03, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_FrameLayout,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x02, 0x02, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_StoredWidth,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x02, 0x01, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_StoredHeight,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x03, 0x02, 0x08, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_StoredF2Offset,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x08, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_SampledWidth,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x07, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_SampledHeight,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x09, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_SampledXOffset,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x0a, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_SampledYOffset,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x0b, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_DisplayHeight,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x0c, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_DisplayWidth,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x0d, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_DisplayXOffset,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x0e, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_DisplayYOffset,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x03, 0x02, 0x07, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_DisplayF2Offset,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_AspectRatio,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x03, 0x02, 0x09, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_ActiveFormatDescriptor,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x03, 0x02, 0x05, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_VideoLineMap,
      (0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_AlphaTransparency,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x02, 0x01, 0x01, 0x01, 0x02, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_TransferCharacteristic,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x18, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_ImageAlignmentOffset,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x18, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_ImageStartOffset,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x18, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_ImageEndOffset,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x03, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_FieldDominance,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x06, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_PictureEssenceCoding,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x02, 0x01, 0x01, 0x03, 0x01, 0x00) =>
        Ul::GenericPictureEssenceDescriptor_CodingEquations,

      (0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x03, 0x0a, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ComponentDepth,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_HorizontalSubsampling,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x01, 0x10, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_VerticalSubsampling,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ColorSiting,
      (0x01, 0x01, 0x01, 0x05, 0x03, 0x01, 0x02, 0x01, 0x0a, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ReversedByteOrder,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x18, 0x01, 0x04, 0x00, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_PaddingBits,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x03, 0x07, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_AlphaSampleDepth,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x03, 0x03, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_BlackRefLevel,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x01, 0x05, 0x03, 0x04, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_WhiteReflevel,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x03, 0x05, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ColorRange,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x05, 0x03, 0x0b, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ComponentMaxRef,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x05, 0x03, 0x0c, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ComponentMinRef,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x05, 0x03, 0x0d, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_AlphaMaxRef,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x05, 0x03, 0x0e, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_AlphaMinRef,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x04, 0x04, 0x01, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_ScanningDirection,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x03, 0x06, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_PixelLayout,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x03, 0x08, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_Palette,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x01, 0x05, 0x03, 0x09, 0x00, 0x00, 0x00) =>
        Ul::CdciEssenceDescriptor_PaletteLayout,

      (0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x10, 0x02, 0x04, 0x00, 0x00) =>
        Ul::Preface_LastModifiedDate,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x02, 0x01, 0x00, 0x00) =>
        Ul::Preface_ContentStorage,
      (0x01, 0x01, 0x01, 0x02, 0x03, 0x01, 0x02, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::Preface_Version,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x06, 0x04, 0x00, 0x00) =>
        Ul::Preface_Identifications,
      (0x01, 0x01, 0x01, 0x05, 0x01, 0x02, 0x02, 0x10, 0x02, 0x02, 0x00, 0x00) =>
        Ul::Preface_DMSchemes,

      (0x01, 0x01, 0x01, 0x02, 0x03, 0x01, 0x02, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::Preface_ObjectModelVersion,
      (0x01, 0x01, 0x01, 0x04, 0x06, 0x01, 0x01, 0x04, 0x01, 0x08, 0x00, 0x00) =>
        Ul::Preface_PrimaryPackage,
      (0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x08, 0x00, 0x00, 0x00) =>
        Ul::LinkedGenerationID,

      (0x01, 0x01, 0x01, 0x05, 0x01, 0x02, 0x02, 0x03, 0x00, 0x00, 0x00, 0x00) =>
        Ul::OperationalPattern,
      (0x01, 0x01, 0x01, 0x05, 0x01, 0x02, 0x02, 0x10, 0x02, 0x01, 0x00, 0x00) =>
        Ul::EssenceContainers,
      (0x01, 0x01, 0x01, 0x0e, 0x04, 0x04, 0x05, 0x03, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IsRipPresent,
      (0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x02, 0x01, 0x00, 0x00) =>
        Ul::Identification_CompanyName,
      (0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x03, 0x01, 0x00, 0x00) =>
        Ul::Identification_ProductName,
      (0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::Identification_ProductVersion,
      (0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x05, 0x01, 0x00, 0x00) =>
        Ul::Identification_VersionString,
      (0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x07, 0x00, 0x00, 0x00) =>
        Ul::Identification_ProductUid,
      (0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x10, 0x02, 0x03, 0x00, 0x00) =>
        Ul::Identification_ModificationDate,
      (0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x0a, 0x00, 0x00, 0x00) =>
        Ul::Identification_ToolkitVersion,
      (0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x06, 0x01, 0x00, 0x00) =>
        Ul::Identification_Platform,
      (0x01, 0x01, 0x01, 0x02, 0x05, 0x20, 0x07, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::Identification_ThisGenerationUid,
      (0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x15, 0x02, 0x00, 0x00, 0x00, 0x00) =>
        Ul::InterchangeObjectInstanceUid,
      (0x01, 0x01, 0x01, 0x04, 0x04, 0x02, 0x03, 0x03, 0x04, 0x00, 0x00, 0x00) =>
        Ul::GenericSoundEssenceDescriptorQuantizationBits,
      (0x01, 0x01, 0x01, 0x04, 0x04, 0x02, 0x03, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::GenericSoundEssenceDescriptorLocked,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x01, 0x01, 0x01, 0x00, 0x00) =>
        Ul::GenericSoundEssenceDescriptorAudioSamplingRate,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x02, 0x01, 0x01, 0x03, 0x00, 0x00, 0x00) =>
        Ul::GenericSoundEssenceDescriptorAudioRefLevel,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x01, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::GenericSoundEssenceDescriptorChannelCount,

      (0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x03, 0x05, 0x00, 0x00, 0x00) =>
        Ul::WaveAudioDescriptorAvgBps,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x03, 0x02, 0x01, 0x00, 0x00, 0x00) =>
        Ul::WaveAudioDescriptorBlockAlign,

      (0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x05, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorAuxBitsMode,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x05, 0x01, 0x02, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorChannelStatusMode,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x05, 0x01, 0x03, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorFixedChannelStatusData,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x05, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorUserDataMode,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x05, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorFixedUserData,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x05, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::Aes3AudioDescriptorEmphasis,

      (0x01, 0x01, 0x01, 0x04, 0x06, 0x01, 0x01, 0x04, 0x06, 0x0b, 0x00, 0x00) =>
        Ul::MultipleDescriptorSubDescriptorUids,
      (0x01, 0x01, 0x01, 0x04, 0x01, 0x03, 0x04, 0x05, 0x00, 0x00, 0x00, 0x00) =>
        Ul::IndexSid,
      (0x01, 0x01, 0x01, 0x04, 0x01, 0x03, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00) =>
        Ul::BodySid,
      (0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x15, 0x10, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericPackagePackageUid,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x06, 0x05, 0x00, 0x00) =>
        Ul::GenericPackageTracks,
      (0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x10, 0x02, 0x05, 0x00, 0x00) =>
        Ul::GenericPackagePackageModifiedDate,
      (0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x10, 0x01, 0x03, 0x00, 0x00) =>
        Ul::GenericPackagePackageCreationDate,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x02, 0x03, 0x00, 0x00) =>
        Ul::SourcePackageDescriptor,
      (0x01, 0x01, 0x01, 0x02, 0x01, 0x07, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericTrackTrackId,
      (0x01, 0x01, 0x01, 0x02, 0x01, 0x07, 0x01, 0x02, 0x01, 0x00, 0x00, 0x00) =>
        Ul::GenericTrackTrackName,
      (0x01, 0x01, 0x01, 0x02, 0x06, 0x01, 0x01, 0x04, 0x02, 0x04, 0x00, 0x00) =>
        Ul::GenericTrackSequence,
      (0x01, 0x01, 0x01, 0x02, 0x01, 0x04, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00) =>
        Ul::GenericTrackTrackNumber,

      (0x01, 0x01, 0x01, 0x02, 0x05, 0x30, 0x04, 0x05, 0x00, 0x00, 0x00, 0x00) =>
        Ul::TrackEditRate,
      (0x01, 0x01, 0x01, 0x02, 0x07, 0x02, 0x01, 0x03, 0x01, 0x03, 0x00, 0x00) =>
        Ul::TrackOrigin,

      (0x01, 0x01, 0x01, 0x05, 0x06, 0x01, 0x01, 0x04, 0x02, 0x0c, 0x00, 0x00) =>
        Ul::DMSegmentDMFramework,

      (0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x00) =>
        Ul::AS10CoreFramework,
      (0x01, 0x01, 0x01, 0x01, 0x0d, 0x01, 0x07, 0x01, 0x0a, 0x01, 0x01, 0x01) =>
        Ul::AS10ShimName,


      (0x01, 0x01, 0x01, 0x01, 0x01, 0x03, 0x03, 0x02, 0x01, 0x00, 0x00, 0x00) =>
        Ul::PackageName,

      (0x01, 0x01, 0x01, 0x09, 0x06, 0x01, 0x01, 0x04, 0x06, 0x10, 0x00, 0x00) =>
        Ul::SubDescriptors,

      (0x01, 0x01, 0x01, 0x07, 0x06, 0x01, 0x01, 0x03, 0x07, 0x00, 0x00, 0x00) =>
        Ul::SourceDescriptor_ChannelIDs,
      (0x01, 0x01, 0x01, 0x08, 0x06, 0x01, 0x01, 0x03, 0x08, 0x00, 0x00, 0x00) =>
        Ul::SourceDescriptor_MonoSourceTrackIDs,
      (0x01, 0x01, 0x01, 0x09, 0x04, 0x01, 0x02, 0x01, 0x01, 0x06, 0x01, 0x00) =>
        Ul::ColorPrimaries,
      (0x01, 0x01, 0x01, 0x01, 0x04, 0x02, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::ElectrospatialFormulation,
      (0x01, 0x01, 0x01, 0x02, 0x04, 0x02, 0x04, 0x02, 0x00, 0x00, 0x00, 0x00) =>
        Ul::SoundCompression,
      (0x01, 0x01, 0x01, 0x05, 0x04, 0x02, 0x07, 0x01, 0x00, 0x00, 0x00, 0x00) =>
        Ul::DialNorm,
      (0x01, 0x01, 0x01, 0x07, 0x04, 0x02, 0x01, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::ChannelAssignment,
      (0x01, 0x01, 0x01, 0x0d, 0x03, 0x01, 0x01, 0x02, 0x03, 0x15, 0x00, 0x00) =>
        Ul::RFC5646SpokenLanguage,

      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x01, 0x11, 0x0c, 0x00, 0x00, 0x00, 0x00) =>
        Ul::Eidr_CanonicalDOIName,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x01, 0x11, 0x0d, 0x00, 0x00, 0x00, 0x00) =>
        Ul::Eidr_CanonicalEIDRIdentifier,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x01, 0x11, 0x0e, 0x00, 0x00, 0x00, 0x00) =>
        Ul::Eidr_CompactAdIDIdentifier,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x01, 0x15, 0x14, 0x00, 0x00, 0x00, 0x00) =>
        Ul::Eidr_DMSEssenceID,

      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x07, 0x01, 0x01, 0x00, 0x00, 0x00) =>
        Ul::MCALabelDictionaryID,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x07, 0x01, 0x02, 0x00, 0x00, 0x00) =>
        Ul::MCATagSymbol,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x07, 0x01, 0x03, 0x00, 0x00, 0x00) =>
        Ul::MCATagname,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x07, 0x01, 0x04, 0x00, 0x00, 0x00) =>
        Ul::GroupOfSoundfieldGroupsLinkID,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x07, 0x01, 0x05, 0x00, 0x00, 0x00) =>
        Ul::MCALinkID,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x07, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::SoundfieldGroupLinkID,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x03, 0x04, 0x0a, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCAChannelID,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x05, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCATitle,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x05, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCATitleVersion,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x05, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCATitleSubVersion,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x05, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCAEpisode,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x04, 0x01, 0x05, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCAPartitionKind,
      (0x01, 0x01, 0x01, 0x0e, 0x01, 0x04, 0x01, 0x06, 0x00, 0x00, 0x00, 0x00) =>
        Ul::MCAPartitionNumber,
      (0x01, 0x01, 0x01, 0x0e, 0x03, 0x02, 0x01, 0x02, 0x20, 0x00, 0x00, 0x00) =>
        Ul::MCAAudioContentKind,
      (0x01, 0x01, 0x01, 0x0e, 0x03, 0x02, 0x01, 0x02, 0x21, 0x00, 0x00, 0x00) =>
        Ul::MCAAudioElementKind,

      (0x01, 0x01, 0x01, 0x0e, 0x04, 0x02, 0x01, 0x01, 0x06, 0x00, 0x00, 0x00) =>
        Ul::ReferenceImageEditRate,
      (0x01, 0x01, 0x01, 0x0e, 0x04, 0x02, 0x01, 0x01, 0x07, 0x00, 0x00, 0x00) =>
        Ul::ReferenceAudioAlignmentLevel,
      (0x01, 0x01, 0x01, 0x0e, 0x04, 0x01, 0x06, 0x03, 0x0e, 0x00, 0x00, 0x00) =>
        Ul::Jpeg2000VideoDescriptor_J2CLayout,

      (0x01, 0x01, 0x01, 0x05, 0x04, 0x01, 0x06, 0x02, 0x01, designator, 0x00, 0x00) =>
        return build_mpeg_description_key(designator),
      (0x01, 0x01, 0x01, 0x0a, 0x04, 0x01, 0x06, 0x03, designator, 0x00, 0x00, 0x00) =>
        return build_jpeg2000_description_key(designator),
      _ => {
        println!("Unknown key: {}", format_ul(&data));
        return None
      }
    };

  Some(ul)
}

fn build_mpeg_description_key(designator: u8) -> Option<Ul> {
  match designator {
    0x02 => Some(Ul::Mpeg2VideoDescriptorSingleSequence),
    0x03 => Some(Ul::Mpeg2VideoDescriptorConstantBframes),
    0x04 => Some(Ul::Mpeg2VideoDescriptorCodedContentType),
    0x05 => Some(Ul::Mpeg2VideoDescriptorLowDelay),
    0x06 => Some(Ul::Mpeg2VideoDescriptorClosedGOP),
    0x07 => Some(Ul::Mpeg2VideoDescriptorIdenticalGOP),
    0x08 => Some(Ul::Mpeg2VideoDescriptorMaxGOP),
    0x09 => Some(Ul::Mpeg2VideoDescriptorBPictureCount),
    0x0A => Some(Ul::Mpeg2VideoDescriptorProfileAndLevel),
    0x0B => Some(Ul::Mpeg2VideoDescriptorBitRate),
       _ => None,
  }
}

fn build_jpeg2000_description_key(designator: u8) -> Option<Ul> {
  match designator {
    0x01 => Some(Ul::Jpeg2000VideoDescriptor_Rsiz),
    0x02 => Some(Ul::Jpeg2000VideoDescriptor_Xsiz),
    0x03 => Some(Ul::Jpeg2000VideoDescriptor_Ysiz),
    0x04 => Some(Ul::Jpeg2000VideoDescriptor_XOsiz),
    0x05 => Some(Ul::Jpeg2000VideoDescriptor_YOsiz),
    0x06 => Some(Ul::Jpeg2000VideoDescriptor_XTsiz),
    0x07 => Some(Ul::Jpeg2000VideoDescriptor_YTsiz),
    0x08 => Some(Ul::Jpeg2000VideoDescriptor_XTOsiz),
    0x09 => Some(Ul::Jpeg2000VideoDescriptor_YTOsiz),
    0x0A => Some(Ul::Jpeg2000VideoDescriptor_Csiz),
    0x0B => Some(Ul::Jpeg2000VideoDescriptor_PictureComponentSizing),
    0x0C => Some(Ul::Jpeg2000VideoDescriptor_CodingStyleDefault),
    0x0D => Some(Ul::Jpeg2000VideoDescriptor_QuantizationDefault),
       _ => None,
  }
}
pub fn format_ul(key : &Vec<u8>) -> String {
  let (first, next) = key.split_at(1);
  let mut string_key: String = format!("0x{:02x}", first[0]);

  for v in next {
    string_key = format!("{}, 0x{:02x}", string_key, v);
  }

  string_key
}
