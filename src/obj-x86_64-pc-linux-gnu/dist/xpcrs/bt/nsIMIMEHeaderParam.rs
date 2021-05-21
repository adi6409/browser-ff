//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/mime/nsIMIMEHeaderParam.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMIMEHeaderParam",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* AString getParameter (in ACString aHeaderVal, in string aParamName, in ACString aFallbackCharset, in boolean aTryLocaleCharset, out string aLang); */
                    Method {
                        name: "GetParameter",
                        params: &[Param { name: "aHeaderVal", ty: "*const ::nsstring::nsACString" }, Param { name: "aParamName", ty: "*const libc::c_char" }, Param { name: "aFallbackCharset", ty: "*const ::nsstring::nsACString" }, Param { name: "aTryLocaleCharset", ty: "bool" }, Param { name: "aLang", ty: "*mut *const libc::c_char" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getParameterHTTP (in ACString aHeaderVal, in string aParamName, in ACString aFallbackCharset, in boolean aTryLocaleCharset, out string aLang); */
                    Method {
                        name: "GetParameterHTTP",
                        params: &[Param { name: "aHeaderVal", ty: "*const ::nsstring::nsACString" }, Param { name: "aParamName", ty: "*const libc::c_char" }, Param { name: "aFallbackCharset", ty: "*const ::nsstring::nsACString" }, Param { name: "aTryLocaleCharset", ty: "bool" }, Param { name: "aLang", ty: "*mut *const libc::c_char" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString decodeRFC5987Param (in ACString aParamVal, out ACString aLang); */
                    Method {
                        name: "DecodeRFC5987Param",
                        params: &[Param { name: "aParamVal", ty: "*const ::nsstring::nsACString" }, Param { name: "aLang", ty: "*mut ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] string getParameterInternal (in string aHeaderVal, in string aParamName, out string aCharset, out string aLang); */
                    Method {
                        name: "GetParameterInternal",
                        params: &[Param { name: "aHeaderVal", ty: "*const libc::c_char" }, Param { name: "aParamName", ty: "*const libc::c_char" }, Param { name: "aCharset", ty: "*mut *const libc::c_char" }, Param { name: "aLang", ty: "*mut *const libc::c_char" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] ACString decodeRFC2047Header (in string aHeaderVal, in string aDefaultCharset, in boolean aOverrideCharset, in boolean aEatContinuation); */
                    Method {
                        name: "DecodeRFC2047Header",
                        params: &[Param { name: "aHeaderVal", ty: "*const libc::c_char" }, Param { name: "aDefaultCharset", ty: "*const libc::c_char" }, Param { name: "aOverrideCharset", ty: "bool" }, Param { name: "aEatContinuation", ty: "bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] ACString decodeParameter (in ACString aParamValue, in string aCharset, in string aDefaultCharset, in boolean aOverrideCharset); */
                    Method {
                        name: "DecodeParameter",
                        params: &[Param { name: "aParamValue", ty: "*const ::nsstring::nsACString" }, Param { name: "aCharset", ty: "*const libc::c_char" }, Param { name: "aDefaultCharset", ty: "*const libc::c_char" }, Param { name: "aOverrideCharset", ty: "bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

