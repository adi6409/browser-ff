//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIDialogParamBlock.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDialogParamBlock",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* int32_t GetInt (in int32_t inIndex); */
                    Method {
                        name: "GetInt",
                        params: &[Param { name: "inIndex", ty: "int32_t" }, Param { name: "_retval", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void SetInt (in int32_t inIndex, in int32_t inInt); */
                    Method {
                        name: "SetInt",
                        params: &[Param { name: "inIndex", ty: "int32_t" }, Param { name: "inInt", ty: "int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void SetNumberStrings (in int32_t inNumStrings); */
                    Method {
                        name: "SetNumberStrings",
                        params: &[Param { name: "inNumStrings", ty: "int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* wstring GetString (in int32_t inIndex); */
                    Method {
                        name: "GetString",
                        params: &[Param { name: "inIndex", ty: "int32_t" }, Param { name: "_retval", ty: "*mut *const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void SetString (in int32_t inIndex, in wstring inString); */
                    Method {
                        name: "SetString",
                        params: &[Param { name: "inIndex", ty: "int32_t" }, Param { name: "inString", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIMutableArray objects; */
                    Method {
                        name: "GetObjects",
                        params: &[Param { name: "aObjects", ty: "*mut*const nsIMutableArray" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetObjects",
                        params: &[Param { name: "aObjects", ty: "*const nsIMutableArray" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

