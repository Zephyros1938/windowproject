use std::collections::HashMap;

// https://tchayen.github.io/posts/ttf-file-parsing

use log::debug;

use crate::{asset_management::get_asset, util::BinaryReader};

pub struct Font {}

impl Font {
    pub fn load_ttf(path: &str) -> Result<(), String> {
        let file = get_asset(path)?;

        let mut reader = BinaryReader::new(file);

        reader.get_uint32();
        let NUM_TABLES = reader.get_uint16() as usize;
        reader.get_uint16();
        reader.get_uint16();
        reader.get_uint16();

        let mut tables: HashMap<String, (u32, u32, u32)> = HashMap::new();

        for _ in 0..NUM_TABLES {
            let tag = reader.get_string(4);
            tables.insert(
                tag,
                (
                    reader.get_uint32(), //checksum
                    reader.get_uint32(), //length
                    reader.get_uint32(), //offset
                ),
            );
        }

        let head = ttf::head::new(&mut reader);
        debug!("{:#?}", head);
        let maxp = ttf::maxp::new(&mut reader);
        debug!("{:#?}", maxp);
        let hhea = ttf::hhea::new(&mut reader);
        debug!("{:#?}", hhea);
        let hmtx = ttf::hmtx::new(&mut reader, hhea, maxp);
        debug!("{:?}", hmtx);

        let loca = {
            let mut loca = Vec::new();
            for _ in 0..maxp.numGlyphs + 1 {
                loca.push(if head.indexToLocFormat == 0 {
                    reader.get_offset16() as u32 * 2
                } else {
                    reader.get_offset32()
                });
            }
            loca
        };
        debug!("{:?}", loca);

        let glyf = {
            let mut glyf = Vec::new();
            let offset = tables.get("glyf").unwrap().2;
            for i in 0..loca.len() - 1 {
                let multiplier = if head.indexToLocFormat == 0 { 2 } else { 1 };
                let locaoffset = loca[i] * multiplier;

                // glyf table offset

                reader.set_position((offset + locaoffset) as u64);
                glyf.push((
                    reader.get_int16(),
                    reader.get_int16(),
                    reader.get_int16(),
                    reader.get_int16(),
                    reader.get_int16(),
                ));
            }
            glyf
        };
        debug!("{:#?}", glyf);

        let cmap = ttf::cmap::new(&mut reader);
        debug!("{:#?}", cmap);

        Ok(())
    }
}

impl Default for Font {
    fn default() -> Self {
        todo!()
    }
}

mod ttf {
    use std::collections::HashMap;

    use log::{info, trace};

    use crate::util::BinaryReader;

    #[derive(Debug, Clone, Copy)]
    pub struct head {
        pub majorVersion: u16,
        pub minorVersion: u16,
        pub fontRevision: i32,
        pub checksumAdjustment: u32,
        pub magicNumber: u32,
        pub flags: u16,
        pub unitsPerEm: u16,
        pub created: std::time::SystemTime,
        pub modified: std::time::SystemTime,
        pub xMin: i16,
        pub yMin: i16,
        pub xMax: i16,
        pub yMax: i16,
        pub macStyle: u16,
        pub lowestRecPPEM: u16,
        pub fontDirectionHint: i16,
        pub indexToLocFormat: i16,
        pub glyphDataFormat: i16,
    }

