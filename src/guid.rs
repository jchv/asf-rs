use uuid::Uuid;


pub(crate) trait AsBytesMs {
    fn as_bytes_ms(&self) -> [u8; 16];
}

impl AsBytesMs for Uuid {
    fn as_bytes_ms(&self) -> [u8; 16] {
        let b = self.as_bytes();
        [
            b[3], b[2], b[1], b[0], b[5], b[4], b[7], b[6],
            b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15],
        ]
    }
}

pub const HEADER_OBJECT: Uuid                                 = Uuid::from_u128(0x75b22630668e11cfa6d900aa0062ce6c);
pub const DATA_OBJECT: Uuid                                   = Uuid::from_u128(0x75b22636668e11cfa6d900aa0062ce6c);
pub const SIMPLE_INDEX_OBJECT: Uuid                           = Uuid::from_u128(0x33000890e5b111cf89f400a0c90349cb);
pub const INDEX_OBJECT: Uuid                                  = Uuid::from_u128(0xd6e229d335da11d1903400a0c90349be);
pub const MEDIA_OBJECT_INDEX_OBJECT: Uuid                     = Uuid::from_u128(0xfeb103f812ad4c64840f2a1d2f7ad48c);
pub const TIMECODE_INDEX_OBJECT: Uuid                         = Uuid::from_u128(0x3cb73fd00c4a4803953dedf7b6228f0c);

pub const FILE_PROPERTIES_OBJECT: Uuid                        = Uuid::from_u128(0x8cabdca1a94711cf8ee400c00c205365);
pub const STREAM_PROPERTIES_OBJECT: Uuid                      = Uuid::from_u128(0xb7dc0791a9b711cf8ee600c00c205365);
pub const HEADER_EXTENSION_OBJECT: Uuid                       = Uuid::from_u128(0x5fbf03b5a92e11cf8ee300c00c205365);
pub const CODEC_LIST_OBJECT: Uuid                             = Uuid::from_u128(0x86d15240311d11d0a3a400a0c90348f6);
pub const SCRIPT_COMMAND_OBJECT: Uuid                         = Uuid::from_u128(0x1efb1a300b6211d0a39b00a0c90348f6);
pub const MARKER_OBJECT: Uuid                                 = Uuid::from_u128(0xf487cd01a95111cf8ee600c00c205365);
pub const BITRATE_MUTUAL_EXCLUSION_OBJECT: Uuid               = Uuid::from_u128(0xd6e229dc35da11d1903400a0c90349be);
pub const ERROR_CORRECTION_OBJECT: Uuid                       = Uuid::from_u128(0x75b22635668e11cfa6d900aa0062ce6c);
pub const CONTENT_DESCRIPTION_OBJECT: Uuid                    = Uuid::from_u128(0x75b22633668e11cfa6d900aa0062ce6c);
pub const EXTENDED_CONTENT_DESCRIPTION_OBJECT: Uuid           = Uuid::from_u128(0xd2d0a440e30711d297f000a0c95ea850);
pub const CONTENT_BRANDING_OBJECT: Uuid                       = Uuid::from_u128(0x2211b3fabd2311d2b4b700a0c955fc6e);
pub const STREAM_BITRATE_PROPERTIES_OBJECT: Uuid              = Uuid::from_u128(0x7bf875ce468d11d18d82006097c9a2b2);
pub const CONTENT_ENCRYPTION_OBJECT: Uuid                     = Uuid::from_u128(0x2211b3fbbd2311d2b4b700a0c955fc6e);
pub const EXTENDED_CONTENT_ENCRYPTION_OBJECT: Uuid            = Uuid::from_u128(0x298ae61426224c17b935dae07ee9289c);
pub const DIGITAL_SIGNATURE_OBJECT: Uuid                      = Uuid::from_u128(0x2211b3fcbd2311d2b4b700a0c955fc6e);
pub const PADDING_OBJECT: Uuid                                = Uuid::from_u128(0x1806d474cadf4509a4ba9aabcb96aae8);

pub const EXTENDED_STREAM_PROPERTIES_OBJECT: Uuid             = Uuid::from_u128(0x14e6a5cbc67243328399a96952065b5a);
pub const ADVANCED_MUTUAL_EXCLUSION_OBJECT: Uuid              = Uuid::from_u128(0xa08649cf477546708a166e35357566cd);
pub const GROUP_MUTUAL_EXCLUSION_OBJECT: Uuid                 = Uuid::from_u128(0xd1465a405a794338b71be36b8fd6c249);
pub const STREAM_PRIORITIZATION_OBJECT: Uuid                  = Uuid::from_u128(0xd4fed15b88d3454f81f0ed5c45999e24);
pub const BANDWIDTH_SHARING_OBJECT: Uuid                      = Uuid::from_u128(0xa69609e6517b11d2b6af00c04fd908e9);
pub const LANGUAGE_LIST_OBJECT: Uuid                          = Uuid::from_u128(0x7c4346a9efe04bfcb229393ede415c85);
pub const METADATA_OBJECT: Uuid                               = Uuid::from_u128(0xc5f8cbea5baf48778467aa8c44fa4cca);
pub const METADATA_LIBRARY_OBJECT: Uuid                       = Uuid::from_u128(0x44231c94949849d1a1411d134e457054);
pub const INDEX_PARAMETERS_OBJECT: Uuid                       = Uuid::from_u128(0xd6e229df35da11d1903400a0c90349be);
pub const MEDIA_OBJECT_INDEX_PARAMETERS_OBJECT: Uuid          = Uuid::from_u128(0x6b203bad3f1148e4aca8d7613de2cfa7);
pub const TIMECODE_INDEX_PARAMETERS_OBJECT: Uuid              = Uuid::from_u128(0xf55e496d97974b5d8c8b604dfe9bfb24);
pub const COMPATIBILITY_OBJECT: Uuid                          = Uuid::from_u128(0x75b22630668e11cfa6d900aa0062ce6c);
pub const ADVANCED_CONTENT_ENCRYPTION_OBJECT: Uuid            = Uuid::from_u128(0x43058533698149e69b74ad12cb86d58c);

