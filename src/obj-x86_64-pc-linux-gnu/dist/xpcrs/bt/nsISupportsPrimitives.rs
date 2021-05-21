//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsISupportsPrimitives.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISupportsPrimitive",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned short type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsID",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute nsIDPtr data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut *mut nsID" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "*const nsID" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsCString",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute ACString data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsString",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute AString data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* wstring toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsPRBool",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute boolean data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsPRUint8",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute uint8_t data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut uint8_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "uint8_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsPRUint16",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute uint16_t data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut uint16_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "uint16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsPRUint32",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute uint32_t data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsPRUint64",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute uint64_t data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsPRTime",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute PRTime data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsChar",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute char data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsPRInt16",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute int16_t data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut int16_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "int16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsPRInt32",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute int32_t data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsPRInt64",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute int64_t data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut int64_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsFloat",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute float data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut libc::c_float" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "libc::c_float" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsDouble",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute double data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsInterfacePointer",
            base: Some("nsISupportsPrimitive"),
            methods: Ok(&[
                    /* attribute nsISupports data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetData",
                        params: &[Param { name: "aData", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIDPtr dataIID; */
                    Method {
                        name: "GetDataIID",
                        params: &[Param { name: "aDataIID", ty: "*mut *mut nsID" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDataIID",
                        params: &[Param { name: "aDataIID", ty: "*const nsID" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