    impl head {
        pub fn new(reader: &mut BinaryReader) -> Self {
            Self {
                majorVersion: reader.get_uint16(),
                minorVersion: reader.get_uint16(),
                fontRevision: reader.get_fixed(),
                checksumAdjustment: reader.get_uint32(),
                magicNumber: reader.get_uint32(),
                flags: reader.get_uint16(),
                unitsPerEm: reader.get_uint16(),
                created: reader.get_date(),
                modified: reader.get_date(),
                xMin: reader.get_fword(),
                yMin: reader.get_fword(),
                xMax: reader.get_fword(),
                yMax: reader.get_fword(),
                macStyle: reader.get_uint16(),
                lowestRecPPEM: reader.get_uint16(),
                fontDirectionHint: reader.get_int16(),
                indexToLocFormat: reader.get_int16(),
                glyphDataFormat: reader.get_int16(),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct maxp {
        pub version: i32,
        pub numGlyphs: u16,
        pub maxPoints: u16,
        pub maxContours: u16,
        pub maxCompositePoints: u16,
        pub maxCompositeContours: u16,
        pub maxZones: u16,
        pub maxTwilightPoints: u16,
        pub maxStorage: u16,
        pub maxFunctionDefs: u16,
        pub maxInstructionDefs: u16,
        pub maxStackElements: u16,
        pub maxSizeOfInstructions: u16,
        pub maxComponentElements: u16,
        pub maxComponentDepth: u16,
    }

    impl maxp {
        pub fn new(reader: &mut BinaryReader) -> Self {
            Self {
                version: reader.get_fixed(),
                numGlyphs: reader.get_uint16(),
                maxPoints: reader.get_uint16(),
                maxContours: reader.get_uint16(),
                maxCompositePoints: reader.get_uint16(),
                maxCompositeContours: reader.get_uint16(),
                maxZones: reader.get_uint16(),
                maxTwilightPoints: reader.get_uint16(),
                maxStorage: reader.get_uint16(),
                maxFunctionDefs: reader.get_uint16(),
                maxInstructionDefs: reader.get_uint16(),
                maxStackElements: reader.get_uint16(),
                maxSizeOfInstructions: reader.get_uint16(),
                maxComponentElements: reader.get_uint16(),
                maxComponentDepth: reader.get_uint16(),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct hhea {
        version: i32,
        ascent: i16,
        descent: i16,
        lineGap: i16,
        advanceWidthMax: i32,
        minLeftSideBearing: i16,
        minRightSideBearing: i16,
        xMaxExtent: i16,
        caretSlopeRise: i16,
        caretSlopeRun: i16,
        caretOffset: i16,
        metricDataFormat: i16,
        numOfLongHorMetrics: u16,
    }

    impl hhea {
        pub fn new(reader: &mut BinaryReader) -> Self {
            let mut h = hhea {
                version: reader.get_fixed(),
                ascent: reader.get_fword(),
                descent: reader.get_fword(),
                lineGap: reader.get_fword(),
                advanceWidthMax: reader.get_ufword(),
                minLeftSideBearing: reader.get_fword(),
                minRightSideBearing: reader.get_fword(),
                xMaxExtent: reader.get_fword(),
                caretSlopeRise: reader.get_int16(),
                caretSlopeRun: reader.get_int16(),
                caretOffset: reader.get_fword(),
                metricDataFormat: 0,
                numOfLongHorMetrics: 0,
            };
            reader.get_int16();
            reader.get_int16();
            reader.get_int16();
            reader.get_int16();
            h.metricDataFormat = reader.get_int16();
            h.numOfLongHorMetrics = reader.get_uint16();
            h
        }
    }

    #[derive(Debug, Clone)]
    pub struct hmtx {
        hMetrics: Vec<(u16, i16)>,
        leftSideBearing: Vec<i16>,
    }

    impl hmtx {
        pub fn new(reader: &mut BinaryReader, hhea: hhea, maxp: maxp) -> Self {
            let mut hMetrics = Vec::new();
            for _ in 0..hhea.numOfLongHorMetrics {
                hMetrics.push((reader.get_uint16(), reader.get_int16()));
            }
            let mut leftSideBearing = Vec::new();
            for _ in 0..(maxp.numGlyphs - hhea.numOfLongHorMetrics) {
                leftSideBearing.push(reader.get_fword());
            }
            Self {
                hMetrics,
                leftSideBearing,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct cmap {
        version: u16,
        numTables: u16,
        encodingRecords: Vec<(u16, u16, u32)>,
        glyphIndexMap: HashMap<u16, u16>,
    }

    impl cmap {
        pub fn new(reader: &mut BinaryReader) -> cmap {
            let mut c = cmap {
                version: reader.get_uint16(),
                numTables: reader.get_uint16(),
                encodingRecords: Vec::new(),
                glyphIndexMap: HashMap::new(),
            };

            if c.version != 0 {
                info!("{:#?}", c);
                panic!("cmap version should be 0 but is {}!", c.version);
            }

            for _ in 0..c.numTables {
                c.encodingRecords.push((
                    reader.get_uint16(),
                    reader.get_uint16(),
                    reader.get_offset32(),
                ))
            }

            let mut selectedOffset = -1;
            for i in 0..c.numTables as usize {
                let (platformID, encodingID, offset) = c.encodingRecords[i];
                let isWindowsPlatform =
                    platformID == 3 && (encodingID == 0 || encodingID == 1 || encodingID == 10);
                let isUnicodePlatform = platformID == 0
                    && (encodingID == 0
                        || encodingID == 1
                        || encodingID == 2
                        || encodingID == 3
                        || encodingID == 4);
                if (isWindowsPlatform || isUnicodePlatform) {
                    selectedOffset = offset as i32;
                    break;
                }
            }

            if selectedOffset == -1 {
                panic!("The font doesn't contain any recognized platform and encoding.");
            }

            let format = reader.get_uint16();
            if format == 4 {
                c.glyphIndexMap = format4::Format4::new(reader).glyphIndexMap;
            } else {
                panic!("Unsupported format {}. Required: 4", format);
            };

            c
        }
    }

    mod format4 {
        use std::collections::HashMap;

        use crate::util::BinaryReader;

        pub struct Format4 {
            format: u16,
            length: u16,
            language: u16,
            segCountX2: u16,
            searchRange: u16,
            entrySelector: u16,
            rangeShift: u16,
            endCode: Vec<u16>,
            startCode: Vec<u16>,
            idDelta: Vec<i16>,
            idRangeOffset: Vec<u16>,
            pub glyphIndexMap: HashMap<u16, u16>,
        }

        impl Format4 {
            pub fn new(reader: &mut BinaryReader) -> Self {
                let mut f4 = Format4 {
                    format: reader.get_uint16(),
                    length: reader.get_uint16(),
                    language: reader.get_uint16(),
                    segCountX2: reader.get_uint16(),
                    searchRange: reader.get_uint16(),
                    entrySelector: reader.get_uint16(),
                    rangeShift: reader.get_uint16(),
                    endCode: Vec::new(),
                    startCode: Vec::new(),
                    idDelta: Vec::new(),
                    idRangeOffset: Vec::new(),
                    glyphIndexMap: HashMap::new(),
                };

                let segCount = f4.segCountX2 >> 1;

                for _ in 0..segCount {
                    f4.endCode.push(reader.get_uint16());
                }

                reader.get_uint16();

                for _ in 0..segCount {
                    f4.startCode.push(reader.get_uint16());
                }

                for _ in 0..segCount {
                    f4.idDelta.push(reader.get_int16());
                }

                let idRangeOffsetStart = reader.get_position();

                for _ in 0..segCount {
                    f4.idRangeOffset.push(reader.get_uint16());
                }

                for i in 0..segCount {
                    let mut glyphIndex = 0;
                    let endCode = f4.endCode[i as usize];
                    let startCode = f4.startCode[i as usize];
                    let idDelta = f4.idDelta[i as usize];
                    let idRangeOffset = f4.idRangeOffset[i as usize] as u64;

                    for c in startCode..endCode {
                        if idRangeOffset != 0 {
                            let startCodeOffset = (c - startCode) as u64 * 2;
                            let currentRangeOffset = i as u64 * 2;

                            let glyphIndexOffset = idRangeOffsetStart
                                + currentRangeOffset
                                + idRangeOffset
                                + startCodeOffset;

                            reader.set_position(glyphIndexOffset);
                            glyphIndex = reader.get_uint16();

                            if glyphIndex != 0 {
                                glyphIndex = (glyphIndex + idDelta as u16) & 0xffff
                            }
                        } else {
                            glyphIndex = (c + idDelta as u16) & 0xffff
                        }
                        f4.glyphIndexMap.insert(c, glyphIndex);
                    }
                }

                f4
            }
        }
    }
}
