
use klv::ul::format_ul;

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum EssenceIdentifier {
  BwfFrameWrapped,
  BwfClipWrapped,
  AesFrameWrapped,
  AesClipWrapped,
  MpegEsWithStreamIdFrameWrapped,
  MpegEsWithStreamIdClipWrapped,
  MpegEsWithStreamIdStripeWrapped,
  MpegEsWithStreamIdFixedAudioSizeWrapped,
  MpegEsWithStreamIdSpliceWrapped,
  MpegEsWithStreamIdClosedGopWrapped,
  MpegEsWithStreamIdSlaveWrapped,
  MpegEsWithStreamIdNoSpecificWrappingConstraints,
  AVCByteStream_FrameWrapped,
  AVCByteStream_ClipedWrapped,
  AVCByteStream_StripedWrapped,
  AVCByteStream_PesWrapped,
  AVCByteStream_FixedAudioSizeWrapped,
  AVCByteStream_SpliceWrapped,
  AVCByteStream_ClosedGopWrapped,
  AVCByteStream_SlaveWrapped,
  Jpeg2000_FrameWrapped,
  Jpeg2000_ClipedWrapped,
  MXFGCP1FrameWrappedPictureElement,
  GenericEssenceContainerMultipleWrappings,
  ANCData,
  Unknown
}

macro_rules! build_identifier {
  (version_number => $vn:expr, kind => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x06, $k, 0x00)
  );
  (version_number => $vn:expr, mpeg_es => $me:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x04, _, $me)
  );
  (version_number => $vn:expr, avc => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x10, 0x60, $k)
  );
  (version_number => $vn:expr, jpeg2000 => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x0c, $k, 0x00)
  );
  (version_number => $vn:expr, gc => $k:expr) => (
    smpte_identifier!(0x04, 0x01, 0x01, $vn, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x0c, 0x06, 0x00)
  );
}

pub fn parse_essence_ul(ul: Vec<u8>) -> EssenceIdentifier {
  match (ul[0], ul[1], ul[2], ul[3], ul[4], ul[5], ul[6], ul[7], ul[8], ul[9], ul[10], ul[11], ul[12], ul[13], ul[14], ul[15]) {
    build_identifier!(version_number => 0x01, kind => 0x01) => {
      EssenceIdentifier::BwfFrameWrapped
    },
    build_identifier!(version_number => 0x01, kind => 0x02) => {
      EssenceIdentifier::BwfClipWrapped
    },
    build_identifier!(version_number => 0x01, kind => 0x03) => {
      EssenceIdentifier::AesFrameWrapped
    },
    build_identifier!(version_number => 0x01, kind => 0x04) => {
      EssenceIdentifier::AesClipWrapped
    },

    build_identifier!(version_number => 0x02, mpeg_es => 0x01) => {
      EssenceIdentifier::MpegEsWithStreamIdFrameWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x02) => {
      EssenceIdentifier::MpegEsWithStreamIdClipWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x03) => {
      EssenceIdentifier::MpegEsWithStreamIdStripeWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x05) => {
      EssenceIdentifier::MpegEsWithStreamIdFixedAudioSizeWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x06) => {
      EssenceIdentifier::MpegEsWithStreamIdSpliceWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x07) => {
      EssenceIdentifier::MpegEsWithStreamIdClosedGopWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x08) => {
      EssenceIdentifier::MpegEsWithStreamIdSlaveWrapped
    },
    build_identifier!(version_number => 0x02, mpeg_es => 0x7F) => {
      EssenceIdentifier::MpegEsWithStreamIdNoSpecificWrappingConstraints
    },

    build_identifier!(version_number => 0x0a, avc => 0x01) => {
      EssenceIdentifier::AVCByteStream_FrameWrapped
    },
    build_identifier!(version_number => 0x0a, avc => 0x02) => {
      EssenceIdentifier::AVCByteStream_ClipedWrapped
    },
    build_identifier!(version_number => 0x0a, avc => 0x03) => {
      EssenceIdentifier::AVCByteStream_StripedWrapped
    },
    build_identifier!(version_number => 0x0a, avc => 0x04) => {
      EssenceIdentifier::AVCByteStream_PesWrapped
    },
    build_identifier!(version_number => 0x0a, avc => 0x05) => {
      EssenceIdentifier::AVCByteStream_FixedAudioSizeWrapped
    },
    build_identifier!(version_number => 0x0a, avc => 0x06) => {
      EssenceIdentifier::AVCByteStream_SpliceWrapped
    },
    build_identifier!(version_number => 0x0a, avc => 0x07) => {
      EssenceIdentifier::AVCByteStream_ClosedGopWrapped
    },
    build_identifier!(version_number => 0x0a, avc => 0x08) => {
      EssenceIdentifier::AVCByteStream_SlaveWrapped
    },

    build_identifier!(version_number => 0x07, jpeg2000 => 0x01) => {
      EssenceIdentifier::Jpeg2000_FrameWrapped
    },
    build_identifier!(version_number => 0x07, jpeg2000 => 0x06) => {
      EssenceIdentifier::Jpeg2000_FrameWrapped
    },
    build_identifier!(version_number => 0x0d, gc => 0x06) => {
      EssenceIdentifier::MXFGCP1FrameWrappedPictureElement
    },

    (0x06, 0x0e, 0x2b, 0x34, 0x04, 0x01, 0x01, 0x03, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x7F, 0x01, 0x00) => {
      EssenceIdentifier::GenericEssenceContainerMultipleWrappings
    },
    (0x06, 0x0e, 0x2b, 0x34, 0x04, 0x01, 0x01, 0x09, 0x0d, 0x01, 0x03, 0x01, 0x02, 0x0e, 0x00, 0x00) => {
      EssenceIdentifier::GenericEssenceContainerMultipleWrappings
    },
    _ => {
      println!("Unknown essence identifier: {}", format_ul(&ul));
      EssenceIdentifier::Unknown
    }
  }
}
