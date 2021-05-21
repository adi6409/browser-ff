//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsICommandParams.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICommandParams",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* short getValueType (in string name); */
                    Method {
                        name: "GetValueType",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean getBooleanValue (in string name); */
                    Method {
                        name: "GetBooleanValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getLongValue (in string name); */
                    Method {
                        name: "GetLongValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* double getDoubleValue (in string name); */
                    Method {
                        name: "GetDoubleValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getStringValue (in string name); */
                    Method {
                        name: "GetStringValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getCStringValue (in string name); */
                    Method {
                        name: "GetCStringValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISupports getISupportsValue (in string name); */
                    Method {
                        name: "GetISupportsValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setBooleanValue (in string name, in boolean value); */
                    Method {
                        name: "SetBooleanValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setLongValue (in string name, in long value); */
                    Method {
                        name: "SetLongValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setDoubleValue (in string name, in double value); */
                    Method {
                        name: "SetDoubleValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setStringValue (in string name, in AString value); */
                    Method {
                        name: "SetStringValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setCStringValue (in string name, in ACString value); */
                    Method {
                        name: "SetCStringValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setISupportsValue (in string name, in nsISupports value); */
                    Method {
                        name: "SetISupportsValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeValue (in string name); */
                    Method {
                        name: "RemoveValue",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

