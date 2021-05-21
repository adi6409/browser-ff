//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIWritablePropertyBag2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWritablePropertyBag2",
            base: Some("nsIPropertyBag2"),
            methods: Ok(&[
                    /* void setPropertyAsInt32 (in AString prop, in int32_t value); */
                    Method {
                        name: "SetPropertyAsInt32",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPropertyAsUint32 (in AString prop, in uint32_t value); */
                    Method {
                        name: "SetPropertyAsUint32",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPropertyAsInt64 (in AString prop, in int64_t value); */
                    Method {
                        name: "SetPropertyAsInt64",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPropertyAsUint64 (in AString prop, in uint64_t value); */
                    Method {
                        name: "SetPropertyAsUint64",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPropertyAsDouble (in AString prop, in double value); */
                    Method {
                        name: "SetPropertyAsDouble",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPropertyAsAString (in AString prop, in AString value); */
                    Method {
                        name: "SetPropertyAsAString",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPropertyAsACString (in AString prop, in ACString value); */
                    Method {
                        name: "SetPropertyAsACString",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPropertyAsAUTF8String (in AString prop, in AUTF8String value); */
                    Method {
                        name: "SetPropertyAsAUTF8String",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPropertyAsBool (in AString prop, in boolean value); */
                    Method {
                        name: "SetPropertyAsBool",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPropertyAsInterface (in AString prop, in nsISupports value); */
                    Method {
                        name: "SetPropertyAsInterface",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

