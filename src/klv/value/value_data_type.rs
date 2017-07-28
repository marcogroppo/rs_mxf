
#[derive(Debug, Clone, Copy)]
pub enum ValueDataType {
  Boolean,
  Uint8Array,
  BytesArray,
  Int8,
  Int16,
  Uint8,
  Uint16,
  Uint32,
  Uint32X2,
  Length,
  Position,
  Ul,
  UlBatch,
  String,
  Utf16,
  Uuid,
  Umid,
  Locators,
  Rational,
  Timestamp,
  PackageId,
  Stream,
  Version,
  ProductVersion,
  ChannelLayout,
  Orientation,
  DataValue,
  WeakRef,
  StrongRef,
  StrongRefArray,
  StrongRefBatch,
  TrackIdBatch,
  J2KComponentSizing,
  J2KCodingStyleDefault,
  J2KQuantisationDefault,
  DeltaEntries,
  IndexEntries,
  Unknown,
}