//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/locale/mozIOSPreferences.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIOSPreferences",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute Array<ACString> systemLocales; */
                    Method {
                        name: "GetSystemLocales",
                        params: &[Param { name: "aSystemLocales", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<ACString> regionalPrefsLocales; */
                    Method {
                        name: "GetRegionalPrefsLocales",
                        params: &[Param { name: "aRegionalPrefsLocales", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString systemLocale; */
                    Method {
                        name: "GetSystemLocale",
                        params: &[Param { name: "aSystemLocale", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getDateTimePattern (in long timeFormatStyle, in long dateFormatStyle, [optional] in ACString locale); */
                    Method {
                        name: "GetDateTimePattern",
                        params: &[Param { name: "timeFormatStyle", ty: "i32" }, Param { name: "dateFormatStyle", ty: "i32" }, Param { name: "locale", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

