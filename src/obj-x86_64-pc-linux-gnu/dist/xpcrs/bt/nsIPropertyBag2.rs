//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIPropertyBag2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPropertyBag2",
            base: Some("nsIPropertyBag"),
            methods: Ok(&[
                    /* int32_t getPropertyAsInt32 (in AString prop); */
                    Method {
                        name: "GetPropertyAsInt32",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* uint32_t getPropertyAsUint32 (in AString prop); */
                    Method {
                        name: "GetPropertyAsUint32",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* int64_t getPropertyAsInt64 (in AString prop); */
                    Method {
                        name: "GetPropertyAsInt64",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* uint64_t getPropertyAsUint64 (in AString prop); */
                    Method {
                        name: "GetPropertyAsUint64",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* double getPropertyAsDouble (in AString prop); */
                    Method {
                        name: "GetPropertyAsDouble",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getPropertyAsAString (in AString prop); */
                    Method {
                        name: "GetPropertyAsAString",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getPropertyAsACString (in AString prop); */
                    Method {
                        name: "GetPropertyAsACString",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getPropertyAsAUTF8String (in AString prop); */
                    Method {
                        name: "GetPropertyAsAUTF8String",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean getPropertyAsBool (in AString prop); */
                    Method {
                        name: "GetPropertyAsBool",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getPropertyAsInterface (in AString prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
                    Method {
                        name: "GetPropertyAsInterface",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "iid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIVariant get (in AString prop); */
                    Method {
                        name: "Get",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean hasKey (in AString prop); */
                    Method {
                        name: "HasKey",
                        params: &[Param { name: "prop", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

