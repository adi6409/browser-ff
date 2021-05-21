//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/uconv/nsIScriptableUConv.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptableUnicodeConverter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString ConvertFromUnicode (in AString aSrc); */
                    Method {
                        name: "ConvertFromUnicode",
                        params: &[Param { name: "aSrc", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString Finish (); */
                    Method {
                        name: "Finish",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString ConvertToUnicode (in ACString aSrc); */
                    Method {
                        name: "ConvertToUnicode",
                        params: &[Param { name: "aSrc", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void convertToByteArray (in AString aString, [optional] out unsigned long aLen, [array, size_is (aLen), retval] out octet aData); */
                    Method {
                        name: "ConvertToByteArray",
                        params: &[Param { name: "aString", ty: "*const ::nsstring::nsAString" }, Param { name: "aLen", ty: "*mut u32" }, Param { name: "aData", ty: "*mut *mut u8" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIInputStream convertToInputStream (in AString aString); */
                    Method {
                        name: "ConvertToInputStream",
                        params: &[Param { name: "aString", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute ACString charset; */
                    Method {
                        name: "GetCharset",
                        params: &[Param { name: "aCharset", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCharset",
                        params: &[Param { name: "aCharset", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean isInternal; */
                    Method {
                        name: "GetIsInternal",
                        params: &[Param { name: "aIsInternal", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetIsInternal",
                        params: &[Param { name: "aIsInternal", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

