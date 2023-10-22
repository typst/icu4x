// @generated
/// Implement `DataProvider<BuddhistYearSymbolsV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_datetime_symbols_buddhist_years_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker>, icu_provider::DataError> {
                static BR_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0A.B.") })
                });
                static BR_X_4: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0AB") })
                });
                static DE_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0B.E.") })
                });
                static IS_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0BD") })
                });
                static UND_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0BE") })
                });
                static EU_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0BG") })
                });
                static ET_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0BK") })
                });
                static SV_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0Buddhistisk era") })
                });
                static FR_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0E. B.") })
                });
                static SC_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0E.B.") })
                });
                static AST_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0EB") })
                });
                static ID_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0Era Buddhis") })
                });
                static GA_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0RB") })
                });
                static FR_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC3\xA8re bouddhique") })
                });
                static EL_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xCE\x92.\xCE\x95.") })
                });
                static BS_CYRL_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\x91\xD0\x95") })
                });
                static UK_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xB1. \xD0\xB5.") })
                });
                static UK_X_4: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xB1.\xD0\xB5.") })
                });
                static BE_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xB1.\xD1\x8D.") })
                });
                static RU_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xB1\xD1\x83\xD0\xB4\xD0\xB4\xD0\xB8\xD0\xB9\xD1\x81\xD0\xBA\xD0\xB0\xD1\x8F \xD1\x8D\xD1\x80\xD0\xB0") })
                });
                static RU_X_4: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xB1\xD1\x8D") })
                });
                static HE_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD7\x94\xD7\xA1\xD7\xA4\xD7\x99\xD7\xA8\xD7\x94 \xD7\x94\xD7\x91\xD7\x95\xD7\x93\xD7\x94\xD7\x99\xD7\xA1\xD7\x98\xD7\x99\xD7\xAA") })
                });
                static AR_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD8\xA7\xD9\x84\xD8\xAA\xD9\x82\xD9\x88\xD9\x8A\xD9\x85 \xD8\xA7\xD9\x84\xD8\xA8\xD9\x88\xD8\xB0\xD9\x8A") })
                });
                static FA_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD8\xAA\xD9\x82\xD9\x88\xDB\x8C\xD9\x85 \xD8\xA8\xD9\x88\xD8\xAF\xD8\xA7\xDB\x8C\xDB\x8C") })
                });
                static MR_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\x87\xE0\xA4\xB8\xE0\xA4\xAA\xE0\xA5\x82.") })
                });
                static MR_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\x87\xE0\xA4\xB8\xE0\xA4\xB5\xE0\xA5\x80\xE0\xA4\xB8\xE0\xA4\xA8 \xE0\xA4\xAA\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5") })
                });
                static HI_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\xAC\xE0\xA5\x8C\xE0\xA4\xA6\xE0\xA5\x8D\xE0\xA4\xA7 \xE0\xA4\xB8\xE0\xA4\x82\xE0\xA4\xB5\xE0\xA4\xA4") })
                });
                static PA_X_4: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA8\x88. \xE0\xA8\xAA\xE0\xA9\x82.") })
                });
                static PA_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA8\x88\xE0\xA8\xB8\xE0\xA8\xB5\xE0\xA9\x80 \xE0\xA8\xAA\xE0\xA9\x82\xE0\xA8\xB0\xE0\xA8\xB5") })
                });
                static TH_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB8\x9E.\xE0\xB8\xA8.") })
                });
                static TH_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB8\x9E\xE0\xB8\xB8\xE0\xB8\x97\xE0\xB8\x98\xE0\xB8\xA8\xE0\xB8\xB1\xE0\xB8\x81\xE0\xB8\xA3\xE0\xB8\xB2\xE0\xB8\x8A") })
                });
                static LO_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xBA\x9E.\xE0\xBA\xAA.") })
                });
                static JA_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE4\xBB\x8F\xE6\x9A\xA6") })
                });
                static YUE_HANS_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE4\xBD\x9B\xE5\x8E\x86") })
                });
                static YUE_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE4\xBD\x9B\xE6\x9B\x86") })
                });
                static KO_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xEB\xB6\x88\xEA\xB8\xB0") })
                });
                static FF_ADLM_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xF0\x9E\xA4\x98\xF0\x9E\xA4\x84") })
                });
                static FF_ADLM_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xF0\x9E\xA4\x98\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xAA\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xA4 \xF0\x9E\xA4\x84\xF0\x9E\xA4\xB5\xF0\x9E\xA5\x85\xF0\x9E\xA4\xA3\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xB3\xF0\x9E\xA4\xAE\xF0\x9E\xA4\xB1\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xA4") })
                });
                static BR_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0amzervezh voudaek") })
                });
                static IS_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0b\xC3\xBAddhadagatal") })
                });
                static SL_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0bud. kol.") })
                });
                static FI_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0buddhalainen aika") })
                });
                static SL_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0budisti\xC4\x8Dni koledar") })
                });
                static LV_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0budistu \xC4\x93ra") })
                });
                static PL_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0e.b.") })
                });
                static CA_X_3: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0eB") })
                });
                static SC_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0era buddhista") })
                });
                static RO_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0era budist\xC4\x83") })
                });
                static AST_X_5: <icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0be") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0era budista") })
                });
                static VALUES: [&<icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable; 114usize] = [&AR_X_5, &AST_X_3, &AST_X_3, &AST_X_5, &BE_X_3, &BE_X_3, &BE_X_3, &BR_X_3, &BR_X_4, &BR_X_5, &BS_CYRL_X_3, &BS_CYRL_X_3, &BS_CYRL_X_3, &CA_X_3, &CA_X_3, &CA_X_3, &DE_X_5, &EL_X_5, &ET_X_3, &ET_X_3, &ET_X_3, &EU_X_3, &EU_X_3, &EU_X_3, &FA_X_3, &FA_X_3, &FA_X_3, &FF_ADLM_X_3, &FF_ADLM_X_3, &FF_ADLM_X_5, &FI_X_5, &FR_X_3, &AST_X_3, &FR_X_5, &GA_X_3, &GA_X_3, &GA_X_3, &HE_X_5, &HI_X_3, &HI_X_3, &HI_X_3, &ET_X_3, &ET_X_3, &ET_X_3, &AST_X_3, &AST_X_3, &ID_X_5, &IS_X_3, &IS_X_3, &IS_X_5, &AST_X_3, &AST_X_3, &AST_X_3, &JA_X_5, &AST_X_3, &AST_X_3, &KO_X_3, &KO_X_3, &KO_X_3, &LO_X_3, &LO_X_3, &LO_X_3, &DE_X_5, &DE_X_5, &LV_X_5, &MR_X_3, &MR_X_3, &MR_X_5, &PA_X_3, &PA_X_4, &PA_X_3, &PL_X_3, &PL_X_3, &PL_X_3, &AST_X_3, &AST_X_3, &PL_X_3, &PL_X_3, &RO_X_5, &RU_X_4, &RU_X_5, &SC_X_3, &AST_X_3, &SC_X_5, &SL_X_3, &ET_X_3, &SL_X_5, &BS_CYRL_X_3, &BS_CYRL_X_3, &BS_CYRL_X_3, &SV_X_5, &TH_X_3, &TH_X_3, &TH_X_5, &UK_X_3, &UK_X_4, &UK_X_3, &UND_X_3, &UND_X_3, &UND_X_3, &AST_X_3, &AST_X_3, &YUE_HANS_X_3, &YUE_HANS_X_3, &YUE_HANS_X_3, &YUE_X_3, &YUE_X_3, &YUE_X_3, &YUE_X_3, &YUE_X_3, &YUE_X_3, &YUE_HANS_X_3, &YUE_HANS_X_3, &YUE_HANS_X_3];
                static KEYS: [&str; 114usize] = ["ar-x-5", "ast-x-3", "ast-x-4", "ast-x-5", "be-x-3", "be-x-4", "be-x-5", "br-x-3", "br-x-4", "br-x-5", "bs-Cyrl-x-3", "bs-Cyrl-x-4", "bs-Cyrl-x-5", "ca-x-3", "ca-x-4", "ca-x-5", "de-x-5", "el-x-5", "et-x-3", "et-x-4", "et-x-5", "eu-x-3", "eu-x-4", "eu-x-5", "fa-x-3", "fa-x-4", "fa-x-5", "ff-Adlm-x-3", "ff-Adlm-x-4", "ff-Adlm-x-5", "fi-x-5", "fr-x-3", "fr-x-4", "fr-x-5", "ga-x-3", "ga-x-4", "ga-x-5", "he-x-5", "hi-x-3", "hi-x-4", "hi-x-5", "hu-x-3", "hu-x-4", "hu-x-5", "id-x-3", "id-x-4", "id-x-5", "is-x-3", "is-x-4", "is-x-5", "it-x-3", "it-x-4", "it-x-5", "ja-x-5", "kgp-x-4", "kgp-x-5", "ko-x-3", "ko-x-4", "ko-x-5", "lo-x-3", "lo-x-4", "lo-x-5", "lv-x-3", "lv-x-4", "lv-x-5", "mr-x-3", "mr-x-4", "mr-x-5", "pa-x-3", "pa-x-4", "pa-x-5", "pl-x-3", "pl-x-4", "pl-x-5", "pt-x-4", "pt-x-5", "ro-x-3", "ro-x-4", "ro-x-5", "ru-x-4", "ru-x-5", "sc-x-3", "sc-x-4", "sc-x-5", "sl-x-3", "sl-x-4", "sl-x-5", "sr-x-3", "sr-x-4", "sr-x-5", "sv-x-5", "th-x-3", "th-x-4", "th-x-5", "uk-x-3", "uk-x-4", "uk-x-5", "und-x-3", "und-x-4", "und-x-5", "yrl-x-4", "yrl-x-5", "yue-Hans-x-3", "yue-Hans-x-4", "yue-Hans-x-5", "yue-x-3", "yue-x-4", "yue-x-5", "zh-Hant-x-3", "zh-Hant-x-4", "zh-Hant-x-5", "zh-x-3", "zh-x-4", "zh-x-5"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
                    let mut fallback_iterator = FALLBACKER.fallback_for(req.locale.clone());
                    loop {
                        if fallback_iterator.get().is_und() {
                            return Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu::datetime::provider::neo::BuddhistYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY, req));
                        }
                        if let Ok(payload) = KEYS.binary_search_by(|k| fallback_iterator.get().strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                            metadata.locale = Some(fallback_iterator.take());
                            break payload;
                        }
                        fallback_iterator.step();
                    }
                };
                Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata })
            }
        }
    };
}