pub const AUDIO_MEDIA: Uuid                                   = Uuid::from_u128(0xf8699e405b4d11cfa8fd00805f5c442b);
pub const VIDEO_MEDIA: Uuid                                   = Uuid::from_u128(0xbc19efc05b4d11cfa8fd00805f5c442b);
pub const COMMAND_MEDIA: Uuid                                 = Uuid::from_u128(0x59dacfc059e611d0a3ac00a0c90348f6);
pub const JFIF_MEDIA: Uuid                                    = Uuid::from_u128(0xb61be1005b4e11cfa8fd00805f5c442b);
pub const DEGRADABLE_JPEG_MEDIA: Uuid                         = Uuid::from_u128(0x35907de0e41511cfa91700805f5c442b);
pub const FILE_TRANSFER_MEDIA: Uuid                           = Uuid::from_u128(0x91bd222cf21c497a8b6d5aa86bfc0185);
pub const BINARY_MEDIA: Uuid                                  = Uuid::from_u128(0x3afb65e247ef40f2ac2c70a90d71d343);

pub const WEB_STREAM_MEDIA_SUBTYPE: Uuid                      = Uuid::from_u128(0x776257d4c62741cb8f817ac7ff1c40cc);
pub const WEB_STREAM_FORMAT: Uuid                             = Uuid::from_u128(0xda1e6b1383594050b398388e965bf00c);

pub const NO_ERROR_CORRECTION: Uuid                           = Uuid::from_u128(0x20fb57005b5511cfa8fd00805f5c442b);
pub const AUDIO_SPREAD: Uuid                                  = Uuid::from_u128(0xbfc3cd50618f11cf8bb200aa00b4e220);

pub const RESERVED_1: Uuid                                    = Uuid::from_u128(0xabd3d211a9ba11cf8ee600c00c205365);
pub const RESERVED_2: Uuid                                    = Uuid::from_u128(0x86d15241311d11d0a3a400a0c90348f6);
pub const RESERVED_3: Uuid                                    = Uuid::from_u128(0x4b1acbe3100b11d0a39b00a0c90348f6);
pub const RESERVED_4: Uuid                                    = Uuid::from_u128(0x4cfedb2075f611cf9c0f00a0c90349cb);

pub const MUTEX_LANGUAGE: Uuid                                = Uuid::from_u128(0xd6e22a0035da11d1903400a0c90349be);
pub const MUTEX_BITRATE: Uuid                                 = Uuid::from_u128(0xd6e22a0135da11d1903400a0c90349be);
pub const MUTEX_UNKNOWN: Uuid                                 = Uuid::from_u128(0xd6e22a0235da11d1903400a0c90349be);

pub const BANDWIDTH_SHARING_EXCLUSIVE: Uuid                   = Uuid::from_u128(0xaf6060aa519711d2b6af00c04fd908e9);
pub const BANDWIDTH_SHARING_PARTIAL: Uuid                     = Uuid::from_u128(0xaf6060ab519711d2b6af00c04fd908e9);

pub const PAYLOAD_EXTENSION_SYSTEM_TIMECODE: Uuid             = Uuid::from_u128(0x399595ec86674e2d8fdb98814ce76c1e);
pub const PAYLOAD_EXTENSION_SYSTEM_FILE_NAME: Uuid            = Uuid::from_u128(0xe165ec0e19ed45d7b4a725cbd1e28e9b);
pub const PAYLOAD_EXTENSION_SYSTEM_CONTENT_TYPE: Uuid         = Uuid::from_u128(0xd590dc2007bc436c9cf7f3bbfbf1a4dc);
pub const PAYLOAD_EXTENSION_SYSTEM_PIXEL_ASPECT_RATIO: Uuid   = Uuid::from_u128(0x1b1ee554f9ea4bc8821a376b74e4c4b8);
pub const PAYLOAD_EXTENSION_SYSTEM_SAMPLE_DURATION: Uuid      = Uuid::from_u128(0xc6bd9450867f490783a3c77921b733ad);
pub const PAYLOAD_EXTENSION_SYSTEM_ENCRYPTION_SAMPLE_ID: Uuid = Uuid::from_u128(0x6698b84e0afa4330aeb21c0a98d7a44d);
