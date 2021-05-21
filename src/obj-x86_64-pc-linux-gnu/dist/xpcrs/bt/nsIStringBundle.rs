//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/strres/nsIStringBundle.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStringBundle",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* AString GetStringFromID (in long aID); */
                    Method {
                        name: "GetStringFromID",
                        params: &[Param { name: "aID", ty: "i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [binaryname(GetStringFromAUTF8Name)] AString GetStringFromName (in AUTF8String aName); */
                    Method {
                        name: "GetStringFromAUTF8Name",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [binaryname(GetStringFromName),noscript] AString GetStringFromNameCpp (in string aName); */
                    Method {
                        name: "GetStringFromName",
                        params: &[Param { name: "aName", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString formatStringFromID (in long aID, in Array<AString> params); */
                    Method {
                        name: "FormatStringFromID",
                        params: &[Param { name: "aID", ty: "i32" }, Param { name: "params", ty: "*const thin_vec::ThinVec<::nsstring::nsString>" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [binaryname(FormatStringFromAUTF8Name)] AString formatStringFromName (in AUTF8String aName, in Array<AString> params); */
                    Method {
                        name: "FormatStringFromAUTF8Name",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsACString" }, Param { name: "params", ty: "*const thin_vec::ThinVec<::nsstring::nsString>" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [binaryname(FormatStringFromName),noscript] AString formatStringFromNameCpp (in string aName, in Array<AString> params); */
                    Method {
                        name: "FormatStringFromName",
                        params: &[Param { name: "aName", ty: "*const libc::c_char" }, Param { name: "params", ty: "*const thin_vec::ThinVec<::nsstring::nsString>" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISimpleEnumerator getSimpleEnumeration (); */
                    Method {
                        name: "GetSimpleEnumeration",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncPreload (); */
                    Method {
                        name: "AsyncPreload",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIStringBundleService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIStringBundle createBundle (in string aURLSpec); */
                    Method {
                        name: "CreateBundle",
                        params: &[Param { name: "aURLSpec", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIStringBundle" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString formatStatusMessage (in nsresult aStatus, in wstring aStatusArg); */
                    Method {
                        name: "FormatStatusMessage",
                        params: &[Param { name: "aStatus", ty: "::nserror::nsresult" }, Param { name: "aStatusArg", ty: "*const i16" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void flushBundles (); */
                    Method {
                        name: "FlushBundles",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

