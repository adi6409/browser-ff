//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIVariant.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIVariant",
            base: Some("nsISupports"),
            methods: Err("nostdcall is unsupported"),
        },

        Interface {
            name: "nsIWritableVariant",
            base: Some("nsIVariant"),
            methods: Ok(&[
                    /* attribute boolean writable; */
                    Method {
                        name: "GetWritable",
                        params: &[Param { name: "aWritable", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetWritable",
                        params: &[Param { name: "aWritable", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsInt8 (in uint8_t aValue); */
                    Method {
                        name: "SetAsInt8",
                        params: &[Param { name: "aValue", ty: "uint8_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsInt16 (in int16_t aValue); */
                    Method {
                        name: "SetAsInt16",
                        params: &[Param { name: "aValue", ty: "int16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsInt32 (in int32_t aValue); */
                    Method {
                        name: "SetAsInt32",
                        params: &[Param { name: "aValue", ty: "int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsInt64 (in int64_t aValue); */
                    Method {
                        name: "SetAsInt64",
                        params: &[Param { name: "aValue", ty: "int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsUint8 (in uint8_t aValue); */
                    Method {
                        name: "SetAsUint8",
                        params: &[Param { name: "aValue", ty: "uint8_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsUint16 (in uint16_t aValue); */
                    Method {
                        name: "SetAsUint16",
                        params: &[Param { name: "aValue", ty: "uint16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsUint32 (in uint32_t aValue); */
                    Method {
                        name: "SetAsUint32",
                        params: &[Param { name: "aValue", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsUint64 (in uint64_t aValue); */
                    Method {
                        name: "SetAsUint64",
                        params: &[Param { name: "aValue", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsFloat (in float aValue); */
                    Method {
                        name: "SetAsFloat",
                        params: &[Param { name: "aValue", ty: "libc::c_float" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsDouble (in double aValue); */
                    Method {
                        name: "SetAsDouble",
                        params: &[Param { name: "aValue", ty: "libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsBool (in boolean aValue); */
                    Method {
                        name: "SetAsBool",
                        params: &[Param { name: "aValue", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsChar (in char aValue); */
                    Method {
                        name: "SetAsChar",
                        params: &[Param { name: "aValue", ty: "libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsWChar (in wchar aValue); */
                    Method {
                        name: "SetAsWChar",
                        params: &[Param { name: "aValue", ty: "i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsID (in nsIDRef aValue); */
                    Method {
                        name: "SetAsID",
                        params: &[Param { name: "aValue", ty: "*const nsID" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsAString (in AString aValue); */
                    Method {
                        name: "SetAsAString",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsACString (in ACString aValue); */
                    Method {
                        name: "SetAsACString",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsAUTF8String (in AUTF8String aValue); */
                    Method {
                        name: "SetAsAUTF8String",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsString (in string aValue); */
                    Method {
                        name: "SetAsString",
                        params: &[Param { name: "aValue", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsWString (in wstring aValue); */
                    Method {
                        name: "SetAsWString",
                        params: &[Param { name: "aValue", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsISupports (in nsISupports aValue); */
                    Method {
                        name: "SetAsISupports",
                        params: &[Param { name: "aValue", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsInterface (in nsIIDRef iid, [iid_is (iid)] in nsQIResult iface); */
                    Method {
                        name: "SetAsInterface",
                        params: &[Param { name: "iid", ty: "*const nsIID" }, Param { name: "iface", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void setAsArray (in uint16_t type, in nsIIDPtr iid, in uint32_t count, in voidPtr ptr); */
                    Method {
                        name: "SetAsArray",
                        params: &[Param { name: "type_", ty: "uint16_t" }, Param { name: "iid", ty: "*const nsIID" }, Param { name: "count", ty: "uint32_t" }, Param { name: "ptr", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsStringWithSize (in uint32_t size, [size_is (size)] in string str); */
                    Method {
                        name: "SetAsStringWithSize",
                        params: &[Param { name: "size", ty: "uint32_t" }, Param { name: "str", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsWStringWithSize (in uint32_t size, [size_is (size)] in wstring str); */
                    Method {
                        name: "SetAsWStringWithSize",
                        params: &[Param { name: "size", ty: "uint32_t" }, Param { name: "str", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsVoid (); */
                    Method {
                        name: "SetAsVoid",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsEmpty (); */
                    Method {
                        name: "SetAsEmpty",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void setAsEmptyArray (); */
                    Method {
                        name: "SetAsEmptyArray",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void setFromVariant (in nsIVariant aValue); */
                    Method {
                        name: "SetFromVariant",
                        params: &[Param { name: "aValue", ty: "*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

