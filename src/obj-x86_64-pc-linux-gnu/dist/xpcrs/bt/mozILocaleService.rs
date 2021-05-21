//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/locale/mozILocaleService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozILocaleService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString defaultLocale; */
                    Method {
                        name: "GetDefaultLocale",
                        params: &[Param { name: "aDefaultLocale", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString lastFallbackLocale; */
                    Method {
                        name: "GetLastFallbackLocale",
                        params: &[Param { name: "aLastFallbackLocale", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<ACString> appLocalesAsLangTags; */
                    Method {
                        name: "GetAppLocalesAsLangTags",
                        params: &[Param { name: "aAppLocalesAsLangTags", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<ACString> appLocalesAsBCP47; */
                    Method {
                        name: "GetAppLocalesAsBCP47",
                        params: &[Param { name: "aAppLocalesAsBCP47", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<ACString> regionalPrefsLocales; */
                    Method {
                        name: "GetRegionalPrefsLocales",
                        params: &[Param { name: "aRegionalPrefsLocales", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<ACString> webExposedLocales; */
                    Method {
                        name: "GetWebExposedLocales",
                        params: &[Param { name: "aWebExposedLocales", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> negotiateLanguages (in Array<AUTF8String> aRequested, in Array<AUTF8String> aAvailable, [optional] in ACString aDefaultLocale, [optional] in long langNegStrategy); */
                    Method {
                        name: "NegotiateLanguages",
                        params: &[Param { name: "aRequested", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "aAvailable", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "aDefaultLocale", ty: "*const ::nsstring::nsACString" }, Param { name: "langNegStrategy", ty: "i32" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString appLocaleAsLangTag; */
                    Method {
                        name: "GetAppLocaleAsLangTag",
                        params: &[Param { name: "aAppLocaleAsLangTag", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString appLocaleAsBCP47; */
                    Method {
                        name: "GetAppLocaleAsBCP47",
                        params: &[Param { name: "aAppLocaleAsBCP47", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute Array<ACString> requestedLocales; */
                    Method {
                        name: "GetRequestedLocales",
                        params: &[Param { name: "aRequestedLocales", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetRequestedLocales",
                        params: &[Param { name: "aRequestedLocales", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString requestedLocale; */
                    Method {
                        name: "GetRequestedLocale",
                        params: &[Param { name: "aRequestedLocale", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute Array<ACString> availableLocales; */
                    Method {
                        name: "GetAvailableLocales",
                        params: &[Param { name: "aAvailableLocales", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetAvailableLocales",
                        params: &[Param { name: "aAvailableLocales", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isAppLocaleRTL; */
                    Method {
                        name: "GetIsAppLocaleRTL",
                        params: &[Param { name: "aIsAppLocaleRTL", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<ACString> packagedLocales; */
                    Method {
                        name: "GetPackagedLocales",
                        params: &[Param { name: "aPackagedLocales", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

